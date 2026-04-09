// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivVideo {
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
    pub aspect: Option<DivAspect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_variable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatal_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<DivFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_settings_payload: Option<Expression<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_required: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeatable: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Expression<DivVideoScale>>,
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
    #[serde(default = "DivVideo::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_triggers: Option<Vec<DivTrigger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<DivVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_sources: Option<Vec<DivVideoSource>>,
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
pub struct DivVideoBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub aspect: Option<DivAspect>,
    pub autostart: Option<Expression<bool>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub buffering_actions: Option<Vec<DivAction>>,
    pub column_span: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub elapsed_time_variable: Option<String>,
    pub end_actions: Option<Vec<DivAction>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub fatal_actions: Option<Vec<DivAction>>,
    pub focus: Option<DivFocus>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub id: Option<String>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub margins: Option<DivEdgeInsets>,
    pub muted: Option<Expression<bool>>,
    pub paddings: Option<DivEdgeInsets>,
    pub pause_actions: Option<Vec<DivAction>>,
    pub player_settings_payload: Option<Expression<serde_json::Value>>,
    pub preload_required: Option<Expression<bool>>,
    pub preview: Option<Expression<String>>,
    pub repeatable: Option<Expression<bool>>,
    pub resume_actions: Option<Vec<DivAction>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub scale: Option<Expression<DivVideoScale>>,
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
    pub video_sources: Option<Vec<DivVideoSource>>,
    pub visibility: Option<Expression<DivVisibility>>,
    pub visibility_action: Option<DivVisibilityAction>,
    pub visibility_actions: Option<Vec<DivVisibilityAction>>,
    pub width: Option<DivSize>,
}

impl DivVideo {
    pub fn builder() -> DivVideoBuilder {
        DivVideoBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_autostart() -> Expression<bool> { Expression::value(true) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_muted() -> Expression<bool> { Expression::value(true) }
    pub fn default_preload_required() -> Expression<bool> { Expression::value(true) }
    pub fn default_repeatable() -> Expression<bool> { Expression::value(true) }
    pub fn default_scale() -> Expression<DivVideoScale> { Expression::value(serde_json::from_str(r#""fit""#).unwrap()) }
    pub fn default_type() -> String { "video".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivVideoBuilder {
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
    pub fn aspect(mut self, value: DivAspect) -> Self {
        self.aspect = Some(value);
        self
    }
    pub fn autostart(mut self, value: Expression<bool>) -> Self {
        self.autostart = Some(value);
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
    pub fn buffering_actions(mut self, value: Vec<DivAction>) -> Self {
        self.buffering_actions = Some(value);
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
    pub fn elapsed_time_variable(mut self, value: String) -> Self {
        self.elapsed_time_variable = Some(value);
        self
    }
    pub fn end_actions(mut self, value: Vec<DivAction>) -> Self {
        self.end_actions = Some(value);
        self
    }
    pub fn extensions(mut self, value: Vec<DivExtension>) -> Self {
        self.extensions = Some(value);
        self
    }
    pub fn fatal_actions(mut self, value: Vec<DivAction>) -> Self {
        self.fatal_actions = Some(value);
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
    pub fn layout_provider(mut self, value: DivLayoutProvider) -> Self {
        self.layout_provider = Some(value);
        self
    }
    pub fn margins(mut self, value: DivEdgeInsets) -> Self {
        self.margins = Some(value);
        self
    }
    pub fn muted(mut self, value: Expression<bool>) -> Self {
        self.muted = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn pause_actions(mut self, value: Vec<DivAction>) -> Self {
        self.pause_actions = Some(value);
        self
    }
    pub fn player_settings_payload(mut self, value: Expression<serde_json::Value>) -> Self {
        self.player_settings_payload = Some(value);
        self
    }
    pub fn preload_required(mut self, value: Expression<bool>) -> Self {
        self.preload_required = Some(value);
        self
    }
    pub fn preview(mut self, value: Expression<String>) -> Self {
        self.preview = Some(value);
        self
    }
    pub fn repeatable(mut self, value: Expression<bool>) -> Self {
        self.repeatable = Some(value);
        self
    }
    pub fn resume_actions(mut self, value: Vec<DivAction>) -> Self {
        self.resume_actions = Some(value);
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
    pub fn scale(mut self, value: Expression<DivVideoScale>) -> Self {
        self.scale = Some(value);
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
    pub fn video_sources(mut self, value: Vec<DivVideoSource>) -> Self {
        self.video_sources = Some(value);
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
    pub fn build(self) -> DivVideo {
        DivVideo {
            accessibility: self.accessibility,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivVideo::default_alpha())),
            animators: self.animators,
            aspect: self.aspect,
            autostart: self.autostart.or_else(|| Some(DivVideo::default_autostart())),
            background: self.background,
            border: self.border,
            buffering_actions: self.buffering_actions,
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            elapsed_time_variable: self.elapsed_time_variable,
            end_actions: self.end_actions,
            extensions: self.extensions,
            fatal_actions: self.fatal_actions,
            focus: self.focus,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivVideo::default_height())),
            id: self.id,
            layout_provider: self.layout_provider,
            margins: self.margins,
            muted: self.muted.or_else(|| Some(DivVideo::default_muted())),
            paddings: self.paddings,
            pause_actions: self.pause_actions,
            player_settings_payload: self.player_settings_payload,
            preload_required: self.preload_required.or_else(|| Some(DivVideo::default_preload_required())),
            preview: self.preview,
            repeatable: self.repeatable.or_else(|| Some(DivVideo::default_repeatable())),
            resume_actions: self.resume_actions,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            scale: self.scale.or_else(|| Some(DivVideo::default_scale())),
            selected_actions: self.selected_actions,
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("video"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            video_sources: self.video_sources,
            visibility: self.visibility.or_else(|| Some(DivVideo::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivVideo::default_width())),
        }
    }
}
