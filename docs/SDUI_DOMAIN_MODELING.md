# Enterprise SaaS Domain Modeling & SDUI Architecture

This document provides a rigorously structured Domain-Driven Design (DDD) model for the SaaS backend. It maps **over 180 generated DivKit objects** into formal, strictly-typed Rust domain entities and abstract value objects.

---

## 1. Core Architectural Strategy

The system is partitioned into explicit Bounded Contexts. 
- **Domain Layer:** Models E-Commerce intent, hierarchy, and business logic independent of UI libraries.
- **Presentation Layer (Adapter):** Translates these domain models into the verbose `div-*` JSON primitives required by the DivKit runtime.

---

## 2. Storefront Experience (Aggregate Roots)

**Responsibility:** Manages the high-level hierarchy of the tenant's space and global aesthetic rules. Removes hardcoded colors/fonts, abstracting them behind tokens.

```rust
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value;     // Used for generic payloads
use url::Url;              // Standard URL type
use std::time::Duration;   // Standard Time duration

// --- Core Identity Primitives ---
pub type StorefrontId = Uuid;
pub type MerchantId = Uuid;
pub type PageId = String;
pub type NodeId = String;
pub type FormId = String;

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

/// The Single Source of Truth for translating abstract Tokens into concrete DivKit primitives.
pub struct ThemeTokenRegistry {
    pub theme_id: String,
    pub color_palette: HashMap<String, ColorHex>, 
    pub typography_scales: HashMap<TypographyToken, FontSpecification>,
    pub elevation_scales: HashMap<ElevationLevel, ShadowSpecification>,
}

pub type ColorHex = String; // e.g. "#RRGGBBAA"
```

---

## 3. Page Routing, Context & Triggers

**Responsibility:** Defines routing constraints, data fetching prerequisites, and page-level side effects (abstracting `divtrigger.rs`).

```rust
/// Aggregate Root for a specific navigable screen structure
pub struct Page {
    pub id: PageId,
    pub storefront_id: StorefrontId,
    pub route_matcher: RouteMatcher,           // Defines URLs that resolve to this page
    pub required_context: Vec<ContextRequirement>, // Data variables needed before rendering
    pub event_triggers: Vec<EventTrigger>,     // Global page behaviors
    pub root_layout: LayoutNode,               // The root of the UI tree
}

pub enum RouteMatcher {
    Exact(String),         // e.g., "/products/featured"
    Prefix(String),        // e.g., "/help/"
    Regex(String),         // Handles path variables like "/product/:id"
}

/// Declarative schema ensuring necessary context exists 
pub struct ContextRequirement {
    pub key: ContextKey,            // Example: "active_cart_id"
    pub requirement_type: RequirementType, 
    pub fetch_intent: Option<ActionIntent>,
}

pub enum RequirementType { Required, Optional }
pub type ContextKey = String; 

/// Global behaviors triggered passively at the Page level (abstracting divtrigger.rs)
pub struct EventTrigger {
    pub condition: Expression<bool>,       
    pub actions: Vec<ActionIntent>,        
    pub mode: TriggerMode,                 
}

pub enum TriggerMode { OnConditionMet, EveryChange }
```

---

## 4. The Component Tree (Layout Node)

**🚀 ARCHITECTURE IMPROVEMENT:** All generic layout configurations (padding, margin, width, height, alignment) have been unified under `LayoutNode` directly. This eliminates redundancies across individual Widget properties.

```rust
/// Entity: A recursive node representing an element in the UI tree
pub struct LayoutNode {
    pub node_id: NodeId,                       // DivKit ID for targeting actions
    
    // --- Physical Constraints ---
    pub layout_cfg: LayoutConfig,              // Unifies width, height, margins, padding, alignment
    pub transform: Option<TransformToken>,     // Abstracts divtransform.rs
    
    // --- Meta Decorators ---
    pub visibility: Expression<bool>,          // Abstracts divvisibility.rs
    pub focus: Option<FocusConfig>,            // Abstracts divfocus.rs
    pub tooltip: Option<TooltipConfig>,        // Abstracts divtooltip.rs
    pub accessibility: Option<AccessibilityConfig>, // Abstracts divaccessibility.rs
    pub extensions: Option<Vec<ExtensionConfig>>, // Abstracts divextension.rs
    
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

/// Primitive Abstraction for DivKit Variables (*value.rs, *variable.rs)
pub enum Expression<T> {
    Literal(T),
    Binding(String), // e.g. "product.price" mapped to divkit variable
    Calc(String),    // e.g. "@{total_price * 1.2}" mapped to divkit eval function
}
```

---

## 5. Abstract Widgets (Component Implementations)

**Responsibility:** Maps the raw UI leaves to domain properties, removing generic layout logic.

```rust
pub enum Widget {
    Stack(StackProps),       // Abstracts divcontainer.rs
    Grid(GridProps),         // Abstracts divgrid.rs
    Carousel(CarouselProps), // Abstracts divgallery.rs / divpager.rs
    Tabs(TabProps),          // Abstracts divtabs.rs
    Text(TextProps),         // Abstracts divtext.rs
    Media(MediaProps),       // Abstracts divimage, divgifimage, divvideo
    Divider(DividerProps),   // Abstracts divseparator.rs
    Indicator(IndicatorProps), // Abstracts divindicator.rs
    InputField(InputProps),  // Abstracts divinput.rs
    Toggle(ToggleProps),     // Abstracts divswitch.rs
    Select(SelectProps),     // Abstracts divselect.rs
    Slider(SliderProps),     // Abstracts divslider.rs
    StateBlock(StatefulBlock), // Abstracts divstate.rs
    Iterator(IteratorBlock),   // Abstracts divcollectionitembuilder.rs
    Timer(TimerProps),         // Abstracts divtimer.rs
    NativeIntegration(CustomProps), // Abstracts divcustom.rs
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

pub struct CarouselProps {
    pub direction: AxisDirection,
    pub snap_mode: SnapMode,
    pub cross_alignment: Alignment,
    pub children: Vec<LayoutNode>,
}

pub struct TabProps { pub tabs: Vec<TabItem> }
pub struct TabItem { pub title: Expression<String>, pub content: LayoutNode }

pub struct MediaProps {
    pub source: Expression<Url>,
    pub alt_text: Expression<String>,
    pub kind: MediaKind,     
}

pub struct TextProps {
    pub content: Expression<String>,
    pub typography: TypographyToken,
    pub overflow: TextOverflowMode,
    pub ranges: Vec<TextRangeStyle>, // Abstracts divtextrangemask*.rs
}

pub struct DividerProps {
    pub orientation: AxisDirection,
    pub weight: u32,
    pub color: SurfaceToken,
}

pub struct IndicatorProps {
    pub linked_carousel_id: String,
    pub active_token: SurfaceToken,
    pub inactive_token: SurfaceToken,
}

pub struct StatefulBlock {
    pub default_state_id: String,
    pub state_variants: HashMap<String, Box<LayoutNode>>,
    pub default_transition: MotionToken,
}

pub struct IteratorBlock {
    pub dataset: Expression<Vec<Value>>,
    pub count_limit: Option<CountLimit>, 
    pub item_template: Box<LayoutNode>, 
    pub empty_state: Option<Box<LayoutNode>>,
}

pub struct TimerProps {
    pub id: String, 
    pub duration: Duration,
    pub tick_interval: Option<Duration>,
    pub on_tick: Option<Vec<ActionIntent>>,
    pub on_end: Option<Vec<ActionIntent>>,
}

pub struct CustomProps {
    pub extension_id: String, 
    pub custom_data: HashMap<String, Value>, 
}
```

---

## 6. Forms & Input Validation

**Responsibility:** Ensures heavy validation runs on the domain side.

```rust
pub struct InputProps {
    pub field_id: String,
    pub initial_value: Expression<String>,
    pub type_hint: InputFormat,           
    pub constraints: Vec<ValidationRule>, // Mirrors regex/expression validators
    pub filter: Option<InputFilter>,      // Limits keystrokes
    pub mask: Option<InputMask>,          // Display masks
}

pub struct ToggleProps {
    pub checked_value: Expression<bool>,
    pub on_toggle: Option<Vec<ActionIntent>>,
    pub active_color: SurfaceToken,
}

pub struct SelectProps {
    pub value: Expression<String>,
    pub options: Vec<SelectOption>,
    pub on_select: Option<Vec<ActionIntent>>,
}

pub struct SliderProps {
    pub value: Expression<f32>,
    pub min: Expression<f32>,
    pub max: Expression<f32>,
    pub step: Option<f32>,
    pub on_change: Option<Vec<ActionIntent>>,
}

pub struct SelectOption { pub label: String, pub value: String }

pub enum InputFormat { Text, Email, Password, Number, Phone, Uri }

pub enum InputMask { Phone(String), Currency(String), Percentage(String), CustomRegex(String) }

pub struct InputFilter { pub regex: Expression<String> }

pub enum ValidationRule {
    Regex(String, ErrorMessage),          
    MinLength(u32, ErrorMessage),     
    Expression(String, ErrorMessage),     
}
pub type ErrorMessage = Expression<String>;
```

---

## 7. Interaction & Intents (The Action Engine)

**Responsibility:** Replaces 30+ DivKit Action Structs with generic Semantic Intents.

```rust
pub struct InteractionConfig {
    pub on_click: Option<Vec<ActionIntent>>,
    pub on_long_click: Option<Vec<ActionIntent>>,
    pub on_appear: Option<Vec<ActionIntent>>,    
    pub on_disappear: Option<Vec<ActionIntent>>, 
}

pub enum ActionIntent {
    Navigate(RouteIntent),               
    ScrollToTarget(NodeId, ScrollOffset), 
    TriggerMotion(NodeId, MotionToken),  
    FocusElement(NodeId),                
    ClearFocus,                          
    TooltipControl(NodeId, TooltipOp),   
    VideoControl(NodeId, VideoOp),       
    MutateContext(ContextKey, Expression<String>), 
    MutateArray(ArrayMutation),          
    MutateDict(DictMutation),            
    UpdateStructure(NodeId),             
    CopyToClipboard(Expression<String>), 
    DownloadMedia(Url),                  
    TriggerTimer(NodeId, TimerAction),   
    DispatchForm(FormId),                
    DispatchBusinessEvent(String, Payload), 
}

pub type Payload = Value;

pub enum RouteIntent { Internal(PageId), External(Url), Modal(PageId) }
pub enum ScrollOffset { Exact(u32), Relative(i32) }
pub enum TooltipOp { Show, Hide }
pub enum VideoOp { Play, Pause, Resume }
pub enum TimerAction { Start, Stop, Reset, Resume }

pub struct ArrayMutation { pub target_variable: String, pub operation: ArrayOp }
pub enum ArrayOp { Insert(usize, Value), Remove(usize), Set(usize, Value), Append(Value) }
pub struct DictMutation { pub target_variable: String, pub key: String, pub value: Value }
```

---

## 8. Meta Definitions & Tokens (Value Objects)

**Responsibility:** Pure design tokens mitigating direct style injection.

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
pub enum MediaKind { StaticImage, AnimatedGif, VideoPlaylist }
pub enum TextOverflowMode { Wrap, Ellipsis, Truncate, Marquee }
pub struct TextRangeStyle {
    pub start_index: u32,
    pub end_index: u32,
    pub typography: TypographyToken,
    pub color: SurfaceToken,
    pub link_intent: Option<ActionIntent>, 
}

// Meta Configs (Focus, A11y, Tooltips)
pub struct FocusConfig {
    pub on_focus: Option<Vec<ActionIntent>>,
    pub on_blur: Option<Vec<ActionIntent>>,
    pub background: Option<SurfaceToken>, 
    pub border: Option<BorderToken>,
}

pub struct TooltipConfig {
    pub id: String,
    pub content: Box<LayoutNode>, 
    pub position: TooltipPosition,
    pub duration: Duration,
}
pub enum TooltipPosition { Top, Bottom, Left, Right, TopLeft, TopRight, BottomLeft, BottomRight, Center }

pub struct AccessibilityConfig {
    pub description: Expression<String>,
    pub hint: Option<Expression<String>>,
    pub state_description: Option<Expression<String>>,
    pub role: AccessibilityRole, 
}
pub enum AccessibilityRole { None, Button, Header, Image, Text, Link, Tab }

// Iterators & Extensibility
pub struct ExtensionConfig {
    pub id: String, 
    pub params: HashMap<String, Expression<String>>,
}
pub struct TransformToken {
    pub pivot: PivotPoint, 
    pub rotation: Expression<f32>, 
    pub scale: Expression<f32>,
    pub translation_x: Expression<f32>,
    pub translation_y: Expression<f32>,
}
pub struct PivotPoint { pub x_percentage: Expression<f32>, pub y_percentage: Expression<f32> }
pub enum Direction { TopToBottom, BottomToTop, LeftToRight, RightToLeft }
pub enum SnapMode { None, Start, Center, End }
pub enum MotionToken { FadeIn(Duration), SlideIn(Direction), Bounce }
pub type CountLimit = u32;
```
