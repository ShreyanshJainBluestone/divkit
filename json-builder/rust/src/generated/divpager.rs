// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<DivAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animators: Option<Vec<DivAnimator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_axis_alignment: Option<Expression<DivPagerItemAlignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_item: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<DivFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite_scroll: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_builder: Option<DivCollectionItemBuilder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Div>>,
    pub layout_mode: DivPagerLayoutMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Expression<DivPagerOrientation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_transformation: Option<DivPageTransformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_parent_scroll: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_axis_alignment: Option<Expression<DivPagerItemAlignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<Vec<DivTooltip>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<DivTransform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<DivTransformation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_change: Option<DivChangeTransition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_in: Option<DivAppearanceTransition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_out: Option<DivAppearanceTransition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_triggers: Option<Vec<DivTransitionTrigger>>,
    #[serde(default = "DivPager::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_triggers: Option<Vec<DivTrigger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<DivVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Expression<DivVisibility>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_action: Option<DivVisibilityAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_actions: Option<Vec<DivVisibilityAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<DivSize>,
}

#[derive(Debug, Clone, Default)]
pub struct DivPagerBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub column_span: Option<Expression<i64>>,
    pub cross_axis_alignment: Option<Expression<DivPagerItemAlignment>>,
    pub default_item: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub focus: Option<DivFocus>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub id: Option<String>,
    pub infinite_scroll: Option<Expression<bool>>,
    pub item_builder: Option<DivCollectionItemBuilder>,
    pub item_spacing: Option<DivFixedSize>,
    pub items: Option<Vec<Div>>,
    pub layout_mode: Option<DivPagerLayoutMode>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub margins: Option<DivEdgeInsets>,
    pub orientation: Option<Expression<DivPagerOrientation>>,
    pub paddings: Option<DivEdgeInsets>,
    pub page_transformation: Option<DivPageTransformation>,
    pub restrict_parent_scroll: Option<Expression<bool>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub scroll_axis_alignment: Option<Expression<DivPagerItemAlignment>>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub tooltips: Option<Vec<DivTooltip>>,
    pub transform: Option<DivTransform>,
    pub transformations: Option<Vec<DivTransformation>>,
    pub transition_change: Option<DivChangeTransition>,
    pub transition_in: Option<DivAppearanceTransition>,
    pub transition_out: Option<DivAppearanceTransition>,
    pub transition_triggers: Option<Vec<DivTransitionTrigger>>,
    pub r#type: Option<String>,
    pub variable_triggers: Option<Vec<DivTrigger>>,
    pub variables: Option<Vec<DivVariable>>,
    pub visibility: Option<Expression<DivVisibility>>,
    pub visibility_action: Option<DivVisibilityAction>,
    pub visibility_actions: Option<Vec<DivVisibilityAction>>,
    pub width: Option<DivSize>,
}

impl DivPager {
    pub fn builder() -> DivPagerBuilder {
        DivPagerBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_cross_axis_alignment() -> Expression<DivPagerItemAlignment> { Expression::value(serde_json::from_str(r#""start""#).unwrap()) }
    pub fn default_default_item() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_infinite_scroll() -> Expression<bool> { Expression::value(true) }
    pub fn default_item_spacing() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":0}"#).unwrap() }
    pub fn default_orientation() -> Expression<DivPagerOrientation> { Expression::value(serde_json::from_str(r#""horizontal""#).unwrap()) }
    pub fn default_restrict_parent_scroll() -> Expression<bool> { Expression::value(true) }
    pub fn default_scroll_axis_alignment() -> Expression<DivPagerItemAlignment> { Expression::value(serde_json::from_str(r#""center""#).unwrap()) }
    pub fn default_type() -> String { "pager".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivPagerBuilder {
    pub fn accessibility(mut self, value: DivAccessibility) -> Self {
        self.accessibility = Some(value);
        self
    }
    pub fn alignment_horizontal(mut self, value: Expression<DivAlignmentHorizontal>) -> Self {
        self.alignment_horizontal = Some(value);
        self
    }
    pub fn alignment_vertical(mut self, value: Expression<DivAlignmentVertical>) -> Self {
        self.alignment_vertical = Some(value);
        self
    }
    pub fn alpha(mut self, value: Expression<f64>) -> Self {
        self.alpha = Some(value);
        self
    }
    pub fn animators(mut self, value: Vec<DivAnimator>) -> Self {
        self.animators = Some(value);
        self
    }
    pub fn background(mut self, value: Vec<DivBackground>) -> Self {
        self.background = Some(value);
        self
    }
    pub fn border(mut self, value: DivBorder) -> Self {
        self.border = Some(value);
        self
    }
    pub fn column_span(mut self, value: Expression<i64>) -> Self {
        self.column_span = Some(value);
        self
    }
    pub fn cross_axis_alignment(mut self, value: Expression<DivPagerItemAlignment>) -> Self {
        self.cross_axis_alignment = Some(value);
        self
    }
    pub fn default_item(mut self, value: Expression<i64>) -> Self {
        self.default_item = Some(value);
        self
    }
    pub fn disappear_actions(mut self, value: Vec<DivDisappearAction>) -> Self {
        self.disappear_actions = Some(value);
        self
    }
    pub fn extensions(mut self, value: Vec<DivExtension>) -> Self {
        self.extensions = Some(value);
        self
    }
    pub fn focus(mut self, value: DivFocus) -> Self {
        self.focus = Some(value);
        self
    }
    pub fn functions(mut self, value: Vec<DivFunction>) -> Self {
        self.functions = Some(value);
        self
    }
    pub fn height(mut self, value: DivSize) -> Self {
        self.height = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn infinite_scroll(mut self, value: Expression<bool>) -> Self {
        self.infinite_scroll = Some(value);
        self
    }
    pub fn item_builder(mut self, value: DivCollectionItemBuilder) -> Self {
        self.item_builder = Some(value);
        self
    }
    pub fn item_spacing(mut self, value: DivFixedSize) -> Self {
        self.item_spacing = Some(value);
        self
    }
    pub fn items(mut self, value: Vec<Div>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn layout_mode(mut self, value: DivPagerLayoutMode) -> Self {
        self.layout_mode = Some(value);
        self
    }
    pub fn layout_provider(mut self, value: DivLayoutProvider) -> Self {
        self.layout_provider = Some(value);
        self
    }
    pub fn margins(mut self, value: DivEdgeInsets) -> Self {
        self.margins = Some(value);
        self
    }
    pub fn orientation(mut self, value: Expression<DivPagerOrientation>) -> Self {
        self.orientation = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn page_transformation(mut self, value: DivPageTransformation) -> Self {
        self.page_transformation = Some(value);
        self
    }
    pub fn restrict_parent_scroll(mut self, value: Expression<bool>) -> Self {
        self.restrict_parent_scroll = Some(value);
        self
    }
    pub fn reuse_id(mut self, value: Expression<String>) -> Self {
        self.reuse_id = Some(value);
        self
    }
    pub fn row_span(mut self, value: Expression<i64>) -> Self {
        self.row_span = Some(value);
        self
    }
    pub fn scroll_axis_alignment(mut self, value: Expression<DivPagerItemAlignment>) -> Self {
        self.scroll_axis_alignment = Some(value);
        self
    }
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
        self
    }
    pub fn tooltips(mut self, value: Vec<DivTooltip>) -> Self {
        self.tooltips = Some(value);
        self
    }
    pub fn transform(mut self, value: DivTransform) -> Self {
        self.transform = Some(value);
        self
    }
    pub fn transformations(mut self, value: Vec<DivTransformation>) -> Self {
        self.transformations = Some(value);
        self
    }
    pub fn transition_change(mut self, value: DivChangeTransition) -> Self {
        self.transition_change = Some(value);
        self
    }
    pub fn transition_in(mut self, value: DivAppearanceTransition) -> Self {
        self.transition_in = Some(value);
        self
    }
    pub fn transition_out(mut self, value: DivAppearanceTransition) -> Self {
        self.transition_out = Some(value);
        self
    }
    pub fn transition_triggers(mut self, value: Vec<DivTransitionTrigger>) -> Self {
        self.transition_triggers = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn variable_triggers(mut self, value: Vec<DivTrigger>) -> Self {
        self.variable_triggers = Some(value);
        self
    }
    pub fn variables(mut self, value: Vec<DivVariable>) -> Self {
        self.variables = Some(value);
        self
    }
    pub fn visibility(mut self, value: Expression<DivVisibility>) -> Self {
        self.visibility = Some(value);
        self
    }
    pub fn visibility_action(mut self, value: DivVisibilityAction) -> Self {
        self.visibility_action = Some(value);
        self
    }
    pub fn visibility_actions(mut self, value: Vec<DivVisibilityAction>) -> Self {
        self.visibility_actions = Some(value);
        self
    }
    pub fn width(mut self, value: DivSize) -> Self {
        self.width = Some(value);
        self
    }
    pub fn build(self) -> DivPager {
        DivPager {
            accessibility: self.accessibility,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivPager::default_alpha())),
            animators: self.animators,
            background: self.background,
            border: self.border,
            column_span: self.column_span,
            cross_axis_alignment: self.cross_axis_alignment.or_else(|| Some(DivPager::default_cross_axis_alignment())),
            default_item: self.default_item.or_else(|| Some(DivPager::default_default_item())),
            disappear_actions: self.disappear_actions,
            extensions: self.extensions,
            focus: self.focus,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivPager::default_height())),
            id: self.id,
            infinite_scroll: self.infinite_scroll.or_else(|| Some(DivPager::default_infinite_scroll())),
            item_builder: self.item_builder,
            item_spacing: self.item_spacing.or_else(|| Some(DivPager::default_item_spacing())),
            items: self.items,
            layout_mode: self.layout_mode.expect("missing required field 'layout_mode'"),
            layout_provider: self.layout_provider,
            margins: self.margins,
            orientation: self.orientation.or_else(|| Some(DivPager::default_orientation())),
            paddings: self.paddings,
            page_transformation: self.page_transformation,
            restrict_parent_scroll: self.restrict_parent_scroll.or_else(|| Some(DivPager::default_restrict_parent_scroll())),
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            scroll_axis_alignment: self.scroll_axis_alignment.or_else(|| Some(DivPager::default_scroll_axis_alignment())),
            selected_actions: self.selected_actions,
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("pager"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivPager::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivPager::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivPagerItemAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "end")]
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivPagerOrientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}
