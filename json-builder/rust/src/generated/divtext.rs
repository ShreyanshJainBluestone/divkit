// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivText {
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
    pub auto_ellipsize: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_focus_on_action: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doubletap_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<DivTextEllipsis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused_text_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_feature_settings: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<Expression<DivFontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight_value: Option<Expression<i64>>,
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
    pub images: Option<Vec<DivTextImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longtap_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_hidden_lines: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_end_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_start_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<DivTextRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<Expression<DivLineStyle>>,
    pub text: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_gradient: Option<DivTextGradient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_shadow: Option<DivShadow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tighten_width: Option<Expression<bool>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<Expression<DivTextTruncate>>,
    #[serde(default = "DivText::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<Expression<DivLineStyle>>,
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
pub struct DivTextBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub action: Option<DivAction>,
    pub action_animation: Option<DivAnimation>,
    pub actions: Option<Vec<DivAction>>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub auto_ellipsize: Option<Expression<bool>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub capture_focus_on_action: Option<Expression<bool>>,
    pub column_span: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub doubletap_actions: Option<Vec<DivAction>>,
    pub ellipsis: Option<DivTextEllipsis>,
    pub extensions: Option<Vec<DivExtension>>,
    pub focus: Option<DivFocus>,
    pub focused_text_color: Option<Expression<String>>,
    pub font_family: Option<Expression<String>>,
    pub font_feature_settings: Option<Expression<String>>,
    pub font_size: Option<Expression<i64>>,
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub font_weight: Option<Expression<DivFontWeight>>,
    pub font_weight_value: Option<Expression<i64>>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub hover_end_actions: Option<Vec<DivAction>>,
    pub hover_start_actions: Option<Vec<DivAction>>,
    pub id: Option<String>,
    pub images: Option<Vec<DivTextImage>>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub letter_spacing: Option<Expression<f64>>,
    pub line_height: Option<Expression<i64>>,
    pub longtap_actions: Option<Vec<DivAction>>,
    pub margins: Option<DivEdgeInsets>,
    pub max_lines: Option<Expression<i64>>,
    pub min_hidden_lines: Option<Expression<i64>>,
    pub paddings: Option<DivEdgeInsets>,
    pub press_end_actions: Option<Vec<DivAction>>,
    pub press_start_actions: Option<Vec<DivAction>>,
    pub ranges: Option<Vec<DivTextRange>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub selectable: Option<Expression<bool>>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub strike: Option<Expression<DivLineStyle>>,
    pub text: Option<Expression<String>>,
    pub text_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub text_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub text_color: Option<Expression<String>>,
    pub text_gradient: Option<DivTextGradient>,
    pub text_shadow: Option<DivShadow>,
    pub tighten_width: Option<Expression<bool>>,
    pub tooltips: Option<Vec<DivTooltip>>,
    pub transform: Option<DivTransform>,
    pub transformations: Option<Vec<DivTransformation>>,
    pub transition_change: Option<DivChangeTransition>,
    pub transition_in: Option<DivAppearanceTransition>,
    pub transition_out: Option<DivAppearanceTransition>,
    pub transition_triggers: Option<Vec<DivTransitionTrigger>>,
    pub truncate: Option<Expression<DivTextTruncate>>,
    pub r#type: Option<String>,
    pub underline: Option<Expression<DivLineStyle>>,
    pub variable_triggers: Option<Vec<DivTrigger>>,
    pub variables: Option<Vec<DivVariable>>,
    pub visibility: Option<Expression<DivVisibility>>,
    pub visibility_action: Option<DivVisibilityAction>,
    pub visibility_actions: Option<Vec<DivVisibilityAction>>,
    pub width: Option<DivSize>,
}

impl DivText {
    pub fn builder() -> DivTextBuilder {
        DivTextBuilder::default()
    }
    pub fn default_action_animation() -> DivAnimation { serde_json::from_str(r#"{"name": "fade", "start_value": 1, "end_value": 0.6, "duration": 100 }"#).unwrap() }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_capture_focus_on_action() -> Expression<bool> { Expression::value(true) }
    pub fn default_font_size() -> Expression<i64> { Expression::value(12i64) }
    pub fn default_font_size_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""sp""#).unwrap()) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_letter_spacing() -> Expression<f64> { Expression::value(0_f64) }
    pub fn default_selectable() -> Expression<bool> { Expression::value(true) }
    pub fn default_strike() -> Expression<DivLineStyle> { Expression::value(serde_json::from_str(r#""none""#).unwrap()) }
    pub fn default_text_alignment_horizontal() -> Expression<DivAlignmentHorizontal> { Expression::value(serde_json::from_str(r#""start""#).unwrap()) }
    pub fn default_text_alignment_vertical() -> Expression<DivAlignmentVertical> { Expression::value(serde_json::from_str(r#""top""#).unwrap()) }
    pub fn default_text_color() -> Expression<String> { Expression::value("#FF000000".to_string()) }
    pub fn default_tighten_width() -> Expression<bool> { Expression::value(true) }
    pub fn default_truncate() -> Expression<DivTextTruncate> { Expression::value(serde_json::from_str(r#""end""#).unwrap()) }
    pub fn default_type() -> String { "text".to_string() }
    pub fn default_underline() -> Expression<DivLineStyle> { Expression::value(serde_json::from_str(r#""none""#).unwrap()) }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivTextBuilder {
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
    pub fn auto_ellipsize(mut self, value: Expression<bool>) -> Self {
        self.auto_ellipsize = Some(value);
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
    pub fn column_span(mut self, value: Expression<i64>) -> Self {
        self.column_span = Some(value);
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
    pub fn ellipsis(mut self, value: DivTextEllipsis) -> Self {
        self.ellipsis = Some(value);
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
    pub fn focused_text_color(mut self, value: Expression<String>) -> Self {
        self.focused_text_color = Some(value);
        self
    }
    pub fn font_family(mut self, value: Expression<String>) -> Self {
        self.font_family = Some(value);
        self
    }
    pub fn font_feature_settings(mut self, value: Expression<String>) -> Self {
        self.font_feature_settings = Some(value);
        self
    }
    pub fn font_size(mut self, value: Expression<i64>) -> Self {
        self.font_size = Some(value);
        self
    }
    pub fn font_size_unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.font_size_unit = Some(value);
        self
    }
    pub fn font_variation_settings(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.font_variation_settings = Some(value);
        self
    }
    pub fn font_weight(mut self, value: Expression<DivFontWeight>) -> Self {
        self.font_weight = Some(value);
        self
    }
    pub fn font_weight_value(mut self, value: Expression<i64>) -> Self {
        self.font_weight_value = Some(value);
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
    pub fn images(mut self, value: Vec<DivTextImage>) -> Self {
        self.images = Some(value);
        self
    }
    pub fn layout_provider(mut self, value: DivLayoutProvider) -> Self {
        self.layout_provider = Some(value);
        self
    }
    pub fn letter_spacing(mut self, value: Expression<f64>) -> Self {
        self.letter_spacing = Some(value);
        self
    }
    pub fn line_height(mut self, value: Expression<i64>) -> Self {
        self.line_height = Some(value);
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
    pub fn max_lines(mut self, value: Expression<i64>) -> Self {
        self.max_lines = Some(value);
        self
    }
    pub fn min_hidden_lines(mut self, value: Expression<i64>) -> Self {
        self.min_hidden_lines = Some(value);
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
    pub fn ranges(mut self, value: Vec<DivTextRange>) -> Self {
        self.ranges = Some(value);
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
    pub fn selectable(mut self, value: Expression<bool>) -> Self {
        self.selectable = Some(value);
        self
    }
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
        self
    }
    pub fn strike(mut self, value: Expression<DivLineStyle>) -> Self {
        self.strike = Some(value);
        self
    }
    pub fn text(mut self, value: Expression<String>) -> Self {
        self.text = Some(value);
        self
    }
    pub fn text_alignment_horizontal(mut self, value: Expression<DivAlignmentHorizontal>) -> Self {
        self.text_alignment_horizontal = Some(value);
        self
    }
    pub fn text_alignment_vertical(mut self, value: Expression<DivAlignmentVertical>) -> Self {
        self.text_alignment_vertical = Some(value);
        self
    }
    pub fn text_color(mut self, value: Expression<String>) -> Self {
        self.text_color = Some(value);
        self
    }
    pub fn text_gradient(mut self, value: DivTextGradient) -> Self {
        self.text_gradient = Some(value);
        self
    }
    pub fn text_shadow(mut self, value: DivShadow) -> Self {
        self.text_shadow = Some(value);
        self
    }
    pub fn tighten_width(mut self, value: Expression<bool>) -> Self {
        self.tighten_width = Some(value);
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
    pub fn truncate(mut self, value: Expression<DivTextTruncate>) -> Self {
        self.truncate = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn underline(mut self, value: Expression<DivLineStyle>) -> Self {
        self.underline = Some(value);
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
    pub fn build(self) -> DivText {
        DivText {
            accessibility: self.accessibility,
            action: self.action,
            action_animation: self.action_animation.or_else(|| Some(DivText::default_action_animation())),
            actions: self.actions,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivText::default_alpha())),
            animators: self.animators,
            auto_ellipsize: self.auto_ellipsize,
            background: self.background,
            border: self.border,
            capture_focus_on_action: self.capture_focus_on_action.or_else(|| Some(DivText::default_capture_focus_on_action())),
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            doubletap_actions: self.doubletap_actions,
            ellipsis: self.ellipsis,
            extensions: self.extensions,
            focus: self.focus,
            focused_text_color: self.focused_text_color,
            font_family: self.font_family,
            font_feature_settings: self.font_feature_settings,
            font_size: self.font_size.or_else(|| Some(DivText::default_font_size())),
            font_size_unit: self.font_size_unit.or_else(|| Some(DivText::default_font_size_unit())),
            font_variation_settings: self.font_variation_settings,
            font_weight: self.font_weight,
            font_weight_value: self.font_weight_value,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivText::default_height())),
            hover_end_actions: self.hover_end_actions,
            hover_start_actions: self.hover_start_actions,
            id: self.id,
            images: self.images,
            layout_provider: self.layout_provider,
            letter_spacing: self.letter_spacing.or_else(|| Some(DivText::default_letter_spacing())),
            line_height: self.line_height,
            longtap_actions: self.longtap_actions,
            margins: self.margins,
            max_lines: self.max_lines,
            min_hidden_lines: self.min_hidden_lines,
            paddings: self.paddings,
            press_end_actions: self.press_end_actions,
            press_start_actions: self.press_start_actions,
            ranges: self.ranges,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            selectable: self.selectable.or_else(|| Some(DivText::default_selectable())),
            selected_actions: self.selected_actions,
            strike: self.strike.or_else(|| Some(DivText::default_strike())),
            text: self.text.expect("missing required field 'text'"),
            text_alignment_horizontal: self.text_alignment_horizontal.or_else(|| Some(DivText::default_text_alignment_horizontal())),
            text_alignment_vertical: self.text_alignment_vertical.or_else(|| Some(DivText::default_text_alignment_vertical())),
            text_color: self.text_color.or_else(|| Some(DivText::default_text_color())),
            text_gradient: self.text_gradient,
            text_shadow: self.text_shadow,
            tighten_width: self.tighten_width.or_else(|| Some(DivText::default_tighten_width())),
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            truncate: self.truncate.or_else(|| Some(DivText::default_truncate())),
            r#type: String::from("text"),
            underline: self.underline.or_else(|| Some(DivText::default_underline())),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivText::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivText::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextEllipsis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<DivTextImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<DivTextRange>>,
    pub text: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextEllipsisBuilder {
    pub actions: Option<Vec<DivAction>>,
    pub images: Option<Vec<DivTextImage>>,
    pub ranges: Option<Vec<DivTextRange>>,
    pub text: Option<Expression<String>>,
}

impl DivTextEllipsis {
    pub fn builder() -> DivTextEllipsisBuilder {
        DivTextEllipsisBuilder::default()
    }
}

impl DivTextEllipsisBuilder {
    pub fn actions(mut self, value: Vec<DivAction>) -> Self {
        self.actions = Some(value);
        self
    }
    pub fn images(mut self, value: Vec<DivTextImage>) -> Self {
        self.images = Some(value);
        self
    }
    pub fn ranges(mut self, value: Vec<DivTextRange>) -> Self {
        self.ranges = Some(value);
        self
    }
    pub fn text(mut self, value: Expression<String>) -> Self {
        self.text = Some(value);
        self
    }
    pub fn build(self) -> DivTextEllipsis {
        DivTextEllipsis {
            actions: self.actions,
            images: self.images,
            ranges: self.ranges,
            text: self.text.expect("missing required field 'text'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<DivTextImageAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_vertical: Option<Expression<DivTextAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_direction: Option<Expression<DivTextImageIndexingDirection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_required: Option<Expression<bool>>,
    pub start: Expression<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_mode: Option<Expression<DivBlendMode>>,
    pub url: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<DivFixedSize>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextImageBuilder {
    pub accessibility: Option<DivTextImageAccessibility>,
    pub alignment_vertical: Option<Expression<DivTextAlignmentVertical>>,
    pub height: Option<DivFixedSize>,
    pub indexing_direction: Option<Expression<DivTextImageIndexingDirection>>,
    pub preload_required: Option<Expression<bool>>,
    pub start: Option<Expression<i64>>,
    pub tint_color: Option<Expression<String>>,
    pub tint_mode: Option<Expression<DivBlendMode>>,
    pub url: Option<Expression<String>>,
    pub width: Option<DivFixedSize>,
}

impl DivTextImage {
    pub fn builder() -> DivTextImageBuilder {
        DivTextImageBuilder::default()
    }
    pub fn default_alignment_vertical() -> Expression<DivTextAlignmentVertical> { Expression::value(serde_json::from_str(r#""center""#).unwrap()) }
    pub fn default_height() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":20}"#).unwrap() }
    pub fn default_indexing_direction() -> Expression<DivTextImageIndexingDirection> { Expression::value(serde_json::from_str(r#""normal""#).unwrap()) }
    pub fn default_preload_required() -> Expression<bool> { Expression::value(true) }
    pub fn default_tint_mode() -> Expression<DivBlendMode> { Expression::value(serde_json::from_str(r#""source_in""#).unwrap()) }
    pub fn default_width() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":20}"#).unwrap() }
}

impl DivTextImageBuilder {
    pub fn accessibility(mut self, value: DivTextImageAccessibility) -> Self {
        self.accessibility = Some(value);
        self
    }
    pub fn alignment_vertical(mut self, value: Expression<DivTextAlignmentVertical>) -> Self {
        self.alignment_vertical = Some(value);
        self
    }
    pub fn height(mut self, value: DivFixedSize) -> Self {
        self.height = Some(value);
        self
    }
    pub fn indexing_direction(mut self, value: Expression<DivTextImageIndexingDirection>) -> Self {
        self.indexing_direction = Some(value);
        self
    }
    pub fn preload_required(mut self, value: Expression<bool>) -> Self {
        self.preload_required = Some(value);
        self
    }
    pub fn start(mut self, value: Expression<i64>) -> Self {
        self.start = Some(value);
        self
    }
    pub fn tint_color(mut self, value: Expression<String>) -> Self {
        self.tint_color = Some(value);
        self
    }
    pub fn tint_mode(mut self, value: Expression<DivBlendMode>) -> Self {
        self.tint_mode = Some(value);
        self
    }
    pub fn url(mut self, value: Expression<String>) -> Self {
        self.url = Some(value);
        self
    }
    pub fn width(mut self, value: DivFixedSize) -> Self {
        self.width = Some(value);
        self
    }
    pub fn build(self) -> DivTextImage {
        DivTextImage {
            accessibility: self.accessibility,
            alignment_vertical: self.alignment_vertical.or_else(|| Some(DivTextImage::default_alignment_vertical())),
            height: self.height.or_else(|| Some(DivTextImage::default_height())),
            indexing_direction: self.indexing_direction.or_else(|| Some(DivTextImage::default_indexing_direction())),
            preload_required: self.preload_required.or_else(|| Some(DivTextImage::default_preload_required())),
            start: self.start.expect("missing required field 'start'"),
            tint_color: self.tint_color,
            tint_mode: self.tint_mode.or_else(|| Some(DivTextImage::default_tint_mode())),
            url: self.url.expect("missing required field 'url'"),
            width: self.width.or_else(|| Some(DivTextImage::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextImageAccessibility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DivTextImageAccessibilityType>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextImageAccessibilityBuilder {
    pub description: Option<Expression<String>>,
    pub r#type: Option<DivTextImageAccessibilityType>,
}

impl DivTextImageAccessibility {
    pub fn builder() -> DivTextImageAccessibilityBuilder {
        DivTextImageAccessibilityBuilder::default()
    }
    pub fn default_type() -> DivTextImageAccessibilityType { serde_json::from_str(r#""auto""#).unwrap() }
}

impl DivTextImageAccessibilityBuilder {
    pub fn description(mut self, value: Expression<String>) -> Self {
        self.description = Some(value);
        self
    }
    pub fn r#type(mut self, value: DivTextImageAccessibilityType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivTextImageAccessibility {
        DivTextImageAccessibility {
            description: self.description,
            r#type: self.r#type.or_else(|| Some(DivTextImageAccessibility::default_type())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTextImageAccessibilityType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTextImageIndexingDirection {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "reversed")]
    Reversed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_vertical: Option<Expression<DivTextAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<DivTextRangeBackground>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_offset: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivTextRangeBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_feature_settings: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<Expression<DivFontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight_value: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<DivTextRangeMask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<Expression<DivLineStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_shadow: Option<DivShadow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<Expression<DivLineStyle>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextRangeBuilder {
    pub actions: Option<Vec<DivAction>>,
    pub alignment_vertical: Option<Expression<DivTextAlignmentVertical>>,
    pub background: Option<DivTextRangeBackground>,
    pub baseline_offset: Option<Expression<f64>>,
    pub border: Option<DivTextRangeBorder>,
    pub end: Option<Expression<i64>>,
    pub font_family: Option<Expression<String>>,
    pub font_feature_settings: Option<Expression<String>>,
    pub font_size: Option<Expression<i64>>,
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub font_weight: Option<Expression<DivFontWeight>>,
    pub font_weight_value: Option<Expression<i64>>,
    pub letter_spacing: Option<Expression<f64>>,
    pub line_height: Option<Expression<i64>>,
    pub mask: Option<DivTextRangeMask>,
    pub start: Option<Expression<i64>>,
    pub strike: Option<Expression<DivLineStyle>>,
    pub text_color: Option<Expression<String>>,
    pub text_shadow: Option<DivShadow>,
    pub top_offset: Option<Expression<i64>>,
    pub underline: Option<Expression<DivLineStyle>>,
}

impl DivTextRange {
    pub fn builder() -> DivTextRangeBuilder {
        DivTextRangeBuilder::default()
    }
    pub fn default_baseline_offset() -> Expression<f64> { Expression::value(0_f64) }
    pub fn default_font_size_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""sp""#).unwrap()) }
    pub fn default_start() -> Expression<i64> { Expression::value(0i64) }
}

impl DivTextRangeBuilder {
    pub fn actions(mut self, value: Vec<DivAction>) -> Self {
        self.actions = Some(value);
        self
    }
    pub fn alignment_vertical(mut self, value: Expression<DivTextAlignmentVertical>) -> Self {
        self.alignment_vertical = Some(value);
        self
    }
    pub fn background(mut self, value: DivTextRangeBackground) -> Self {
        self.background = Some(value);
        self
    }
    pub fn baseline_offset(mut self, value: Expression<f64>) -> Self {
        self.baseline_offset = Some(value);
        self
    }
    pub fn border(mut self, value: DivTextRangeBorder) -> Self {
        self.border = Some(value);
        self
    }
    pub fn end(mut self, value: Expression<i64>) -> Self {
        self.end = Some(value);
        self
    }
    pub fn font_family(mut self, value: Expression<String>) -> Self {
        self.font_family = Some(value);
        self
    }
    pub fn font_feature_settings(mut self, value: Expression<String>) -> Self {
        self.font_feature_settings = Some(value);
        self
    }
    pub fn font_size(mut self, value: Expression<i64>) -> Self {
        self.font_size = Some(value);
        self
    }
    pub fn font_size_unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.font_size_unit = Some(value);
        self
    }
    pub fn font_variation_settings(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.font_variation_settings = Some(value);
        self
    }
    pub fn font_weight(mut self, value: Expression<DivFontWeight>) -> Self {
        self.font_weight = Some(value);
        self
    }
    pub fn font_weight_value(mut self, value: Expression<i64>) -> Self {
        self.font_weight_value = Some(value);
        self
    }
    pub fn letter_spacing(mut self, value: Expression<f64>) -> Self {
        self.letter_spacing = Some(value);
        self
    }
    pub fn line_height(mut self, value: Expression<i64>) -> Self {
        self.line_height = Some(value);
        self
    }
    pub fn mask(mut self, value: DivTextRangeMask) -> Self {
        self.mask = Some(value);
        self
    }
    pub fn start(mut self, value: Expression<i64>) -> Self {
        self.start = Some(value);
        self
    }
    pub fn strike(mut self, value: Expression<DivLineStyle>) -> Self {
        self.strike = Some(value);
        self
    }
    pub fn text_color(mut self, value: Expression<String>) -> Self {
        self.text_color = Some(value);
        self
    }
    pub fn text_shadow(mut self, value: DivShadow) -> Self {
        self.text_shadow = Some(value);
        self
    }
    pub fn top_offset(mut self, value: Expression<i64>) -> Self {
        self.top_offset = Some(value);
        self
    }
    pub fn underline(mut self, value: Expression<DivLineStyle>) -> Self {
        self.underline = Some(value);
        self
    }
    pub fn build(self) -> DivTextRange {
        DivTextRange {
            actions: self.actions,
            alignment_vertical: self.alignment_vertical,
            background: self.background,
            baseline_offset: self.baseline_offset.or_else(|| Some(DivTextRange::default_baseline_offset())),
            border: self.border,
            end: self.end,
            font_family: self.font_family,
            font_feature_settings: self.font_feature_settings,
            font_size: self.font_size,
            font_size_unit: self.font_size_unit.or_else(|| Some(DivTextRange::default_font_size_unit())),
            font_variation_settings: self.font_variation_settings,
            font_weight: self.font_weight,
            font_weight_value: self.font_weight_value,
            letter_spacing: self.letter_spacing,
            line_height: self.line_height,
            mask: self.mask,
            start: self.start.or_else(|| Some(DivTextRange::default_start())),
            strike: self.strike,
            text_color: self.text_color,
            text_shadow: self.text_shadow,
            top_offset: self.top_offset,
            underline: self.underline,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTextTruncate {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "middle")]
    Middle,
}
