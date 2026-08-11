use crate::binary::DataType;
use crate::model::property::PropertyValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TdmsChannel {
    pub path: String,
    pub name: String,
    pub group_name: String,
    pub data_type: Option<DataType>,
    pub properties: HashMap<String, PropertyValue>,
    pub number_of_values: u64,
}

impl TdmsChannel {
    pub fn new(path: String, group_name: String, name: String) -> Self {
        Self {
            path,
            name,
            group_name,
            data_type: None,
            properties: HashMap::new(),
            number_of_values: 0,
        }
    }

    pub fn get_property(&self, name: &str) -> Option<&PropertyValue> {
        self.properties.get(name)
    }
}
