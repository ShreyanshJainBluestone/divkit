// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCollectionItemBuilder {
    pub data: Expression<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_element_name: Option<String>,
    pub prototypes: Vec<DivCollectionItemBuilderPrototype>,
}

#[derive(Debug, Clone, Default)]
pub struct DivCollectionItemBuilderBuilder {
    pub data: Option<Expression<Vec<serde_json::Value>>>,
    pub data_element_name: Option<String>,
    pub prototypes: Option<Vec<DivCollectionItemBuilderPrototype>>,
}

impl DivCollectionItemBuilder {
    pub fn builder() -> DivCollectionItemBuilderBuilder {
        DivCollectionItemBuilderBuilder::default()
    }
    pub fn default_data_element_name() -> String { "it".to_string() }
}

impl DivCollectionItemBuilderBuilder {
    pub fn data(mut self, value: Expression<Vec<serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn data_element_name(mut self, value: String) -> Self {
        self.data_element_name = Some(value);
        self
    }
    pub fn prototypes(mut self, value: Vec<DivCollectionItemBuilderPrototype>) -> Self {
        self.prototypes = Some(value);
        self
    }
    pub fn build(self) -> DivCollectionItemBuilder {
        DivCollectionItemBuilder {
            data: self.data.expect("missing required field 'data'"),
            data_element_name: self.data_element_name.or_else(|| Some(DivCollectionItemBuilder::default_data_element_name())),
            prototypes: self.prototypes.expect("missing required field 'prototypes'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCollectionItemBuilderPrototype {
    pub div: Div,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<Expression<bool>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivCollectionItemBuilderPrototypeBuilder {
    pub div: Option<Div>,
    pub id: Option<Expression<String>>,
    pub selector: Option<Expression<bool>>,
}

impl DivCollectionItemBuilderPrototype {
    pub fn builder() -> DivCollectionItemBuilderPrototypeBuilder {
        DivCollectionItemBuilderPrototypeBuilder::default()
    }
    pub fn default_selector() -> Expression<bool> { Expression::value(true) }
}

impl DivCollectionItemBuilderPrototypeBuilder {
    pub fn div(mut self, value: Div) -> Self {
        self.div = Some(value);
        self
    }
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn selector(mut self, value: Expression<bool>) -> Self {
        self.selector = Some(value);
        self
    }
    pub fn build(self) -> DivCollectionItemBuilderPrototype {
        DivCollectionItemBuilderPrototype {
            div: self.div.expect("missing required field 'div'"),
            id: self.id,
            selector: self.selector.or_else(|| Some(DivCollectionItemBuilderPrototype::default_selector())),
        }
    }
}
