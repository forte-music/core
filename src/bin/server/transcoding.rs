extern crate ffmpeg;

use actix_web;
use actix_web::Path;
use actix_web::State;
use forte_core::models::Song;
use server::graphql::AppState;
use std::path;

use self::ffmpeg::codec;
use self::ffmpeg::filter;
use self::ffmpeg::format;
use self::ffmpeg::format::context::IOContextWrite;
use self::ffmpeg::frame;
use self::ffmpeg::media;
use actix_web::error;
use diesel;
use server::stream::RangeStream;
use std::io::Cursor;
use std::ops::Deref;
use uuid::Uuid;

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub fn handler(
    state: State<AppState>,
    song_id: Path<Uuid>,
) -> actix_web::Result<RangeStream<Cursor<Vec<u8>>>> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context, &song_id.into_inner().into()).map_err(convert_diesel_err)?;
    let song_path = song.path.deref();

    let mut transcode_stream = TranscodeStream::open(song_path, 320_000).unwrap();
    let cursor = transcode_stream.transcode_all().unwrap();

    // TODO: Remove Clone
    let transcoded_vec = cursor.get_ref().clone();
    let size = transcoded_vec.len();

    Ok(RangeStream::new(Cursor::new(transcoded_vec), size as u64))
}

pub struct TranscodeStream {
    input_context: Option<format::context::Input>,
    encoder: codec::encoder::Audio,
    input_stream_idx: usize,

    output_context_stream: format::context::OutputStream<Cursor<Vec<u8>>>,
    decoder: codec::decoder::Audio,

    filter_graph: filter::Graph,
}

// TODO: Remove Expects
// TODO: Remove Unwraps
impl TranscodeStream {
    fn make_decoder(
        input_stream: &format::stream::Stream,
    ) -> Result<codec::decoder::Audio, ffmpeg::Error> {
        let mut decoder = input_stream.codec().decoder().audio()?;
        decoder.set_parameters(input_stream.parameters())?;

        Ok(decoder)
    }

    fn make_encoder<P: AsRef<path::Path>>(
        output_context: &mut format::context::Output,
        decoder: &codec::decoder::Audio,
        bit_rate: usize,
        path: &P,
    ) -> Result<codec::encoder::Audio, ffmpeg::Error> {
        let encoder_codec = ffmpeg::encoder::find(
            output_context.format().codec(path, media::Type::Audio),
        ).expect("failed to find encoder")
            .audio()?;

        let global = output_context
            .format()
            .flags()
            .contains(ffmpeg::format::flag::GLOBAL_HEADER);

        let mut output_stream = output_context.add_stream(encoder_codec)?;
        let mut encoder = output_stream.codec().encoder().audio()?;

        let channel_layout = encoder_codec
            .channel_layouts()
            .map(|cls| cls.best(decoder.channel_layout().channels()))
            .unwrap_or(ffmpeg::channel_layout::STEREO);

        if global {
            encoder.set_flags(ffmpeg::codec::flag::GLOBAL_HEADER);
        }

        encoder.set_rate(decoder.rate() as i32);
        encoder.set_channel_layout(channel_layout);
        encoder.set_channels(channel_layout.channels());
        encoder.set_format(
            encoder_codec
                .formats()
                .expect("unknown supported formats")
                .next()
                .unwrap(),
        );
        encoder.set_bit_rate(bit_rate);

        encoder.set_time_base((1, decoder.rate() as i32));
        output_stream.set_time_base((1, decoder.rate() as i32));

        let encoder = encoder.open_as(encoder_codec)?;
        output_stream.set_parameters(&encoder);

        Ok(encoder)
    }

    fn make_filter_graph(
        decoder: &codec::decoder::Audio,
        encoder: &codec::encoder::Audio,
    ) -> Result<filter::Graph, ffmpeg::Error> {
        let mut graph = filter::Graph::new();

        Self::make_and_add_filter_input(&mut graph, decoder)?;
        Self::make_and_add_filter_output(&mut graph, encoder)?;

        graph.output("in", 0)?.input("out", 0)?.parse("anull")?;
        graph.validate()?;

        if let Some(codec) = encoder.codec() {
            if !codec
                .capabilities()
                .contains(ffmpeg::codec::capabilities::VARIABLE_FRAME_SIZE)
            {
                graph
                    .get("out")
                    .unwrap()
                    .sink()
                    .set_frame_size(encoder.frame_size());
            }
        }

        Ok(graph)
    }

    fn make_and_add_filter_input(
        graph: &mut filter::Graph,
        decoder: &codec::decoder::Audio,
    ) -> Result<(), ffmpeg::Error> {
        let args = format!(
            "time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
            decoder.time_base(),
            decoder.rate(),
            decoder.format().name(),
            decoder.channel_layout().bits()
        );
        let filter_source = filter::find("abuffer").unwrap();

        graph.add(&filter_source, "in", &args)?;

        Ok(())
    }

    fn make_and_add_filter_output(
        graph: &mut filter::Graph,
        encoder: &codec::encoder::Audio,
    ) -> Result<(), ffmpeg::Error> {
        let filter_source = filter::find("abuffersink").unwrap();
        graph.add(&filter_source, "out", "")?;

        let mut filter = graph.get("out").unwrap();
        filter.set_sample_format(encoder.format());
        filter.set_channel_layout(encoder.channel_layout());
        filter.set_sample_rate(encoder.rate());

        Ok(())
    }

    pub fn open<T: AsRef<path::Path>>(
        input_path: &T,
        bit_rate: usize,
    ) -> Result<TranscodeStream, ffmpeg::Error> {
        let input_context = format::input(input_path)?;
        let output_cursor = Cursor::new(Vec::new());

        let output_wrapper = IOContextWrite::new(output_cursor, 32 * 1024);
        let mut output_context_stream = format::output_stream(output_wrapper, "mp3")?;

        let (decoder, input_stream_idx) = {
            // Find Best Audio Stream
            let input_stream = input_context
                .streams()
                .best(media::Type::Audio)
                .expect("could not find best audio stream");

            (Self::make_decoder(&input_stream)?, input_stream.index())
        };

        let encoder = Self::make_encoder(
            &mut output_context_stream.inner_mut(),
            &decoder,
            bit_rate,
            input_path,
        )?;

        let filter_graph = Self::make_filter_graph(&decoder, &encoder)?;

        let ret = TranscodeStream {
            input_context: Some(input_context),
            decoder,
            input_stream_idx,
            output_context_stream,
            encoder,
            filter_graph,
        };

        Ok(ret)
    }

    fn write_packet(
        output_context: &mut format::context::Output,
        encoded_packet: &mut ffmpeg::Packet,
        decoder_time_base: ffmpeg::Rational,
        encoder_time_base: ffmpeg::Rational,
    ) -> Result<(), ffmpeg::Error> {
        encoded_packet.set_stream(0);
        encoded_packet.rescale_ts(decoder_time_base, encoder_time_base);
        encoded_packet.write_interleaved(output_context)?;

        Ok(())
    }

    fn write_all_filtered(
        &mut self,
        decoder_time_base: ffmpeg::Rational,
        decoded_frame: &mut frame::Audio,
        encoder_time_base: ffmpeg::Rational,
        encoded_packet: &mut ffmpeg::packet::Packet,
    ) -> Result<(), ffmpeg::Error> {
        // The error here is suppressed because ffmpeg doesn't wrap errors correctly. If there
        // is an AVERROR(EAGAIN) or AVERROR_EOF return code from av_buffersink_get_frame, this
        // loop should stop, it currently stops on any error without propagating it.
        while let Ok(..) = self.filter_graph
            .get("out")
            .unwrap()
            .sink()
            .frame(decoded_frame)
        {
            if !self.encoder.encode(&decoded_frame, encoded_packet)? {
                continue;
            }

            Self::write_packet(
                self.output_context_stream.inner_mut(),
                encoded_packet,
                decoder_time_base,
                encoder_time_base,
            )?;
        }

        Ok(())
    }

    fn transcode_all(&mut self) -> Result<&Cursor<Vec<u8>>, ffmpeg::Error> {
        self.output_context_stream.inner_mut().write_header()?;

        let decoder_time_base = self.decoder.time_base();
        let encoder_time_base = self.output_context_stream
            .inner_mut()
            .stream(0)
            .unwrap()
            .time_base();

        let mut decoded_frame = frame::Audio::empty();
        let mut encoded_packet = ffmpeg::Packet::empty();

        let mut input_context = self.input_context.take().unwrap();

        for (input_stream, mut decoded_packet) in input_context.packets() {
            if input_stream.index() != self.input_stream_idx {
                continue;
            }

            decoded_packet.rescale_ts(input_stream.time_base(), decoder_time_base);

            if !self.decoder.decode(&decoded_packet, &mut decoded_frame)? {
                continue;
            }

            let timestamp = decoded_frame.timestamp();
            decoded_frame.set_pts(timestamp);

            self.filter_graph
                .get("in")
                .unwrap()
                .source()
                .add(&mut decoded_frame)?;

            self.write_all_filtered(
                decoder_time_base,
                &mut decoded_frame,
                encoder_time_base,
                &mut encoded_packet,
            )?;
        }

        self.filter_graph.get("in").unwrap().source().flush()?;
        self.write_all_filtered(
            decoder_time_base,
            &mut decoded_frame,
            encoder_time_base,
            &mut encoded_packet,
        )?;

        if self.encoder.flush(&mut encoded_packet)? {
            Self::write_packet(
                self.output_context_stream.inner_mut(),
                &mut encoded_packet,
                decoder_time_base,
                encoder_time_base,
            )?;
        }

        self.input_context = Some(input_context);

        self.output_context_stream.inner_mut().write_trailer()?;

        Ok(&self.output_context_stream.get_stream())
    }
}
