// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Div {
    #[serde(rename = "image")]
    DivImage(DivImage),
    #[serde(rename = "gif")]
    DivGifImage(DivGifImage),
    #[serde(rename = "text")]
    DivText(DivText),
    #[serde(rename = "separator")]
    DivSeparator(DivSeparator),
    #[serde(rename = "container")]
    DivContainer(DivContainer),
    #[serde(rename = "grid")]
    DivGrid(DivGrid),
    #[serde(rename = "gallery")]
    DivGallery(DivGallery),
    #[serde(rename = "pager")]
    DivPager(DivPager),
    #[serde(rename = "tabs")]
    DivTabs(DivTabs),
    #[serde(rename = "state")]
    DivState(DivState),
    #[serde(rename = "custom")]
    DivCustom(DivCustom),
    #[serde(rename = "indicator")]
    DivIndicator(DivIndicator),
    #[serde(rename = "slider")]
    DivSlider(DivSlider),
    #[serde(rename = "switch")]
    DivSwitch(DivSwitch),
    #[serde(rename = "input")]
    DivInput(DivInput),
    #[serde(rename = "select")]
    DivSelect(DivSelect),
    #[serde(rename = "video")]
    DivVideo(DivVideo),
}
