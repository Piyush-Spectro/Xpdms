use crate::error::Result;
use crate::model::file::TdmsFile;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Defragmenter;

impl Defragmenter {
    /// Defragment a TDMS file, writing a consolidated and optimized version to `output_path`.
    pub fn defragment<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, output_path: Q) -> Result<()> {
        let tdms_file = TdmsFile::open(input_path)?;
        let mut out_file = File::create(output_path)?;

        let mut buffer = Vec::new();

        // 1. Header & Lead-in
        buffer.extend_from_slice(b"TDSm");
        buffer.extend_from_slice(&(0x0eu32).to_le_bytes()); // TOC: Has Metadata + Has Raw Data
        buffer.extend_from_slice(&(4713u32).to_le_bytes()); // TDMS 2.0 version

        let header_offsets_pos = buffer.len();
        buffer.extend_from_slice(&(0u64).to_le_bytes()); // Placeholder: next segment offset
        buffer.extend_from_slice(&(0u64).to_le_bytes()); // Placeholder: raw data offset

        let metadata_start = buffer.len();

        // Count objects
        let mut total_objects = 1; // Root
        for group in tdms_file.groups.values() {
            total_objects += 1;
            total_objects += group.channels.len();
        }

        buffer.extend_from_slice(&(total_objects as u32).to_le_bytes());

        // Object 1: Root "/"
        buffer.extend_from_slice(&(1u32).to_le_bytes());
        buffer.extend_from_slice(b"/");
        buffer.extend_from_slice(&(0xFFFF_FFFFu32).to_le_bytes()); // No raw data
        buffer.extend_from_slice(&(tdms_file.properties.len() as u32).to_le_bytes());
        for (name, val) in &tdms_file.properties {
            buffer.extend_from_slice(&(name.len() as u32).to_le_bytes());
            buffer.extend_from_slice(name.as_bytes());
            buffer.extend_from_slice(&(val.data_type() as u32).to_le_bytes());
            // Write property value bytes
            match val {
                crate::model::property::PropertyValue::String(s) => {
                    buffer.extend_from_slice(&(s.len() as u32).to_le_bytes());
                    buffer.extend_from_slice(s.as_bytes());
                }
                crate::model::property::PropertyValue::I32(v) => buffer.extend_from_slice(&v.to_le_bytes()),
                crate::model::property::PropertyValue::I64(v) => buffer.extend_from_slice(&v.to_le_bytes()),
                crate::model::property::PropertyValue::DoubleFloat(v) => buffer.extend_from_slice(&v.to_le_bytes()),
                crate::model::property::PropertyValue::Boolean(b) => buffer.push(if *b { 1 } else { 0 }),
                _ => {}
            }
        }

        // Object Groups & Channels
        for group in tdms_file.groups.values() {
            buffer.extend_from_slice(&(group.path.len() as u32).to_le_bytes());
            buffer.extend_from_slice(group.path.as_bytes());
            buffer.extend_from_slice(&(0xFFFF_FFFFu32).to_le_bytes());
            buffer.extend_from_slice(&(group.properties.len() as u32).to_le_bytes());

            for channel in group.channels.values() {
                buffer.extend_from_slice(&(channel.path.len() as u32).to_le_bytes());
                buffer.extend_from_slice(channel.path.as_bytes());

                if let Some(dtype) = channel.data_type {
                    buffer.extend_from_slice(&(20u32).to_le_bytes());
                    buffer.extend_from_slice(&(dtype as u32).to_le_bytes());
                    buffer.extend_from_slice(&(1u32).to_le_bytes());
                    buffer.extend_from_slice(&(channel.number_of_values as u64).to_le_bytes());
                } else {
                    buffer.extend_from_slice(&(0xFFFF_FFFFu32).to_le_bytes());
                }

                buffer.extend_from_slice(&(channel.properties.len() as u32).to_le_bytes());
            }
        }

        let raw_data_start = buffer.len();
        let raw_data_offset = (raw_data_start - metadata_start) as u64;

        // Copy consolidated raw data blocks
        for group in tdms_file.groups.values() {
            for channel in group.channels.values() {
                if let Some(dtype) = channel.data_type {
                    match dtype {
                        crate::binary::DataType::DoubleFloat => {
                            if let Ok(data) = tdms_file.read_channel_data::<f64>(&group.name, &channel.name) {
                                for v in data {
                                    buffer.extend_from_slice(&v.to_le_bytes());
                                }
                            }
                        }
                        crate::binary::DataType::SingleFloat => {
                            if let Ok(data) = tdms_file.read_channel_data::<f32>(&group.name, &channel.name) {
                                for v in data {
                                    buffer.extend_from_slice(&v.to_le_bytes());
                                }
                            }
                        }
                        crate::binary::DataType::I32 => {
                            if let Ok(data) = tdms_file.read_channel_data::<i32>(&group.name, &channel.name) {
                                for v in data {
                                    buffer.extend_from_slice(&v.to_le_bytes());
                                }
                            }
                        }
                        crate::binary::DataType::I64 => {
                            if let Ok(data) = tdms_file.read_channel_data::<i64>(&group.name, &channel.name) {
                                for v in data {
                                    buffer.extend_from_slice(&v.to_le_bytes());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let total_size = buffer.len() as u64;
        let next_segment_offset = total_size - 28;

        buffer[header_offsets_pos..header_offsets_pos + 8].copy_from_slice(&next_segment_offset.to_le_bytes());
        buffer[header_offsets_pos + 8..header_offsets_pos + 16].copy_from_slice(&raw_data_offset.to_le_bytes());

        out_file.write_all(&buffer)?;
        Ok(())
    }
}
