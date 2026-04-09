// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTooltipModeNonModal {
    #[serde(default = "DivTooltipModeNonModal::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivTooltipModeNonModalBuilder {
    pub r#type: Option<String>,
}

impl DivTooltipModeNonModal {
    pub fn builder() -> DivTooltipModeNonModalBuilder {
        DivTooltipModeNonModalBuilder::default()
    }
    pub fn default_type() -> String { "non_modal".to_string() }
}

impl DivTooltipModeNonModalBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivTooltipModeNonModal {
        DivTooltipModeNonModal {
            r#type: String::from("non_modal"),
        }
    }
}
