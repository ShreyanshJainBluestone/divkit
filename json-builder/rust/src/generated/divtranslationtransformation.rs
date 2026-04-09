// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTranslationTransformation {
    #[serde(default = "DivTranslationTransformation::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<DivTranslation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<DivTranslation>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTranslationTransformationBuilder {
    pub r#type: Option<String>,
    pub x: Option<DivTranslation>,
    pub y: Option<DivTranslation>,
}

impl DivTranslationTransformation {
    pub fn builder() -> DivTranslationTransformationBuilder {
        DivTranslationTransformationBuilder::default()
    }
    pub fn default_type() -> String { "translation".to_string() }
}

impl DivTranslationTransformationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn x(mut self, value: DivTranslation) -> Self {
        self.x = Some(value);
        self
    }
    pub fn y(mut self, value: DivTranslation) -> Self {
        self.y = Some(value);
        self
    }
    pub fn build(self) -> DivTranslationTransformation {
        DivTranslationTransformation {
            r#type: String::from("translation"),
            x: self.x,
            y: self.y,
        }
    }
}
