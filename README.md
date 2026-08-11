# xpTDMS 

**xpTDMS** is an ultra fast, memory efficient, zero copy **LabVIEW TDMS (Technical Data Management Streaming)** file reader, writer, and defragmenter written in **Rust** with native **Python bindings (`PyO3`)**.

It serves as a high performance replacement for `npTDMS`, eliminating memory leaks, 32-bit segment overflow errors, and slow parsing bottlenecks.

---

## Performance Benchmarks

Measured on 1,000,000 samples (8 MB dataset):

| Operation | Performance Metrics |
|---|---|
| **Zero Copy File Open & Indexing** | **150 µs** (0.15 ms) |
| **Write Speed** | **3.91 ms** / million samples |
| **Read Throughput** | **1,414.54 MB/s** |

---

## Target Problems Solved (vs `npTDMS`)

1. **Zero Memory Exhaustion**: Uses `memmap2` for zero copy memory mapping. Large files (>10 GB) are indexed without loading data into RAM.
2. **64 bit Segment Indexing**: Prevents `OverflowError: int32 out of bounds` on files with segments > 2 GB.
3. **Exact 128 bit Timestamps**: Full support for National Instruments 64.64 bit fixed-point timestamp format with nanosecond precision.
4. **Flexible Chunk Streaming**: Stream channel data in arbitrary chunk sizes without `ValueError: Data size not multiple of chunk size`.
5. **No Property Datatype Loss**: Preserves exact property data types (`i32`, `f64`, `String`, `Timestamp`, `bool`) on reads, writes, and defragmentation.
6. **Consolidated Defragmentation Engine**: Merges fragmented segment blocks into contiguous data sections without timestamp corruption.

---

## Rust Usage Quickstart

### 1. Read Channel Data
```rust
use xpTDMS::TdmsFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Zero copy open
    let tdms = TdmsFile::open("sample.tdms")?;

    // Read full numeric channel data
    let values: Vec<f64> = tdms.read_channel_data::<f64>("EngineGroup", "RPM")?;
    println!("Read {} samples. First value: {}", values.len(), values[0]);

    Ok(())
}
```

### 2. Stream Data in Custom Chunks
```rust
use xpTDMS::{TdmsFile, ChunkIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tdms = TdmsFile::open("large_file.tdms")?;
    let data: Vec<f64> = tdms.read_channel_data::<f64>("Sensors", "Temperature")?;

    // Process data in chunks of 500 samples at a time
    for chunk in ChunkIterator::new(&data, 500) {
        println!("Processing chunk of size: {}", chunk.len());
    }

    Ok(())
}
```

### 3. Write TDMS Files
```rust
use xpTDMS::TdmsWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = TdmsWriter::create("output.tdms")?;
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    writer.write_channel("Group1", "Channel1", &data)?;
    Ok(())
}
```

---

## Python Usage Quickstart

### 1. Installation via Maturin
```bash
pip install maturin
maturin develop --features python
```

### 2. Python Script
```python
import xpTDMS

# Open TDMS file
tdms = xpTDMS.TdmsFile.open("sample.tdms")

# List groups and channels
groups = tdms.group_names()
print("Groups:", groups)

channels = tdms.channel_names(groups[0])
print("Channels:", channels)

# Read raw channel data into Python list
rpm_data = tdms.read_channel_f64("EngineGroup", "RPM")
print(f"Loaded {len(rpm_data)} samples!")
```

---

## Building & Testing

### Run Unit & Integration Tests
```bash
cargo test
```

### Run Benchmarks
```bash
cargo bench
```

### Build Python C-Extension
```bash
cargo build --release --features python
```

---

## License
Licensed under Apache-2.0 or MIT.
