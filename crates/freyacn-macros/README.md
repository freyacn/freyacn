# `freyacn-macros`

> ⚠️ **Work in progress.** This crate is under active development and its
> public API is **not yet stable**. Breaking changes may land in any release.
> The shapes shown below are the current target, not a compatibility promise.
> Expect rough edges, missing features, and doc/code drift.

Procedural macros for the [`freyacn`][freyacn] crate. They turn plain functions
and enums into styled, prop‑driven components for [Freya][freya], with
Tailwind‑inspired styling built in and complete generated documentation.

---

## Relationship to `freyacn`

**`freyacn-macros` is an internal dependency of `freyacn`.** It is not meant
to be used on its own, and there are two reasons you almost never depend on
it directly:

1. **`freyacn` re‑exports every macro.** Everything documented here is
   available from the parent crate:
   ```rust
   use freyacn::{component, extensions, struct_fields, variant};
   ```
   Adding `freyacn-macros` to your `Cargo.toml` alongside `freyacn` would
   pull in the same code twice and, worse, risk pulling *different* versions.

2. **The macros emit paths to `freyacn` types.** The generated code refers to
   `::freyacn::Component`, `::freyacn::Style`, `::freyacn::ChildrenExt`, and
   so on. Without `freyacn` in the dependency graph, expansion would fail with
   "cannot find crate `freyacn`".

The only reason to depend on `freyacn-macros` directly is if you are forking
the macro implementation itself. For every other use case, use `freyacn`.

---

The crate exposes four attributes:

* **`#[component(...)]`** — turns a function into a component struct plus a
  PascalCase constructor function and a companion macro. All three field
  groups (`props`, `ext`, `variants`) live inside this attribute.
* **`#[variant]`** — turns an enum into a style‑resolving variant type with
  `From<&str>` / `From<String>`, `.resolve()`, and an `impl freyacn::Variant`.
* **`#[struct_fields(...)]`** — placed below `#[component]`, adds internal
  storage fields the user's own trait impls can manage.
* **`#[extensions(...)]`** — standalone attribute that opts any struct into
  freyacn's built‑in extension bundles. The `ext(...)` group inside
  `#[component]` is the same feature for generated components.

---

## Installation

```toml
[dependencies]
freyacn = "0.1"
freya = "0.4"
```

```rust
use freyacn::{component, extensions, struct_fields, variant, StyleExt};
use freya::prelude::*;
```

---

## Quick start

```rust
use freyacn::{component, variant, StyleExt};
use freya::prelude::*;

/// Visual variants for a button.
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground px-4 py-2")]
    #[default]
    Default,

    #[variant("bg-destructive text-destructive-foreground px-4 py-2")]
    Destructive,

    #[variant("border-2 border-primary bg-transparent text-primary px-4 py-2")]
    Outline,
}

#[component(
    props( badge: String = "new".to_string()),
    ext(children, style),
    variants( variant: ButtonVariant = ButtonVariant::Default),
)]
fn Button(label: String) {
    rect()
        .background(style.background.clone().unwrap_or_default())
        .padding(style.padding.unwrap_or_default())
        .child(label().text(label.as_str()))
        .children(children.clone())
}
```

> **No return type needed.** The macro generates the `impl IntoElement`
> signature for the component's `render` method. Write the function body as
> the expression you want rendered, and skip both the `-> impl IntoElement`
> annotation and any trailing `.into()`.

Three equivalent ways to construct it:

```rust
Button().label("Save").variant(ButtonVariant::Destructive);   // typed
Button().label("Save").variant("destructive");                // &str
Button!(label = "Save", variant = "destructive");             // macro
```

---

## Style layering

The most important runtime behaviour to understand is **how `style` is
built at render time** when a component has `variants(...)`:

```text
effective style = Style::default()
                    .merge(variant.resolve())     // declared order
                    .merge(size.resolve())
                    .merge(self.style.clone())    // user overrides win
```

Every `StyleExt` method called on the component writes to the *override
layer* (`self.style`). Variant styles are computed fresh from the theme at
render time and merged underneath.

### Consequences

**Overrides persist across variant changes.**

```rust
let mut b = Button()
.label("Save")
.variant("destructive")
.bg_green_50();     // override

b.variant("outline");   // base recomputes; the override still wins
```

After the second call, the effective background is still `green_50` — the
`Outline` variant's `bg-transparent` is applied first, then overridden.

**Every `StyleExt` call is an override.**

```rust
Button()
.variant("primary")
.p_6()              // override padding — wins over the variant's px-4 py-2
.text_xl()          // override font size
.class("font-bold text-white")   // class strings also go to the override layer
```

**Changing the variant leaves user intent intact.** If a caller explicitly
set a property, they meant it. `.variant("x")` only updates the *base*.

### Merging rules

`Style::merge(other)` produces a new `Style` where:

* **Scalar fields** (`Option<T>`) → `other.field.or(self.field)`. Set values
  in `other` win; unset values leave `self` alone.
* **Stack fields** (`shadows`, `font_families`) → concatenated.

There is also `Style::or(defaults)` for the inverse (fill gaps only),
plus `merge_from` / `or_from` for in‑place mutation. See the
[`Style`](https://docs.rs/freyacn) documentation for details.

---

## `#[component(...)]`

Every component is written as a normal Rust function. The attribute groups
declare where each field comes from; the source decides its kind.

### Four field sources

| Source                          | Kind          | Field type       | Init             | Body sees                   | Setter accepts              |
|---------------------------------|---------------|------------------|------------------|-----------------------------|-----------------------------|
| `fn Card(name: String)`         | **required**  | `Option<String>` | `None`           | `&String` (panics if unset) | `impl Into<String>`         |
| `props(name: String)`           | **optional**  | `Option<String>` | `None`           | `&Option<String>`           | `impl Into<Option<String>>` |
| `props(name: String = "x")`     | **defaulted** | `String`         | `"x"`            | `&String`                   | `impl Into<String>`         |
| `variants(name: T = default)`   | **variant**   | `T`              | `default`        | `&T`                        | `impl Into<T>`              |
| `#[struct_fields(name: T)]`     | **internal**  | `Option<T>`      | `None`           | `&Option<T>`                | *(no setter)*               |
| `#[struct_fields(name: T = x)]` | **internal**  | `T`              | `x`              | `&T`                        | *(no setter)*               |
| `ext(children, style, …)`       | **extension** | *trait-specific* | *trait-specific* | *trait-specific*            | *(no setter)*               |

You **never** write `Option<T>` yourself for an optional prop or a struct
field — writing `name: String` is enough; the macro wraps it.

### Grammar

```text
#[component(
    props(    prop1: Type1, prop2: Type2 = default_expr, … ),
    ext(      extension1, extension2, … ),
    variants( variant: EnumType = EnumType::Default, … ),
)]
```

Groups may appear in any order, and each is optional. `props` and `variants`
accept comma‑separated declarations of the same shape:

```text
field := ident ":" type [ "=" expr ]
```

### Return type

The component function does **not** need a return type annotation, and does **not** need a trailing `.into()`. The macro
wraps the body in a
`Component::render` method that returns `impl IntoElement`, so the last
expression of the body just needs to implement `IntoElement`:

```rust
#[component]
fn Greeting(name: String) {
    label().text(format!("Hello, {name}!"))   // no `-> …`, no `.into()`
}
```

### Required props — function parameters

```rust
#[component]
fn Greeting(name: String) {
    label().text(format!("Hello, {name}!"))
}

Greeting().name("Ada");           // OK
// Greeting();                    // compiles, panics at render
// Greeting().name(None);         // type error — intentional
```

### Optional props — `props(name: T)`

```rust
#[component(props( subtitle: String))]
fn Heading(subtitle: String) {
    match subtitle {
        Some(s) => label().text(s.as_str()),
        None => rect(),
    }
}

h.subtitle("hi");                       // &str  → Some("hi".into())
h.subtitle(String::from("hi"));         // String → Some
h.subtitle(Some("hi".to_string()));     // Option<String>
h.subtitle(None);                       // Option<String>
```

### Defaulted props — `props(name: T = expr)`

```rust
#[component(props(
    size:   f32    = 16.0,
    weight: String = "normal".to_string(),
))]
fn Text(size: f32, weight: String) {
    label().text(format!("{}pt {}", size, weight))
}
```

### Variant props — `variants(name: T = default)`

A default is required: variant fields are stored as `T`, not `Option<T>`.
Setter takes `impl Into<T>`, which lets you pass the enum value or any type
`T: From<…>`. Pairs naturally with `#[variant]`‑annotated enums.

```rust
#[component(
    variants( variant: ButtonVariant = ButtonVariant::Default),
    ext(style),
)]
fn Button(label: String) {
    let style = style;   // ← merged result
    rect()
        .background(style.background.clone().unwrap_or_default())
        .child(label().text(label.as_str()))
}

b.variant(ButtonVariant::Destructive);   // typed
b.variant("destructive");                // &str
b.variant(String::from("destructive"));  // String
```

> **`variants(...)` requires `ext(style)`.** The merged style is bound as a
> local called `style` in the render body — that requires the `style`
> extension. If you declare variants without `ext(style)`, the macro emits a
> targeted compile error.

### Struct fields — `#[struct_fields(...)]`

Placed **below** `#[component]`. Same grammar as `props(...)`, but no setter
is generated and they are documented in a separate section. Use them to store
data your own trait impls need.

```rust
use freyacn::{component, struct_fields, StyleExt};

#[component(props( subtitle: String))]
#[struct_fields(
    tooltip: MyTooltip,              // Option<MyTooltip> = None
    hover_count: usize = 0,          // usize = 0
)]
fn Card(name: String) {
    let _ = tooltip.as_ref().map(|t| t.text.as_str()).unwrap_or("");
    let _ = hover_count;
    rect().child(label().text(name.as_str()))
}

// Your own trait — plain Rust, no macro.
#[derive(Default)]
pub struct MyTooltip {
    pub text: String,
    pub visible: bool
}

pub trait TooltipExt {
    fn set_tooltip(&mut self, text: impl Into<String>);
}

impl TooltipExt for CardComponent {
    fn set_tooltip(&mut self, text: impl Into<String>) {
        let tip = self.tooltip.get_or_insert_with(MyTooltip::default);
        tip.text = text.into();
    }
}

let mut card = Card().name("Ada");
card.set_tooltip("Click to edit");
card.hover_count = 7;
```

### Extension fields — `ext(...)`

See [`#[extensions(...)]`](#extensions) for the full list. Names inside
`ext(...)` are the same names that `#[extensions(...)]` accepts.

---

## `#[variant]`

Applied to an enum, generates:

* `impl MyEnum { fn style_str(&self) -> &'static str; }`
* `impl MyEnum { fn resolve(&self) -> freyacn::Style; }` — convenience
  alias that doesn't require importing `Variant`.
* `impl freyacn::Variant for MyEnum` — the trait‑based resolver.
* `impl From<&str> for MyEnum`
* `impl From<String> for MyEnum`

### Declaration

```rust
use freyacn::variant;

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground")]
    #[default]
    Default,

    #[variant("bg-destructive text-destructive-foreground")]
    Destructive,

    #[variant("border-2 border-primary bg-transparent text-primary")]
    Outline,

    #[variant("bg-secondary text-secondary-foreground")]
    Secondary,

    #[variant("bg-transparent text-foreground")]
    Ghost,

    #[variant("bg-transparent text-primary underline")]
    Link,
}
```

### Kebab-case conversion

| PascalCase             | Kebab                   |
|------------------------|-------------------------|
| `Default`              | `default`               |
| `Md`                   | `md`                    |
| `XS`                   | `xs`                    |
| `SecondaryDestructive` | `secondary-destructive` |

### Fallback for unknown names

`From<&str>` falls back to the **first declared variant**. Put your natural
default first and mark it `#[default]` if you use `derive(Default)`.

### Resolving to a `Style`

`.resolve()` reads the active theme and parses the variant's style string.
It must be called inside a Freya render context. When used as a prop inside
a component that declares `variants(...)` and `ext(style)`, the macro calls
`.resolve()` for you and merges the result into the render body's `style`
local.

You can also resolve manually:

```rust
let v = ButtonVariant::Destructive;
let s = v.resolve();
rect().background(s.background.clone().unwrap_or_default());
```

### The `Variant` trait

`#[variant]` generates `impl freyacn::Variant for MyEnum`. The trait is:

```rust
pub trait Variant {
    fn resolve(&self) -> Style;
}
```

It exists so the component macro can merge variant styles without knowing
the concrete enum type — it just calls `Variant::resolve(&self.field)`.

---

## `#[extensions(...)]` — standalone on any struct

The `#[extensions(...)]` attribute (and its `ext(...)` sibling inside
`#[component]`) opt a struct into freyacn's built‑in **extension bundles**.
Each name maps to a bundle of:

* a field to add to the struct,
* an initialiser for that field,
* a render‑body local binding, and
* an `impl XxxExt for YourStruct` block.

### Supported names

| Name             | Trait                       | Field(s) added                                           | Trait method         |
|------------------|-----------------------------|----------------------------------------------------------|----------------------|
| `children`       | `freyacn::ChildrenExt`      | `children: Vec<Element>`                                 | `get_children`       |
| `key`            | `freyacn::KeyExt`           | `key: DiffKey`                                           | `write_key`          |
| `style`          | `freyacn::StyleExt`         | `style: Style`                                           | `get_style`          |
| `event_handlers` | `freyacn::EventHandlersExt` | `event_handlers: FxHashMap<EventName, EventHandlerType>` | `get_event_handlers` |
| `corner_radius`  | `freyacn::CornerRadiusExt`  | *(none — delegates to `style`)*                          | `with_corner_radius` |

### Dependencies

`corner_radius` requires `style`. Missing dependencies produce a targeted
compile error at the attribute site.

`variants(...)` inside `#[component]` also requires `style`, for the same
reason — the resolved styles need a place to be merged.

### On a hand‑written struct

The attribute works on any struct with named fields. The struct must
derive or implement `Default` so the added fields can be initialised.

```rust
use freyacn::extensions;
use freya::prelude::*;

#[extensions(children, key, style)]
#[derive(Clone, PartialEq, Default)]
pub struct PlainBox;
```

After expansion, `PlainBox` has:

```rust
pub struct PlainBox {
    pub children: Vec<Element>,
    pub key: DiffKey,
    pub style: Style,
}

impl ChildrenExt for PlainBox { /* get_children */ }
impl KeyExt for PlainBox { /* write_key */ }
impl StyleExt for PlainBox { /* get_style */ }
```

Because `#[derive(Default)]` was forwarded, `PlainBox::default()` gives you
an empty instance ready to be built up:

```rust
let mut b = PlainBox::default ();
b.push(label().text("hi"));               // ChildrenExt
b.set_key(DiffKey::Id("root".into()));    // KeyExt
let b = b.bg_primary().p_4();             // StyleExt
```

### Choosing a subset

You can opt into just the extensions you need:

```rust
#[extensions(style)]
#[derive(Clone, PartialEq, Default)]
pub struct Styled {
    pub label: String,
}
```

`Styled` gets a `style: Style` field and the `StyleExt` impl — but no
`children`, no `key`, no event handlers. Pay only for what you request.

### What a bundle contributes

For `style`, the expansion contributes:

```rust
// Field
pub style:::freyacn::Style

// new() initialiser
style: <::freyacn::Style as::core::default::Default>::default ()

// render-body binding (no variants)
let style: &::freyacn::Style = & self .style;

// render-body binding (with variants)
let __freyacn_merged_style:::freyacn::Style = {
let mut __freyacn_base =
<::freyacn::Style as ::core::default::Default >::default();
__freyacn_base = __freyacn_base.merge(
::freyacn::Variant::resolve( & self.variant)
);
__freyacn_base.merge( self.style.clone())
};
let style: &::freyacn::Style = & __freyacn_merged_style;

// trait impl
impl ::freyacn::StyleExt for YourStruct {
    fn get_style(&mut self) -> &mut ::freyacn::Style { &mut self.style }
}
```

---

## Style merging utilities

`Style` exposes a small set of merging methods used by the component macro.
They are public because they are useful on their own.

| Method                                                | Effect                                                        |
|-------------------------------------------------------|---------------------------------------------------------------|
| `Style::merge(self, other) -> Style`                  | `other` wins on every field it set. Stack fields concatenate. |
| `Style::or(self, defaults) -> Style`                  | `self` wins; `defaults` fills gaps only.                      |
| `Style::merge_from(&mut self, other)`                 | In‑place variant of `merge`.                                  |
| `Style::or_from(&mut self, defaults)`                 | In‑place variant of `or`.                                     |
| `Style::is_empty(&self) -> bool`                      | `true` if no field has been set.                              |
| `Style::style_debug(self, s) -> (Style, Vec<String>)` | Like `.style(s)` but also returns the unrecognised tokens.    |

`StyleExt` gets two provided methods that lift these onto every component:

| Method                | Effect                                 |
|-----------------------|----------------------------------------|
| `.merge_style(other)` | `self.map_style(\|s\| s.merge(other))` |
| `.or_style(defaults)` | `self.map_style(\|s\| s.or(defaults))` |

### `style_debug`

The regular `.style("...")` method silently ignores tokens it doesn't
recognise — matching Tailwind's permissive behaviour. When you want to
know which tokens had no effect (for typo detection or test assertions),
use `style_debug`:

```rust
let (style, unknown) = Style::default ()
.style_debug("p-4 bg-primary shdaow-md not-a-thing");

assert_eq!(unknown, vec!["shdaow-md", "not-a-thing"]);
```

---

## Attribute forwarding

Every attribute on the function other than `#[doc]`, `#[struct_fields(...)]`,
and the `#[component(...)]` groups is forwarded **verbatim** to the generated
struct. The macro does **not** bake in any derives.

```rust
#[component(props( label: String))]
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg(feature = "experimental")]
#[my_custom_macro(arg = 42)]
fn Widget(label: String) { /* … */ }
```

The generated struct carries the same `#[derive(...)]`, `#[cfg(...)]`, and
custom macro, in the same order. Doc comments on the function become the
struct's docs; a generated summary (listing props, struct fields, and
extensions) is appended after them.

Three attributes are **not** forwarded, because the macro handles them itself:

* `#[doc]` — re‑emitted first, so it isn't duplicated.
* `#[struct_fields(...)]` — consumed and turned into storage fields.
* `#[component(...)]` groups — consumed and turned into the struct's shape.

---

## Field ordering in the generated struct

```text
┌── props(...)           attribute props
├── struct_fields(...)   internal storage
├── variants(...)        variant props
├── fn params            required props
└── ext(...)             extension fields
```

## Collision rules

* **Duplicate names anywhere** (across `props`, `struct_fields`, `variants`,
  fn params) → **compile error** naming both sources.
* **A field whose name matches an extension field** from `ext(...)` or
  `#[extensions(...)]` → **compile error**, because extension fields are
  reserved by freyacn.

---

## Generated code

For:

```rust
#[component(
    props( subtitle: String),
    ext(children, style),
    variants( variant: ButtonVariant = ButtonVariant::Default),
)]
#[struct_fields( tooltip: MyTooltip, hover_count: usize = 0)]
#[derive(Clone, Debug, PartialEq)]
fn Button(label: String) { /* … */ }
```

the macro emits:

```rust
#[derive(Clone, Debug, PartialEq)]              // ← forwarded from the user
pub struct ButtonComponent {
    pub subtitle: Option<String>,               // prop
    pub tooltip: Option<MyTooltip>,             // struct field
    pub hover_count: usize,                     // struct field
    pub variant: ButtonVariant,                 // variant prop
    pub label: Option<String>,                  // required prop
    pub children: Vec<Element>,                 // extension
    pub style: Style,                           // extension
}

impl ButtonComponent {
    pub fn new() -> Self { /* … */ }

    pub fn label(self, value: impl Into<String>) -> Self { /* … */ }
    pub fn subtitle(self, value: impl Into<Option<String>>) -> Self { /* … */ }
    pub fn variant(self, value: impl Into<ButtonVariant>) -> Self { /* … */ }
}

impl Default for ButtonComponent { /* delegates to new() */ }

impl freyacn::ChildrenExt for ButtonComponent { /* … */ }
impl freyacn::StyleExt for ButtonComponent { /* … */ }

impl freyacn::Component for ButtonComponent {
    fn render(&self) -> impl freyacn::IntoElement {
        let children: &Vec<Element> = &self.children;

        // The `style` local is the merged result:
        //   variant.resolve()  +  self.style (user overrides)
        let __freyacn_merged_style: Style = {
            let mut __freyacn_base = Style::default();
            __freyacn_base = __freyacn_base.merge(
                freyacn::Variant::resolve(&self.variant)
            );
            __freyacn_base.merge(self.style.clone())
        };
        let style: &Style = &__freyacn_merged_style;

        let subtitle: &Option<String> = &self.subtitle;
        let tooltip: &Option<MyTooltip> = &self.tooltip;
        let hover_count: &usize = &self.hover_count;
        let variant: &ButtonVariant = &self.variant;

        let label: &String = self.label.as_ref()
            .expect("required prop `label` was not set before render");

        /* original function body */
        // The last expression is the return value; no `.into()` required.
    }
}

pub fn Button() -> ButtonComponent { ButtonComponent::new() }

#[macro_export]
macro_rules! Button {
    ($($key:ident = $val:expr),* $(,)?) => {
        ButtonComponent::new()$(.$key($val))*
    };
}
```

Every item carries generated documentation describing its role and kind —
`cargo doc` produces a complete reference for free.

---

## Compile errors

### Duplicate name across sources

```rust
#[component(props( name: String = "ada".to_string()))]
fn Card(name: String) { /* … */ }
```

```
error: `name` is declared more than once (a function parameter and an
       attribute prop (`props(...)`)). Each field name may be used by
       exactly one declaration.
```

### Missing type

```rust
#[component(props(name))]
fn Card() { /* … */ }
```

```
error: missing type — every field must declare its type, e.g. `name: String`.
       (Required props are declared via the function signature instead.)
```

### `required` keyword

```rust
#[component(props( required name: String))]
fn Card(name: String) { /* … */ }
```

```
error: `required` is not a keyword. Function parameters are the only way to
       declare required props: `fn Card(name: String)`. Attribute props are
       optional (or defaulted with `= expr`).
```

### Collision with an extension field

```rust
#[component(props( children: Vec<Element>), ext(children))]
fn Broken() { /* … */ }
```

```
error: `children` collides with a field contributed by `ext(...)`.
       Extension fields are reserved by freyacn and cannot be shadowed by
       an attribute prop (`props(...)`).
```

### Missing extension dependency

```rust
#[extensions(corner_radius)]
#[derive(Default)]
pub struct Widget;
```

```
error: `corner_radius` requires `style` to also be listed in `ext(...)`.
       Add it, e.g. `ext(style, corner_radius)`.
```

### Variants without `ext(style)`

```rust
#[component(
    variants( variant: ButtonVariant = ButtonVariant::Default),
)]
fn Button() { /* … */ }
```

```
error: `variants(...)` requires `style` to also be listed in `ext(...)`,
       so the resolved variant styles have somewhere to be merged.
       Add `style`, e.g. `ext(style, children)`.
```

### Variant without a default

```rust
#[component(variants( variant: ButtonVariant), ext(style))]
fn Button() { /* … */ }
```

```
error: variant prop `variant` requires a default, e.g.
       `variant: MyEnum = MyEnum::Default`
```

### Unknown group

```rust
#[component(styles( foo: String))]
fn Card() { /* … */ }
```

```
error: unknown group `styles`. Expected `props(...)`, `ext(...)`,
       or `variants(...)`.
```

### Unknown extension name

```rust
#[extensions(shadow)]
#[derive(Default)]
pub struct Widget;
```

```
error: Unsupported extension `shadow`. Valid names: `children`, `key`,
       `style`, `event_handlers`, `corner_radius`.
```

### Misplaced `#[struct_fields]`

```rust
#[struct_fields(tooltip: MyTooltip)]   // ← wrong order
#[component(props( name: String))]
fn Card(name: String) { /* … */ }
```

```
error: `#[struct_fields(...)]` must be placed **below** `#[component(...)]`
```

---

## Full worked example

```rust
use freyacn::{component, extensions, struct_fields, variant, StyleExt};
use freya::prelude::*;

// ── Variant enums ────────────────────────────────────────────────────

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground px-4 py-2")]
    #[default]
    Default,

    #[variant("bg-destructive text-destructive-foreground px-4 py-2")]
    Destructive,

    #[variant("border-2 border-primary bg-transparent text-primary px-4 py-2")]
    Outline,
}

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[variant("text-xs px-2 py-1")]    Sm,
    #[variant("text-sm px-4 py-2")]
    #[default] Md,
    #[variant("text-base px-6 py-3")] Lg,
}

// ── Component ────────────────────────────────────────────────────────

/// A button with variant, size, optional tooltip and full styling.
#[component(
    props(
        tooltip: String,
        disabled: bool = false,
    ),
    ext(children, style),
    variants(
        variant: ButtonVariant = ButtonVariant::Default,
        size:    ButtonSize    = ButtonSize::Md,
    ),
)]
#[struct_fields( hover_count: usize = 0)]
#[derive(Clone, PartialEq)]
fn Button(label: String) {
    // The `style` local is the merged result of:
    //   Style::default()
    //     .merge(variant.resolve())
    //     .merge(size.resolve())
    //     .merge(self.style.clone())   ← user overrides win
    let tip = tooltip.as_deref().unwrap_or("");

    rect()
        .background(style.background.clone().unwrap_or_default())
        .padding(style.padding.unwrap_or_default())
        .corner_radius(style.corner_radius.unwrap_or_default())
        .opacity(if *disabled { 0.5 } else { 1.0 })
        .child(label().text(label.as_str()))
        .child(label().text(tip))
        .children(children.clone())
}

// ── Standalone extensions on a hand-written struct ──────────────────

/// A very small wrapper used for pure styling, no component.
#[extensions(style)]
#[derive(Clone, PartialEq, Default)]
pub struct Chip {
    pub label: String,
}

impl Chip {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), ..Default::default() }
    }
}

fn render_chip(chip: Chip) -> impl IntoElement {
    let chip = chip.bg_slate_100().text_slate_800().px_2().py_1();
    label().text(chip.label.as_str())
}

// ── Usage ────────────────────────────────────────────────────────────

fn app() -> impl IntoElement {
    let mut b = Button()
        .label("Save")
        .variant("destructive")       // base layer
        .size(ButtonSize::Lg)         // base layer
        .tooltip("Save the current document")
        .disabled(false)
        .bg_green_50()                // override — wins over the variant
        .p_6()                        // override — wins over the size
        .corner_radius(12.0)
        .shadow_md()
        .class("font-semibold border-2 border-slate-200");

    b.hover_count = 3;

    // Change the variant after the fact. Base recomputes; overrides persist.
    b.variant("outline");

    let chip = Chip::new("Beta");
    let _ = render_chip(chip);

    b
}
```

---

## Design notes

**Why does each source decide the prop kind?**

* *Is this part of the component's public API?* — `props(...)`,
  `variants(...)`, and fn params: **yes**. They get setters and appear in
  the docs as props.
* *Is this internal storage the user manages?* — `#[struct_fields(...)]`:
  **no**. No setters, documented separately.
* *Is this framework‑provided machinery?* — `ext(...)` / `#[extensions(...)]`:
  **no**. Reserved by freyacn, checked for collisions, documented as
  extensions.

Collapsing them would force every declaration to be ambiguous about intent,
and make "no setter" and "reserved name" impossible to express.

**Why doesn't the component function need a return type?**

Because the macro is the one that emits the `Component::render` method,
and `render` already has a fixed signature — `fn render(&self) -> impl
IntoElement`. The user's body is spliced in as the body of `render`, so
the return type comes from the surrounding signature, not from the
function the user wrote. That also means no `.into()` is needed: the last
expression of the body is coerced to `impl IntoElement` by the standard
`impl Trait` return mechanism.

**Why does the style merge happen at render time, not construction time?**

Because `Variant::resolve()` reads the active theme. Resolving at
construction time would freeze the theme — dark/light toggles, theme
switches, and any dynamic theming would be invisible to already‑built
components. Rendering happens inside a Freya context, so it's the natural
place to read the theme.

The cost is one parse + one merge per variant per render. For a typical
button that's two parses and three merges. It's cheap relative to the
renderer's work. If it ever shows up in a profile, memoizing per
`(component, theme)` is straightforward.

**Why do user overrides persist across variant changes?**

Because intent should not be discarded silently. If a caller explicitly
set `.bg_green_50()`, they meant it. Changing the variant only changes the *base* style; the override remains until the
caller changes it. This is
consistent with how shadcn/ui and similar layered systems behave — the
user's `className` always wins.

**Why is `freyacn-macros` internal to `freyacn`?**

Because the macros emit paths that point at `freyacn` types (`::freyacn::Component`, `::freyacn::Style`,
`::freyacn::Variant`, …).
Exposing `freyacn-macros` as an end‑user dependency would mean users who
pull it directly also have to pull `freyacn`, and could end up with
mismatched versions if they pin them independently. Re‑exporting from
`freyacn` keeps the version surface a single crate.

**Why `impl Into<Option<T>>` for optional setters?**

Because std already gives us `From<T> for Option<T>` and `From<T> for T`,
so a single signature accepts a bare `T` (auto‑wrapped), an `Option<T>`
(identity), and `None` — with no runtime overhead and no ambiguity.

**Why a compile error instead of a silent override for name collisions?**

Silent override hides a real bug: the user declared the same thing twice,
or tried to shadow an extension field. Refusing to compile is the loudest,
earliest signal.

**Why are extension fields appended last?**

Props, variants, and struct fields are what the user wrote; extensions are
opt‑in machinery. Putting user‑written declarations first keeps the common
case at the top of the struct.

**Why generate a free function *and* a macro with the same name?**

They live in different Rust namespaces (value vs macro), so there's no
conflict. The function is the zero‑argument fast path; the macro adds
named‑argument construction.

**Why forward user attributes instead of baking in derives?**

Because the right set of derives depends on how the component is used: a
keyed child needs `PartialEq`; one stored in a collection needs `Clone`; one
inspected during debugging needs `Debug`. Baking any of them in forces users
to accept them and hides the cost. Forwarding keeps the macro agnostic — the
user writes what they want and sees the exact set of traits the struct
implements in its source.

**Why is there a stub `#[struct_fields]` proc‑macro?**

Because attribute macros are processed top‑to‑bottom. If
`#[struct_fields(...)]` is placed above `#[component(...)]`, the compiler
reaches `struct_fields` first — and without a stub, it errors with "cannot
find attribute". The stub produces a friendly message. When the ordering is
correct, `#[component]` consumes the attribute before the compiler ever sees
the stub, so the stub never fires.

---

## Roadmap

The following items are planned but not yet implemented:

* **`resolve_with(&Theme)`.** `.resolve()` requires a Freya render context.
  A theme‑explicit variant is planned for tests and out‑of‑render use. This
  requires threading the theme through the palette lookup machinery, which
  is a standalone change.
* **More built‑in extensions.** More bundles will land once the current
  surface stabilises.
* **Auto‑set position when a side is set.** Setting `.top(10.0)` without a
  position kind currently leaves the value stranded. A behavior decision is
  pending.

---

## Contributing

This crate is unstable and moves fast. Before opening a PR, please open an
issue describing what you want to change and why — the API surface is still
in flux, and a quick design discussion will save everyone time.

---

## License

MIT.

[freyacn]: https://github.com/freyacn
[freya]: https://crates.io/crates/freya