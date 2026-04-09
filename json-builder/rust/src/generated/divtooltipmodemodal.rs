// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTooltipModeModal {
    #[serde(default = "DivTooltipModeModal::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivTooltipModeModalBuilder {
    pub r#type: Option<String>,
}

impl DivTooltipModeModal {
    pub fn builder() -> DivTooltipModeModalBuilder {
        DivTooltipModeModalBuilder::default()
    }
    pub fn default_type() -> String { "modal".to_string() }
}

impl DivTooltipModeModalBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivTooltipModeModal {
        DivTooltipModeModal {
            r#type: String::from("modal"),
        }
    }
}
