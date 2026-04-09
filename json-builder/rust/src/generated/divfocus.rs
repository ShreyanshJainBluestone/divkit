// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFocus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_focus_ids: Option<DivFocusNextFocusIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_blur: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus: Option<Vec<DivAction>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFocusBuilder {
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub next_focus_ids: Option<DivFocusNextFocusIds>,
    pub on_blur: Option<Vec<DivAction>>,
    pub on_focus: Option<Vec<DivAction>>,
}

impl DivFocus {
    pub fn builder() -> DivFocusBuilder {
        DivFocusBuilder::default()
    }
}

impl DivFocusBuilder {
    pub fn background(mut self, value: Vec<DivBackground>) -> Self {
        self.background = Some(value);
        self
    }
    pub fn border(mut self, value: DivBorder) -> Self {
        self.border = Some(value);
        self
    }
    pub fn next_focus_ids(mut self, value: DivFocusNextFocusIds) -> Self {
        self.next_focus_ids = Some(value);
        self
    }
    pub fn on_blur(mut self, value: Vec<DivAction>) -> Self {
        self.on_blur = Some(value);
        self
    }
    pub fn on_focus(mut self, value: Vec<DivAction>) -> Self {
        self.on_focus = Some(value);
        self
    }
    pub fn build(self) -> DivFocus {
        DivFocus {
            background: self.background,
            border: self.border,
            next_focus_ids: self.next_focus_ids,
            on_blur: self.on_blur,
            on_focus: self.on_focus,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFocusNextFocusIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<Expression<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFocusNextFocusIdsBuilder {
    pub down: Option<Expression<String>>,
    pub forward: Option<Expression<String>>,
    pub left: Option<Expression<String>>,
    pub right: Option<Expression<String>>,
    pub up: Option<Expression<String>>,
}

impl DivFocusNextFocusIds {
    pub fn builder() -> DivFocusNextFocusIdsBuilder {
        DivFocusNextFocusIdsBuilder::default()
    }
}

impl DivFocusNextFocusIdsBuilder {
    pub fn down(mut self, value: Expression<String>) -> Self {
        self.down = Some(value);
        self
    }
    pub fn forward(mut self, value: Expression<String>) -> Self {
        self.forward = Some(value);
        self
    }
    pub fn left(mut self, value: Expression<String>) -> Self {
        self.left = Some(value);
        self
    }
    pub fn right(mut self, value: Expression<String>) -> Self {
        self.right = Some(value);
        self
    }
    pub fn up(mut self, value: Expression<String>) -> Self {
        self.up = Some(value);
        self
    }
    pub fn build(self) -> DivFocusNextFocusIds {
        DivFocusNextFocusIds {
            down: self.down,
            forward: self.forward,
            left: self.left,
            right: self.right,
            up: self.up,
        }
    }
}
