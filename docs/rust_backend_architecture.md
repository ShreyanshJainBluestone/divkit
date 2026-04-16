# SDUI Architecture & Domain Playbook

*A comprehensive narrative and technical blueprint for the Server-Driven UI platform.*

---

## Table of Contents

*   **Volume I: The Engine & Request Lifecycle**
    *   [1.1 System Ecosystem & The Translation Pipeline](#11-system-ecosystem--the-translation-pipeline)
    *   [1.2 The Request Lifecycle Flow](#12-the-request-lifecycle-flow)
*   **Volume II: The Global Boundaries (Domain Roots)**
    *   [2.1 The Storefront Context (Multi-tenancy)](#21-the-storefront-context-multi-tenancy)
    *   [2.2 The Page Context (Routing & Data Hydration)](#22-the-page-context-routing--data-hydration)
*   **Volume III: The Component Tree**
    *   [3.1 The Unifying Wrapper: LayoutNode](#31-the-unifying-wrapper-layoutnode)
    *   [3.2 The Widget AST Core (Component Implementations)](#32-the-widget-ast-core-component-implementations)
    *   [3.3 Dynamic Components: Iterators & State Blocks](#33-dynamic-components-iterators--state-blocks)
*   **Volume IV: The Design Token System**
    *   [4.1 Theme Token Registry & Semantic Surfaces](#41-theme-token-registry--semantic-surfaces)
*   **Volume V: Interactivity & Business Logic**
    *   [5.1 The Interaction Config](#51-the-interaction-config)
    *   [5.2 Array & Sub-Mutations (The Reactive Engine)](#52-array--sub-mutations-the-reactive-engine)
    *   [5.3 End-to-End Dynamic Interaction Flow](#53-end-to-end-dynamic-interaction-flow)
    *   [5.4 Form Validation & Trust](#54-form-validation--trust)

---

## Volume I: The Engine & Request Lifecycle

This volume defines the macro-architecture of the SDUI platform. The objective is to decouple the native client applications (iOS, Android, Web) from business logic and UI updates. The client relies on a lightweight rendering engine (**DivKit**), while the Rust backend serves as the brain—dictating layout, design tokens, and interactions.

### 1.1 System Ecosystem & The Translation Pipeline

The architecture is not just a JSON generator; it is a rigid pipeline that ingests business requirements, maps them to a Domain-Driven Design (DDD) model in Rust, and translates them into DivKit's JSON schema at the edge.

```mermaid
flowchart TD
    subgraph ClientApp ["Client App (iOS / Android / Web)"]
        NativeUI[Native UI Views]
        DivKit[DivKit Rendering Engine]
        LocalState[Local State / Variables]
        NativeUI --- DivKit
        DivKit --- LocalState
    end

    subgraph API [API Gateway / Edge]
        Ingress[Load Balancer & Auth Check]
    end

    subgraph SDUI [Rust SDUI Backend]
        Router[Page & Route Resolver]
        Hydrator[Context & Data Hydrator]
        DomainAST[Domain Layout Engine]
        Translator[DivKit JSON Translator]
        
        Router --> Hydrator
        Hydrator --> DomainAST
        DomainAST --> Translator
    end

    subgraph Business [External Microservices]
        Commerce[(Commerce System)]
        CMS[(Content / Catalog)]
        Profile[(User Profiles)]
    end

    Client <-->|1. HTTP / WebSockets| Ingress
    Ingress <-->|2. Validated Request| Router
    Hydrator <-->|3. Fetch Required Data| Business
    Translator -->|4. Return DivKit JSON| Ingress
```

### 1.2 The Request Lifecycle Flow

When a user launches the app or taps a navigation link, the backend must resolve *who* they are, *what* page they requested, and *what data* that page needs before drawing the UI.

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant App as Client (DivKit)
    participant SDUI as Rust SDUI Engine
    participant Data as Microservices (Auth/Commerce)
    
    User->>App: Tap "Go to Cart" button
    App->>SDUI: GET /sdui/page?route=/cart&storefront=US
    
        note right of SDUI: Phase A: Resolution & Data Fetching
        SDUI->>SDUI: Resolve StorefrontId == "US"
        SDUI->>SDUI: Match Route "/cart" to PageId
        SDUI->>Data: Fetch `ContextRequirement` (e.g., Cart Items)
        Data-->>SDUI: Return Cart Data JSON
    
        note right of SDUI: Phase B: AST Generation & Translation
        SDUI->>SDUI: Inject Cart Data into `Expression<T>` variables
        SDUI->>SDUI: Build Rust `LayoutNode` Tree
        SDUI->>SDUI: Translate LayoutNode to DivKit layout schema
    
    SDUI-->>App: Return 200 OK (DivKit JSON Payload)
    App->>User: Render Native Page
```

---

## Volume II: The Global Boundaries (Domain Roots)

Before we draw a single UI element on the screen, the system must set up the global environment. In DDD, these are our **Aggregate Roots**. 

### 2.1 The Storefront Context (Multi-tenancy)

A single SDUI backend can serve multiple apps, brands, or geographical regions. The `Storefront` model defines the absolute highest boundary. It manages global settings and controls the overall design language through the `ThemeTokenRegistry`.

```mermaid
classDiagram
    class Storefront {
        <<Aggregate Root>>
        +StorefrontId id
        +MerchantId merchant_id
        +ThemeTokenRegistry theme_registry
        +StorefrontSettings global_settings
    }
    class StorefrontSettings {
        <<Value Object>>
        +String default_language
        +Vec~String~ supported_languages
        +String default_currency
        +AppEnvironment environment
    }
    class AppEnvironment {
        <<Enumeration>>
        Development
        Staging
        Production
    }

    Storefront *-- StorefrontSettings
    Storefront *-- AppEnvironment : uses
```

*   **Flow & Responsibility:** When an API request comes in, the API Gateway identifies the `StorefrontId` (e.g., via domain name or headers). The backend loads this exact `Storefront`. If the `AppEnvironment` is `Production`, testing widgets or draft pages are stripped from the pipeline. 

### 2.2 The Page Context (Routing & Data Hydration)

The `Page` model represents a single, navigable screen in the application. However, it's not just a collection of UI widgets. It is the controller for routing, reactive triggers, and data dependencies. 

```mermaid
classDiagram
    class Page {
        <<Aggregate Root>>
        +PageId id
        +StorefrontId storefront_id
        +RouteMatcher route_matcher
        +Vec~ContextRequirement~ required_context
        +Vec~EventTrigger~ event_triggers
        +LayoutNode root_layout
    }
    class RouteMatcher {
        <<Value Object>>
        Exact(String)
        Prefix(String)
        Regex(String)
    }
    class ContextRequirement {
        <<Entity>>
        +ContextKey key
        +RequirementType requirement_type
        +Option~ActionIntent~ fetch_intent
    }
    class EventTrigger {
        <<Entity>>
        +Expression~bool~ condition
        +Vec~ActionIntent~ actions
        +TriggerMode mode
    }

    Page *-- RouteMatcher
    Page *-- ContextRequirement
    Page *-- EventTrigger
    Storefront "1" --o "*" Page : manages
```

#### Detailed Sub-Flow: Context Hydration
A major product requirement is avoiding UI jank and endless loading spinners. By defining `ContextRequirement` at the `Page` level, a screen will **never** attempt to draw its UI tree (`root_layout`) until the backend has fetched the required data. 

1.  **Route Match:** User navigates to `/profile/settings`. `RouteMatcher::Prefix("/profile")` matches.
2.  **Context Fetch:** The screen lists a `ContextRequirement` for `ContextKey("user_profile")` marked as `RequirementType::Required`.
3.  **Action Intent Evaluation:** The SDUI Engine uses the provided `fetch_intent` (e.g., an internal HTTP call to the Profile Service) to resolve the data map *before* translation.
4.  **Event Triggers:** Upon successfully loading the page on the client, an `EventTrigger` configured as `TriggerMode::OnConditionMet` might automatically fire an analytics tracking event using an `ActionIntent`.



---

## Volume III: The Component Tree

Once the domain routing is finalized and data context hydrated, the backend begins to formulate the generic abstract syntax tree (AST). We call this the **Component Tree**. It serves as the "Flesh" of the layout.

### 3.1 The Unifying Wrapper: LayoutNode

In traditional systems, every widget (Image, text, button) has its own margin, padding, visibility, and size properties. In our SDUI domain, these duplicate fields are stripped out recursively and controlled by a master entity called `LayoutNode`.

```mermaid
classDiagram
    class LayoutNode {
        <<Entity>>
        +NodeId node_id
        +LayoutConfig layout_cfg
        +Expression~bool~ visibility
        +Option~TransformToken~ transform
        +Option~FocusConfig~ focus
        +Option~TooltipConfig~ tooltip
        +Option~AccessibilityConfig~ accessibility
        +Option~InteractionConfig~ interactions
        +Widget widget
    }
    class LayoutConfig {
        <<Value Object>>
        +LayoutSize width
        +LayoutSize height
        +SpacingToken margin
        +SpacingToken padding
        +Alignment alignment
    }
    class NodeId {
        <<Type Alias>> String
    }
    class LayoutSize {
        <<Enumeration>>
        MatchParent
        WrapContent
        Fixed(u32)
    }

    LayoutNode *-- NodeId
    LayoutNode *-- LayoutConfig
    LayoutConfig *-- LayoutSize
```

*   **DRY Layout & Rendering Engine Mapping:** This separation allows our Rust engine to apply rendering optimizations uniformly. When DivKit translates a `div-text`, it does not need to compute margins individually. It recursively processes `LayoutNode` definitions and applies padding globally, drastically reducing schema payload size.

### 3.2 The Widget AST Core (Component Implementations)

The `Widget` enum is the exhaustive container of all functional components the platform supports. It acts as the switchboard that the DivKit translation engine reads to determine the concrete JSON schema element (`div-image`, `div-input`, `div-container`).

```mermaid
classDiagram
    class Widget {
        <<Enumeration>>
        Stack(StackProps)
        Grid(GridProps)
        Carousel(CarouselProps)
        Tabs(TabProps)
        Text(TextProps)
        Media(MediaProps)
        Divider(DividerProps)
        Indicator(IndicatorProps)
        InputField(InputProps)
        Toggle(ToggleProps)
        Select(SelectProps)
        Slider(SliderProps)
        StateBlock(StatefulBlock)
        Iterator(IteratorBlock)
        Timer(TimerProps)
        NativeIntegration(CustomProps)
    }

    class StackProps {
        <<Value Object>>
        +AxisDirection direction
        +SurfaceToken background
        +Vec~LayoutNode~ children
    }
    class TextProps {
        <<Value Object>>
        +Expression~String~ content
        +TypographyToken typography
        +TextOverflowMode overflow
        +Vec~TextRangeStyle~ ranges
    }

    Widget *-- StackProps
    Widget *-- TextProps
    StackProps *-- LayoutNode : owns recursive children
```

### 3.3 Dynamic Components: Iterators & State Blocks

A key value proposition of the DivKit engine is the ability to handle state and iteration on the client device without backend round-trips. We model this via specialized complex blocks.

```mermaid
classDiagram
    class StatefulBlock {
        <<Value Object>>
        +String default_state_id
        +HashMap~String, Box~LayoutNode~~ state_variants
        +MotionToken default_transition
    }
    class IteratorBlock {
        <<Value Object>>
        +Expression~Vec~Value~~ dataset
        +Option~CountLimit~ count_limit
        +Box~LayoutNode~ item_template
        +Option~Box~LayoutNode~~ empty_state
    }

    StatefulBlock *-- LayoutNode : recursively stores states
    IteratorBlock *-- LayoutNode : iteratively draws template
```

#### Detailed Sub-Flow: Iterator Expansion
1.  **Backend Definition:** The Product team defines a gallery of product cards. The SDUI backend does not loop over the products in Rust. Instead, it outputs an `IteratorBlock`.
2.  **Dataset Binding:** `IteratorBlock.dataset` is bound to `@{"context.products"}`.
3.  **Template Binding:** `item_template` is a single `LayoutNode` (a card) bound to `@{"it.price"}` and `@{"it.title"}`.
4.  **Client Execution:** The DivKit container takes the iterator schema and stamps it natively on the GPU, yielding massive performance gains.

---

## Volume IV: The Design Token System

To remain genuinely tenant-agnostic, the Rust backend is completely scrubbed of raw hexadecimal values or pixel shadow offsets. Instead, the design system utilizes a structured taxonomy of tokens.

### 4.1 Theme Token Registry & Semantic Surfaces

The `ThemeTokenRegistry` is the translation dictionary loaded by the `Storefront`.

```mermaid
classDiagram
    class ThemeTokenRegistry {
        <<Entity>>
        +String theme_id
        +HashMap~String, ColorHex~ color_palette
        +HashMap~TypographyToken, FontSpecification~ typography_scales
        +HashMap~ElevationLevel, ShadowSpecification~ elevation_scales
    }
    class FontSpecification {
        <<Value Object>>
        +String family
        +u32 size
        +FontWeight weight
        +f32 line_height
    }
    class ShadowSpecification {
        <<Value Object>>
        +ColorHex color
        +u32 offset_x
        +u32 offset_y
        +u32 blur_radius
    }

    ThemeTokenRegistry *-- FontSpecification
    ThemeTokenRegistry *-- ShadowSpecification
```

When a layout component (like a Card) requires a background, it asks for a **Surface**, never a color. The `SurfaceToken` encapsulates borders, elevation drops, and background fills as semantic intents. 

```mermaid
classDiagram
    class SurfaceToken {
        <<Enumeration>>
        Transparent
        ElevatedCard(ElevationLevel)
        OutlinedContainer(BorderToken)
        Ghost
    }
    class BorderToken {
        <<Value Object>>
        +u32 corner_radius
        +ColorHex stroke_color
        +u32 stroke_width
        +bool has_shadow
    }

    SurfaceToken *-- BorderToken : contains
```

*   **Sub-Flow: Token Translation:** When `Widget::Stack(StackProps)` evaluates to DivKit, the SDUI translator runs `StackProps.background` (e.g., `ElevatedCard(Medium)`) through the `ThemeTokenRegistry` belonging to the current `Storefront`, outputting absolute DivKit JSON `{ "color": "#1F2328", "shadow_color": "#00000033" }`.

---

## Volume V: Interactivity & Business Logic

SDUI without interactivity is just a static website constraint. We utilize a powerful **Action Intent Engine** bound directly alongside our **Form Validation Engine**. This allows UI gestures to mutate state or trigger business rules remotely.

### 5.1 The Interaction Config

Interactions are tracked not on the Widget, but natively on the `LayoutNode`, allowing any abstract element to capture touches or visibility flashes.

```mermaid
classDiagram
    class InteractionConfig {
        <<Entity>>
        +Option~Vec~ActionIntent~~ on_click
        +Option~Vec~ActionIntent~~ on_long_click
        +Option~Vec~ActionIntent~~ on_appear
        +Option~Vec~ActionIntent~~ on_disappear
    }
    class ActionIntent {
        <<Enumeration>>
        Navigate(RouteIntent)
        ScrollToTarget(NodeId, ScrollOffset)
        TriggerMotion(NodeId, MotionToken)
        FocusElement(NodeId)
        ClearFocus
        TooltipControl(NodeId, TooltipOp)
        VideoControl(NodeId, VideoOp)
        MutateContext(ContextKey, Expression~String~)
        MutateArray(ArrayMutation)
        MutateDict(DictMutation)
        UpdateStructure(NodeId)
        CopyToClipboard(Expression~String~)
        DownloadMedia(Url)
        TriggerTimer(NodeId, TimerAction)
        DispatchForm(FormId)
        DispatchBusinessEvent(String, Payload)
    }

    InteractionConfig *-- ActionIntent
```

### 5.2 Array & Sub-Mutations (The Reactive Engine)

A critical component of avoiding constant API reloading is performing logical memory mutations safely. `ArrayMutation` is how the Rust backend instructs the client device to modify the `dataset` used by an `IteratorBlock`.

```mermaid
classDiagram
    class ArrayMutation {
        +String target_variable
        +ArrayOp operation
    }
    class ArrayOp {
        <<Enumeration>>
        Insert(usize, Value)
        Remove(usize)
        Set(usize, Value)
        Append(Value)
    }
    
    ActionIntent *-- ArrayMutation
    ArrayMutation *-- ArrayOp
```

### 5.3 End-to-End Dynamic Interaction Flow

To see the system run natively, consider the sequence where a user interacts with a dynamic shopping cart powered by our DDD interactions.

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant Div as Client App (DivKit UI View)
    participant Data as App Memory Store
    participant Rust as SDUI Backend
    
    User->>Div: Tap "Remove Item" Trash Can
    
        note right of Div: Client resolves interaction locally first
        Div->>Div: Execute `ActionIntent::MutateArray`
        Div->>Data: Fire `ArrayOp::Remove(Item_Index)`
        Data->>Div: Reactively destroys bounding `LayoutNode`
    
        note right of Div: Client syncs state to server silently
        Div->>Rust: Background HTTP: `DispatchBusinessEvent("Cart_Item_Removed", { id: "XYZ" })`
        Rust->>Rust: Persists updated user cart to SQL database
        Rust-->>Div: Return 200 OK (Sync Success)
```

### 5.4 Form Validation & Trust

Inputs and Forms have dual-layer verification strategies natively built into `InputProps`.

```mermaid
classDiagram
    class InputProps {
        +String field_id
        +Expression~String~ initial_value
        +InputFormat type_hint
        +Vec~ValidationRule~ constraints
        +Option~InputFilter~ filter
        +Option~InputMask~ mask
    }
    class ValidationRule {
        <<Enumeration>>
        Regex(String, ErrorMessage)
        MinLength(u32, ErrorMessage)
        Expression(String, ErrorMessage)
    }

    InputProps *-- ValidationRule
```

1.  **Definition Generation:** SDUI sends `Regex("^[0-9]+$", "Invalid Zip")`.
2.  **Client Typing:** User types letters. DivKit rejects input instantly based on the rules, illuminating the "Invalid Zip" string. 
3.  **Dispatch Form:** If client passes, the submit button fires `ActionIntent::DispatchForm("checkout_form")`.
4.  **Backend Re-validation:** The Rust routing engine intercepts the `DispatchForm` intent, unmarshalls the input keys, structurally verifies the exact `ValidationRule` enum array server-side to prevent network attacks, and finally processes the checkout. 

