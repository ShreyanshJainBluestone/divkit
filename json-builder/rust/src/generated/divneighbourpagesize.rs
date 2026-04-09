// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivNeighbourPageSize {
    pub neighbour_page_width: DivFixedSize,
    #[serde(default = "DivNeighbourPageSize::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivNeighbourPageSizeBuilder {
    pub neighbour_page_width: Option<DivFixedSize>,
    pub r#type: Option<String>,
}

impl DivNeighbourPageSize {
    pub fn builder() -> DivNeighbourPageSizeBuilder {
        DivNeighbourPageSizeBuilder::default()
    }
    pub fn default_type() -> String { "fixed".to_string() }
}

impl DivNeighbourPageSizeBuilder {
    pub fn neighbour_page_width(mut self, value: DivFixedSize) -> Self {
        self.neighbour_page_width = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivNeighbourPageSize {
        DivNeighbourPageSize {
            neighbour_page_width: self.neighbour_page_width.expect("missing required field 'neighbour_page_width'"),
            r#type: String::from("fixed"),
        }
    }
}
