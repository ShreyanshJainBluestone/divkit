// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivNinePatchBackground {
    pub image_url: Expression<String>,
    pub insets: DivAbsoluteEdgeInsets,
    #[serde(default = "DivNinePatchBackground::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivNinePatchBackgroundBuilder {
    pub image_url: Option<Expression<String>>,
    pub insets: Option<DivAbsoluteEdgeInsets>,
    pub r#type: Option<String>,
}

impl DivNinePatchBackground {
    pub fn builder() -> DivNinePatchBackgroundBuilder {
        DivNinePatchBackgroundBuilder::default()
    }
    pub fn default_type() -> String { "nine_patch_image".to_string() }
}

impl DivNinePatchBackgroundBuilder {
    pub fn image_url(mut self, value: Expression<String>) -> Self {
        self.image_url = Some(value);
        self
    }
    pub fn insets(mut self, value: DivAbsoluteEdgeInsets) -> Self {
        self.insets = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivNinePatchBackground {
        DivNinePatchBackground {
            image_url: self.image_url.expect("missing required field 'image_url'"),
            insets: self.insets.expect("missing required field 'insets'"),
            r#type: String::from("nine_patch_image"),
        }
    }
}
