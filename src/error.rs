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
