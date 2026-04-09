// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_callbacks: Option<DivDownloadCallbacks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
    pub log_id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_items: Option<Vec<DivActionMenuItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Expression<DivActionTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed: Option<DivActionTyped>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Expression<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionBuilder {
    pub download_callbacks: Option<DivDownloadCallbacks>,
    pub is_enabled: Option<Expression<bool>>,
    pub log_id: Option<Expression<String>>,
    pub log_url: Option<Expression<String>>,
    pub menu_items: Option<Vec<DivActionMenuItem>>,
    pub payload: Option<serde_json::Value>,
    pub referer: Option<Expression<String>>,
    pub scope_id: Option<String>,
    pub target: Option<Expression<DivActionTarget>>,
    pub typed: Option<DivActionTyped>,
    pub url: Option<Expression<String>>,
}

impl DivAction {
    pub fn builder() -> DivActionBuilder {
        DivActionBuilder::default()
    }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
}

impl DivActionBuilder {
    pub fn download_callbacks(mut self, value: DivDownloadCallbacks) -> Self {
        self.download_callbacks = Some(value);
        self
    }
    pub fn is_enabled(mut self, value: Expression<bool>) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn log_id(mut self, value: Expression<String>) -> Self {
        self.log_id = Some(value);
        self
    }
    pub fn log_url(mut self, value: Expression<String>) -> Self {
        self.log_url = Some(value);
        self
    }
    pub fn menu_items(mut self, value: Vec<DivActionMenuItem>) -> Self {
        self.menu_items = Some(value);
        self
    }
    pub fn payload(mut self, value: serde_json::Value) -> Self {
        self.payload = Some(value);
        self
    }
    pub fn referer(mut self, value: Expression<String>) -> Self {
        self.referer = Some(value);
        self
    }
    pub fn scope_id(mut self, value: String) -> Self {
        self.scope_id = Some(value);
        self
    }
    pub fn target(mut self, value: Expression<DivActionTarget>) -> Self {
        self.target = Some(value);
        self
    }
    pub fn typed(mut self, value: DivActionTyped) -> Self {
        self.typed = Some(value);
        self
    }
    pub fn url(mut self, value: Expression<String>) -> Self {
        self.url = Some(value);
        self
    }
    pub fn build(self) -> DivAction {
        DivAction {
            download_callbacks: self.download_callbacks,
            is_enabled: self.is_enabled.or_else(|| Some(DivAction::default_is_enabled())),
            log_id: self.log_id.expect("missing required field 'log_id'"),
            log_url: self.log_url,
            menu_items: self.menu_items,
            payload: self.payload,
            referer: self.referer,
            scope_id: self.scope_id,
            target: self.target,
            typed: self.typed,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionMenuItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DivAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DivAction>>,
    pub text: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionMenuItemBuilder {
    pub action: Option<DivAction>,
    pub actions: Option<Vec<DivAction>>,
    pub text: Option<Expression<String>>,
}

impl DivActionMenuItem {
    pub fn builder() -> DivActionMenuItemBuilder {
        DivActionMenuItemBuilder::default()
    }
}

impl DivActionMenuItemBuilder {
    pub fn action(mut self, value: DivAction) -> Self {
        self.action = Some(value);
        self
    }
    pub fn actions(mut self, value: Vec<DivAction>) -> Self {
        self.actions = Some(value);
        self
    }
    pub fn text(mut self, value: Expression<String>) -> Self {
        self.text = Some(value);
        self
    }
    pub fn build(self) -> DivActionMenuItem {
        DivActionMenuItem {
            action: self.action,
            actions: self.actions,
            text: self.text.expect("missing required field 'text'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivActionTarget {
    #[serde(rename = "_self")]
    Self_,
    #[serde(rename = "_blank")]
    Blank,
}
