// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPatch {
    pub changes: Vec<DivPatchChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Expression<DivPatchMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_applied_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failed_actions: Option<Vec<DivAction>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivPatchBuilder {
    pub changes: Option<Vec<DivPatchChange>>,
    pub mode: Option<Expression<DivPatchMode>>,
    pub on_applied_actions: Option<Vec<DivAction>>,
    pub on_failed_actions: Option<Vec<DivAction>>,
}

impl DivPatch {
    pub fn builder() -> DivPatchBuilder {
        DivPatchBuilder::default()
    }
    pub fn default_mode() -> Expression<DivPatchMode> { Expression::value(serde_json::from_str(r#""partial""#).unwrap()) }
}

impl DivPatchBuilder {
    pub fn changes(mut self, value: Vec<DivPatchChange>) -> Self {
        self.changes = Some(value);
        self
    }
    pub fn mode(mut self, value: Expression<DivPatchMode>) -> Self {
        self.mode = Some(value);
        self
    }
    pub fn on_applied_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_applied_actions = Some(value);
        self
    }
    pub fn on_failed_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_failed_actions = Some(value);
        self
    }
    pub fn build(self) -> DivPatch {
        DivPatch {
            changes: self.changes.expect("missing required field 'changes'"),
            mode: self.mode.or_else(|| Some(DivPatch::default_mode())),
            on_applied_actions: self.on_applied_actions,
            on_failed_actions: self.on_failed_actions,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPatchChange {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Div>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivPatchChangeBuilder {
    pub id: Option<String>,
    pub items: Option<Vec<Div>>,
}

impl DivPatchChange {
    pub fn builder() -> DivPatchChangeBuilder {
        DivPatchChangeBuilder::default()
    }
}

impl DivPatchChangeBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn items(mut self, value: Vec<Div>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn build(self) -> DivPatchChange {
        DivPatchChange {
            id: self.id.expect("missing required field 'id'"),
            items: self.items,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivPatchMode {
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "partial")]
    Partial,
}
