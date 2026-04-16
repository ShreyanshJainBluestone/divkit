# SDUI Dependency Graph

This document exhaustively maps every field constraint, alias dependence, enum variant, and structural ID dependency defined natively in `SDUI_DOMAIN_MODELING.md`, resulting in 100% field equivalence.

### `AccessibilityConfig`
 - `AccessibilityConfig` -> `Expression` (via field/variant `description`)
 - `AccessibilityConfig` -> `Expression` (via field/variant `hint`)
 - `AccessibilityConfig` -> `Expression` (via field/variant `state_description`)
 - `AccessibilityConfig` -> `AccessibilityRole` (via field/variant `role`)

### `AccessibilityRole`

### `ActionIntent`
 - `ActionIntent` -> `RouteIntent` (via field/variant `Navigate`)
 - `ActionIntent` -> `NodeId` (via field/variant `ScrollToTarget`)
 - `ActionIntent` -> `ScrollOffset` (via field/variant `ScrollToTarget`)
 - `ActionIntent` -> `NodeId` (via field/variant `TriggerMotion`)
 - `ActionIntent` -> `MotionToken` (via field/variant `TriggerMotion`)
 - `ActionIntent` -> `NodeId` (via field/variant `FocusElement`)
 - `ActionIntent` -> `NodeId` (via field/variant `TooltipControl`)
 - `ActionIntent` -> `TooltipOp` (via field/variant `TooltipControl`)
 - `ActionIntent` -> `NodeId` (via field/variant `VideoControl`)
 - `ActionIntent` -> `VideoOp` (via field/variant `VideoControl`)
 - `ActionIntent` -> `ContextKey` (via field/variant `MutateContext`)
 - `ActionIntent` -> `Expression` (via field/variant `MutateContext`)
 - `ActionIntent` -> `ArrayMutation` (via field/variant `MutateArray`)
 - `ActionIntent` -> `DictMutation` (via field/variant `MutateDict`)
 - `ActionIntent` -> `NodeId` (via field/variant `UpdateStructure`)
 - `ActionIntent` -> `Expression` (via field/variant `CopyToClipboard`)
 - `ActionIntent` -> `NodeId` (via field/variant `TriggerTimer`)
 - `ActionIntent` -> `TimerAction` (via field/variant `TriggerTimer`)
 - `ActionIntent` -> `FormId` (via field/variant `DispatchForm`)
 - `ActionIntent` -> `Payload` (via field/variant `DispatchBusinessEvent`)

### `Alignment`

### `AppEnvironment`

### `ArrayMutation`
 - `ArrayMutation` -> `ArrayOp` (via field/variant `operation`)

### `ArrayOp`

### `AxisDirection`

### `BorderToken`
 - `BorderToken` -> `ColorHex` (via field/variant `stroke_color`)

### `CarouselProps`
 - `CarouselProps` -> `AxisDirection` (via field/variant `direction`)
 - `CarouselProps` -> `SnapMode` (via field/variant `snap_mode`)
 - `CarouselProps` -> `Alignment` (via field/variant `cross_alignment`)
 - `CarouselProps` -> `LayoutNode` (via field/variant `children`)

### `ContextRequirement`
 - `ContextRequirement` -> `ContextKey` (via field/variant `key`)
 - `ContextRequirement` -> `RequirementType` (via field/variant `requirement_type`)
 - `ContextRequirement` -> `ActionIntent` (via field/variant `fetch_intent`)

### `CustomProps`

### `DictMutation`

### `Direction`

### `DividerProps`
 - `DividerProps` -> `AxisDirection` (via field/variant `orientation`)
 - `DividerProps` -> `SurfaceToken` (via field/variant `color`)

### `ElevationLevel`

### `EventTrigger`
 - `EventTrigger` -> `Expression` (via field/variant `condition`)
 - `EventTrigger` -> `ActionIntent` (via field/variant `actions`)
 - `EventTrigger` -> `TriggerMode` (via field/variant `mode`)

### `Expression`

### `ExtensionConfig`

### `FocusConfig`
 - `FocusConfig` -> `ActionIntent` (via field/variant `on_focus`)
 - `FocusConfig` -> `ActionIntent` (via field/variant `on_blur`)
 - `FocusConfig` -> `SurfaceToken` (via field/variant `background`)
 - `FocusConfig` -> `BorderToken` (via field/variant `border`)

### `FontSpecification`
 - `FontSpecification` -> `FontWeight` (via field/variant `weight`)

### `FontWeight`

### `GridProps`
 - `GridProps` -> `Alignment` (via field/variant `cross_alignment`)
 - `GridProps` -> `LayoutNode` (via field/variant `children`)

### `IndicatorProps`
 - `IndicatorProps` -> `SurfaceToken` (via field/variant `active_token`)
 - `IndicatorProps` -> `SurfaceToken` (via field/variant `inactive_token`)

### `InputFilter`
 - `InputFilter` -> `Expression` (via field/variant `regex`)

### `InputFormat`

### `InputMask`

### `InputProps`
 - `InputProps` -> `Expression` (via field/variant `initial_value`)
 - `InputProps` -> `InputFormat` (via field/variant `type_hint`)
 - `InputProps` -> `ValidationRule` (via field/variant `constraints`)
 - `InputProps` -> `InputFilter` (via field/variant `filter`)
 - `InputProps` -> `InputMask` (via field/variant `mask`)

### `InteractionConfig`
 - `InteractionConfig` -> `ActionIntent` (via field/variant `on_click`)
 - `InteractionConfig` -> `ActionIntent` (via field/variant `on_long_click`)
 - `InteractionConfig` -> `ActionIntent` (via field/variant `on_appear`)
 - `InteractionConfig` -> `ActionIntent` (via field/variant `on_disappear`)

### `IteratorBlock`
 - `IteratorBlock` -> `Expression` (via field/variant `dataset`)
 - `IteratorBlock` -> `CountLimit` (via field/variant `count_limit`)
 - `IteratorBlock` -> `LayoutNode` (via field/variant `item_template`)
 - `IteratorBlock` -> `LayoutNode` (via field/variant `empty_state`)

### `LayoutConfig`
 - `LayoutConfig` -> `LayoutSize` (via field/variant `width`)
 - `LayoutConfig` -> `LayoutSize` (via field/variant `height`)
 - `LayoutConfig` -> `SpacingToken` (via field/variant `margin`)
 - `LayoutConfig` -> `SpacingToken` (via field/variant `padding`)
 - `LayoutConfig` -> `Alignment` (via field/variant `alignment`)

### `LayoutNode`
 - `LayoutNode` -> `NodeId` (via field/variant `node_id`)
 - `LayoutNode` -> `LayoutConfig` (via field/variant `layout_cfg`)
 - `LayoutNode` -> `TransformToken` (via field/variant `transform`)
 - `LayoutNode` -> `Expression` (via field/variant `visibility`)
 - `LayoutNode` -> `FocusConfig` (via field/variant `focus`)
 - `LayoutNode` -> `TooltipConfig` (via field/variant `tooltip`)
 - `LayoutNode` -> `AccessibilityConfig` (via field/variant `accessibility`)
 - `LayoutNode` -> `ExtensionConfig` (via field/variant `extensions`)
 - `LayoutNode` -> `Widget` (via field/variant `widget`)

### `LayoutSize`

### `MediaKind`

### `MediaProps`
 - `MediaProps` -> `Expression` (via field/variant `source`)
 - `MediaProps` -> `Expression` (via field/variant `alt_text`)
 - `MediaProps` -> `MediaKind` (via field/variant `kind`)

### `MotionToken`
 - `MotionToken` -> `Direction` (via field/variant `SlideIn`)

### `Page`
 - `Page` -> `PageId` (via field/variant `id`)
 - `Page` -> `StorefrontId` (via field/variant `storefront_id`)
 - `Page` -> `RouteMatcher` (via field/variant `route_matcher`)
 - `Page` -> `ContextRequirement` (via field/variant `required_context`)
 - `Page` -> `EventTrigger` (via field/variant `event_triggers`)
 - `Page` -> `LayoutNode` (via field/variant `root_layout`)

### `PivotPoint`
 - `PivotPoint` -> `Expression` (via field/variant `x_percentage`)
 - `PivotPoint` -> `Expression` (via field/variant `y_percentage`)

### `RequirementType`

### `RouteIntent`
 - `RouteIntent` -> `PageId` (via field/variant `Internal`)
 - `RouteIntent` -> `PageId` (via field/variant `Modal`)

### `RouteMatcher`

### `ScrollOffset`

### `SelectOption`

### `SelectProps`
 - `SelectProps` -> `Expression` (via field/variant `value`)
 - `SelectProps` -> `SelectOption` (via field/variant `options`)
 - `SelectProps` -> `ActionIntent` (via field/variant `on_select`)

### `ShadowSpecification`
 - `ShadowSpecification` -> `ColorHex` (via field/variant `color`)

### `SliderProps`
 - `SliderProps` -> `Expression` (via field/variant `value`)
 - `SliderProps` -> `Expression` (via field/variant `min`)
 - `SliderProps` -> `Expression` (via field/variant `max`)
 - `SliderProps` -> `ActionIntent` (via field/variant `on_change`)

### `SnapMode`

### `SpacingToken`

### `StackProps`
 - `StackProps` -> `AxisDirection` (via field/variant `direction`)
 - `StackProps` -> `SurfaceToken` (via field/variant `background`)
 - `StackProps` -> `LayoutNode` (via field/variant `children`)

### `StatefulBlock`
 - `StatefulBlock` -> `MotionToken` (via field/variant `default_transition`)

### `Storefront`
 - `Storefront` -> `StorefrontId` (via field/variant `id`)
 - `Storefront` -> `MerchantId` (via field/variant `merchant_id`)
 - `Storefront` -> `ThemeTokenRegistry` (via field/variant `theme_registry`)
 - `Storefront` -> `StorefrontSettings` (via field/variant `global_settings`)

### `StorefrontSettings`
 - `StorefrontSettings` -> `AppEnvironment` (via field/variant `environment`)

### `SurfaceToken`
 - `SurfaceToken` -> `ElevationLevel` (via field/variant `ElevatedCard`)
 - `SurfaceToken` -> `BorderToken` (via field/variant `OutlinedContainer`)

### `TabItem`
 - `TabItem` -> `Expression` (via field/variant `title`)
 - `TabItem` -> `LayoutNode` (via field/variant `content`)

### `TabProps`
 - `TabProps` -> `TabItem` (via field/variant `tabs`)

### `TextOverflowMode`

### `TextProps`
 - `TextProps` -> `Expression` (via field/variant `content`)
 - `TextProps` -> `TypographyToken` (via field/variant `typography`)
 - `TextProps` -> `TextOverflowMode` (via field/variant `overflow`)
 - `TextProps` -> `TextRangeStyle` (via field/variant `ranges`)

### `TextRangeStyle`
 - `TextRangeStyle` -> `TypographyToken` (via field/variant `typography`)
 - `TextRangeStyle` -> `SurfaceToken` (via field/variant `color`)
 - `TextRangeStyle` -> `ActionIntent` (via field/variant `link_intent`)

### `ThemeTokenRegistry`
 - `ThemeTokenRegistry` -> `TypographyToken` (via field/variant `typography_scales`)
 - `ThemeTokenRegistry` -> `ElevationLevel` (via field/variant `elevation_scales`)

### `TimerAction`

### `TimerProps`
 - `TimerProps` -> `ActionIntent` (via field/variant `on_tick`)
 - `TimerProps` -> `ActionIntent` (via field/variant `on_end`)

### `ToggleProps`
 - `ToggleProps` -> `Expression` (via field/variant `checked_value`)
 - `ToggleProps` -> `ActionIntent` (via field/variant `on_toggle`)
 - `ToggleProps` -> `SurfaceToken` (via field/variant `active_color`)

### `TooltipConfig`
 - `TooltipConfig` -> `LayoutNode` (via field/variant `content`)
 - `TooltipConfig` -> `TooltipPosition` (via field/variant `position`)

### `TooltipOp`

### `TooltipPosition`

### `TransformToken`
 - `TransformToken` -> `PivotPoint` (via field/variant `pivot`)
 - `TransformToken` -> `Expression` (via field/variant `rotation`)
 - `TransformToken` -> `Expression` (via field/variant `scale`)
 - `TransformToken` -> `Expression` (via field/variant `translation_x`)
 - `TransformToken` -> `Expression` (via field/variant `translation_y`)

### `TriggerMode`

### `TypographyToken`

### `ValidationRule`
 - `ValidationRule` -> `ErrorMessage` (via field/variant `Regex`)
 - `ValidationRule` -> `ErrorMessage` (via field/variant `MinLength`)
 - `ValidationRule` -> `ErrorMessage` (via field/variant `Expression`)

### `VideoOp`

### `Widget`
 - `Widget` -> `StackProps` (via field/variant `Stack`)
 - `Widget` -> `GridProps` (via field/variant `Grid`)
 - `Widget` -> `CarouselProps` (via field/variant `Carousel`)
 - `Widget` -> `TabProps` (via field/variant `Tabs`)
 - `Widget` -> `TextProps` (via field/variant `Text`)
 - `Widget` -> `MediaProps` (via field/variant `Media`)
 - `Widget` -> `DividerProps` (via field/variant `Divider`)
 - `Widget` -> `IndicatorProps` (via field/variant `Indicator`)
 - `Widget` -> `InputProps` (via field/variant `InputField`)
 - `Widget` -> `ToggleProps` (via field/variant `Toggle`)
 - `Widget` -> `SelectProps` (via field/variant `Select`)
 - `Widget` -> `SliderProps` (via field/variant `Slider`)
 - `Widget` -> `StatefulBlock` (via field/variant `StateBlock`)
 - `Widget` -> `IteratorBlock` (via field/variant `Iterator`)
 - `Widget` -> `TimerProps` (via field/variant `Timer`)
 - `Widget` -> `CustomProps` (via field/variant `NativeIntegration`)

## Inconsistencies Identified

No missing types. The Graph and Modeling file are 100% consistent.
