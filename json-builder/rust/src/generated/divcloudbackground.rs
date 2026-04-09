// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCloudBackground {
    pub color: Expression<String>,
    pub corner_radius: Expression<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(default = "DivCloudBackground::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivCloudBackgroundBuilder {
    pub color: Option<Expression<String>>,
    pub corner_radius: Option<Expression<i64>>,
    pub paddings: Option<DivEdgeInsets>,
    pub r#type: Option<String>,
}

impl DivCloudBackground {
    pub fn builder() -> DivCloudBackgroundBuilder {
        DivCloudBackgroundBuilder::default()
    }
    pub fn default_type() -> String { "cloud".to_string() }
}

impl DivCloudBackgroundBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn corner_radius(mut self, value: Expression<i64>) -> Self {
        self.corner_radius = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivCloudBackground {
        DivCloudBackground {
            color: self.color.expect("missing required field 'color'"),
            corner_radius: self.corner_radius.expect("missing required field 'corner_radius'"),
            paddings: self.paddings,
            r#type: String::from("cloud"),
        }
    }
}
