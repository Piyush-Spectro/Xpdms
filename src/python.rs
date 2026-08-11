#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use crate::model::file::TdmsFile;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "TdmsFile")]
pub struct PyTdmsFile {
    inner: TdmsFile,
}

#[pymethods]
impl PyTdmsFile {
    #[staticmethod]
    pub fn open(path: &str) -> PyResult<Self> {
        let inner = TdmsFile::open(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to open TDMS file: {}", e)))?;
        Ok(Self { inner })
    }

    pub fn group_names(&self) -> Vec<String> {
        self.inner.groups.keys().cloned().collect()
    }

    pub fn channel_names(&self, group_name: &str) -> PyResult<Vec<String>> {
        let group = self
            .inner
            .group(group_name)
            .ok_or_else(|| PyValueError::new_err(format!("Group '{}' not found", group_name)))?;
        Ok(group.channels.keys().cloned().collect())
    }

    pub fn read_channel_f64(&self, group_name: &str, channel_name: &str) -> PyResult<Vec<f64>> {
        self.inner
            .read_channel_data::<f64>(group_name, channel_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn read_channel_f32(&self, group_name: &str, channel_name: &str) -> PyResult<Vec<f32>> {
        self.inner
            .read_channel_data::<f32>(group_name, channel_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn read_channel_i32(&self, group_name: &str, channel_name: &str) -> PyResult<Vec<i32>> {
        self.inner
            .read_channel_data::<i32>(group_name, channel_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn read_channel_i64(&self, group_name: &str, channel_name: &str) -> PyResult<Vec<i64>> {
        self.inner
            .read_channel_data::<i64>(group_name, channel_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn read_channel_u8(&self, group_name: &str, channel_name: &str) -> PyResult<Vec<u8>> {
        self.inner
            .read_channel_data::<u8>(group_name, channel_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymodule]
fn xpTDMS(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTdmsFile>()?;
    Ok(())
}
