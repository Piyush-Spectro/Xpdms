use crate::model::channel::TdmsChannel;
use crate::model::property::PropertyValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TdmsGroup {
    pub name: String,
    pub path: String,
    pub properties: HashMap<String, PropertyValue>,
    pub channels: HashMap<String, TdmsChannel>,
}

impl TdmsGroup {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            properties: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    pub fn channel(&self, name: &str) -> Option<&TdmsChannel> {
        self.channels.get(name)
    }

    pub fn get_property(&self, name: &str) -> Option<&PropertyValue> {
        self.properties.get(name)
    }
}
