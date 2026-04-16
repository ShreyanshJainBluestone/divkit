# Static SDUI Dependency Graph

This document exhaustively maps every field constraint, enum variant, and structural dependency defined in `STATIC_SDUI_DOMAIN_MODELING.md`, resulting in 100% field equivalence for the static layout system.

### `AccessibilityConfig`
 - `AccessibilityConfig` -> `AccessibilityRole` (via field/variant `role`)

### `AccessibilityRole`

### `Alignment`

### `AppEnvironment`

### `AxisDirection`

### `BorderToken`
 - `BorderToken` -> `ColorHex` (via field/variant `stroke_color`)

### `DividerProps`
 - `DividerProps` -> `AxisDirection` (via field/variant `orientation`)
 - `DividerProps` -> `SurfaceToken` (via field/variant `color`)

### `ElevationLevel`

### `FontSpecification`
 - `FontSpecification` -> `FontWeight` (via field/variant `weight`)

### `FontWeight`

### `GridProps`
 - `GridProps` -> `Alignment` (via field/variant `cross_alignment`)
 - `GridProps` -> `LayoutNode` (via field/variant `children`)

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
 - `LayoutNode` -> `AccessibilityConfig` (via field/variant `accessibility`)
 - `LayoutNode` -> `Widget` (via field/variant `widget`)

### `LayoutSize`

### `MediaKind`

### `MediaProps`
 - `MediaProps` -> `MediaKind` (via field/variant `kind`)

### `Page`
 - `Page` -> `PageId` (via field/variant `id`)
 - `Page` -> `StorefrontId` (via field/variant `storefront_id`)
 - `Page` -> `RouteMatcher` (via field/variant `route_matcher`)
 - `Page` -> `LayoutNode` (via field/variant `root_layout`)

### `PivotPoint`

### `RouteMatcher`

### `ShadowSpecification`
 - `ShadowSpecification` -> `ColorHex` (via field/variant `color`)

### `SpacingToken`

### `StackProps`
 - `StackProps` -> `AxisDirection` (via field/variant `direction`)
 - `StackProps` -> `SurfaceToken` (via field/variant `background`)
 - `StackProps` -> `LayoutNode` (via field/variant `children`)

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

### `TextOverflowMode`

### `TextProps`
 - `TextProps` -> `TypographyToken` (via field/variant `typography`)
 - `TextProps` -> `TextOverflowMode` (via field/variant `overflow`)
 - `TextProps` -> `TextRangeStyle` (via field/variant `ranges`)

### `TextRangeStyle`
 - `TextRangeStyle` -> `TypographyToken` (via field/variant `typography`)
 - `TextRangeStyle` -> `SurfaceToken` (via field/variant `color`)

### `ThemeTokenRegistry`
 - `ThemeTokenRegistry` -> `ColorHex` (via field/variant `color_palette`)
 - `ThemeTokenRegistry` -> `TypographyToken` (via field/variant `typography_scales`)
 - `ThemeTokenRegistry` -> `FontSpecification` (via field/variant `typography_scales`)
 - `ThemeTokenRegistry` -> `ElevationLevel` (via field/variant `elevation_scales`)
 - `ThemeTokenRegistry` -> `ShadowSpecification` (via field/variant `elevation_scales`)

### `TransformToken`
 - `TransformToken` -> `PivotPoint` (via field/variant `pivot`)

### `TypographyToken`

### `Widget`
 - `Widget` -> `StackProps` (via field/variant `Stack`)
 - `Widget` -> `GridProps` (via field/variant `Grid`)
 - `Widget` -> `TextProps` (via field/variant `Text`)
 - `Widget` -> `MediaProps` (via field/variant `Media`)
 - `Widget` -> `DividerProps` (via field/variant `Divider`)

## Inconsistencies Identified

No missing types. The Graph and Modeling file are 100% consistent.
