// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivPagerLayoutMode {
    #[serde(rename = "percentage")]
    DivPageSize(DivPageSize),
    #[serde(rename = "fixed")]
    DivNeighbourPageSize(DivNeighbourPageSize),
    #[serde(rename = "wrap_content")]
    DivPageContentSize(DivPageContentSize),
}
