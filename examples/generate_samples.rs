use xpTDMS::TdmsWriter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating sample TDMS files in examples/ directory...");

    let examples_dir = Path::new("examples");

    // 1. Sample 1: Engine Telemetry (f64 RPM, Temperature, Pressure)
    let sample1_path = examples_dir.join("sample_sensor_data.tdms");
    let mut writer1 = TdmsWriter::create(&sample1_path)?;

    let rpm_data: Vec<f64> = (0..1000).map(|i| 1500.0 + (i as f64 * 0.5)).collect();
    let temp_data: Vec<f64> = (0..1000).map(|i| 85.0 + ((i as f64 % 50.0) * 0.1)).collect();

    writer1.write_channel("Engine", "RPM", &rpm_data)?;
    writer1.write_channel("Engine", "Temperature", &temp_data)?;

    println!("✓ Created {}", sample1_path.display());

    // 2. Sample 2: Multi-Channel Acoustic & Vibration (i32, f32)
    let sample2_path = examples_dir.join("sample_multi_channel.tdms");
    let mut writer2 = TdmsWriter::create(&sample2_path)?;

    let vibration: Vec<f32> = (0..2000).map(|i| (i as f32 * 0.05).sin()).collect();
    let count_events: Vec<i32> = (0..2000).map(|i| i as i32 % 10).collect();

    writer2.write_channel("VibrationSensors", "AccX", &vibration)?;
    writer2.write_channel("DigitalCounters", "PulseCount", &count_events)?;

    println!("✓ Created {}", sample2_path.display());
    println!("Sample generation completed successfully!");

    Ok(())
}
