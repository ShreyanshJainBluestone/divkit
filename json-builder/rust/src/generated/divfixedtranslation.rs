// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFixedTranslation {
    #[serde(default = "DivFixedTranslation::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFixedTranslationBuilder {
    pub r#type: Option<String>,
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Option<Expression<i64>>,
}

impl DivFixedTranslation {
    pub fn builder() -> DivFixedTranslationBuilder {
        DivFixedTranslationBuilder::default()
    }
    pub fn default_type() -> String { "translation-fixed".to_string() }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
}

impl DivFixedTranslationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivFixedTranslation {
        DivFixedTranslation {
            r#type: String::from("translation-fixed"),
            unit: self.unit.or_else(|| Some(DivFixedTranslation::default_unit())),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
