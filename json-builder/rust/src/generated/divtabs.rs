// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTabs {
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
    pub dynamic_height: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<DivExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<DivFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<DivFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_separator: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub items: Vec<DivTabsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_provider: Option<DivLayoutProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_parent_scroll: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_tab: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_paddings: Option<DivEdgeInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_tabs_by_content_swipe_enabled: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_title_delimiter: Option<DivTabsTabTitleDelimiter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_title_style: Option<DivTabsTabTitleStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_paddings: Option<DivEdgeInsets>,
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
    #[serde(default = "DivTabs::default_type")]
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
pub struct DivTabsBuilder {
    pub accessibility: Option<DivAccessibility>,
    pub alignment_horizontal: Option<Expression<DivAlignmentHorizontal>>,
    pub alignment_vertical: Option<Expression<DivAlignmentVertical>>,
    pub alpha: Option<Expression<f64>>,
    pub animators: Option<Vec<DivAnimator>>,
    pub background: Option<Vec<DivBackground>>,
    pub border: Option<DivBorder>,
    pub column_span: Option<Expression<i64>>,
    pub disappear_actions: Option<Vec<DivDisappearAction>>,
    pub dynamic_height: Option<Expression<bool>>,
    pub extensions: Option<Vec<DivExtension>>,
    pub focus: Option<DivFocus>,
    pub functions: Option<Vec<DivFunction>>,
    pub has_separator: Option<Expression<bool>>,
    pub height: Option<DivSize>,
    pub id: Option<String>,
    pub items: Option<Vec<DivTabsItem>>,
    pub layout_provider: Option<DivLayoutProvider>,
    pub margins: Option<DivEdgeInsets>,
    pub paddings: Option<DivEdgeInsets>,
    pub restrict_parent_scroll: Option<Expression<bool>>,
    pub reuse_id: Option<Expression<String>>,
    pub row_span: Option<Expression<i64>>,
    pub selected_actions: Option<Vec<DivAction>>,
    pub selected_tab: Option<Expression<i64>>,
    pub separator_color: Option<Expression<String>>,
    pub separator_paddings: Option<DivEdgeInsets>,
    pub switch_tabs_by_content_swipe_enabled: Option<Expression<bool>>,
    pub tab_title_delimiter: Option<DivTabsTabTitleDelimiter>,
    pub tab_title_style: Option<DivTabsTabTitleStyle>,
    pub title_paddings: Option<DivEdgeInsets>,
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

impl DivTabs {
    pub fn builder() -> DivTabsBuilder {
        DivTabsBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_dynamic_height() -> Expression<bool> { Expression::value(true) }
    pub fn default_has_separator() -> Expression<bool> { Expression::value(true) }
    pub fn default_height() -> DivSize { serde_json::from_str(r#"{"type": "wrap_content"}"#).unwrap() }
    pub fn default_restrict_parent_scroll() -> Expression<bool> { Expression::value(true) }
    pub fn default_selected_tab() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_separator_color() -> Expression<String> { Expression::value("#14000000".to_string()) }
    pub fn default_separator_paddings() -> DivEdgeInsets { serde_json::from_str(r#"{"left": 12,"right": 12,"top": 0,"bottom": 0}"#).unwrap() }
    pub fn default_switch_tabs_by_content_swipe_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_title_paddings() -> DivEdgeInsets { serde_json::from_str(r#"{"left": 12,"right": 12,"top": 0,"bottom": 8}"#).unwrap() }
    pub fn default_type() -> String { "tabs".to_string() }
    pub fn default_visibility() -> Expression<DivVisibility> { Expression::value(serde_json::from_str(r#""visible""#).unwrap()) }
    pub fn default_width() -> DivSize { serde_json::from_str(r#"{"type": "match_parent"}"#).unwrap() }
}

impl DivTabsBuilder {
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
    pub fn dynamic_height(mut self, value: Expression<bool>) -> Self {
        self.dynamic_height = Some(value);
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
    pub fn has_separator(mut self, value: Expression<bool>) -> Self {
        self.has_separator = Some(value);
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
    pub fn items(mut self, value: Vec<DivTabsItem>) -> Self {
        self.items = Some(value);
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
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
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
    pub fn selected_actions(mut self, value: Vec<DivAction>) -> Self {
        self.selected_actions = Some(value);
        self
    }
    pub fn selected_tab(mut self, value: Expression<i64>) -> Self {
        self.selected_tab = Some(value);
        self
    }
    pub fn separator_color(mut self, value: Expression<String>) -> Self {
        self.separator_color = Some(value);
        self
    }
    pub fn separator_paddings(mut self, value: DivEdgeInsets) -> Self {
        self.separator_paddings = Some(value);
        self
    }
    pub fn switch_tabs_by_content_swipe_enabled(mut self, value: Expression<bool>) -> Self {
        self.switch_tabs_by_content_swipe_enabled = Some(value);
        self
    }
    pub fn tab_title_delimiter(mut self, value: DivTabsTabTitleDelimiter) -> Self {
        self.tab_title_delimiter = Some(value);
        self
    }
    pub fn tab_title_style(mut self, value: DivTabsTabTitleStyle) -> Self {
        self.tab_title_style = Some(value);
        self
    }
    pub fn title_paddings(mut self, value: DivEdgeInsets) -> Self {
        self.title_paddings = Some(value);
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
    pub fn build(self) -> DivTabs {
        DivTabs {
            accessibility: self.accessibility,
            alignment_horizontal: self.alignment_horizontal,
            alignment_vertical: self.alignment_vertical,
            alpha: self.alpha.or_else(|| Some(DivTabs::default_alpha())),
            animators: self.animators,
            background: self.background,
            border: self.border,
            column_span: self.column_span,
            disappear_actions: self.disappear_actions,
            dynamic_height: self.dynamic_height.or_else(|| Some(DivTabs::default_dynamic_height())),
            extensions: self.extensions,
            focus: self.focus,
            functions: self.functions,
            has_separator: self.has_separator.or_else(|| Some(DivTabs::default_has_separator())),
            height: self.height.or_else(|| Some(DivTabs::default_height())),
            id: self.id,
            items: self.items.expect("missing required field 'items'"),
            layout_provider: self.layout_provider,
            margins: self.margins,
            paddings: self.paddings,
            restrict_parent_scroll: self.restrict_parent_scroll.or_else(|| Some(DivTabs::default_restrict_parent_scroll())),
            reuse_id: self.reuse_id,
            row_span: self.row_span,
            selected_actions: self.selected_actions,
            selected_tab: self.selected_tab.or_else(|| Some(DivTabs::default_selected_tab())),
            separator_color: self.separator_color.or_else(|| Some(DivTabs::default_separator_color())),
            separator_paddings: self.separator_paddings.or_else(|| Some(DivTabs::default_separator_paddings())),
            switch_tabs_by_content_swipe_enabled: self.switch_tabs_by_content_swipe_enabled.or_else(|| Some(DivTabs::default_switch_tabs_by_content_swipe_enabled())),
            tab_title_delimiter: self.tab_title_delimiter,
            tab_title_style: self.tab_title_style,
            title_paddings: self.title_paddings.or_else(|| Some(DivTabs::default_title_paddings())),
            tooltips: self.tooltips,
            transform: self.transform,
            transformations: self.transformations,
            transition_change: self.transition_change,
            transition_in: self.transition_in,
            transition_out: self.transition_out,
            transition_triggers: self.transition_triggers,
            r#type: String::from("tabs"),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
            visibility: self.visibility.or_else(|| Some(DivTabs::default_visibility())),
            visibility_action: self.visibility_action,
            visibility_actions: self.visibility_actions,
            width: self.width.or_else(|| Some(DivTabs::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTabsItem {
    pub div: Div,
    pub title: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_click_action: Option<DivAction>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTabsItemBuilder {
    pub div: Option<Div>,
    pub title: Option<Expression<String>>,
    pub title_click_action: Option<DivAction>,
}

impl DivTabsItem {
    pub fn builder() -> DivTabsItemBuilder {
        DivTabsItemBuilder::default()
    }
}

impl DivTabsItemBuilder {
    pub fn div(mut self, value: Div) -> Self {
        self.div = Some(value);
        self
    }
    pub fn title(mut self, value: Expression<String>) -> Self {
        self.title = Some(value);
        self
    }
    pub fn title_click_action(mut self, value: DivAction) -> Self {
        self.title_click_action = Some(value);
        self
    }
    pub fn build(self) -> DivTabsItem {
        DivTabsItem {
            div: self.div.expect("missing required field 'div'"),
            title: self.title.expect("missing required field 'title'"),
            title_click_action: self.title_click_action,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTabsTabTitleDelimiter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<DivFixedSize>,
    pub image_url: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<DivFixedSize>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTabsTabTitleDelimiterBuilder {
    pub height: Option<DivFixedSize>,
    pub image_url: Option<Expression<String>>,
    pub width: Option<DivFixedSize>,
}

impl DivTabsTabTitleDelimiter {
    pub fn builder() -> DivTabsTabTitleDelimiterBuilder {
        DivTabsTabTitleDelimiterBuilder::default()
    }
    pub fn default_height() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed", "value": 12}"#).unwrap() }
    pub fn default_width() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed", "value": 12}"#).unwrap() }
}

impl DivTabsTabTitleDelimiterBuilder {
    pub fn height(mut self, value: DivFixedSize) -> Self {
        self.height = Some(value);
        self
    }
    pub fn image_url(mut self, value: Expression<String>) -> Self {
        self.image_url = Some(value);
        self
    }
    pub fn width(mut self, value: DivFixedSize) -> Self {
        self.width = Some(value);
        self
    }
    pub fn build(self) -> DivTabsTabTitleDelimiter {
        DivTabsTabTitleDelimiter {
            height: self.height.or_else(|| Some(DivTabsTabTitleDelimiter::default_height())),
            image_url: self.image_url.expect("missing required field 'image_url'"),
            width: self.width.or_else(|| Some(DivTabsTabTitleDelimiter::default_width())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTabsTabTitleStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_background_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_font_weight: Option<Expression<DivFontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_font_weight_value: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_text_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<Expression<DivTabsTabTitleStyleAnimationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corners_radius: Option<DivCornersRadius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<Expression<DivFontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_background_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_font_weight: Option<Expression<DivFontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_font_weight_value: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_text_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddings: Option<DivEdgeInsets>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTabsTabTitleStyleBuilder {
    pub active_background_color: Option<Expression<String>>,
    pub active_font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub active_font_weight: Option<Expression<DivFontWeight>>,
    pub active_font_weight_value: Option<Expression<i64>>,
    pub active_text_color: Option<Expression<String>>,
    pub animation_duration: Option<Expression<i64>>,
    pub animation_type: Option<Expression<DivTabsTabTitleStyleAnimationType>>,
    pub corner_radius: Option<Expression<i64>>,
    pub corners_radius: Option<DivCornersRadius>,
    pub font_family: Option<Expression<String>>,
    pub font_size: Option<Expression<i64>>,
    pub font_size_unit: Option<Expression<DivSizeUnit>>,
    pub font_weight: Option<Expression<DivFontWeight>>,
    pub inactive_background_color: Option<Expression<String>>,
    pub inactive_font_variation_settings: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
    pub inactive_font_weight: Option<Expression<DivFontWeight>>,
    pub inactive_font_weight_value: Option<Expression<i64>>,
    pub inactive_text_color: Option<Expression<String>>,
    pub item_spacing: Option<Expression<i64>>,
    pub letter_spacing: Option<Expression<f64>>,
    pub line_height: Option<Expression<i64>>,
    pub paddings: Option<DivEdgeInsets>,
}

impl DivTabsTabTitleStyle {
    pub fn builder() -> DivTabsTabTitleStyleBuilder {
        DivTabsTabTitleStyleBuilder::default()
    }
    pub fn default_active_background_color() -> Expression<String> { Expression::value("#FFFFDC60".to_string()) }
    pub fn default_active_text_color() -> Expression<String> { Expression::value("#CC000000".to_string()) }
    pub fn default_animation_duration() -> Expression<i64> { Expression::value(300i64) }
    pub fn default_animation_type() -> Expression<DivTabsTabTitleStyleAnimationType> { Expression::value(serde_json::from_str(r#""slide""#).unwrap()) }
    pub fn default_font_size() -> Expression<i64> { Expression::value(12i64) }
    pub fn default_font_size_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""sp""#).unwrap()) }
    pub fn default_font_weight() -> Expression<DivFontWeight> { Expression::value(serde_json::from_str(r#""regular""#).unwrap()) }
    pub fn default_inactive_text_color() -> Expression<String> { Expression::value("#80000000".to_string()) }
    pub fn default_item_spacing() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_letter_spacing() -> Expression<f64> { Expression::value(0_f64) }
    pub fn default_paddings() -> DivEdgeInsets { serde_json::from_str(r#"{"left": 8,"right": 8,"top": 6,"bottom": 6}"#).unwrap() }
}

impl DivTabsTabTitleStyleBuilder {
    pub fn active_background_color(mut self, value: Expression<String>) -> Self {
        self.active_background_color = Some(value);
        self
    }
    pub fn active_font_variation_settings(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.active_font_variation_settings = Some(value);
        self
    }
    pub fn active_font_weight(mut self, value: Expression<DivFontWeight>) -> Self {
        self.active_font_weight = Some(value);
        self
    }
    pub fn active_font_weight_value(mut self, value: Expression<i64>) -> Self {
        self.active_font_weight_value = Some(value);
        self
    }
    pub fn active_text_color(mut self, value: Expression<String>) -> Self {
        self.active_text_color = Some(value);
        self
    }
    pub fn animation_duration(mut self, value: Expression<i64>) -> Self {
        self.animation_duration = Some(value);
        self
    }
    pub fn animation_type(mut self, value: Expression<DivTabsTabTitleStyleAnimationType>) -> Self {
        self.animation_type = Some(value);
        self
    }
    pub fn corner_radius(mut self, value: Expression<i64>) -> Self {
        self.corner_radius = Some(value);
        self
    }
    pub fn corners_radius(mut self, value: DivCornersRadius) -> Self {
        self.corners_radius = Some(value);
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
    pub fn font_weight(mut self, value: Expression<DivFontWeight>) -> Self {
        self.font_weight = Some(value);
        self
    }
    pub fn inactive_background_color(mut self, value: Expression<String>) -> Self {
        self.inactive_background_color = Some(value);
        self
    }
    pub fn inactive_font_variation_settings(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.inactive_font_variation_settings = Some(value);
        self
    }
    pub fn inactive_font_weight(mut self, value: Expression<DivFontWeight>) -> Self {
        self.inactive_font_weight = Some(value);
        self
    }
    pub fn inactive_font_weight_value(mut self, value: Expression<i64>) -> Self {
        self.inactive_font_weight_value = Some(value);
        self
    }
    pub fn inactive_text_color(mut self, value: Expression<String>) -> Self {
        self.inactive_text_color = Some(value);
        self
    }
    pub fn item_spacing(mut self, value: Expression<i64>) -> Self {
        self.item_spacing = Some(value);
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
    pub fn paddings(mut self, value: DivEdgeInsets) -> Self {
        self.paddings = Some(value);
        self
    }
    pub fn build(self) -> DivTabsTabTitleStyle {
        DivTabsTabTitleStyle {
            active_background_color: self.active_background_color.or_else(|| Some(DivTabsTabTitleStyle::default_active_background_color())),
            active_font_variation_settings: self.active_font_variation_settings,
            active_font_weight: self.active_font_weight,
            active_font_weight_value: self.active_font_weight_value,
            active_text_color: self.active_text_color.or_else(|| Some(DivTabsTabTitleStyle::default_active_text_color())),
            animation_duration: self.animation_duration.or_else(|| Some(DivTabsTabTitleStyle::default_animation_duration())),
            animation_type: self.animation_type.or_else(|| Some(DivTabsTabTitleStyle::default_animation_type())),
            corner_radius: self.corner_radius,
            corners_radius: self.corners_radius,
            font_family: self.font_family,
            font_size: self.font_size.or_else(|| Some(DivTabsTabTitleStyle::default_font_size())),
            font_size_unit: self.font_size_unit.or_else(|| Some(DivTabsTabTitleStyle::default_font_size_unit())),
            font_weight: self.font_weight.or_else(|| Some(DivTabsTabTitleStyle::default_font_weight())),
            inactive_background_color: self.inactive_background_color,
            inactive_font_variation_settings: self.inactive_font_variation_settings,
            inactive_font_weight: self.inactive_font_weight,
            inactive_font_weight_value: self.inactive_font_weight_value,
            inactive_text_color: self.inactive_text_color.or_else(|| Some(DivTabsTabTitleStyle::default_inactive_text_color())),
            item_spacing: self.item_spacing.or_else(|| Some(DivTabsTabTitleStyle::default_item_spacing())),
            letter_spacing: self.letter_spacing.or_else(|| Some(DivTabsTabTitleStyle::default_letter_spacing())),
            line_height: self.line_height,
            paddings: self.paddings.or_else(|| Some(DivTabsTabTitleStyle::default_paddings())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTabsTabTitleStyleAnimationType {
    #[serde(rename = "slide")]
    Slide,
    #[serde(rename = "fade")]
    Fade,
    #[serde(rename = "none")]
    None,
}
