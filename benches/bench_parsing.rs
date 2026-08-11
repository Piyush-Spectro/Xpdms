use std::time::Instant;
use xpTDMS::{TdmsFile, TdmsWriter};

fn main() {
    println!("=== xpTDMS Benchmark Suite ===");

    let dir = std::env::temp_dir();
    let bench_file = dir.join("benchmark_large.tdms");
    let num_samples = 1_000_000;

    println!("Generating benchmark dataset with {} samples...", num_samples);
    let sample_data: Vec<f64> = (0..num_samples).map(|i| (i as f64) * 0.01).collect();

    let start_write = Instant::now();
    let mut writer = TdmsWriter::create(&bench_file).expect("Failed to create writer");
    writer
        .write_channel("EngineGroup", "RPM", &sample_data)
        .expect("Failed to write channel");
    let write_duration = start_write.elapsed();

    println!("Write speed: {:?}", write_duration);

    // Benchmark Opening and Indexing
    let start_open = Instant::now();
    let tdms_file = TdmsFile::open(&bench_file).expect("Failed to open TDMS file");
    let open_duration = start_open.elapsed();
    println!("Zero-Copy Open & Indexing time: {:?}", open_duration);

    // Benchmark Typed Data Reading
    let start_read = Instant::now();
    let read_data = tdms_file
        .read_channel_data::<f64>("EngineGroup", "RPM")
        .expect("Failed to read data");
    let read_duration = start_read.elapsed();

    let throughput_mb = (num_samples * 8) as f64 / (1024.0 * 1024.0) / read_duration.as_secs_f64();
    println!(
        "Read 1M samples ({:.2} MB) in {:?} ({:.2} MB/s)",
        (num_samples * 8) as f64 / (1024.0 * 1024.0),
        read_duration,
        throughput_mb
    );

    assert_eq!(read_data.len(), num_samples);

    let _ = std::fs::remove_file(bench_file);
    println!("Benchmark completed successfully!");
}
