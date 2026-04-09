// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInput {
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
    pub autocapitalization: Option<Expression<DivInputAutocapitalization>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<DivBackground>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<DivBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_key_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_key_type: Option<Expression<DivInputEnterKeyType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DivInputFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<Expression<String>>,
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
    pub highlight_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_text: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_type: Option<Expression<DivInputKeyboardType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<DivInputMask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_visible_lines: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_interface: Option<DivInputNativeInterface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_on_focus: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Expression<String>>,
    pub text_variable: String,
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
    #[serde(default = "DivInput::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<DivInputValidator>>,
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
pub struct DivInputBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub autocapitalization: Option<Expression<DivInputAutocapitalization>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub column_span: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub enter_key_actions: Option<Vec<DivAction>>,
    pub enter_key_type: Option<Expression<DivInputEnterKeyType>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub filters: Option<Vec<DivInputFilter>>,
    pub focus: Option<DivFocus>,
    pub font_family: Option<Expression<String>>,
    pub font_size: Option<Expression<i64>>,
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    pub font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub font_weight: Option<Expression<DivFontWeight>>,
    pub font_weight_value: Option<Expression<i64>>,
    pub functions: Option<Vec<DivFunction>>,
    pub height: Option<DivSize>,
    pub highlight_color: Option<Expression<String>>,
    pub hint_color: Option<Expression<String>>,
    pub hint_text: Option<Expression<String>>,
    pub id: Option<String>,
    pub is_enabled: Option<Expression<bool>>,
    pub keyboard_type: Option<Expression<DivInputKeyboardType>>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub letter_spacing: Option<Expression<f64>>,
    pub line_height: Option<Expression<i64>>,
    pub margins: Option<DivEdgeInsets>,
    pub mask: Option<DivInputMask>,
    pub max_length: Option<Expression<i64>>,
    pub max_visible_lines: Option<Expression<i64>>,
    pub native_interface: Option<DivInputNativeInterface>,
    pub paddings: Option<DivEdgeInsets>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub select_all_on_focus: Option<Expression<bool>>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub text_alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub text_alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub text_color: Option<Expression<String>>,
    pub text_variable: Option<String>,
    pub tooltips: Option<Vec<DivTooltip>>,
    pub transform: Option<DivTransform>,
    pub transformations: Option<Vec<DivTransformation>>,
    pub transition_change: Option<DivChangeTransition>,
    pub transition_in: Option<DivAppearanceTransition>,
    pub transition_out: Option<DivAppearanceTransition>,
    pub transition_triggers: Option<Vec<DivTransitionTrigger>>,
    pub r#type: Option<String>,
    pub validators: Option<Vec<DivInputValidator>>,
    pub variable_triggers: Option<Vec<DivTrigger>>,
    pub variables: Option<Vec<DivVariable>>,
    pub visibility: Option<Expression<DivVisibility>>,
    pub visibility_action: Option<DivVisibilityAction>,
    pub visibility_actions: Option<Vec<DivVisibilityAction>>,
    pub width: Option<DivSize>,
}

impl DivInput {
    pub fn builder() -> DivInputBuilder {
        DivInputBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_autocapitalization() -> Expression<DivInputAutocapitalization> { Expression::value(serde_json::from_str(r#""auto""#).unwrap()) }
    pub fn default_enter_key_type() -> Expression<DivInputEnterKeyType> { Expression::value(serde_json::from_str(r#""default""#).unwrap()) }
    pub fn default_font_size() -> Expression<i64> { Expression::value(12i64) }
    pub fn default_font_size_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""sp""#).unwrap()) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_hint_color() -> Expression<String> { Expression::value("#73000000".to_string()) }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_keyboard_type() -> Expression<DivInputKeyboardType> { Expression::value(serde_json::from_str(r#""multi_line_text""#).unwrap()) }
    pub fn default_letter_spacing() -> Expression<f64> { Expression::value(0_f64) }
    pub fn default_select_all_on_focus() -> Expression<bool> { Expression::value(true) }
    pub fn default_text_alignment_horizontal() -> Expression<DivAlignmentHorizontal> { Expression::value(serde_json::from_str(r#""start""#).unwrap()) }
    pub fn default_text_alignment_vertical() -> Expression<DivAlignmentVertical> { Expression::value(serde_json::from_str(r#""center""#).unwrap()) }
    pub fn default_text_color() -> Expression<String> { Expression::value("#FF000000".to_string()) }
    pub fn default_type() -> String { "input".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivInputBuilder {
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
    pub fn autocapitalization(mut self, value: Expression<DivInputAutocapitalization>) -> Self {
        self.autocapitalization = Some(value);
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
    pub fn enter_key_actions(mut self, value: Vec<DivAction>) -> Self {
        self.enter_key_actions = Some(value);
        self
    }
    pub fn enter_key_type(mut self, value: Expression<DivInputEnterKeyType>) -> Self {
        self.enter_key_type = Some(value);
        self
    }
    pub fn extensions(mut self, value: Vec<DivExtension>) -> Self {
        self.extensions = Some(value);
        self
    }
    pub fn filters(mut self, value: Vec<DivInputFilter>) -> Self {
        self.filters = Some(value);
        self
    }
    pub fn focus(mut self, value: DivFocus) -> Self {
        self.focus = Some(value);
        self
    }
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
    pub fn functions(mut self, value: Vec<DivFunction>) -> Self {
        self.functions = Some(value);
        self
    }
    pub fn height(mut self, value: DivSize) -> Self {
        self.height = Some(value);
        self
    }
    pub fn highlight_color(mut self, value: Expression<String>) -> Self {
        self.highlight_color = Some(value);
        self
    }
    pub fn hint_color(mut self, value: Expression<String>) -> Self {
        self.hint_color = Some(value);
        self
    }
    pub fn hint_text(mut self, value: Expression<String>) -> Self {
        self.hint_text = Some(value);
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
    pub fn keyboard_type(mut self, value: Expression<DivInputKeyboardType>) -> Self {
        self.keyboard_type = Some(value);
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
    pub fn margins(mut self, value: DivEdgeInsets) -> Self {
        self.margins = Some(value);
        self
    }
    pub fn mask(mut self, value: DivInputMask) -> Self {
        self.mask = Some(value);
        self
    }
    pub fn max_length(mut self, value: Expression<i64>) -> Self {
        self.max_length = Some(value);
        self
    }
    pub fn max_visible_lines(mut self, value: Expression<i64>) -> Self {
        self.max_visible_lines = Some(value);
        self
    }
    pub fn native_interface(mut self, value: DivInputNativeInterface) -> Self {
        self.native_interface = Some(value);
        self
    }
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
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
    pub fn select_all_on_focus(mut self, value: Expression<bool>) -> Self {
        self.select_all_on_focus = Some(value);
        self
    }
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
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
    pub fn text_variable(mut self, value: String) -> Self {
        self.text_variable = Some(value);
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
    pub fn validators(mut self, value: Vec<DivInputValidator>) -> Self {
        self.validators = Some(value);
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
    pub fn build(self) -> DivInput {
        DivInput {
            accessibility: self.accessibility,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivInput::default_alpha())),
            animators: self.animators,
            autocapitalization: self.autocapitalization.or_else(|| Some(DivInput::default_autocapitalization())),
            background: self.background,
            border: self.border,
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            enter_key_actions: self.enter_key_actions,
            enter_key_type: self.enter_key_type.or_else(|| Some(DivInput::default_enter_key_type())),
            extensions: self.extensions,
            filters: self.filters,
            focus: self.focus,
            font_family: self.font_family,
            font_size: self.font_size.or_else(|| Some(DivInput::default_font_size())),
            font_size_unit: self.font_size_unit.or_else(|| Some(DivInput::default_font_size_unit())),
            font_variation_settings: self.font_variation_settings,
            font_weight: self.font_weight,
            font_weight_value: self.font_weight_value,
            functions: self.functions,
            height: self.height.or_else(|| Some(DivInput::default_height())),
            highlight_color: self.highlight_color,
            hint_color: self.hint_color.or_else(|| Some(DivInput::default_hint_color())),
            hint_text: self.hint_text,
            id: self.id,
            is_enabled: self.is_enabled.or_else(|| Some(DivInput::default_is_enabled())),
            keyboard_type: self.keyboard_type.or_else(|| Some(DivInput::default_keyboard_type())),
            layout_provider: self.layout_provider,
            letter_spacing: self.letter_spacing.or_else(|| Some(DivInput::default_letter_spacing())),
            line_height: self.line_height,
            margins: self.margins,
            mask: self.mask,
            max_length: self.max_length,
            max_visible_lines: self.max_visible_lines,
            native_interface: self.native_interface,
            paddings: self.paddings,
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            select_all_on_focus: self.select_all_on_focus.or_else(|| Some(DivInput::default_select_all_on_focus())),
            selected_actions: self.selected_actions,
            text_alignment_horizontal: self.text_alignment_horizontal.or_else(|| Some(DivInput::default_text_alignment_horizontal())),
            text_alignment_vertical: self.text_alignment_vertical.or_else(|| Some(DivInput::default_text_alignment_vertical())),
            text_color: self.text_color.or_else(|| Some(DivInput::default_text_color())),
            text_variable: self.text_variable.expect("missing required field 'text_variable'"),
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("input"),
            validators: self.validators,
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivInput::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivInput::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivInputAutocapitalization {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "words")]
    Words,
    #[serde(rename = "sentences")]
    Sentences,
    #[serde(rename = "all_characters")]
    AllCharacters,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivInputEnterKeyType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "done")]
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivInputKeyboardType {
    #[serde(rename = "single_line_text")]
    SingleLineText,
    #[serde(rename = "multi_line_text")]
    MultiLineText,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "password")]
    Password,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputNativeInterface {
    pub color: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputNativeInterfaceBuilder {
    pub color: Option<Expression<String>>,
}

impl DivInputNativeInterface {
    pub fn builder() -> DivInputNativeInterfaceBuilder {
        DivInputNativeInterfaceBuilder::default()
    }
}

impl DivInputNativeInterfaceBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn build(self) -> DivInputNativeInterface {
        DivInputNativeInterface {
            color: self.color.expect("missing required field 'color'"),
        }
    }
}
