// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionHideTooltip {
    pub id: Expression<String>,
    #[serde(default = "DivActionHideTooltip::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionHideTooltipBuilder {
    pub id: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivActionHideTooltip {
    pub fn builder() -> DivActionHideTooltipBuilder {
        DivActionHideTooltipBuilder::default()
    }
    pub fn default_type() -> String { "hide_tooltip".to_string() }
}

impl DivActionHideTooltipBuilder {
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionHideTooltip {
        DivActionHideTooltip {
            id: self.id.expect("missing required field 'id'"),
            r#type: String::from("hide_tooltip"),
        }
    }
}
