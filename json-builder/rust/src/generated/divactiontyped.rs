// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivActionTyped {
    #[serde(rename = "animator_start")]
    DivActionAnimatorStart(DivActionAnimatorStart),
    #[serde(rename = "animator_stop")]
    DivActionAnimatorStop(DivActionAnimatorStop),
    #[serde(rename = "array_insert_value")]
    DivActionArrayInsertValue(DivActionArrayInsertValue),
    #[serde(rename = "array_remove_value")]
    DivActionArrayRemoveValue(DivActionArrayRemoveValue),
    #[serde(rename = "array_set_value")]
    DivActionArraySetValue(DivActionArraySetValue),
    #[serde(rename = "clear_focus")]
    DivActionClearFocus(DivActionClearFocus),
    #[serde(rename = "copy_to_clipboard")]
    DivActionCopyToClipboard(DivActionCopyToClipboard),
    #[serde(rename = "dict_set_value")]
    DivActionDictSetValue(DivActionDictSetValue),
    #[serde(rename = "download")]
    DivActionDownload(DivActionDownload),
    #[serde(rename = "focus_element")]
    DivActionFocusElement(DivActionFocusElement),
    #[serde(rename = "hide_tooltip")]
    DivActionHideTooltip(DivActionHideTooltip),
    #[serde(rename = "scroll_by")]
    DivActionScrollBy(DivActionScrollBy),
    #[serde(rename = "scroll_to")]
    DivActionScrollTo(DivActionScrollTo),
    #[serde(rename = "set_state")]
    DivActionSetState(DivActionSetState),
    #[serde(rename = "set_stored_value")]
    DivActionSetStoredValue(DivActionSetStoredValue),
    #[serde(rename = "set_variable")]
    DivActionSetVariable(DivActionSetVariable),
    #[serde(rename = "show_tooltip")]
    DivActionShowTooltip(DivActionShowTooltip),
    #[serde(rename = "submit")]
    DivActionSubmit(DivActionSubmit),
    #[serde(rename = "timer")]
    DivActionTimer(DivActionTimer),
    #[serde(rename = "update_structure")]
    DivActionUpdateStructure(DivActionUpdateStructure),
    #[serde(rename = "video")]
    DivActionVideo(DivActionVideo),
    #[serde(rename = "custom")]
    DivActionCustom(DivActionCustom),
}
