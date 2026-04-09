// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSlider {
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
    pub is_enabled: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<DivSliderRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_value_accessibility: Option<DivAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_secondary_style: Option<DivDrawable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_secondary_text_style: Option<DivSliderTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_secondary_value_variable: Option<String>,
    pub thumb_style: DivDrawable,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_text_style: Option<DivSliderTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_value_variable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_mark_active_style: Option<DivDrawable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_mark_inactive_style: Option<DivDrawable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<Vec<DivTooltip>>,
    pub track_active_style: DivDrawable,
    pub track_inactive_style: DivDrawable,
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
    #[serde(default = "DivSlider::default_type")]
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
pub struct DivSliderBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
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
    pub is_enabled: Option<Expression<bool>>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub margins: Option<DivEdgeInsets>,
    pub max_value: Option<Expression<i64>>,
    pub min_value: Option<Expression<i64>>,
    pub paddings: Option<DivEdgeInsets>,
    pub ranges: Option<Vec<DivSliderRange>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub secondary_value_accessibility: Option<DivAccessibility>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub thumb_secondary_style: Option<DivDrawable>,
    pub thumb_secondary_text_style: Option<DivSliderTextStyle>,
    pub thumb_secondary_value_variable: Option<String>,
    pub thumb_style: Option<DivDrawable>,
    pub thumb_text_style: Option<DivSliderTextStyle>,
    pub thumb_value_variable: Option<String>,
    pub tick_mark_active_style: Option<DivDrawable>,
    pub tick_mark_inactive_style: Option<DivDrawable>,
    pub tooltips: Option<Vec<DivTooltip>>,
    pub track_active_style: Option<DivDrawable>,
    pub track_inactive_style: Option<DivDrawable>,
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

impl DivSlider {
    pub fn builder() -> DivSliderBuilder {
        DivSliderBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_max_value() -> Expression<i64> { Expression::value(100i64) }
    pub fn default_min_value() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_type() -> String { "slider".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivSliderBuilder {
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
    pub fn is_enabled(mut self, value: Expression<bool>) -> Self {
        self.is_enabled = Some(value);
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
    pub fn max_value(mut self, value: Expression<i64>) -> Self {
        self.max_value = Some(value);
        self
    }
    pub fn min_value(mut self, value: Expression<i64>) -> Self {
        self.min_value = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn ranges(mut self, value: Vec<DivSliderRange>) -> Self {
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
    pub fn secondary_value_accessibility(mut self, value: DivAccessibility) -> Self {
        self.secondary_value_accessibility = Some(value);
        self
    }
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
        self
    }
    pub fn thumb_secondary_style(mut self, value: DivDrawable) -> Self {
        self.thumb_secondary_style = Some(value);
        self
    }
    pub fn thumb_secondary_text_style(mut self, value: DivSliderTextStyle) -> Self {
        self.thumb_secondary_text_style = Some(value);
        self
    }
    pub fn thumb_secondary_value_variable(mut self, value: String) -> Self {
        self.thumb_secondary_value_variable = Some(value);
        self
    }
    pub fn thumb_style(mut self, value: DivDrawable) -> Self {
        self.thumb_style = Some(value);
        self
    }
    pub fn thumb_text_style(mut self, value: DivSliderTextStyle) -> Self {
        self.thumb_text_style = Some(value);
        self
    }
    pub fn thumb_value_variable(mut self, value: String) -> Self {
        self.thumb_value_variable = Some(value);
        self
    }
    pub fn tick_mark_active_style(mut self, value: DivDrawable) -> Self {
        self.tick_mark_active_style = Some(value);
        self
    }
    pub fn tick_mark_inactive_style(mut self, value: DivDrawable) -> Self {
        self.tick_mark_inactive_style = Some(value);
        self
    }
    pub fn tooltips(mut self, value: Vec<DivTooltip>) -> Self {
        self.tooltips = Some(value);
        self
    }
    pub fn track_active_style(mut self, value: DivDrawable) -> Self {
        self.track_active_style = Some(value);
        self
    }
    pub fn track_inactive_style(mut self, value: DivDrawable) -> Self {
        self.track_inactive_style = Some(value);
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
    pub fn build(self) -> DivSlider {
        DivSlider {
            accessibility: self.accessibility,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivSlider::default_alpha())),
            animators: self.animators,
            background: self.background,
            border: self.border,
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            extensions: self.extensions,
            focus: self.focus,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivSlider::default_height())),
            id: self.id,
            is_enabled: self.is_enabled.or_else(|| Some(DivSlider::default_is_enabled())),
            layout_provider: self.layout_provider,
            margins: self.margins,
            max_value: self.max_value.or_else(|| Some(DivSlider::default_max_value())),
            min_value: self.min_value.or_else(|| Some(DivSlider::default_min_value())),
            paddings: self.paddings,
            ranges: self.ranges,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            secondary_value_accessibility: self.secondary_value_accessibility,
            selected_actions: self.selected_actions,
            thumb_secondary_style: self.thumb_secondary_style,
            thumb_secondary_text_style: self.thumb_secondary_text_style,
            thumb_secondary_value_variable: self.thumb_secondary_value_variable,
            thumb_style: self.thumb_style.expect("missing required field 'thumb_style'"),
            thumb_text_style: self.thumb_text_style,
            thumb_value_variable: self.thumb_value_variable,
            tick_mark_active_style: self.tick_mark_active_style,
            tick_mark_inactive_style: self.tick_mark_inactive_style,
            tooltips: self.tooltips,
            track_active_style: self.track_active_style.expect("missing required field 'track_active_style'"),
            track_inactive_style: self.track_inactive_style.expect("missing required field 'track_inactive_style'"),
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("slider"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivSlider::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivSlider::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSliderRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_active_style: Option<DivDrawable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_inactive_style: Option<DivDrawable>,
}

#[derive(Debug, Clone, Default)]
pub struct DivSliderRangeBuilder {
    pub end: Option<Expression<i64>>,
    pub margins: Option<DivEdgeInsets>,
    pub start: Option<Expression<i64>>,
    pub track_active_style: Option<DivDrawable>,
    pub track_inactive_style: Option<DivDrawable>,
}

impl DivSliderRange {
    pub fn builder() -> DivSliderRangeBuilder {
        DivSliderRangeBuilder::default()
    }
}

impl DivSliderRangeBuilder {
    pub fn end(mut self, value: Expression<i64>) -> Self {
        self.end = Some(value);
        self
    }
    pub fn margins(mut self, value: DivEdgeInsets) -> Self {
        self.margins = Some(value);
        self
    }
    pub fn start(mut self, value: Expression<i64>) -> Self {
        self.start = Some(value);
        self
    }
    pub fn track_active_style(mut self, value: DivDrawable) -> Self {
        self.track_active_style = Some(value);
        self
    }
    pub fn track_inactive_style(mut self, value: DivDrawable) -> Self {
        self.track_inactive_style = Some(value);
        self
    }
    pub fn build(self) -> DivSliderRange {
        DivSliderRange {
            end: self.end,
            margins: self.margins,
            start: self.start,
            track_active_style: self.track_active_style,
            track_inactive_style: self.track_inactive_style,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSliderTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<Expression<String>>,
    pub font_size: Expression<i64>,
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
    pub offset: Option<DivPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Expression<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivSliderTextStyleBuilder {
    pub font_family: Option<Expression<String>>,
    pub font_size: Option<Expression<i64>>,
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub font_weight: Option<Expression<DivFontWeight>>,
    pub font_weight_value: Option<Expression<i64>>,
    pub letter_spacing: Option<Expression<f64>>,
    pub offset: Option<DivPoint>,
    pub text_color: Option<Expression<String>>,
}

impl DivSliderTextStyle {
    pub fn builder() -> DivSliderTextStyleBuilder {
        DivSliderTextStyleBuilder::default()
    }
    pub fn default_font_size() -> Expression<i64> { Expression::value(12i64) }
    pub fn default_font_size_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""sp""#).unwrap()) }
    pub fn default_letter_spacing() -> Expression<f64> { Expression::value(0_f64) }
    pub fn default_text_color() -> Expression<String> { Expression::value("#FF000000".to_string()) }
}

impl DivSliderTextStyleBuilder {
    pub fn font_family(mut self, value: Expression<String>) -> Self {
        self.font_family = Some(value);
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
    pub fn offset(mut self, value: DivPoint) -> Self {
        self.offset = Some(value);
        self
    }
    pub fn text_color(mut self, value: Expression<String>) -> Self {
        self.text_color = Some(value);
        self
    }
    pub fn build(self) -> DivSliderTextStyle {
        DivSliderTextStyle {
            font_family: self.font_family,
            font_size: self.font_size.or_else(|| Some(DivSliderTextStyle::default_font_size())).expect("missing required field 'font_size'"),
            font_size_unit: self.font_size_unit.or_else(|| Some(DivSliderTextStyle::default_font_size_unit())),
            font_variation_settings: self.font_variation_settings,
            font_weight: self.font_weight,
            font_weight_value: self.font_weight_value,
            letter_spacing: self.letter_spacing.or_else(|| Some(DivSliderTextStyle::default_letter_spacing())),
            offset: self.offset,
            text_color: self.text_color.or_else(|| Some(DivSliderTextStyle::default_text_color())),
        }
    }
}
