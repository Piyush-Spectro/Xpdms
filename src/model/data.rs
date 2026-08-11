use crate::binary::{DataType, SliceReader, TdmsTimestamp};
use crate::error::{TdmsError, Result};
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub trait TdmsPrimitive: Sized + Copy + 'static {
    fn data_type() -> DataType;
    fn read_slice(bytes: &[u8], is_big_endian: bool, count: usize) -> Result<Vec<Self>>;
}

impl TdmsPrimitive for u8 {
    fn data_type() -> DataType {
        DataType::U8
    }

    fn read_slice(bytes: &[u8], _is_big_endian: bool, count: usize) -> Result<Vec<Self>> {
        if bytes.len() < count {
            return Err(TdmsError::UnexpectedEof {
                required: count,
                available: bytes.len(),
            });
        }
        Ok(bytes[..count].to_vec())
    }
}

impl TdmsPrimitive for i8 {
    fn data_type() -> DataType {
        DataType::I8
    }

    fn read_slice(bytes: &[u8], _is_big_endian: bool, count: usize) -> Result<Vec<Self>> {
        if bytes.len() < count {
            return Err(TdmsError::UnexpectedEof {
                required: count,
                available: bytes.len(),
            });
        }
        Ok(bytes[..count].iter().map(|&b| b as i8).collect())
    }
}

macro_rules! impl_tdms_primitive {
    ($type:ty, $dtype:expr, $read_le:ident, $read_be:ident, $size:expr) => {
        impl TdmsPrimitive for $type {
            fn data_type() -> DataType {
                $dtype
            }

            fn read_slice(bytes: &[u8], is_big_endian: bool, count: usize) -> Result<Vec<Self>> {
                let required = count * $size;
                if bytes.len() < required {
                    return Err(TdmsError::UnexpectedEof {
                        required,
                        available: bytes.len(),
                    });
                }
                let mut vec = Vec::with_capacity(count);
                if is_big_endian {
                    for chunk in bytes[..required].chunks_exact($size) {
                        vec.push(BigEndian::$read_be(chunk));
                    }
                } else {
                    for chunk in bytes[..required].chunks_exact($size) {
                        vec.push(LittleEndian::$read_le(chunk));
                    }
                }
                Ok(vec)
            }
        }
    };
}

impl_tdms_primitive!(u16, DataType::U16, read_u16, read_u16, 2);
impl_tdms_primitive!(i16, DataType::I16, read_i16, read_i16, 2);
impl_tdms_primitive!(u32, DataType::U32, read_u32, read_u32, 4);
impl_tdms_primitive!(i32, DataType::I32, read_i32, read_i32, 4);
impl_tdms_primitive!(u64, DataType::U64, read_u64, read_u64, 8);
impl_tdms_primitive!(i64, DataType::I64, read_i64, read_i64, 8);
impl_tdms_primitive!(f32, DataType::SingleFloat, read_f32, read_f32, 4);
impl_tdms_primitive!(f64, DataType::DoubleFloat, read_f64, read_f64, 8);

impl TdmsPrimitive for bool {
    fn data_type() -> DataType {
        DataType::Boolean
    }

    fn read_slice(bytes: &[u8], _is_big_endian: bool, count: usize) -> Result<Vec<Self>> {
        if bytes.len() < count {
            return Err(TdmsError::UnexpectedEof {
                required: count,
                available: bytes.len(),
            });
        }
        Ok(bytes[..count].iter().map(|&b| b != 0).collect())
    }
}

impl TdmsPrimitive for TdmsTimestamp {
    fn data_type() -> DataType {
        DataType::Timestamp
    }

    fn read_slice(bytes: &[u8], is_big_endian: bool, count: usize) -> Result<Vec<Self>> {
        let required = count * 16;
        if bytes.len() < required {
            return Err(TdmsError::UnexpectedEof {
                required,
                available: bytes.len(),
            });
        }
        let mut vec = Vec::with_capacity(count);
        let mut reader = SliceReader::new(&bytes[..required], is_big_endian);
        for _ in 0..count {
            vec.push(reader.read_timestamp()?);
        }
        Ok(vec)
    }
}
