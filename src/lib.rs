use std::error;
use std::fmt;
use std::io::{BufReader, Cursor};
use rodio::Source;
use std::thread::sleep;
use std::time::Duration;

const EEP: &[u8] = include_bytes!("../assets/wild_eep.wav");

#[derive(Debug)]
pub enum RodioError {
    DecoderError(rodio::decoder::DecoderError),
    DevicesError(rodio::DevicesError),
    PlayError(rodio::PlayError),
    StreamError(rodio::StreamError),
}

impl From<rodio::decoder::DecoderError> for RodioError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self::DecoderError(err)
    }
}

impl From<rodio::DevicesError> for RodioError {
    fn from(err: rodio::DevicesError) -> Self {
        Self::DevicesError(err)
    }
}

impl From<rodio::PlayError> for RodioError {
    fn from(err: rodio::PlayError) -> Self {
        Self::PlayError(err)
    }
}

impl From<rodio::StreamError> for RodioError {
    fn from(err: rodio::StreamError) -> Self {
        Self::StreamError(err)
    }
}

impl fmt::Display for RodioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecoderError(e) => e.fmt(f),
            Self::DevicesError(e) => e.fmt(f),
            Self::PlayError(e) => e.fmt(f),
            Self::StreamError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for RodioError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::DecoderError(e) => Some(e),
            Self::DevicesError(e) => Some(e),
            Self::PlayError(e) => Some(e),
            Self::StreamError(e) => Some(e),
        }
    }
}

pub fn beep() -> Result<(), RodioError> {
    let mut stream = rodio::OutputStreamBuilder::open_default_stream()?;
    stream.log_on_drop(false);
    let mixer = stream.mixer();
    let decoder = rodio::Decoder::new_wav(BufReader::new(Cursor::new(EEP)))?;
    let dur: Duration = decoder.total_duration().unwrap();

    let sink: rodio::Sink = rodio::Sink::connect_new(mixer);
    sink.append(decoder);
    sink.set_volume(0.4);
    // sink.sleep_until_end();
    // Below is crufty, and I dislike it, but it works where sleep_until_end() does not, with such a short clip
    sleep(dur);
    Ok(())
}
