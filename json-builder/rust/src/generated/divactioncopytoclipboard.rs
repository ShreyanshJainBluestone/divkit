// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionCopyToClipboard {
    pub content: DivActionCopyToClipboardContent,
    #[serde(default = "DivActionCopyToClipboard::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionCopyToClipboardBuilder {
    pub content: Option<DivActionCopyToClipboardContent>,
    pub r#type: Option<String>,
}

impl DivActionCopyToClipboard {
    pub fn builder() -> DivActionCopyToClipboardBuilder {
        DivActionCopyToClipboardBuilder::default()
    }
    pub fn default_type() -> String { "copy_to_clipboard".to_string() }
}

impl DivActionCopyToClipboardBuilder {
    pub fn content(mut self, value: DivActionCopyToClipboardContent) -> Self {
        self.content = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionCopyToClipboard {
        DivActionCopyToClipboard {
            content: self.content.expect("missing required field 'content'"),
            r#type: String::from("copy_to_clipboard"),
        }
    }
}
