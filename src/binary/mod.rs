pub mod header;
pub mod reader;
pub mod types;

pub use header::{SegmentHeader, HEADER_SIZE, TDMS_LEAD_IN};
pub use reader::SliceReader;
pub use types::{DataType, TdmsTimestamp};
