// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivGrid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<DivAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DivAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_animation: Option<DivAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DivAction>>,
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
    pub capture_focus_on_action: Option<Expression<bool>>,
    pub column_count: Expression<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doubletap_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<DivFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_end_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_start_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Div>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longtap_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_end_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_start_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
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
    #[serde(default = "DivGrid::default_type")]
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
pub struct DivGridBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub action: Option<DivAction>,
    pub action_animation: Option<DivAnimation>,
    pub actions: Option<Vec<DivAction>>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub capture_focus_on_action: Option<Expression<bool>>,
    pub column_count: Option<Expression<i64>>,
    pub column_span: Option<Expression<i64>>,
    pub content_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub content_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub doubletap_actions: Option<Vec<DivAction>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub focus: Option<DivFocus>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub hover_end_actions: Option<Vec<DivAction>>,
    pub hover_start_actions: Option<Vec<DivAction>>,
    pub id: Option<String>,
    pub items: Option<Vec<Div>>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub longtap_actions: Option<Vec<DivAction>>,
    pub margins: Option<DivEdgeInsets>,
    pub paddings: Option<DivEdgeInsets>,
    pub press_end_actions: Option<Vec<DivAction>>,
    pub press_start_actions: Option<Vec<DivAction>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
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

impl DivGrid {
    pub fn builder() -> DivGridBuilder {
        DivGridBuilder::default()
    }
    pub fn default_action_animation() -> DivAnimation { serde_json::from_str(r#"{"name": "fade", "start_value": 1, "end_value": 0.6, "duration": 100 }"#).unwrap() }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_capture_focus_on_action() -> Expression<bool> { Expression::value(true) }
    pub fn default_content_alignment_horizontal() -> Expression<DivAlignmentHorizontal> { Expression::value(serde_json::from_str(r#""start""#).unwrap()) }
    pub fn default_content_alignment_vertical() -> Expression<DivAlignmentVertical> { Expression::value(serde_json::from_str(r#""top""#).unwrap()) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_type() -> String { "grid".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivGridBuilder {
    pub fn accessibility(mut self, value: DivAccessibility) -> Self {
        self.accessibility = Some(value);
        self
    }
    pub fn action(mut self, value: DivAction) -> Self {
        self.action = Some(value);
        self
    }
    pub fn action_animation(mut self, value: DivAnimation) -> Self {
        self.action_animation = Some(value);
        self
    }
    pub fn actions(mut self, value: Vec<DivAction>) -> Self {
        self.actions = Some(value);
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
    pub fn capture_focus_on_action(mut self, value: Expression<bool>) -> Self {
        self.capture_focus_on_action = Some(value);
        self
    }
    pub fn column_count(mut self, value: Expression<i64>) -> Self {
        self.column_count = Some(value);
        self
    }
    pub fn column_span(mut self, value: Expression<i64>) -> Self {
        self.column_span = Some(value);
        self
    }
    pub fn content_alignment_horizontal(mut self, value: Expression<DivAlignmentHorizontal>) -> Self {
        self.content_alignment_horizontal = Some(value);
        self
    }
    pub fn content_alignment_vertical(mut self, value: Expression<DivAlignmentVertical>) -> Self {
        self.content_alignment_vertical = Some(value);
        self
    }
    pub fn disappear_actions(mut self, value: Vec<DivDisappearAction>) -> Self {
        self.disappear_actions = Some(value);
        self
    }
    pub fn doubletap_actions(mut self, value: Vec<DivAction>) -> Self {
        self.doubletap_actions = Some(value);
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
    pub fn hover_end_actions(mut self, value: Vec<DivAction>) -> Self {
        self.hover_end_actions = Some(value);
        self
    }
    pub fn hover_start_actions(mut self, value: Vec<DivAction>) -> Self {
        self.hover_start_actions = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn items(mut self, value: Vec<Div>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn layout_provider(mut self, value: DivLayoutProvider) -> Self {
        self.layout_provider = Some(value);
        self
    }
    pub fn longtap_actions(mut self, value: Vec<DivAction>) -> Self {
        self.longtap_actions = Some(value);
        self
    }
    pub fn margins(mut self, value: DivEdgeInsets) -> Self {
        self.margins = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn press_end_actions(mut self, value: Vec<DivAction>) -> Self {
        self.press_end_actions = Some(value);
        self
    }
    pub fn press_start_actions(mut self, value: Vec<DivAction>) -> Self {
        self.press_start_actions = Some(value);
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
    pub fn build(self) -> DivGrid {
        DivGrid {
            accessibility: self.accessibility,
            action: self.action,
            action_animation: self.action_animation.or_else(|| Some(DivGrid::default_action_animation())),
            actions: self.actions,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivGrid::default_alpha())),
            animators: self.animators,
            background: self.background,
            border: self.border,
            capture_focus_on_action: self.capture_focus_on_action.or_else(|| Some(DivGrid::default_capture_focus_on_action())),
            column_count: self.column_count.expect("missing required field 'column_count'"),
            column_span: self.column_span,
            content_alignment_horizontal: self.content_alignment_horizontal.or_else(|| Some(DivGrid::default_content_alignment_horizontal())),
            content_alignment_vertical: self.content_alignment_vertical.or_else(|| Some(DivGrid::default_content_alignment_vertical())),
            disappear_actions: self.disappear_actions,
            doubletap_actions: self.doubletap_actions,
            extensions: self.extensions,
            focus: self.focus,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivGrid::default_height())),
            hover_end_actions: self.hover_end_actions,
            hover_start_actions: self.hover_start_actions,
            id: self.id,
            items: self.items,
            layout_provider: self.layout_provider,
            longtap_actions: self.longtap_actions,
            margins: self.margins,
            paddings: self.paddings,
            press_end_actions: self.press_end_actions,
            press_start_actions: self.press_start_actions,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            selected_actions: self.selected_actions,
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("grid"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivGrid::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivGrid::default_width())),
        }
    }
}
