// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAccessibility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Expression<DivAccessibilityMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_after_action: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_description: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DivAccessibilityType>,
}

#[derive(Debug, Clone, Default)]
pub struct DivAccessibilityBuilder {
    pub description: Option<Expression<String>>,
    pub hint: Option<Expression<String>>,
    pub is_checked: Option<Expression<bool>>,
    pub mode: Option<Expression<DivAccessibilityMode>>,
    pub mute_after_action: Option<Expression<bool>>,
    pub state_description: Option<Expression<String>>,
    pub r#type: Option<DivAccessibilityType>,
}

impl DivAccessibility {
    pub fn builder() -> DivAccessibilityBuilder {
        DivAccessibilityBuilder::default()
    }
    pub fn default_mode() -> Expression<DivAccessibilityMode> { Expression::value(serde_json::from_str(r#""default""#).unwrap()) }
    pub fn default_mute_after_action() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> DivAccessibilityType { serde_json::from_str(r#""auto""#).unwrap() }
}

impl DivAccessibilityBuilder {
    pub fn description(mut self, value: Expression<String>) -> Self {
        self.description = Some(value);
        self
    }
    pub fn hint(mut self, value: Expression<String>) -> Self {
        self.hint = Some(value);
        self
    }
    pub fn is_checked(mut self, value: Expression<bool>) -> Self {
        self.is_checked = Some(value);
        self
    }
    pub fn mode(mut self, value: Expression<DivAccessibilityMode>) -> Self {
        self.mode = Some(value);
        self
    }
    pub fn mute_after_action(mut self, value: Expression<bool>) -> Self {
        self.mute_after_action = Some(value);
        self
    }
    pub fn state_description(mut self, value: Expression<String>) -> Self {
        self.state_description = Some(value);
        self
    }
    pub fn r#type(mut self, value: DivAccessibilityType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivAccessibility {
        DivAccessibility {
            description: self.description,
            hint: self.hint,
            is_checked: self.is_checked,
            mode: self.mode.or_else(|| Some(DivAccessibility::default_mode())),
            mute_after_action: self.mute_after_action.or_else(|| Some(DivAccessibility::default_mute_after_action())),
            state_description: self.state_description,
            r#type: self.r#type.or_else(|| Some(DivAccessibility::default_type())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivAccessibilityMode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "exclude")]
    Exclude,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivAccessibilityType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "edit_text")]
    EditText,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "tab_bar")]
    TabBar,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "auto")]
    Auto,
}
