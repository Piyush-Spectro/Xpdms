use thiserror::Error;

#[derive(Error, Debug)]
pub enum TdmsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid TDMS preamble header: expected 'TDSm', found {0:?}")]
    InvalidLeadIn([u8; 4]),

    #[error("Unsupported TDMS version: {0}")]
    UnsupportedVersion(u32),

    #[error("Invalid data type raw ID: {0:#x}")]
    InvalidDataType(u32),

    #[error("Unexpected End of File: required {required} bytes, available {available} bytes")]
    UnexpectedEof { required: usize, available: usize },

    #[error("Corrupted metadata at offset {offset}: {reason}")]
    CorruptedMetadata { offset: u64, reason: String },

    #[error("Segment overflow: {0}")]
    SegmentOverflow(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}

pub type Result<T> = std::result::Result<T, TdmsError>;
