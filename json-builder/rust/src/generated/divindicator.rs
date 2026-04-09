// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivIndicator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<DivAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_item_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_item_size: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_shape: Option<DivRoundedRectangleShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Expression<DivIndicatorAnimation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animators: Option<Vec<DivAnimator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
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
    pub inactive_item_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_minimum_shape: Option<DivRoundedRectangleShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_shape: Option<DivRoundedRectangleShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_placement: Option<DivIndicatorItemPlacement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_item_size: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pager_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<DivShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_between_centers: Option<DivFixedSize>,
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
    #[serde(default = "DivIndicator::default_type")]
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
pub struct DivIndicatorBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub active_item_color: Option<Expression<String>>,
    pub active_item_size: Option<Expression<f64>>,
    pub active_shape: Option<DivRoundedRectangleShape>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animation: Option<Expression<DivIndicatorAnimation>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub column_span: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub focus: Option<DivFocus>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub id: Option<String>,
    pub inactive_item_color: Option<Expression<String>>,
    pub inactive_minimum_shape: Option<DivRoundedRectangleShape>,
    pub inactive_shape: Option<DivRoundedRectangleShape>,
    pub items_placement: Option<DivIndicatorItemPlacement>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub margins: Option<DivEdgeInsets>,
    pub minimum_item_size: Option<Expression<f64>>,
    pub paddings: Option<DivEdgeInsets>,
    pub pager_id: Option<String>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub shape: Option<DivShape>,
    pub space_between_centers: Option<DivFixedSize>,
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

impl DivIndicator {
    pub fn builder() -> DivIndicatorBuilder {
        DivIndicatorBuilder::default()
    }
    pub fn default_active_item_color() -> Expression<String> { Expression::value("#ffdc60".to_string()) }
    pub fn default_active_item_size() -> Expression<f64> { Expression::value(1.3_f64) }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_animation() -> Expression<DivIndicatorAnimation> { Expression::value(serde_json::from_str(r#""scale""#).unwrap()) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_inactive_item_color() -> Expression<String> { Expression::value("#33919cb5".to_string()) }
    pub fn default_minimum_item_size() -> Expression<f64> { Expression::value(0.5_f64) }
    pub fn default_shape() -> DivShape { serde_json::from_str(r#"{"type":"rounded_rectangle"}"#).unwrap() }
    pub fn default_space_between_centers() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":15}"#).unwrap() }
    pub fn default_type() -> String { "indicator".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivIndicatorBuilder {
    pub fn accessibility(mut self, value: DivAccessibility) -> Self {
        self.accessibility = Some(value);
        self
    }
    pub fn active_item_color(mut self, value: Expression<String>) -> Self {
        self.active_item_color = Some(value);
        self
    }
    pub fn active_item_size(mut self, value: Expression<f64>) -> Self {
        self.active_item_size = Some(value);
        self
    }
    pub fn active_shape(mut self, value: DivRoundedRectangleShape) -> Self {
        self.active_shape = Some(value);
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
    pub fn animation(mut self, value: Expression<DivIndicatorAnimation>) -> Self {
        self.animation = Some(value);
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
    pub fn inactive_item_color(mut self, value: Expression<String>) -> Self {
        self.inactive_item_color = Some(value);
        self
    }
    pub fn inactive_minimum_shape(mut self, value: DivRoundedRectangleShape) -> Self {
        self.inactive_minimum_shape = Some(value);
        self
    }
    pub fn inactive_shape(mut self, value: DivRoundedRectangleShape) -> Self {
        self.inactive_shape = Some(value);
        self
    }
    pub fn items_placement(mut self, value: DivIndicatorItemPlacement) -> Self {
        self.items_placement = Some(value);
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
    pub fn minimum_item_size(mut self, value: Expression<f64>) -> Self {
        self.minimum_item_size = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn pager_id(mut self, value: String) -> Self {
        self.pager_id = Some(value);
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
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
        self
    }
    pub fn shape(mut self, value: DivShape) -> Self {
        self.shape = Some(value);
        self
    }
    pub fn space_between_centers(mut self, value: DivFixedSize) -> Self {
        self.space_between_centers = Some(value);
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
    pub fn build(self) -> DivIndicator {
        DivIndicator {
            accessibility: self.accessibility,
            active_item_color: self.active_item_color.or_else(|| Some(DivIndicator::default_active_item_color())),
            active_item_size: self.active_item_size.or_else(|| Some(DivIndicator::default_active_item_size())),
            active_shape: self.active_shape,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivIndicator::default_alpha())),
            animation: self.animation.or_else(|| Some(DivIndicator::default_animation())),
            animators: self.animators,
            background: self.background,
            border: self.border,
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            extensions: self.extensions,
            focus: self.focus,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivIndicator::default_height())),
            id: self.id,
            inactive_item_color: self.inactive_item_color.or_else(|| Some(DivIndicator::default_inactive_item_color())),
            inactive_minimum_shape: self.inactive_minimum_shape,
            inactive_shape: self.inactive_shape,
            items_placement: self.items_placement,
            layout_provider: self.layout_provider,
            margins: self.margins,
            minimum_item_size: self.minimum_item_size.or_else(|| Some(DivIndicator::default_minimum_item_size())),
            paddings: self.paddings,
            pager_id: self.pager_id,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            selected_actions: self.selected_actions,
            shape: self.shape.or_else(|| Some(DivIndicator::default_shape())),
            space_between_centers: self.space_between_centers.or_else(|| Some(DivIndicator::default_space_between_centers())),
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("indicator"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivIndicator::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivIndicator::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivIndicatorAnimation {
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "worm")]
    Worm,
    #[serde(rename = "slider")]
    Slider,
}
