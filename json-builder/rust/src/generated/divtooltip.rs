// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_in: Option<DivAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_out: Option<DivAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_accessibility_description: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bring_to_top_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_by_tap_outside: Option<Expression<bool>>,
    pub div: Div,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DivTooltipMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<DivPoint>,
    pub position: Expression<DivTooltipPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substrate_div: Option<Div>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_outside_actions: Option<Vec<DivAction>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTooltipBuilder {
    pub animation_in: Option<DivAnimation>,
    pub animation_out: Option<DivAnimation>,
    pub background_accessibility_description: Option<Expression<String>>,
    pub bring_to_top_id: Option<String>,
    pub close_by_tap_outside: Option<Expression<bool>>,
    pub div: Option<Div>,
    pub duration: Option<Expression<i64>>,
    pub id: Option<String>,
    pub mode: Option<DivTooltipMode>,
    pub offset: Option<DivPoint>,
    pub position: Option<Expression<DivTooltipPosition>>,
    pub substrate_div: Option<Div>,
    pub tap_outside_actions: Option<Vec<DivAction>>,
}

impl DivTooltip {
    pub fn builder() -> DivTooltipBuilder {
        DivTooltipBuilder::default()
    }
    pub fn default_close_by_tap_outside() -> Expression<bool> { Expression::value(true) }
    pub fn default_duration() -> Expression<i64> { Expression::value(5000i64) }
    pub fn default_mode() -> DivTooltipMode { serde_json::from_str(r#"{ "type": "modal" }"#).unwrap() }
}

impl DivTooltipBuilder {
    pub fn animation_in(mut self, value: DivAnimation) -> Self {
        self.animation_in = Some(value);
        self
    }
    pub fn animation_out(mut self, value: DivAnimation) -> Self {
        self.animation_out = Some(value);
        self
    }
    pub fn background_accessibility_description(mut self, value: Expression<String>) -> Self {
        self.background_accessibility_description = Some(value);
        self
    }
    pub fn bring_to_top_id(mut self, value: String) -> Self {
        self.bring_to_top_id = Some(value);
        self
    }
    pub fn close_by_tap_outside(mut self, value: Expression<bool>) -> Self {
        self.close_by_tap_outside = Some(value);
        self
    }
    pub fn div(mut self, value: Div) -> Self {
        self.div = Some(value);
        self
    }
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn mode(mut self, value: DivTooltipMode) -> Self {
        self.mode = Some(value);
        self
    }
    pub fn offset(mut self, value: DivPoint) -> Self {
        self.offset = Some(value);
        self
    }
    pub fn position(mut self, value: Expression<DivTooltipPosition>) -> Self {
        self.position = Some(value);
        self
    }
    pub fn substrate_div(mut self, value: Div) -> Self {
        self.substrate_div = Some(value);
        self
    }
    pub fn tap_outside_actions(mut self, value: Vec<DivAction>) -> Self {
        self.tap_outside_actions = Some(value);
        self
    }
    pub fn build(self) -> DivTooltip {
        DivTooltip {
            animation_in: self.animation_in,
            animation_out: self.animation_out,
            background_accessibility_description: self.background_accessibility_description,
            bring_to_top_id: self.bring_to_top_id,
            close_by_tap_outside: self.close_by_tap_outside.or_else(|| Some(DivTooltip::default_close_by_tap_outside())),
            div: self.div.expect("missing required field 'div'"),
            duration: self.duration.or_else(|| Some(DivTooltip::default_duration())),
            id: self.id.expect("missing required field 'id'"),
            mode: self.mode.or_else(|| Some(DivTooltip::default_mode())),
            offset: self.offset,
            position: self.position.expect("missing required field 'position'"),
            substrate_div: self.substrate_div,
            tap_outside_actions: self.tap_outside_actions,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTooltipPosition {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "bottom-right")]
    BottomRight,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "center")]
    Center,
}
