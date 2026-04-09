// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFunction {
    pub arguments: Vec<DivFunctionArgument>,
    pub body: String,
    pub name: String,
    pub return_type: DivEvaluableType,
}

#[derive(Debug, Clone, Default)]
pub struct DivFunctionBuilder {
    pub arguments: Option<Vec<DivFunctionArgument>>,
    pub body: Option<String>,
    pub name: Option<String>,
    pub return_type: Option<DivEvaluableType>,
}

impl DivFunction {
    pub fn builder() -> DivFunctionBuilder {
        DivFunctionBuilder::default()
    }
}

impl DivFunctionBuilder {
    pub fn arguments(mut self, value: Vec<DivFunctionArgument>) -> Self {
        self.arguments = Some(value);
        self
    }
    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn return_type(mut self, value: DivEvaluableType) -> Self {
        self.return_type = Some(value);
        self
    }
    pub fn build(self) -> DivFunction {
        DivFunction {
            arguments: self.arguments.expect("missing required field 'arguments'"),
            body: self.body.expect("missing required field 'body'"),
            name: self.name.expect("missing required field 'name'"),
            return_type: self.return_type.expect("missing required field 'return_type'"),
        }
    }
}
