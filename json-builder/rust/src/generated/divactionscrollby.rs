// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionScrollBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<Expression<bool>>,
    pub id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow: Option<Expression<DivActionScrollByOverflow>>,
    #[serde(default = "DivActionScrollBy::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionScrollByBuilder {
    pub animated: Option<Expression<bool>>,
    pub id: Option<Expression<String>>,
    pub item_count: Option<Expression<i64>>,
    pub offset: Option<Expression<i64>>,
    pub overflow: Option<Expression<DivActionScrollByOverflow>>,
    pub r#type: Option<String>,
}

impl DivActionScrollBy {
    pub fn builder() -> DivActionScrollByBuilder {
        DivActionScrollByBuilder::default()
    }
    pub fn default_animated() -> Expression<bool> { Expression::value(true) }
    pub fn default_item_count() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_offset() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_overflow() -> Expression<DivActionScrollByOverflow> { Expression::value(serde_json::from_str(r#""clamp""#).unwrap()) }
    pub fn default_type() -> String { "scroll_by".to_string() }
}

impl DivActionScrollByBuilder {
    pub fn animated(mut self, value: Expression<bool>) -> Self {
        self.animated = Some(value);
        self
    }
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn item_count(mut self, value: Expression<i64>) -> Self {
        self.item_count = Some(value);
        self
    }
    pub fn offset(mut self, value: Expression<i64>) -> Self {
        self.offset = Some(value);
        self
    }
    pub fn overflow(mut self, value: Expression<DivActionScrollByOverflow>) -> Self {
        self.overflow = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionScrollBy {
        DivActionScrollBy {
            animated: self.animated.or_else(|| Some(DivActionScrollBy::default_animated())),
            id: self.id.expect("missing required field 'id'"),
            item_count: self.item_count.or_else(|| Some(DivActionScrollBy::default_item_count())),
            offset: self.offset.or_else(|| Some(DivActionScrollBy::default_offset())),
            overflow: self.overflow.or_else(|| Some(DivActionScrollBy::default_overflow())),
            r#type: String::from("scroll_by"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivActionScrollByOverflow {
    #[serde(rename = "clamp")]
    Clamp,
    #[serde(rename = "ring")]
    Ring,
}
