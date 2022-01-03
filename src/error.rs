#[derive(Debug)]
pub enum Error {
    ChunkTooSmall,
    InvalidChunkType,
    InvalidChunkSize {
        bytes_recv: usize,
        length_field: u32,
    },
    InvalidCRC,
    DataNotUTF8,
    PngTooSmall,
    InvalidHeader,
    NoChunkOfGivenTypeFound,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
