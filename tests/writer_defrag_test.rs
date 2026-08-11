use xpTDMS::{Defragmenter, TdmsFile, TdmsWriter};

#[test]
fn test_writer_and_defragmenter() {
    let dir = std::env::temp_dir();
    let file1_path = dir.join("test_write.tdms");
    let file2_path = dir.join("test_defrag.tdms");

    // 1. Create and Write TDMS file
    let mut writer = TdmsWriter::create(&file1_path).expect("Failed to create TdmsWriter");
    let sample_data: Vec<f64> = (0..500).map(|i| i as f64 * 2.5).collect();

    writer
        .write_channel("Sensors", "Temperature", &sample_data)
        .expect("Failed to write channel");

    // 2. Open and Verify Written File
    let tdms_read = TdmsFile::open(&file1_path).expect("Failed to open written TDMS file");
    let read_data = tdms_read
        .read_channel_data::<f64>("Sensors", "Temperature")
        .expect("Failed to read back channel data");

    assert_eq!(read_data.len(), 500);
    assert_eq!(read_data, sample_data);

    // 3. Defragment File
    Defragmenter::defragment(&file1_path, &file2_path).expect("Defragmentation failed");

    // 4. Open and Verify Defragmented File
    let tdms_defrag = TdmsFile::open(&file2_path).expect("Failed to open defragmented TDMS file");
    let defrag_data = tdms_defrag
        .read_channel_data::<f64>("Sensors", "Temperature")
        .expect("Failed to read defragmented channel data");

    assert_eq!(defrag_data.len(), 500);
    assert_eq!(defrag_data, sample_data);

    // Cleanup temp files
    let _ = std::fs::remove_file(file1_path);
    let _ = std::fs::remove_file(file2_path);
}
