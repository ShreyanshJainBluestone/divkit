// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFilterRtlMirror {
    #[serde(default = "DivFilterRtlMirror::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivFilterRtlMirrorBuilder {
    pub r#type: Option<String>,
}

impl DivFilterRtlMirror {
    pub fn builder() -> DivFilterRtlMirrorBuilder {
        DivFilterRtlMirrorBuilder::default()
    }
    pub fn default_type() -> String { "rtl_mirror".to_string() }
}

impl DivFilterRtlMirrorBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivFilterRtlMirror {
        DivFilterRtlMirror {
            r#type: String::from("rtl_mirror"),
        }
    }
}
