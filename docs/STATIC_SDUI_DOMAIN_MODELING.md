# Static Enterprise SaaS Domain Modeling & SDUI Architecture

This document provides a strictly structural, non-interactive subset of the Domain-Driven Design (DDD) model for the SaaS backend. It focuses exclusively on rendering static UI pages without side effects, statefulness, variables, or user interactions.

All `Expression<T>` wrappers have been flattened into raw `T` values, as variables and calculations are outside the scope of static layout rendering. Forms, inputs, states, iterators, timers, and action intents have been completely removed.

---

## 1. Storefront Experience (Aggregate Roots)

**Responsibility:** Manages the high-level hierarchy of the tenant's space and global aesthetic rules.

```rust
use uuid::Uuid;
use std::collections::HashMap;
use url::Url;

// --- Core Identity Primitives ---
pub type StorefrontId = Uuid;
pub type MerchantId = Uuid;
pub type PageId = String;
pub type NodeId = String;

/// Aggregate Root for the Merchant's Tenant Space
pub struct Storefront {
    pub id: StorefrontId,
    pub merchant_id: MerchantId,
    pub theme_registry: ThemeTokenRegistry,
    pub global_settings: StorefrontSettings,
}

/// Global Application Settings specific to the Tenant/Merchant.
pub struct StorefrontSettings {
    pub default_language: String,       // e.g., "en-US"
    pub supported_languages: Vec<String>,
    pub default_currency: String,       // e.g., "USD"
    pub environment: AppEnvironment, 
}

pub enum AppEnvironment { Development, Staging, Production }

/// The Single Source of Truth for translating abstract Tokens into concrete UI styling.
pub struct ThemeTokenRegistry {
    pub theme_id: String,
    pub color_palette: HashMap<String, ColorHex>, 
    pub typography_scales: HashMap<TypographyToken, FontSpecification>,
    pub elevation_scales: HashMap<ElevationLevel, ShadowSpecification>,
}

pub type ColorHex = String; // e.g. "#RRGGBBAA"
```

---

## 2. Page Routing

**Responsibility:** Defines routing constraints to structural pages, stripped of context fetching requirements and event triggers.

```rust
/// Aggregate Root for a specific navigable screen structure
pub struct Page {
    pub id: PageId,
    pub storefront_id: StorefrontId,
    pub route_matcher: RouteMatcher,           // Defines URLs that resolve to this page
    pub root_layout: LayoutNode,               // The root of the static UI tree
}

pub enum RouteMatcher {
    Exact(String),         // e.g., "/products/featured"
    Prefix(String),        // e.g., "/help/"
    Regex(String),         // Handles path variables like "/product/:id"
}
```

---

## 3. The Component Tree (Layout Node)

**Responsibility:** Represents a strictly static node in the UI tree. Visibility, accessibility, and transforms are hardcoded to direct values rather than reactive expressions.

```rust
/// Entity: A recursive node representing an element in the UI tree
pub struct LayoutNode {
    pub node_id: NodeId,                       // Identifier for debugging or specific targeting
    
    // --- Physical Constraints ---
    pub layout_cfg: LayoutConfig,              // Unifies width, height, margins, padding, alignment
    pub transform: Option<TransformToken>,     // Static spatial transformation
    
    // --- Meta Decorators ---
    pub visibility: bool,                      // Flattened from Expression<bool>
    pub accessibility: Option<AccessibilityConfig>, 
    
    // --- Core Content ---
    pub widget: Widget,                        // The specific component implementation
}

pub struct LayoutConfig {
    pub width: LayoutSize,    
    pub height: LayoutSize,   
    pub margin: SpacingToken, 
    pub padding: SpacingToken,
    pub alignment: Alignment, 
}
```

---

## 4. Static Widgets (Component Implementations)

**Responsibility:** Maps the raw static UI leaves to domain properties. All interactive widgets (Forms, Inputs, Toggles, Timers, States, Iterators) have been eliminated.

```rust
pub enum Widget {
    Stack(StackProps),       // Vertical/Horizontal/Overlap groups
    Grid(GridProps),         // Tabular layouts
    Text(TextProps),         // Static text 
    Media(MediaProps),       // Non-interactive images/video rendering
    Divider(DividerProps),   // Visual separators
}

pub struct StackProps {
    pub direction: AxisDirection,
    pub background: SurfaceToken,
    pub children: Vec<LayoutNode>, // Recursive tree indirection via Vec allocation
}

pub struct GridProps {
    pub columns: u32,
    pub cross_alignment: Alignment,
    pub children: Vec<LayoutNode>,
}

pub struct MediaProps {
    pub source: Url,            // Flattened from Expression<Url>
    pub alt_text: String,       // Flattened from Expression<String>
    pub kind: MediaKind,     
}

pub struct TextProps {
    pub content: String,        // Flattened from Expression<String>
    pub typography: TypographyToken,
    pub overflow: TextOverflowMode,
    pub ranges: Vec<TextRangeStyle>, 
}

pub struct DividerProps {
    pub orientation: AxisDirection,
    pub weight: u32,
    pub color: SurfaceToken,
}
```

---

## 5. Meta Definitions & Tokens (Value Objects)

**Responsibility:** Pure design tokens mitigating direct style injection and structural primitive enums. Stripped of interaction logic natively present in original value objects.

```rust
// Layout Primitives
pub enum LayoutSize { MatchParent, WrapContent, Fixed(u32), Percentage(f32) }
pub enum SpacingToken { None, XSmall, Small, Medium, Large, XLarge, Custom(u32) }
pub enum Alignment { TopLeft, TopCenter, TopRight, CenterLeft, Center, CenterRight, BottomLeft, BottomCenter, BottomRight }
pub enum AxisDirection { Horizontal, Vertical, Overlap } 

// Stylistic Tokens
pub enum SurfaceToken { Transparent, ElevatedCard(ElevationLevel), OutlinedContainer(BorderToken), Ghost }
pub enum TypographyToken { Headline1, BodyPrimary, Caption, PriceHighlight }

pub struct BorderToken {
    pub corner_radius: u32,
    pub stroke_color: ColorHex,
    pub stroke_width: u32,
    pub has_shadow: bool,
}

pub struct FontSpecification {
    pub family: String,
    pub size: u32,
    pub weight: FontWeight, 
    pub line_height: f32,
}

pub enum FontWeight { Light, Regular, Medium, Bold, Heavy }
pub struct ShadowSpecification { pub color: ColorHex, pub offset_x: u32, pub offset_y: u32, pub blur_radius: u32 }
pub enum ElevationLevel { None, Low, Medium, High }

// Media & Text Modifiers
pub enum MediaKind { StaticImage, VectorGraphic }
pub enum TextOverflowMode { Wrap, Ellipsis, Truncate }
pub struct TextRangeStyle {
    pub start_index: u32,
    pub end_index: u32,
    pub typography: TypographyToken,
    pub color: SurfaceToken,
    // Note: link_intent stripped for strictly static text representation
}

// Meta Configs (A11y)
pub struct AccessibilityConfig {
    pub description: String,               // Flattened from Expression
    pub hint: Option<String>,              // Flattened from Expression
    pub role: AccessibilityRole, 
}
pub enum AccessibilityRole { None, Header, Image, Text }

// Transforms
pub struct TransformToken {
    pub pivot: PivotPoint, 
    pub rotation: f32,                     // Flattened from Expression
    pub scale: f32,                        // Flattened from Expression
    pub translation_x: f32,                // Flattened from Expression
    pub translation_y: f32,                // Flattened from Expression
}
pub struct PivotPoint { pub x_percentage: f32, pub y_percentage: f32 }

```