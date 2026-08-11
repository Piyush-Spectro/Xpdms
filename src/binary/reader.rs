use crate::binary::types::{DataType, TdmsTimestamp};
use crate::error::{TdmsError, Result};
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub struct SliceReader<'a> {
    data: &'a [u8],
    offset: usize,
    is_big_endian: bool,
}

impl<'a> SliceReader<'a> {
    pub fn new(data: &'a [u8], is_big_endian: bool) -> Self {
        Self {
            data,
            offset: 0,
            is_big_endian,
        }
    }

    pub fn position(&self) -> usize {
        self.offset
    }

    pub fn set_position(&mut self, pos: usize) {
        self.offset = pos;
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.offset + len > self.data.len() {
            return Err(TdmsError::UnexpectedEof {
                required: len,
                available: self.remaining(),
            });
        }
        let slice = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(1)?[0])
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let bytes = self.read_bytes(2)?;
        Ok(if self.is_big_endian {
            BigEndian::read_u16(bytes)
        } else {
            LittleEndian::read_u16(bytes)
        })
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_bytes(4)?;
        Ok(if self.is_big_endian {
            BigEndian::read_u32(bytes)
        } else {
            LittleEndian::read_u32(bytes)
        })
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let bytes = self.read_bytes(8)?;
        Ok(if self.is_big_endian {
            BigEndian::read_u64(bytes)
        } else {
            LittleEndian::read_u64(bytes)
        })
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let bytes = self.read_bytes(4)?;
        Ok(if self.is_big_endian {
            BigEndian::read_f32(bytes)
        } else {
            LittleEndian::read_f32(bytes)
        })
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        let bytes = self.read_bytes(8)?;
        Ok(if self.is_big_endian {
            BigEndian::read_f64(bytes)
        } else {
            LittleEndian::read_f64(bytes)
        })
    }

    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| TdmsError::CorruptedMetadata {
            offset: self.offset as u64,
            reason: "Invalid UTF-8 string".to_string(),
        })
    }

    pub fn read_timestamp(&mut self) -> Result<TdmsTimestamp> {
        let fraction = self.read_u64()?;
        let seconds = self.read_i64()?;
        Ok(TdmsTimestamp::new(seconds, fraction))
    }

    pub fn read_boolean(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    pub fn read_data_type(&mut self) -> Result<DataType> {
        let raw_id = self.read_u32()?;
        DataType::from_u32(raw_id)
    }
}
