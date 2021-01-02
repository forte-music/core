use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum TranscodeTarget {
    MP3V0,
    AACV5,
}

impl TranscodeTarget {
    pub fn get_ffmpeg_args<'a>(
        &self,
        input_file: &'a Path,
        output_file: &'a Path,
    ) -> Vec<&'a OsStr> {
        match self {
            TranscodeTarget::MP3V0 => vec![
                // Input File
                "-i".as_ref(),
                input_file.as_os_str(),
                // Disable Video
                "-vn".as_ref(),
                // Bitrate
                "-b:a".as_ref(),
                "320k".as_ref(),
                // MP3 Output Format
                "-f".as_ref(),
                "mp3".as_ref(),
                output_file.as_os_str(),
            ],
            TranscodeTarget::AACV5 => vec![
                // Input File
                "-i".as_ref(),
                input_file.as_os_str(),
                // Disable Video
                "-vn".as_ref(),
                // Use Codec
                "-c:a".as_ref(),
                "aac".as_ref(),
                // Quality
                "-q:a".as_ref(),
                "5".as_ref(),
                // MP4 Output Format
                "-f".as_ref(),
                "ipod".as_ref(),
                output_file.as_os_str(),
            ],
        }
    }

    pub fn get_filename(&self, song_title: &str) -> String {
        match self {
            TranscodeTarget::MP3V0 => format!("{}.mp3", song_title),
            TranscodeTarget::AACV5 => format!("{}.aac", song_title),
        }
    }
}

impl FromStr for TranscodeTarget {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mp3/v0" => Ok(TranscodeTarget::MP3V0),
            "aac/v5" => Ok(TranscodeTarget::AACV5),
            _ => Err("Unknown transcode target"),
        }
    }
}

impl fmt::Display for TranscodeTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
