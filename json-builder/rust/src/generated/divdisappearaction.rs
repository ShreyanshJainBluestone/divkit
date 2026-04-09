// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivDisappearAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_callbacks: Option<DivDownloadCallbacks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
    pub log_id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_limit: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed: Option<DivActionTyped>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_percentage: Option<Expression<i64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivDisappearActionBuilder {
    pub disappear_duration: Option<Expression<i64>>,
    pub download_callbacks: Option<DivDownloadCallbacks>,
    pub is_enabled: Option<Expression<bool>>,
    pub log_id: Option<Expression<String>>,
    pub log_limit: Option<Expression<i64>>,
    pub payload: Option<serde_json::Value>,
    pub referer: Option<Expression<String>>,
    pub scope_id: Option<String>,
    pub typed: Option<DivActionTyped>,
    pub url: Option<Expression<String>>,
    pub visibility_percentage: Option<Expression<i64>>,
}

impl DivDisappearAction {
    pub fn builder() -> DivDisappearActionBuilder {
        DivDisappearActionBuilder::default()
    }
    pub fn default_disappear_duration() -> Expression<i64> { Expression::value(800i64) }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_log_limit() -> Expression<i64> { Expression::value(1i64) }
    pub fn default_visibility_percentage() -> Expression<i64> { Expression::value(0i64) }
}

impl DivDisappearActionBuilder {
    pub fn disappear_duration(mut self, value: Expression<i64>) -> Self {
        self.disappear_duration = Some(value);
        self
    }
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
    pub fn log_limit(mut self, value: Expression<i64>) -> Self {
        self.log_limit = Some(value);
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
    pub fn typed(mut self, value: DivActionTyped) -> Self {
        self.typed = Some(value);
        self
    }
    pub fn url(mut self, value: Expression<String>) -> Self {
        self.url = Some(value);
        self
    }
    pub fn visibility_percentage(mut self, value: Expression<i64>) -> Self {
        self.visibility_percentage = Some(value);
        self
    }
    pub fn build(self) -> DivDisappearAction {
        DivDisappearAction {
            disappear_duration: self.disappear_duration.or_else(|| Some(DivDisappearAction::default_disappear_duration())),
            download_callbacks: self.download_callbacks,
            is_enabled: self.is_enabled.or_else(|| Some(DivDisappearAction::default_is_enabled())),
            log_id: self.log_id.expect("missing required field 'log_id'"),
            log_limit: self.log_limit.or_else(|| Some(DivDisappearAction::default_log_limit())),
            payload: self.payload,
            referer: self.referer,
            scope_id: self.scope_id,
            typed: self.typed,
            url: self.url,
            visibility_percentage: self.visibility_percentage.or_else(|| Some(DivDisappearAction::default_visibility_percentage())),
        }
    }
}
