// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionUpdateStructure {
    pub path: Expression<String>,
    #[serde(default = "DivActionUpdateStructure::default_type")]
    pub r#type: String,
    pub value: DivTypedValue,
    pub variable_name: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionUpdateStructureBuilder {
    pub path: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub value: Option<DivTypedValue>,
    pub variable_name: Option<Expression<String>>,
}

impl DivActionUpdateStructure {
    pub fn builder() -> DivActionUpdateStructureBuilder {
        DivActionUpdateStructureBuilder::default()
    }
    pub fn default_type() -> String { "update_structure".to_string() }
}

impl DivActionUpdateStructureBuilder {
    pub fn path(mut self, value: Expression<String>) -> Self {
        self.path = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: DivTypedValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn variable_name(mut self, value: Expression<String>) -> Self {
        self.variable_name = Some(value);
        self
    }
    pub fn build(self) -> DivActionUpdateStructure {
        DivActionUpdateStructure {
            path: self.path.expect("missing required field 'path'"),
            r#type: String::from("update_structure"),
            value: self.value.expect("missing required field 'value'"),
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
