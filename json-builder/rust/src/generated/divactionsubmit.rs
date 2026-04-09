// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSubmit {
    pub container_id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fail_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success_actions: Option<Vec<DivAction>>,
    pub request: DivActionSubmitRequest,
    #[serde(default = "DivActionSubmit::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSubmitBuilder {
    pub container_id: Option<Expression<String>>,
    pub on_fail_actions: Option<Vec<DivAction>>,
    pub on_success_actions: Option<Vec<DivAction>>,
    pub request: Option<DivActionSubmitRequest>,
    pub r#type: Option<String>,
}

impl DivActionSubmit {
    pub fn builder() -> DivActionSubmitBuilder {
        DivActionSubmitBuilder::default()
    }
    pub fn default_type() -> String { "submit".to_string() }
}

impl DivActionSubmitBuilder {
    pub fn container_id(mut self, value: Expression<String>) -> Self {
        self.container_id = Some(value);
        self
    }
    pub fn on_fail_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_fail_actions = Some(value);
        self
    }
    pub fn on_success_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_success_actions = Some(value);
        self
    }
    pub fn request(mut self, value: DivActionSubmitRequest) -> Self {
        self.request = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionSubmit {
        DivActionSubmit {
            container_id: self.container_id.expect("missing required field 'container_id'"),
            on_fail_actions: self.on_fail_actions,
            on_success_actions: self.on_success_actions,
            request: self.request.expect("missing required field 'request'"),
            r#type: String::from("submit"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSubmitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<DivActionSubmitRequestHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Expression<DivActionSubmitRequestMethod>>,
    pub url: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSubmitRequestBuilder {
    pub headers: Option<Vec<DivActionSubmitRequestHeader>>,
    pub method: Option<Expression<DivActionSubmitRequestMethod>>,
    pub url: Option<Expression<String>>,
}

impl DivActionSubmitRequest {
    pub fn builder() -> DivActionSubmitRequestBuilder {
        DivActionSubmitRequestBuilder::default()
    }
    pub fn default_method() -> Expression<DivActionSubmitRequestMethod> { Expression::value(serde_json::from_str(r#""post""#).unwrap()) }
}

impl DivActionSubmitRequestBuilder {
    pub fn headers(mut self, value: Vec<DivActionSubmitRequestHeader>) -> Self {
        self.headers = Some(value);
        self
    }
    pub fn method(mut self, value: Expression<DivActionSubmitRequestMethod>) -> Self {
        self.method = Some(value);
        self
    }
    pub fn url(mut self, value: Expression<String>) -> Self {
        self.url = Some(value);
        self
    }
    pub fn build(self) -> DivActionSubmitRequest {
        DivActionSubmitRequest {
            headers: self.headers,
            method: self.method.or_else(|| Some(DivActionSubmitRequest::default_method())),
            url: self.url.expect("missing required field 'url'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSubmitRequestHeader {
    pub name: Expression<String>,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSubmitRequestHeaderBuilder {
    pub name: Option<Expression<String>>,
    pub value: Option<Expression<String>>,
}

impl DivActionSubmitRequestHeader {
    pub fn builder() -> DivActionSubmitRequestHeaderBuilder {
        DivActionSubmitRequestHeaderBuilder::default()
    }
}

impl DivActionSubmitRequestHeaderBuilder {
    pub fn name(mut self, value: Expression<String>) -> Self {
        self.name = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivActionSubmitRequestHeader {
        DivActionSubmitRequestHeader {
            name: self.name.expect("missing required field 'name'"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivActionSubmitRequestMethod {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "put")]
    Put,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "options")]
    Options,
}
