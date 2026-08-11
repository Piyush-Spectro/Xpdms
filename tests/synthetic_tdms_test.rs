use memmap2::MmapMut;
use xpTDMS::{ChunkIterator, DataType, TdmsFile};

#[test]
fn test_synthetic_tdms_parsing_and_channel_reading() {
    let mut buffer = Vec::new();

    // --- Segment 1 Header (28 bytes) ---
    buffer.extend_from_slice(b"TDSm");
    buffer.extend_from_slice(&(0x0eu32).to_le_bytes()); // TOC: has metadata (0x02) + has raw data (0x08) + Little Endian
    buffer.extend_from_slice(&(4713u32).to_le_bytes()); // Version

    let header_len_pos = buffer.len();
    buffer.extend_from_slice(&(0u64).to_le_bytes()); // Placeholder: next segment offset
    buffer.extend_from_slice(&(0u64).to_le_bytes()); // Placeholder: raw data offset

    let metadata_start = buffer.len();

    // --- Metadata Block ---
    buffer.extend_from_slice(&(3u32).to_le_bytes()); // 3 objects

    // Object 1: Root "/"
    let root_path = "/";
    buffer.extend_from_slice(&(root_path.len() as u32).to_le_bytes());
    buffer.extend_from_slice(root_path.as_bytes());
    buffer.extend_from_slice(&(0xFFFF_FFFFu32).to_le_bytes()); // No raw data
    buffer.extend_from_slice(&(1u32).to_le_bytes()); // 1 property
    let prop_name = "author";
    buffer.extend_from_slice(&(prop_name.len() as u32).to_le_bytes());
    buffer.extend_from_slice(prop_name.as_bytes());
    buffer.extend_from_slice(&(DataType::String as u32).to_le_bytes());
    let prop_val = "xpTDMS Developer";
    buffer.extend_from_slice(&(prop_val.len() as u32).to_le_bytes());
    buffer.extend_from_slice(prop_val.as_bytes());

    // Object 2: Group "/\"Group1\""
    let group_path = "/\"Group1\"";
    buffer.extend_from_slice(&(group_path.len() as u32).to_le_bytes());
    buffer.extend_from_slice(group_path.as_bytes());
    buffer.extend_from_slice(&(0xFFFF_FFFFu32).to_le_bytes());
    buffer.extend_from_slice(&(0u32).to_le_bytes());

    // Object 3: Channel "/\"Group1\"/\"Channel1\""
    let chan_path = "/\"Group1\"/\"Channel1\"";
    buffer.extend_from_slice(&(chan_path.len() as u32).to_le_bytes());
    buffer.extend_from_slice(chan_path.as_bytes());
    buffer.extend_from_slice(&(20u32).to_le_bytes()); // Raw data header length
    buffer.extend_from_slice(&(DataType::DoubleFloat as u32).to_le_bytes());
    buffer.extend_from_slice(&(1u32).to_le_bytes());
    buffer.extend_from_slice(&(100u64).to_le_bytes()); // 100 values
    buffer.extend_from_slice(&(0u32).to_le_bytes());

    let raw_data_start = buffer.len();
    let raw_data_offset = (raw_data_start - metadata_start) as u64;

    // --- Raw Data Block (100 * 8 bytes = 800 bytes) ---
    let mut expected_values = Vec::with_capacity(100);
    for i in 0..100 {
        let val = i as f64 * 1.5;
        expected_values.push(val);
        buffer.extend_from_slice(&val.to_le_bytes());
    }

    let total_len = buffer.len();
    let next_segment_offset = (total_len - 28) as u64;

    buffer[header_len_pos..header_len_pos + 8].copy_from_slice(&next_segment_offset.to_le_bytes());
    buffer[header_len_pos + 8..header_len_pos + 16].copy_from_slice(&raw_data_offset.to_le_bytes());

    let mut mmap_mut = MmapMut::map_anon(buffer.len()).unwrap();
    mmap_mut.copy_from_slice(&buffer);
    let mmap = mmap_mut.make_read_only().unwrap();

    let tdms_file = TdmsFile::from_mmap(mmap).expect("Failed to parse synthetic TDMS");

    // Read full raw channel data
    let channel_data = tdms_file
        .read_channel_data::<f64>("Group1", "Channel1")
        .expect("Should read f64 channel data");

    assert_eq!(channel_data.len(), 100);
    assert_eq!(channel_data, expected_values);

    // Test Chunk Streaming (chunks of size 30)
    let chunks: Vec<&[f64]> = ChunkIterator::new(&channel_data, 30).collect();
    assert_eq!(chunks.len(), 4); // 30 + 30 + 30 + 10
    assert_eq!(chunks[0].len(), 30);
    assert_eq!(chunks[1].len(), 30);
    assert_eq!(chunks[2].len(), 30);
    assert_eq!(chunks[3].len(), 10);
    assert_eq!(chunks[0], &expected_values[0..30]);
}
