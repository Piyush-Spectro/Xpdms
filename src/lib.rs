pub mod binary;
pub mod defrag;
pub mod error;
pub mod index;
pub mod model;
pub mod writer;

#[cfg(feature = "python")]
pub mod python;

pub use binary::{DataType, SegmentHeader, SliceReader, TdmsTimestamp};
pub use defrag::Defragmenter;
pub use error::{TdmsError, Result};
pub use index::{ObjectMetadata, ObjectRawDataIndex, SegmentIndex};
pub use model::{ChunkIterator, PropertyValue, TdmsChannel, TdmsFile, TdmsGroup, TdmsPrimitive};
pub use writer::TdmsWriter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parsing_little_endian() {
        let mut header_bytes = [0u8; 28];
        header_bytes[0..4].copy_from_slice(b"TDSm");
        header_bytes[4..8].copy_from_slice(&(0x0eu32).to_le_bytes()); // TOC flags (has metadata + raw data)
        header_bytes[8..12].copy_from_slice(&(4713u32).to_le_bytes()); // Version
        header_bytes[12..20].copy_from_slice(&(100u64).to_le_bytes()); // Next segment offset
        header_bytes[20..28].copy_from_slice(&(50u64).to_le_bytes()); // Raw data offset

        let header = SegmentHeader::parse(&header_bytes).unwrap();
        assert_eq!(header.version, 4713);
        assert_eq!(header.next_segment_offset, 100);
        assert_eq!(header.raw_data_offset, 50);
        assert!(!header.is_big_endian);
        assert!(header.has_metadata);
        assert!(header.has_raw_data);
    }

    #[test]
    fn test_timestamp_nanoseconds() {
        let ts = TdmsTimestamp::new(2_082_844_800 + 10, 0x8000_0000_0000_0000);
        assert_eq!(ts.unix_seconds(), 10);
        assert_eq!(ts.nanoseconds(), 500_000_000);
    }
}
