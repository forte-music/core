use std::ffi::OsStr;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone)]
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

    pub fn get_template_url(&self) -> &'static str {
        match self {
            TranscodeTarget::MP3V0 => "/files/music/{id}/mp3/v0",
            TranscodeTarget::AACV5 => "/files/music/{id}/aac/v5",
        }
    }
}

impl fmt::Display for TranscodeTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
