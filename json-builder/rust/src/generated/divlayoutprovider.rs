// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivLayoutProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_variable_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_variable_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivLayoutProviderBuilder {
    pub height_variable_name: Option<String>,
    pub width_variable_name: Option<String>,
}

impl DivLayoutProvider {
    pub fn builder() -> DivLayoutProviderBuilder {
        DivLayoutProviderBuilder::default()
    }
}

impl DivLayoutProviderBuilder {
    pub fn height_variable_name(mut self, value: String) -> Self {
        self.height_variable_name = Some(value);
        self
    }
    pub fn width_variable_name(mut self, value: String) -> Self {
        self.width_variable_name = Some(value);
        self
    }
    pub fn build(self) -> DivLayoutProvider {
        DivLayoutProvider {
            height_variable_name: self.height_variable_name,
            width_variable_name: self.width_variable_name,
        }
    }
}
