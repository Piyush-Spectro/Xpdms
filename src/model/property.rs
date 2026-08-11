use crate::binary::{DataType, SliceReader, TdmsTimestamp};
use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Void,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    SingleFloat(f32),
    DoubleFloat(f64),
    String(String),
    Boolean(bool),
    Timestamp(TdmsTimestamp),
}

impl PropertyValue {
    pub fn data_type(&self) -> DataType {
        match self {
            PropertyValue::Void => DataType::Void,
            PropertyValue::I8(_) => DataType::I8,
            PropertyValue::I16(_) => DataType::I16,
            PropertyValue::I32(_) => DataType::I32,
            PropertyValue::I64(_) => DataType::I64,
            PropertyValue::U8(_) => DataType::U8,
            PropertyValue::U16(_) => DataType::U16,
            PropertyValue::U32(_) => DataType::U32,
            PropertyValue::U64(_) => DataType::U64,
            PropertyValue::SingleFloat(_) => DataType::SingleFloat,
            PropertyValue::DoubleFloat(_) => DataType::DoubleFloat,
            PropertyValue::String(_) => DataType::String,
            PropertyValue::Boolean(_) => DataType::Boolean,
            PropertyValue::Timestamp(_) => DataType::Timestamp,
        }
    }

    pub fn parse(reader: &mut SliceReader) -> Result<Self> {
        let dtype = reader.read_data_type()?;
        Self::parse_value(reader, dtype)
    }

    pub fn parse_value(reader: &mut SliceReader, dtype: DataType) -> Result<Self> {
        match dtype {
            DataType::Void => Ok(PropertyValue::Void),
            DataType::I8 => Ok(PropertyValue::I8(reader.read_i8()?)),
            DataType::I16 => Ok(PropertyValue::I16(reader.read_i16()?)),
            DataType::I32 => Ok(PropertyValue::I32(reader.read_i32()?)),
            DataType::I64 => Ok(PropertyValue::I64(reader.read_i64()?)),
            DataType::U8 => Ok(PropertyValue::U8(reader.read_u8()?)),
            DataType::U16 => Ok(PropertyValue::U16(reader.read_u16()?)),
            DataType::U32 => Ok(PropertyValue::U32(reader.read_u32()?)),
            DataType::U64 => Ok(PropertyValue::U64(reader.read_u64()?)),
            DataType::SingleFloat => Ok(PropertyValue::SingleFloat(reader.read_f32()?)),
            DataType::DoubleFloat => Ok(PropertyValue::DoubleFloat(reader.read_f64()?)),
            DataType::String => Ok(PropertyValue::String(reader.read_string()?)),
            DataType::Boolean => Ok(PropertyValue::Boolean(reader.read_boolean()?)),
            DataType::Timestamp => Ok(PropertyValue::Timestamp(reader.read_timestamp()?)),
            _ => Err(crate::error::TdmsError::UnsupportedFeature(format!(
                "Property value data type {:?}",
                dtype
            ))),
        }
    }
}
