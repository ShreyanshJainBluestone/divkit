// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionScrollTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<Expression<bool>>,
    pub destination: DivActionScrollDestination,
    pub id: Expression<String>,
    #[serde(default = "DivActionScrollTo::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionScrollToBuilder {
    pub animated: Option<Expression<bool>>,
    pub destination: Option<DivActionScrollDestination>,
    pub id: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivActionScrollTo {
    pub fn builder() -> DivActionScrollToBuilder {
        DivActionScrollToBuilder::default()
    }
    pub fn default_animated() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "scroll_to".to_string() }
}

impl DivActionScrollToBuilder {
    pub fn animated(mut self, value: Expression<bool>) -> Self {
        self.animated = Some(value);
        self
    }
    pub fn destination(mut self, value: DivActionScrollDestination) -> Self {
        self.destination = Some(value);
        self
    }
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionScrollTo {
        DivActionScrollTo {
            animated: self.animated.or_else(|| Some(DivActionScrollTo::default_animated())),
            destination: self.destination.expect("missing required field 'destination'"),
            id: self.id.expect("missing required field 'id'"),
            r#type: String::from("scroll_to"),
        }
    }
}
