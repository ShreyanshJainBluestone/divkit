// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionVideo {
    pub action: Expression<DivActionVideoAction>,
    pub id: Expression<String>,
    #[serde(default = "DivActionVideo::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionVideoBuilder {
    pub action: Option<Expression<DivActionVideoAction>>,
    pub id: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivActionVideo {
    pub fn builder() -> DivActionVideoBuilder {
        DivActionVideoBuilder::default()
    }
    pub fn default_type() -> String { "video".to_string() }
}

impl DivActionVideoBuilder {
    pub fn action(mut self, value: Expression<DivActionVideoAction>) -> Self {
        self.action = Some(value);
        self
    }
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionVideo {
        DivActionVideo {
            action: self.action.expect("missing required field 'action'"),
            id: self.id.expect("missing required field 'id'"),
            r#type: String::from("video"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivActionVideoAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "pause")]
    Pause,
}
