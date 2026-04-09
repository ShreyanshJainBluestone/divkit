// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSetStoredValue {
    pub lifetime: Expression<i64>,
    pub name: Expression<String>,
    #[serde(default = "DivActionSetStoredValue::default_type")]
    pub r#type: String,
    pub value: DivTypedValue,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSetStoredValueBuilder {
    pub lifetime: Option<Expression<i64>>,
    pub name: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub value: Option<DivTypedValue>,
}

impl DivActionSetStoredValue {
    pub fn builder() -> DivActionSetStoredValueBuilder {
        DivActionSetStoredValueBuilder::default()
    }
    pub fn default_type() -> String { "set_stored_value".to_string() }
}

impl DivActionSetStoredValueBuilder {
    pub fn lifetime(mut self, value: Expression<i64>) -> Self {
        self.lifetime = Some(value);
        self
    }
    pub fn name(mut self, value: Expression<String>) -> Self {
        self.name = Some(value);
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
    pub fn build(self) -> DivActionSetStoredValue {
        DivActionSetStoredValue {
            lifetime: self.lifetime.expect("missing required field 'lifetime'"),
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("set_stored_value"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
