// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFunctionArgument {
    pub name: String,
    pub r#type: DivEvaluableType,
}

#[derive(Debug, Clone, Default)]
pub struct DivFunctionArgumentBuilder {
    pub name: Option<String>,
    pub r#type: Option<DivEvaluableType>,
}

impl DivFunctionArgument {
    pub fn builder() -> DivFunctionArgumentBuilder {
        DivFunctionArgumentBuilder::default()
    }
}

impl DivFunctionArgumentBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: DivEvaluableType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivFunctionArgument {
        DivFunctionArgument {
            name: self.name.expect("missing required field 'name'"),
            r#type: self.r#type.expect("missing required field 'r#type'"),
        }
    }
}
