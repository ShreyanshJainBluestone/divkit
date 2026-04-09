// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionDownload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fail_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success_actions: Option<Vec<DivAction>>,
    #[serde(default = "DivActionDownload::default_type")]
    pub r#type: String,
    pub url: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionDownloadBuilder {
    pub on_fail_actions: Option<Vec<DivAction>>,
    pub on_success_actions: Option<Vec<DivAction>>,
    pub r#type: Option<String>,
    pub url: Option<Expression<String>>,
}

impl DivActionDownload {
    pub fn builder() -> DivActionDownloadBuilder {
        DivActionDownloadBuilder::default()
    }
    pub fn default_type() -> String { "download".to_string() }
}

impl DivActionDownloadBuilder {
    pub fn on_fail_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_fail_actions = Some(value);
        self
    }
    pub fn on_success_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_success_actions = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn url(mut self, value: Expression<String>) -> Self {
        self.url = Some(value);
        self
    }
    pub fn build(self) -> DivActionDownload {
        DivActionDownload {
            on_fail_actions: self.on_fail_actions,
            on_success_actions: self.on_success_actions,
            r#type: String::from("download"),
            url: self.url.expect("missing required field 'url'"),
        }
    }
}
