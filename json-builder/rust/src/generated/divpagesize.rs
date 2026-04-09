// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPageSize {
    pub page_width: DivPercentageSize,
    #[serde(default = "DivPageSize::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivPageSizeBuilder {
    pub page_width: Option<DivPercentageSize>,
    pub r#type: Option<String>,
}

impl DivPageSize {
    pub fn builder() -> DivPageSizeBuilder {
        DivPageSizeBuilder::default()
    }
    pub fn default_type() -> String { "percentage".to_string() }
}

impl DivPageSizeBuilder {
    pub fn page_width(mut self, value: DivPercentageSize) -> Self {
        self.page_width = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivPageSize {
        DivPageSize {
            page_width: self.page_width.expect("missing required field 'page_width'"),
            r#type: String::from("percentage"),
        }
    }
}
