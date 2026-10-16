# `freyacn-macros`

> ⚠️ **Work in progress.** This crate is under active development and its
> public API is **not yet stable**. Breaking changes may land in any release.
> The shapes shown below are the current target, not a compatibility promise.
> Expect rough edges, missing features, and doc/code drift.

Procedural macros for the [`freyacn`][freyacn] crate. They turn plain functions
and enums into styled, prop-driven components for [Freya][freya], with
Tailwind-inspired styling built in and complete generated documentation.

Everything described here is available through the prelude:

```rust
use freyacn::prelude::*;
```

The prelude re-exports the macros, `Style` / `StyleExt`, the styled `Div`
component, `Theme`, the state helpers, and Freya's own prelude — so you do **not** need a direct `freya` dependency in
your `Cargo.toml`.

---

## Relationship to `freyacn`

**`freyacn-macros` is an internal dependency of `freyacn`.** It is not meant
to be used on its own, for two reasons:

1. **`freyacn` re-exports every macro.** Everything documented here is
   available through the prelude:
   ```rust
   use freyacn::prelude::*;
   // component, extensions, struct_fields, variant,
   // Style, StyleExt, Div, Theme, StateFlags, …
   ```
   Adding `freyacn-macros` to your `Cargo.toml` alongside `freyacn` would
   pull in the same code twice and, worse, risk pulling *different* versions.

2. **The macros emit paths to `freyacn` types.** The generated code refers to
   `::freyacn::prelude::Component`, `::freyacn::prelude::Style`, and so on.
   Without `freyacn` in the dependency graph, expansion would fail with
   "cannot find crate `freyacn`".

The only reason to depend on `freyacn-macros` directly is if you are forking
the macro implementation itself. For every other use case, use `freyacn`.

---

The crate exposes four attributes:

* **`#[component(...)]`** — turns a function into a component struct plus a
  PascalCase constructor function and a companion macro. All three field
  groups (`props`, `ext`, `variants`) live inside this attribute.
* **`#[variant]`** — turns an enum into a style-resolving variant type with
  `From<&str>` / `From<String>`, `.resolve()`, and an
  `impl freyacn::prelude::Variant`.
* **`#[struct_fields(...)]`** — placed below `#[component]`, adds internal
  storage fields the user's own trait impls can manage.
* **`#[extensions(...)]`** — standalone attribute that opts any struct into
  freyacn's built-in extension bundles. The `ext(...)` group inside
  `#[component]` is the same feature for generated components.

---

## Installation

```toml
[dependencies]
freyacn = "0.2"
```

That's the whole list. `freya` is a transitive dependency, and its prelude is
re-exported through `freyacn::prelude`. You never need to write
`use freya::prelude::*;` or add `freya` to your own `Cargo.toml`.

```rust
use freyacn::prelude::*;
```

---

## Quick start

```rust
use freyacn::prelude::*;

/// Visual variants for a button.
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground px-4 py-2 rounded-lg")]
    #[default]
    Default,

    #[variant("bg-destructive text-destructive-foreground px-4 py-2 rounded-lg")]
    Destructive,

    #[variant("border-2 border-primary bg-transparent text-primary px-4 py-2 rounded-lg")]
    Outline,
}

#[component(
    props( badge: String = "new".to_string()),
    ext(style, children, key, event_handlers),
    variants( variant: ButtonVariant = ButtonVariant::Default),
)]
pub fn Button() {
    // `style` is the merged result of `variant.resolve()` and the user's
    // own `.style(...)` / `.bg_primary()` / … calls.
    Div()
        .children(children.clone())
        .with_event_handlers(event_handlers.clone())
        .merge_style(style.clone())
        .child(label().text("Save"))
}
```

> **No return type needed.** The macro generates the `impl IntoElement`
> signature for the component's `render` method. Write the function body as
> the expression you want rendered, and skip both the `-> impl IntoElement`
> annotation and any trailing `.into()`.

> **Components don't have a `label` prop by default.** A button in shadcn/ui
> takes its text from its children. Pass content with `.child(label().text(...))`,
> or add a `label: String` function parameter if you'd rather the call site
> look like `Button().label("Save")` — the macro treats function parameters
> as required props.

Three equivalent ways to construct the component above:

```rust
Button().variant(ButtonVariant::Destructive);         // typed
Button().variant("destructive");                      // &str
Button!(variant = "destructive");                     // macro
```

---

## The `Div` component

`Div` is the workhorse you compose inside your own components. It renders a
Freya primitive after folding the `ext`-provided `Style` into concrete
values, and it owns the runtime state observation so consumers don't have to.

### What `Div` observes

| Flag            | Source                               |
|-----------------|--------------------------------------|
| `DARK`          | `use_cn_theme().is_dark`             |
| `HOVER`         | `on_pointer_over` / `on_pointer_out` |
| `ACTIVE`        | `on_pointer_down` / `on_pointer_out` |
| `FOCUS`         | `use_focus(a11y_id)`                 |
| `FOCUS_VISIBLE` | `use_focus(a11y_id)`                 |
| `DISABLED`      | the `disabled` prop                  |

`VISITED`, `CHECKED`, `TARGET`, and `FOCUS_WITHIN` have no observable source
on a plain div and are left unwired.

### Props

`Div` currently exposes:

```rust
#[component(
    props(
        disabled: bool = false,
        focusable: bool = false,
    ),
    ext(style, children, event_handlers, accessibility),
)]
#[struct_fields(on_hover: EventHandler<Event<PointerEventData>>)]
#[derive(PartialEq, Clone)]
pub fn Div() { /* … */ }
```

- **`disabled`** — when `true`, folds `disabled:` state styles, disables
  pointer-event observation, and marks the element as disabled to assistive
  tech.
- **`focusable`** — off by default, so a plain `<div>` isn't in the tab
  order unless you ask for it.
- **`on_hover`** — an optional callback invoked when the pointer enters the
  element. Fires alongside the internal hover state, so `hover:` styles still
  apply whether or not you pass a handler.

`on_hover` is a `#[struct_fields]` entry rather than a `props(...)` entry,
which means it's stored as `Option<EventHandler<…>>` (the macro wraps
struct fields in `Option` unless they carry a default). The builder shape
reflects that:

```rust
Div()
.disabled(false)
.focusable(true)
.merge_style(style.clone())
.on_hover( | _ | println!("hovered"))
```

### Typical use

```rust
Div()
.style(
"flex flex-col gap-4 p-6 \
         bg-card text-card-foreground \
         rounded-lg shadow-sm \
         hover:shadow-md \
         focus-visible:ring-2 focus-visible:ring-ring/50"
)
.focusable(true)
.on_hover( | _ | println!("hovered"))
.child(label().text("Hello"))
```

Every class-string construct the parser supports is available here: palette
colours, semantic tokens, arbitrary values, state prefixes (`hover:`,
`focus-visible:`, `dark:`, `disabled:`, `active:`), and Tailwind v4 ring
utilities.

### What `Div` forwards to the primitive

All fields with a Freya counterpart: background, text colour, typography (size, slant, weight, width, align, decoration,
font families, text height,
text shadows), corner radius, border, padding, margin, all size constraints,
position, overflow, flex layout (direction, alignment, content, spacing),
opacity, shadows, z-index via layer, text overflow, pointer events,
interactive, rotate/scale/blur/transform-origin, child offsets, and
accessibility.

Fields without a primitive today (rings, outlines, per-axis overflow, list
style, `aspect_ratio`, `order`, `flex_shrink`, `visibility`, `user_select`,
`resize`) are parsed and stored on `Style` but not applied. They'll start
rendering the moment Freya exposes the corresponding methods.

### Writing your own `Div`-style component

If you need a variant of `Div` with a different subset of the style surface
or extra domain behaviour, copy its body into your own component. The
pattern:

```rust
#[component(
    props( disabled: bool = false),
    ext(style, children, event_handlers, accessibility),
)]
#[derive(PartialEq, Clone)]
pub fn MyBox() {
    // Compute the active flag set.
    let mut active = StateFlags::NONE;
    let theme = use_cn_theme().read();
    if theme.is_dark { active = active.or(StateFlags::DARK); }
    drop(theme);
    if *disabled { active = active.or(StateFlags::DISABLED); }

    // Fold states, then build the primitive.
    let style = style.clone().with_active_states(active);

    rect()
        .background(style.background.clone().unwrap_or_default())
        .border(style.to_border())
        .padding(style.padding.unwrap_or_default())
        .position(style.to_position().unwrap_or_default())
        // …whatever else your box needs…
        .children(children.clone())
}
```

---

## Style layering

The most important runtime behaviour to understand is **how `style` is built
at render time** when a component has `variants(...)`:

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
.p_6()               // override padding — wins over the variant's px-4 py-2
.text_xl()           // override font size
.style("font-bold text-white")   // class strings also go to the override layer
```

**Changing the variant leaves user intent intact.** If a caller explicitly
set a property, they meant it. `.variant("x")` only updates the *base*.

### Merging rules

`Style::merge(other)` produces a new `Style` where:

* **Scalar fields** (`Option<T>`) → `other.field.or(self.field)`. Set values
  in `other` win; unset values leave `self` alone.
* **Stack fields** (`shadows`, `font_families`, `text_shadows`) →
  concatenated.
* **`Gaps` fields** (`padding`, `margin`) → merged **per side**, so a variant
  setting only `px-4` does not wipe the base class's `py-2`.

There is also `Style::or(defaults)` for the inverse (fill gaps only), plus
`merge_from` / `or_from` for in-place mutation.

### State-conditional styles

Any class string can carry state prefixes:

```rust
Div()
.style(
"bg-background \
         hover:bg-muted \
         focus-visible:ring-2 focus-visible:ring-ring/50 \
         active:bg-muted/80 \
         disabled:opacity-50 \
         dark:bg-input/30 dark:hover:bg-input/50"
)
```

These parse into `Style::states`, a list of `(StateFlags, Style)` pairs.
`Div` computes the active flag set from runtime state and folds the matching
entries at render time via `Style::with_active_states(active)`.

The fold is one-way and in declaration order — later matching entries win.

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
accept comma-separated declarations of the same shape:

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
        None => Div(),
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
`T: From<…>`. Pairs naturally with `#[variant]`-annotated enums.

```rust
#[component(
    variants( variant: ButtonVariant = ButtonVariant::Default),
    ext(style, children),
)]
fn Button() {
    Div()
        .merge_style(style.clone())
        .children(children.clone())
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
is generated and they are documented in a separate section. Use them to
store data your own trait impls need.

```rust
use freyacn::prelude::*;

#[component(props( subtitle: String))]
#[struct_fields(
    tooltip: MyTooltip,              // Option<MyTooltip> = None
    hover_count: usize = 0,          // usize = 0
)]
fn Card(name: String) {
    let _ = tooltip.as_ref().map(|t| t.text.as_str()).unwrap_or("");
    let _ = hover_count;
    Div().child(label().text(name.as_str()))
}

// Your own trait — plain Rust, no macro.
#[derive(Default)]
pub struct MyTooltip {
    pub text: String,
    pub visible: bool,
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
* `impl MyEnum { fn resolve(&self) -> freyacn::prelude::Style; }` —
  convenience alias that doesn't require importing `Variant`.
* `impl freyacn::prelude::Variant for MyEnum` — the trait-based resolver.
* `impl From<&str> for MyEnum`
* `impl From<String> for MyEnum`

### Declaration

```rust
use freyacn::prelude::*;

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

    #[variant("bg-transparent text-primary underline underline-offset-4")]
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
Div().merge_style(s);
```

### The `Variant` trait

`#[variant]` generates `impl freyacn::prelude::Variant for MyEnum`. The
trait is:

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
`#[component]`) opt a struct into freyacn's built-in **extension bundles**.
Each name maps to a bundle of:

* a field to add to the struct,
* an initialiser for that field,
* a render-body local binding, and
* an `impl XxxExt for YourStruct` block.

### Supported names

| Name             | Trait                                | Field(s) added                                           | Trait method             |
|------------------|--------------------------------------|----------------------------------------------------------|--------------------------|
| `children`       | `freyacn::prelude::ChildrenExt`      | `children: Vec<Element>`                                 | `get_children`           |
| `key`            | `freyacn::prelude::KeyExt`           | `key: DiffKey`                                           | `write_key`              |
| `style`          | `freyacn::prelude::StyleExt`         | `style: Style`                                           | `get_style`              |
| `event_handlers` | `freyacn::prelude::EventHandlersExt` | `event_handlers: FxHashMap<EventName, EventHandlerType>` | `get_event_handlers`     |
| `accessibility`  | `freyacn::prelude::AccessibilityExt` | `accessibility: AccessibilityData`                       | `get_accessibility_data` |

### On a hand-written struct

The attribute works on any struct with named fields. The struct must
derive or implement `Default` so the added fields can be initialised.

```rust
use freyacn::prelude::*;

#[extensions(children, key, style, accessibility)]
#[derive(Clone, PartialEq, Default)]
pub struct PlainBox;
```

After expansion, `PlainBox` has:

```rust
pub struct PlainBox {
    pub children: Vec<Element>,
    pub key: DiffKey,
    pub style: Style,
    pub accessibility: AccessibilityData,
}

impl ChildrenExt for PlainBox { /* get_children */ }
impl KeyExt for PlainBox { /* write_key */ }
impl StyleExt for PlainBox { /* get_style */ }
impl AccessibilityExt for PlainBox { /* get_accessibility_data */ }
```

Because `#[derive(Default)]` was forwarded, `PlainBox::default()` gives you
an empty instance ready to be built up:

```rust
let mut b = PlainBox::default ();
b.push(label().text("hi"));                    // ChildrenExt
b.key("root");                                 // KeyExt
let b = b.bg_primary().p_4();                  // StyleExt
let b = b.a11y_role(AccessibilityRole::Group); // AccessibilityExt
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

---

## Style merging utilities

`Style` exposes a small set of merging methods used by the component macro.
They are public because they are useful on their own.

| Method                                                | Effect                                                        |
|-------------------------------------------------------|---------------------------------------------------------------|
| `Style::merge(self, other) -> Style`                  | `other` wins on every field it set. Stack fields concatenate. |
| `Style::or(self, defaults) -> Style`                  | `self` wins; `defaults` fills gaps only.                      |
| `Style::merge_from(&mut self, other)`                 | In-place variant of `merge`.                                  |
| `Style::or_from(&mut self, defaults)`                 | In-place variant of `or`.                                     |
| `Style::is_empty(&self) -> bool`                      | `true` if no field has been set.                              |
| `Style::style_debug(self, s) -> (Style, Vec<String>)` | Like `.style(s)` but also returns the unrecognised tokens.    |

`StyleExt` gets two provided methods that lift these onto every component:

| Method                | Effect                                 |
|-----------------------|----------------------------------------|
| `.merge_style(other)` | `self.map_style(\|s\| s.merge(other))` |
| `.or_style(defaults)` | `self.map_style(\|s\| s.or(defaults))` |

### `style_debug`

The regular `.style("...")` method silently ignores tokens it doesn't
recognise — matching Tailwind's permissive behaviour. When you want to know
which tokens had no effect (for typo detection or test assertions), use
`style_debug`:

```rust
let (style, unknown) = Style::default ()
.style_debug("p-4 bg-primary shdaow-md not-a-thing");

assert_eq!(unknown, vec!["shdaow-md", "not-a-thing"]);
```

### State folding

Call `with_active_states` to fold every matching `hover:` / `focus:` /
`dark:` / `active:` / `disabled:` entry:

```rust
let mut active = StateFlags::NONE;
if theme.is_dark        { active = active.or(StateFlags::DARK); }
if * hovering.read()     { active = active.or(StateFlags::HOVER); }
if focus().is_focused() { active = active.or(StateFlags::FOCUS_VISIBLE); }

let folded = style.with_active_states(active);
```

`Div` does this for you; you only need to call it yourself if you're
building a primitive that doesn't go through `Div`.

### Materialisation helpers

* `to_border() -> Option<Border>` — folds the four per-side widths, the
  border colour, and the divide colour into one `Border`.
* `to_position() -> Option<Position>` — folds `position_kind` plus the side
  offsets into a `Position`. Called at render time, so `relative top-4` and
  `top-4 relative` produce the same result regardless of token order.

---

## Supported class-string tokens

The parser accepts a broad subset of Tailwind plus a few shadcn-specific
conveniences. Highlights:

### Layout

`flex`, `inline-flex`, `flex-row`, `flex-col`, `flex-wrap`, `flex-1`,
`grow`, `grow-0`, `shrink`, `shrink-0`, `shrink-1`, `content-*`,
`justify-*`, `items-*`, `self-*`, `gap-*`, `gap-x-*`, `gap-y-*`, `w-*`,
`h-*`, `min-w-*`, `min-h-*`, `max-w-*`, `max-h-*`, `size-*`.

All spacing utilities accept the full Tailwind numeric scale (`p-2`,
`p-2.5`, `p-2_5`, `p-[10px]`, `p-[1.5rem]`) and follow per-side override
semantics — `p-2 px-4` gives vertical `8`, horizontal `16`.

### Colour

`bg-<family>-<shade>`, `text-<family>-<shade>`, `border-<family>-<shade>`,
`ring-<family>-<shade>`, plus every semantic token (`bg-primary`,
`text-muted-foreground`, `border-input`, `ring-ring`, `bg-sidebar-accent`,
…). Opacity modifiers work everywhere: `bg-primary/50`,
`text-red-500/[0.37]`.

Arbitrary hex values: `bg-[#ff0000]`, `text-[#rgb]`,
`border-[#rrggbbaa]`.

### Typography

`text-xs` … `text-9xl`, `font-thin` … `font-black`, `italic`, `not-italic`,
`underline`, `overline`, `line-through`, `no-underline`, `uppercase`,
`lowercase`, `capitalize`, `normal-case`, `text-left` / `-center` /
`-right` / `-justify`, `leading-*`, `tracking-*`, `truncate`,
`text-ellipsis`, `text-clip`, `line-clamp-*`, `whitespace-*`, `break-*`,
`text-[12.8px]`.

### Border, radius, effects

`border`, `border-0/2/4/8`, `border-t/r/b/l/x/y`, `border-<colour>`,
`border-solid/dashed/dotted/double/none`, `rounded`,
`rounded-none/sm/md/lg/xl/2xl/3xl/full`, `rounded-[10px]`,
`shadow-sm/md/lg/xl/2xl/none`, `opacity-0/25/50/75/100`,
`opacity-[0.37]`.

### Rings (Tailwind v4)

`ring`, `ring-0/1/2/3/4/6/8`, `ring-[3px]`, `ring-<colour>`,
`ring-offset-0/1/2/4/8`, `ring-offset-<colour>`, `inset-ring`,
`inset-ring-0/1/2/4/8`, `inset-ring-<colour>`.

### Position, overflow, sizing

`relative`, `absolute`, `global`, `top-*`, `right-*`, `bottom-*`, `left-*`,
`inset-*`, `inset-x-*`, `inset-y-*`, `z-*`, `overflow-clip`,
`overflow-none`, `overflow-x-*`, `overflow-y-*`.

### Transforms

`rotate-0/1/2/3/6/12/45/90/180`, `-rotate-*`, `rotate-[37.5]`,
`scale-*`, `-scale-*`, `scale-[1.25]`, `blur`, `blur-none/sm/md/lg/xl/2xl/3xl`,
`blur-[3]`, `origin-*`.

### State prefixes

Chainable: `dark:hover:bg-slate-800`, `focus-visible:ring-2`,
`disabled:opacity-50`, `active:bg-muted/80`. Recognised prefixes:
`dark`, `hover`, `focus`, `focus-visible`, `focus-within`, `active`,
`disabled`, `visited`, `checked`, `target`.

### Arbitrary values

Every length-style token accepts `[<value>]` brackets. Units understood:
`px`, `rem`, `em`, `pt`, `%`, `vw` / `vh` (for sizes).

---

## Theme

The theme system lives in `freyacn::theme` and is re-exported through the
prelude. It provides the `Colors` palette and the semantic `Theme`.

### Building a theme

```rust
use freyacn::prelude::*;

// Fluent (recommended)
let theme = Theme::light().base_color("zinc").theme_color("blue");
let theme = Theme::dark().stone().theme_color("rose");
let theme = Theme::dark().neutral();                  // base default accent

// Underlying constructor — useful when the base name is dynamic
let base: & str = pick_base();
let theme = Theme::from_base(base, true).theme_color("emerald");
```

`.base_color(...)` **resets** `primary` / `accent` / `ring` to the new base's
defaults, so `.theme_color(...)` must come **after**.

### Providing the theme

```rust
fn App() -> impl IntoElement {
    use_init_cn_theme(Theme::light().base_color("slate").theme_color("blue"));

    Div()
        .style("min-h-screen bg-background text-foreground")
        .child(MyComponent())
}
```

`use_init_cn_theme` reuses an existing context if there is one. For a scoped
override that always creates a fresh context, use `use_provide_cn_theme`:

```rust
fn DarkPanel() -> impl IntoElement {
    use_provide_cn_theme(Theme::dark().zinc().theme_color("rose"));
    Div().style("bg-background text-foreground").child("…")
}
```

### Consuming the theme

```rust
fn MyComponent() -> impl IntoElement {
    let theme = use_cn_theme().read();

    // The theme carries is_dark plus every semantic token:
    let bg = theme.background;
    let fg = theme.foreground;
    let primary = theme.primary;

    Div()
        .background(bg)
        .color(fg)
        .style("p-4")
}
```

`get_cn_theme_or_default()` returns a `Readable<Theme>` and never panics —
useful for built-in components that should work without an explicit
provider.

### Semantic tokens

`background`, `foreground`, `primary`, `primary_foreground`, `secondary`,
`secondary_foreground`, `muted`, `muted_foreground`, `accent`,
`accent_foreground`, `destructive`, `destructive_foreground`, `card`,
`card_foreground`, `popover`, `popover_foreground`, `border`, `input`, `ring`,
`chart_1` … `chart_5`, `sidebar`, `sidebar_foreground`, `sidebar_primary`,
`sidebar_primary_foreground`, `sidebar_accent`, `sidebar_accent_foreground`,
`sidebar_border`, `sidebar_ring`, `disabled`, `disabled_foreground`, plus the
radius scale (`radius_sm`, `radius_md`, `radius_lg`, `radius_xl`).

Every semantic token is also usable in a style string: `bg-primary`,
`text-primary-foreground`, `border-border`, `ring-ring`, `bg-sidebar-accent`,
and so on all resolve through `semantic_colour` at parse time.

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

Three attributes are **not** forwarded, because the macro handles them
itself:

* `#[doc]` — re-emitted first, so it isn't duplicated.
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

impl freyacn::prelude::ChildrenExt for ButtonComponent { /* … */ }
impl freyacn::prelude::StyleExt for ButtonComponent { /* … */ }

impl freyacn::prelude::Component for ButtonComponent {
    fn render(&self) -> impl freyacn::prelude::IntoElement {
        let children: &Vec<Element> = &self.children;

        // The `style` local is the merged result:
        //   variant.resolve()  +  self.style (user overrides)
        let __freyacn_merged_style: Style = {
            let mut __freyacn_base = Style::default();
            __freyacn_base = __freyacn_base.merge(
                freyacn::prelude::Variant::resolve(&self.variant)
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
       `style`, `event_handlers`, `accessibility`.
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
use freyacn::prelude::*;

// ── Variant enums ────────────────────────────────────────────────────

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground px-4 py-2 rounded-lg")]
    #[default]
    Default,

    #[variant("bg-destructive text-destructive-foreground px-4 py-2 rounded-lg")]
    Destructive,

    #[variant("border border-border bg-background text-foreground px-4 py-2 rounded-lg hover:bg-muted")]
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
    ext(style, children, key, event_handlers, accessibility),
    variants(
        variant: ButtonVariant = ButtonVariant::Default,
        size:    ButtonSize    = ButtonSize::Md,
    ),
)]
#[struct_fields( hover_count: usize = 0)]
#[derive(Clone, PartialEq)]
pub fn Button() {
    // `style` is the merged result of:
    //   Style::default()
    //     .merge(variant.resolve())
    //     .merge(size.resolve())
    //     .merge(self.style.clone())   ← user overrides win
    let tip = tooltip.as_deref().unwrap_or("");

    Div()
        .merge_style(style.clone())
        .disabled(*disabled)
        .focusable(true)
        .accessibility(accessibility.clone())
        .with_event_handlers(event_handlers.clone())
        .children(children.clone())
        .child(label().text(tip))
}

// ── Standalone extensions on a hand-written struct ──────────────────

/// A very small wrapper used for pure styling, no component.
#[extensions(style)]
#[derive(Clone, PartialEq, Default)]
pub struct Chip {
    pub text: String,
}

impl Chip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }
}

fn render_chip(chip: Chip) -> impl IntoElement {
    let chip = chip.bg_slate_100().text_slate_800().px_2().py_1();
    label().text(chip.text.as_str())
}

// ── Usage ────────────────────────────────────────────────────────────

fn app() -> impl IntoElement {
    use_init_cn_theme(Theme::light().base_color("slate").theme_color("blue"));

    let mut b = Button()
        .variant("destructive")       // base layer
        .size(ButtonSize::Lg)         // base layer
        .tooltip("Save the current document")
        .disabled(false)
        .bg_green_50()                // override — wins over the variant
        .p_6()                        // override — wins over the size
        .corner_radius(12.0)
        .shadow_md()
        .style("font-semibold border-2 border-slate-200")
        .child(label().text("Save"));

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
* *Is this framework-provided machinery?* — `ext(...)` /
  `#[extensions(...)]`: **no**. Reserved by freyacn, checked for collisions,
  documented as extensions.

Collapsing them would force every declaration to be ambiguous about intent,
and make "no setter" and "reserved name" impossible to express.

**Why doesn't the component function need a return type?**

Because the macro is the one that emits the `Component::render` method, and
`render` already has a fixed signature — `fn render(&self) -> impl
IntoElement`. The user's body is spliced in as the body of `render`, so the
return type comes from the surrounding signature, not from the function the
user wrote. That also means no `.into()` is needed: the last expression of
the body is coerced to `impl IntoElement` by the standard `impl Trait`
return mechanism.

**Why does the style merge happen at render time, not construction time?**

Because `Variant::resolve()` reads the active theme. Resolving at
construction time would freeze the theme — dark/light toggles, theme
switches, and any dynamic theming would be invisible to already-built
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
user's overrides always win.

**Why does `Div` own state observation?**

Because there is no other reasonable place. State must be observed at the
component that renders the primitive — anything higher would have to
duplicate `Div`'s role and risk drift between the two. Centralising it in
`Div` means the semantics are identical everywhere: a component that
returns a `Div` gets `hover:` / `focus:` / `active:` / `disabled:` folding
for free.

Components that *want* different semantics (say, a checkbox that folds
`checked:` on a child input) can bypass `Div` and build the primitive
directly, computing their own `StateFlags`.

**Why is `freyacn-macros` internal to `freyacn`?**

Because the macros emit paths that point at `freyacn` types (`::freyacn::prelude::Component`,
`::freyacn::prelude::Style`,
`::freyacn::prelude::Variant`, …). Exposing `freyacn-macros` as an end-user
dependency would mean users who pull it directly also have to pull
`freyacn`, and could end up with mismatched versions if they pin them
independently. Re-exporting from `freyacn` keeps the version surface a
single crate.

**Why `impl Into<Option<T>>` for optional setters?**

Because std already gives us `From<T> for Option<T>` and `From<T> for T`,
so a single signature accepts a bare `T` (auto-wrapped), an `Option<T>`
(identity), and `None` — with no runtime overhead and no ambiguity.

**Why a compile error instead of a silent override for name collisions?**

Silent override hides a real bug: the user declared the same thing twice,
or tried to shadow an extension field. Refusing to compile is the loudest,
earliest signal.

**Why are extension fields appended last?**

Props, variants, and struct fields are what the user wrote; extensions are
opt-in machinery. Putting user-written declarations first keeps the common
case at the top of the struct.

**Why generate a free function *and* a macro with the same name?**

They live in different Rust namespaces (value vs macro), so there's no
conflict. The function is the zero-argument fast path; the macro adds
named-argument construction.

**Why forward user attributes instead of baking in derives?**

Because the right set of derives depends on how the component is used: a
keyed child needs `PartialEq`; one stored in a collection needs `Clone`;
one inspected during debugging needs `Debug`. Baking any of them in forces
users to accept them and hides the cost. Forwarding keeps the macro
agnostic — the user writes what they want and sees the exact set of traits
the struct implements in its source.

**Why is there a stub `#[struct_fields]` proc-macro?**

Because attribute macros are processed top-to-bottom. If
`#[struct_fields(...)]` is placed above `#[component(...)]`, the compiler
reaches `struct_fields` first — and without a stub, it errors with "cannot
find attribute". The stub produces a friendly message. When the ordering is
correct, `#[component]` consumes the attribute before the compiler ever
sees the stub, so the stub never fires.

---

## Roadmap

The following items are planned but not yet implemented:

* **`resolve_with(&Theme)`.** `.resolve()` requires a Freya render context.
  A theme-explicit variant is planned for tests and out-of-render use. This
  requires threading the theme through the palette lookup machinery, which
  is a standalone change.
* **Auto-set position when a side is set.** Setting `.top(10.0)` without a
  position kind currently leaves the value stranded. A behaviour decision is
  pending.
* **More built-in extensions.** More bundles will land once the current
  surface stabilises.
* **Ring, outline, and per-axis overflow primitives.** These fields parse
  into `Style` today but don't render, because Freya has no corresponding
  primitive methods. When it gains them, `Div` picks them up with a one-line
  addition each.

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