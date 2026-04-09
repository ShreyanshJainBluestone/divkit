// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivImageBackground {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DivFilter>>,
    pub image_url: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_required: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Expression<DivImageScale>>,
    #[serde(default = "DivImageBackground::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivImageBackgroundBuilder {
    pub alpha: Option<Expression<f64>>,
    pub content_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub content_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub filters: Option<Vec<DivFilter>>,
    pub image_url: Option<Expression<String>>,
    pub preload_required: Option<Expression<bool>>,
    pub scale: Option<Expression<DivImageScale>>,
    pub r#type: Option<String>,
}

impl DivImageBackground {
    pub fn builder() -> DivImageBackgroundBuilder {
        DivImageBackgroundBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_content_alignment_horizontal() -> Expression<DivAlignmentHorizontal> { Expression::value(serde_json::from_str(r#""center""#).unwrap()) }
    pub fn default_content_alignment_vertical() -> Expression<DivAlignmentVertical> { Expression::value(serde_json::from_str(r#""center""#).unwrap()) }
    pub fn default_preload_required() -> Expression<bool> { Expression::value(true) }
    pub fn default_scale() -> Expression<DivImageScale> { Expression::value(serde_json::from_str(r#""fill""#).unwrap()) }
    pub fn default_type() -> String { "image".to_string() }
}

impl DivImageBackgroundBuilder {
    pub fn alpha(mut self, value: Expression<f64>) -> Self {
        self.alpha = Some(value);
        self
    }
    pub fn content_alignment_horizontal(mut self, value: Expression<DivAlignmentHorizontal>) -> Self {
        self.content_alignment_horizontal = Some(value);
        self
    }
    pub fn content_alignment_vertical(mut self, value: Expression<DivAlignmentVertical>) -> Self {
        self.content_alignment_vertical = Some(value);
        self
    }
    pub fn filters(mut self, value: Vec<DivFilter>) -> Self {
        self.filters = Some(value);
        self
    }
    pub fn image_url(mut self, value: Expression<String>) -> Self {
        self.image_url = Some(value);
        self
    }
    pub fn preload_required(mut self, value: Expression<bool>) -> Self {
        self.preload_required = Some(value);
        self
    }
    pub fn scale(mut self, value: Expression<DivImageScale>) -> Self {
        self.scale = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivImageBackground {
        DivImageBackground {
            alpha: self.alpha.or_else(|| Some(DivImageBackground::default_alpha())),
            content_alignment_horizontal: self.content_alignment_horizontal.or_else(|| Some(DivImageBackground::default_content_alignment_horizontal())),
            content_alignment_vertical: self.content_alignment_vertical.or_else(|| Some(DivImageBackground::default_content_alignment_vertical())),
            filters: self.filters,
            image_url: self.image_url.expect("missing required field 'image_url'"),
            preload_required: self.preload_required.or_else(|| Some(DivImageBackground::default_preload_required())),
            scale: self.scale.or_else(|| Some(DivImageBackground::default_scale())),
            r#type: String::from("image"),
        }
    }
}
