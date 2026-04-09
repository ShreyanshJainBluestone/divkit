// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivVideoSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<Expression<i64>>,
    pub mime_type: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<DivVideoSourceResolution>,
    #[serde(default = "DivVideoSource::default_type")]
    pub r#type: String,
    pub url: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivVideoSourceBuilder {
    pub bitrate: Option<Expression<i64>>,
    pub mime_type: Option<Expression<String>>,
    pub resolution: Option<DivVideoSourceResolution>,
    pub r#type: Option<String>,
    pub url: Option<Expression<String>>,
}

impl DivVideoSource {
    pub fn builder() -> DivVideoSourceBuilder {
        DivVideoSourceBuilder::default()
    }
    pub fn default_type() -> String { "video_source".to_string() }
}

impl DivVideoSourceBuilder {
    pub fn bitrate(mut self, value: Expression<i64>) -> Self {
        self.bitrate = Some(value);
        self
    }
    pub fn mime_type(mut self, value: Expression<String>) -> Self {
        self.mime_type = Some(value);
        self
    }
    pub fn resolution(mut self, value: DivVideoSourceResolution) -> Self {
        self.resolution = Some(value);
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
    pub fn build(self) -> DivVideoSource {
        DivVideoSource {
            bitrate: self.bitrate,
            mime_type: self.mime_type.expect("missing required field 'mime_type'"),
            resolution: self.resolution,
            r#type: String::from("video_source"),
            url: self.url.expect("missing required field 'url'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivVideoSourceResolution {
    pub height: Expression<i64>,
    #[serde(default = "DivVideoSourceResolution::default_type")]
    pub r#type: String,
    pub width: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivVideoSourceResolutionBuilder {
    pub height: Option<Expression<i64>>,
    pub r#type: Option<String>,
    pub width: Option<Expression<i64>>,
}

impl DivVideoSourceResolution {
    pub fn builder() -> DivVideoSourceResolutionBuilder {
        DivVideoSourceResolutionBuilder::default()
    }
    pub fn default_type() -> String { "resolution".to_string() }
}

impl DivVideoSourceResolutionBuilder {
    pub fn height(mut self, value: Expression<i64>) -> Self {
        self.height = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn width(mut self, value: Expression<i64>) -> Self {
        self.width = Some(value);
        self
    }
    pub fn build(self) -> DivVideoSourceResolution {
        DivVideoSourceResolution {
            height: self.height.expect("missing required field 'height'"),
            r#type: String::from("resolution"),
            width: self.width.expect("missing required field 'width'"),
        }
    }
}
