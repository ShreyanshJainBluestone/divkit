// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionShowTooltip {
    pub id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<Expression<bool>>,
    #[serde(default = "DivActionShowTooltip::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionShowTooltipBuilder {
    pub id: Option<Expression<String>>,
    pub multiple: Option<Expression<bool>>,
    pub r#type: Option<String>,
}

impl DivActionShowTooltip {
    pub fn builder() -> DivActionShowTooltipBuilder {
        DivActionShowTooltipBuilder::default()
    }
    pub fn default_type() -> String { "show_tooltip".to_string() }
}

impl DivActionShowTooltipBuilder {
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn multiple(mut self, value: Expression<bool>) -> Self {
        self.multiple = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionShowTooltip {
        DivActionShowTooltip {
            id: self.id.expect("missing required field 'id'"),
            multiple: self.multiple,
            r#type: String::from("show_tooltip"),
        }
    }
}
