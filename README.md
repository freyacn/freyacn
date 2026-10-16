# FreyaCN

> ⚠️ **Work in progress.** FreyaCN is under active development and its public
> API is **not yet stable**. Breaking changes may land in any release, and no
> crate in this workspace has been published to crates.io yet.
>
> **Status:** pre-alpha. Expect rough edges, missing features, and occasional
> doc/code drift. If you want to try it, use a `git` dependency pinned to a
> specific commit. If you find a bug or want to discuss the design, please
> open an issue.

A modern, shadcn/ui-inspired component library for
[Freya](https://github.com/marc2332/freya), built with Rust, Tailwind-style
extension traits, and a proc-macro layer that turns plain functions and enums
into theme-aware, prop-driven components.

---

## Overview

**FreyaCN** brings the elegance and consistency of
[shadcn/ui](https://ui.shadcn.com) to Freya, a Rust-based GUI framework. It
provides theme-aware, highly composable components with a Tailwind-inspired
API, plus the macro machinery to author new components in plain Rust.

The library is built around four core ideas:

1. **Theme-first** — every component reads the active theme via
   `use_cn_theme()`. Switching light/dark or changing the base colour updates
   every component automatically, without a recompile.
2. **Tailwind-inspired extensions** — one trait (`StyleExt`) exposes the
   entire styling vocabulary. Familiar utility names (`p_4()`, `bg_primary()`,
   `flex_col()`, `text_xl()`, …) map directly to Freya's renderer.
3. **Macro-driven authoring** — `#[component]` turns a plain function into a
   fully-typed component struct with props, extensions, and variants.
   `#[variant]` turns an enum into a style-resolving type. No boilerplate, no
   manual prop plumbing.
4. **shadcn/ui semantics** — component variants, sizes, and tokens follow the
   design language of shadcn/ui, making them immediately recognisable.

---

## Workspace layout

The project is a Cargo workspace. Every crate lives in this repository.

| Crate            | Path              | Purpose                                                                               |
|------------------|-------------------|---------------------------------------------------------------------------------------|
| `freyacn`        | `freyacn/`        | Runtime library: `Style`, `StyleExt`, `Variant`, theme, components, extension traits. |
| `freyacn-macros` | `freyacn-macros/` | Proc-macros: `#[component]`, `#[variant]`, `#[extensions]`, `#[struct_fields]`.       |
| `freyacn-cli`    | `freyacn-cli/`    | Command-line tool for scaffolding, developing, building, and bundling FreyaCN apps.   |

`freyacn` re-exports every macro from `freyacn-macros`, so users depend on
`freyacn` only. `freyacn-macros` exists as a separate crate because proc-macro
code must compile in a distinct target, but it is **not** meant to be used
directly.

---

## Features

- **Full theming** — light/dark modes, custom base palettes (slate, stone,
  neutral, zinc, gray, …), and shadcn-style `base_color` / `theme_color`
  configuration through a fluent `Theme::light()` / `Theme::dark()` builder.
- **One styling trait** — `StyleExt` provides hundreds of provided methods
  across background, foreground, typography, borders, effects, flexbox,
  sizing, spacing, positioning, and more. Every helper is chainable.
- **Macro-driven components** — `#[component]` handles props, extensions, and
  variants in one attribute. Each field source has a distinct contract, and
  the generated struct carries full rustdoc.
- **Variant system** — `#[variant]` turns an enum into a `Style` resolver with
  string parsing (`From<&str>` / `From<String>`). Component variants merge
  into the effective style at render time.
- **Layered style resolution** — variant styles, size styles, and user
  overrides compose with deterministic `merge` / `or` semantics. Overrides
  persist across variant changes; theme updates propagate without
  reconstruction.
- **State-aware `Div`** — the foundational primitive observes pointer, focus,
  disabled, and dark-mode state, then folds `hover:` / `focus:` / `active:` /
  `disabled:` / `dark:` layers automatically. Any component that returns a
  `Div` inherits this for free.
- **style-string API** — `style("p-4 bg-primary hover:shadow-md")` parses
  Tailwind-style tokens. `style_debug` reports unrecognised tokens for typo
  hunting.
- **Tailwind v4 rings** — `ring`, `ring-offset`, and `inset-ring` utilities
  are parsed end-to-end, with arbitrary-value and palette-colour variants.
- **Accessibility** — focus, keyboard navigation, and ARIA via Freya's
  accessibility traits, wired through the `accessibility` extension.
- **Composable** — every component implements `ChildrenExt`, so nesting is
  natural.
- **Performant** — leverages Freya's diffing and layout engine; only
  re-renders what changes.

---

## Installation

```toml
[dependencies]
freyacn = "0.2.0-alpha.1"
```

> **Not published to crates.io yet.** Use a git dependency pinned to a
> specific commit for reproducible builds.

`freyacn` re-exports Freya's own prelude through `freyacn::prelude`, so you
don't need a direct `freya` dependency.

In `main.rs`:

```rust,no_run
use freyacn::prelude::*;

fn app() -> impl IntoElement {
    let theme = Theme::light().slate();
    use_init_cn_theme(theme);

    Button()
        .child("Click me")
        .variant(ButtonVariant::Default)
        .size(ButtonSize::Md)
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
```

---

## Quick start

```rust,no_run
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

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[variant("text-xs px-2 py-1")]    Sm,
    #[variant("text-sm px-4 py-2")]
    #[default] Md,
    #[variant("text-base px-6 py-3")] Lg,
}

#[component(
    props(tooltip: String),
    ext(children, style),
    variants(
        variant: ButtonVariant = ButtonVariant::Default,
        size:    ButtonSize    = ButtonSize::Md,
    ),
)]
#[derive(Clone, PartialEq)]
fn CustomButton(text: String) {
    Div()
        .merge_style(style.clone())
        .children(children.clone())
        .child(label().text(text.clone()))
}
```

Three equivalent ways to construct it:

```rust,no_run
# use freyacn::prelude::*;
# #[variant] #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)] pub enum ButtonVariant { #[default] #[variant("")] Default, #[variant("")] Destructive }
# #[variant] #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)] pub enum ButtonSize { #[default] #[variant("")] Md }
# #[component(ext(style), variants(variant: ButtonVariant = ButtonVariant::Default, size: ButtonSize = ButtonSize::Md))]
# #[derive(Clone, PartialEq)] fn CustomButton(text: String) { Div() }
CustomButton().text("Save").variant(ButtonVariant::Destructive);   // typed
CustomButton().text("Save").variant("destructive");                // &str
CustomButton!(text = "Save", variant = "destructive");             // macro
```

---

## The `#[component(...)]` macro

Every component is written as a plain Rust function. The attribute declares
the component's public API, internal storage, and variants.

### Groups

```text
#[component(
    props(    prop1: Type1, prop2: Type2 = default_expr, … ),
    ext(      extension1, extension2, … ),
    variants( variant: EnumType = EnumType::Default, … ),
)]
```

Each group is optional and may appear in any order.

### Four field sources

| Source                        | Kind      | Field type         | Initial value  | Body sees                   | Setter accepts              |
|-------------------------------|-----------|--------------------|----------------|-----------------------------|-----------------------------|
| `fn Card(name: String)`       | required  | `Option<String>`   | `None`         | `&String` (panics if unset) | `impl Into<String>`         |
| `props(name: String)`         | optional  | `Option<String>`   | `None`         | `&Option<String>`           | `impl Into<Option<String>>` |
| `props(name: String = "x")`   | defaulted | `String`           | `"x"`          | `&String`                   | `impl Into<String>`         |
| `variants(name: T = default)` | variant   | `T`                | `default`      | `&T`                        | `impl Into<T>`              |
| `#[struct_fields(name: T)]`   | internal  | `T` or `Option<T>` | `None` / —     | `&T` / `&Option<T>`         | *(no setter)*               |
| `ext(...)`                    | extension | trait-specific     | trait-specific | trait-specific              | *(no setter)*               |

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

Never write `Option<T>` yourself; declaring `name: String` in `props(...)` is
enough, and the macro wraps it.

```rust
#[component(props(subtitle: String))]
fn Heading(subtitle: String) {
    match subtitle {
        Some(s) => label().text(s.as_str()),
        None => Div(),
    }
}

h.subtitle("hi");                       // &str  → Some("hi".into())
h.subtitle(Some("hi".to_string()));     // Option<String>
h.subtitle(None);                       // Option<String>
```

The setter takes `impl Into<Option<T>>`, which std's blanket
`From<T> for Option<T>` makes ergonomic — bare `T`, `Some(T)`, and `None` all
work.

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

A default is mandatory — variant fields are stored as `T`, not `Option<T>`.
Pairs with `#[variant]`-annotated enums.

```rust
#[component(
    variants(variant: ButtonVariant = ButtonVariant::Default),
    ext(style),
)]
fn Button() {
    Div().merge_style(style.clone())
}

b.variant(ButtonVariant::Destructive);   // typed
b.variant("destructive");                // &str
b.variant(String::from("destructive"));  // String
```

> **`variants(...)` requires `ext(style)`.** The merged style is bound as a
> local called `style` in the render body, which needs the `style` extension.
> Missing `ext(style)` is a targeted compile error.

### Struct fields — `#[struct_fields(...)]`

Declared **below** `#[component]`. Same grammar as `props(...)`, but no
setter, separate documentation, and the fields exist purely so user-written
trait impls have somewhere to store data.

```rust
#[component(props(subtitle: String))]
#[struct_fields(
    tooltip: MyTooltip,              // Option<MyTooltip>
    hover_count: usize = 0,          // usize
)]
fn Card(name: String) {
    let _ = tooltip.as_ref().map(|t| t.text.as_str()).unwrap_or("");
    let _ = hover_count;
    Div().child(label().text(name.as_str()))
}

impl TooltipExt for CardComponent {
    fn set_tooltip(&mut self, text: impl Into<String>) {
        let tip = self.tooltip.get_or_insert_with(MyTooltip::default);
        tip.text = text.into();
    }
}
```

The point is to let users write `impl MyTrait for MyComponent` in plain Rust —
no `RefCell<HashMap>`, no proc-macro registry.

### Extension fields — `ext(...)`

See [`#[extensions(...)]`](#extensions) for the full list. Each name maps to
a field, an initialiser, a render binding, and a trait impl.

### Return type

The component function does **not** need a return type or a trailing
`.into()`. The macro wraps the body in a `Component::render` method whose
signature is fixed.

```rust
#[component]
fn Greeting(name: String) {
    label().text(format!("Hello, {name}!"))   // no `-> impl IntoElement`, no `.into()`
}
```

### Attribute forwarding

Every attribute on the function other than `#[doc]`, `#[struct_fields(...)]`,
and the `#[component(...)]` groups is forwarded **verbatim** to the generated
struct. The macro does not bake in any derives — you decide.

```rust
#[component(props(label: String))]
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg(feature = "experimental")]
#[my_custom_macro(arg = 42)]
fn Widget(label: String) { /* … */ }
```

### Collision rules

- **Duplicate names anywhere** (across `props`, `struct_fields`, `variants`,
  fn params) — compile error naming both sources.
- **A field whose name matches an extension field** — compile error, because
  extension fields are reserved by freyacn.

---

## The `#[variant]` macro

Applied to an enum, generates:

- `impl MyEnum { fn style_str(&self) -> &'static str; }`
- `impl MyEnum { fn resolve(&self) -> freyacn::Style; }` — convenience alias.
- `impl freyacn::Variant for MyEnum` — the trait-based resolver.
- `impl From<&str>` and `impl From<String>`, matching kebab-cased variant
  names with a fallback to the first declared variant.

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

`From<&str>` falls back to the first declared variant. Put your natural
default first and mark it `#[default]`.

### Resolving to a `Style`

`.resolve()` reads the active theme and parses the variant's style string. It
must be called inside a Freya render context. When used as a prop inside a
component that declares `variants(...)` and `ext(style)`, the component macro
calls `.resolve()` for you and merges the result into the render body's
`style` local.

The `Variant` trait exists so the component macro can merge variant styles
without knowing the concrete enum type:

```rust
pub trait Variant {
    fn resolve(&self) -> Style;
}
```

---

## Style layering

When a component declares `variants(...)` alongside `ext(style)`, the
generated `render` body computes the effective style before user code runs:

```text
Style::default()
    .merge(variant.resolve())      // declared order
    .merge(size.resolve())
    .merge(self.style.clone())     // user overrides win
```

The render body then binds `style: &Style` to the merged value.

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
.p_6()              // override padding — wins over the variant's px-4 py-2
.text_xl()          // override font size
.style("font-bold text-white")
```

**Changing the variant leaves user intent intact.** If a caller explicitly
set a property, they meant it.

### Merging primitives

`Style::merge(other)` produces a new `Style` where:

- **Scalar fields** (`Option<T>`) → `other.field.or(self.field)`. Set values
  in `other` win; unset values leave `self` alone.
- **Stack fields** (`shadows`, `font_families`, `text_shadows`) →
  concatenated.
- **`Gaps` fields** (`padding`, `margin`) → merged per side, so a variant
  setting only `px-4` doesn't wipe the base class's `py-2`.

There is also `Style::or(defaults)` for the inverse (fill gaps only), plus
`merge_from` / `or_from` for in-place mutation.

### State folding

`Div` folds `hover:` / `focus:` / `active:` / `disabled:` / `dark:` layers at
render time. If you build a primitive that doesn't go through `Div`, compute
the flag set yourself and call `with_active_states`:

```rust
let mut active = StateFlags::NONE;
if theme.is_dark        { active = active.or(StateFlags::DARK); }
if * hovering.read()     { active = active.or(StateFlags::HOVER); }
if focus().is_focused() { active = active.or(StateFlags::FOCUS_VISIBLE); }

let folded = style.with_active_states(active);
```

---

## `StyleExt` reference

`Style` is the data container. `StyleExt` is the trait that provides the
method surface. `Style` implements its own trait, so bare `Style` values
chain the same way components do.

### Base setters

`background`, `color`, `corner_radius`, `border_width`, `border_color`,
`border_style`, `font_size`, `font_weight`, `font_slant`, `font_family`,
`text_align`, `text_decoration`, `text_transform`, `line_height`,
`letter_spacing`, `text_indent`, `opacity`, `shadow`, `ring_width`,
`ring_color`, `ring_offset`, `padding`, `margin`, `width`, `height`,
`min_width`, `min_height`, `max_width`, `max_height`, `position`, `z_index`,
`overflow`, `direction`, `main_align`, `cross_align`, `align_self`,
`align_content`, `content`, `spacing`, `gap_x`, `gap_y`, `order`, `visibility`,
`user_select`, `resize`, `pointer_events`, `aspect`, `columns`, `table_layout`,
`border_collapse`, `list_style_type`, `list_style_position`.

### Palette colours

Every family × shade, exposed as `bg_*` and `text_*`:

- Neutrals — `slate`, `gray`, `zinc`, `neutral`, `stone`, `mauve`, `olive`,
  `mist`, `taupe`
- Accents — `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`,
  `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`,
  `pink`, `rose`

Shades: `50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950`.

### Semantic tokens

`background`, `foreground`, `primary`, `primary_foreground`, `secondary`,
`secondary_foreground`, `muted`, `muted_foreground`, `accent`,
`accent_foreground`, `destructive`, `destructive_foreground`, `card`,
`card_foreground`, `popover`, `popover_foreground`, `border`, `input`, `ring`,
`chart_1` … `chart_5`, plus the `sidebar_*` family and `disabled` /
`disabled_foreground`.

### Literal colours

`text_white`, `text_black`, `bg_white`, `bg_black`, `bg_transparent`,
`text_transparent`.

### Typography

Sizes (`text_xs` … `text_9xl`), weights (`font_thin`, `font_extralight`,
`font_light`, `font_normal`, `font_medium`, `font_semibold`, `font_bold`,
`font_extrabold`, `font_black_weight`), slant (`italic`, `not_italic`),
families (`font_sans`, `font_serif`, `font_mono`), alignment (`text_left`,
`text_center`, `text_right`, `text_justify`), decoration (`underline`,
`overline`, `line_through`, `no_underline`, `decoration_color`,
`decoration_thickness`, `underline_offset`), transform (`uppercase`,
`lowercase`, `capitalize`, `normal_case`), extras (`whitespace_*`,
`break_words`, `break_all`, `break_keep`, `text_ellipsis_set`, `text_clip`,
`line_clamp`, `list_none`, `list_disc`, `list_decimal`, `list_inside`,
`list_outside`, `accent_color`, `caret_color`, `selection_color`,
`placeholder_color`, `line_height`, `letter_spacing`, `text_indent`).

### Borders

Width (`border`, `border_0`, `border_2`, `border_4`, `border_8`, and per-side
`border_t` / `border_r` / `border_b` / `border_l` / `border_x` / `border_y`),
colour (`border_primary`, `border_destructive`, and palette tokens), style (`border_solid`, `border_dashed`,
`border_dotted`, `border_double`,
`border_none`), and per-side divide utilities (`divide_x`, `divide_y`,
`divide_color`).

### Corner radius

`rounded_none`, `rounded_sm`, `rounded`, `rounded_md`, `rounded_lg`,
`rounded_xl`, `rounded_2xl`, `rounded_3xl`, `rounded_full`.

### Effects

Opacity (`opacity_0`, `opacity_25`, `opacity_50`, `opacity_75`, `opacity_100`),
shadows (`shadow_sm`, `shadow_md`, `shadow_lg`, `shadow_xl`, `shadow_2xl`,
`shadow_none`), ring (`ring`, `ring_0` … `ring_8`, `ring_primary`,
`ring_offset`, `inset_ring`), outline (`outline_width`, `outline_color`,
`outline_offset`, `outline_none`), transforms (`rotate`, `scale`,
`transform_origin`, `blur`).

### Flex

Direction (`flex_row`, `flex_col`), wrapping (`flex_wrap_content`,
`flex_wrap_spacing`), content mode (`content_flex`, `content_fit`,
`content_normal`), flex sizing (`flex_1`, `grow`, `grow_0`, `shrink`,
`shrink_0`, `shrink_1`), justification (`justify_start`, `justify_center`,
`justify_end`, `justify_between`, `justify_around`, `justify_evenly`),
cross-axis (`items_start`, `items_center`, `items_end`), self alignment (`self_start`, `self_center`, `self_end`),
content alignment (`content_start`,
`content_center`, `content_end`, `content_between`, `content_around`,
`content_evenly`), gaps (`gap`, `gap_x`, `gap_y`, `space_x`, `space_y`), and
`order`.

### Sizing

Raw (`w`, `h`, `min_w`, `min_h`, `max_w`, `max_h`, `size`), percentages (`w_percent`, `h_percent`), keywords (`w_full`,
`w_auto`, `h_full`,
`h_auto`, `size_full`), Tailwind scale (`w_0` … `w_96`, `h_0` … `h_96`, plus
`min_w_*`, `min_h_*`, `max_w_*`, `max_h_*`), aspect ratio (`aspect`,
`aspect_square`, `aspect_video`, `aspect_auto`).

### Spacing

`p_*`, `px_*`, `py_*`, `pt_*`, `pr_*`, `pb_*`, `pl_*` and `m_*`, `mx_*`,
`my_*`, `mt_*`, `mr_*`, `mb_*`, `ml_*` on the Tailwind scale (1 unit = 4 px),
including fractional steps (`p_0_5`, `p_1_5`, `p_2_5`, `p_3_5`).

### Positioning

`relative`, `absolute`, `global`, `top`, `right`, `bottom`, `left`, `inset`,
`inset_x`, `inset_y`, `z_index`.

### Overflow

`overflow_none`, `overflow_clip`, `overflow_x_clip`, `overflow_y_clip`,
`overflow_x_none`, `overflow_y_none`.

### Layout extras

`hidden`, `visible`, `invisible`, `sr_only`, `not_sr_only`,
`pointer_events_none`, `pointer_events_auto`, `select_none`, `select_text`,
`select_all`, `select_auto`, `resize`, `resize_none`, `resize_x`, `resize_y`,
`table_auto`, `table_fixed`, `border_collapse`, `border_separate`, `columns`,
`container`.

### Merging

- `Style::merge(self, other) -> Style` — `other` wins on every field it set.
  Stack fields concatenate. `Gaps` fields merge per side.
- `Style::or(self, defaults) -> Style` — `self` wins; `defaults` fills gaps.
- `Style::merge_from(&mut self, other)` / `or_from` — in-place variants.
- `Style::is_empty(&self) -> bool` — `true` iff the style equals the default.
- `Style::with_active_states(self, flags) -> Style` — fold state-conditional
  styles matching `flags`.
- `StyleExt::merge_style(other)` / `or_style(defaults)` — lift the above onto
  any component.

---

## Style-string API

For developers who prefer Tailwind's own syntax, every `StyleExt` implementor
accepts a style string:

```rust,no_run
# use freyacn::prelude::*;
let _ = Style::default().style("p-4 bg-primary text-white shadow-md font-bold");
```

The parser understands the full Tailwind vocabulary that Freya can render —
palette and semantic colours, opacity modifiers (`/80`), spacing, sizing (including `w-screen`), typography, borders,
rings (`ring`, `ring-offset-*`,
`inset-ring-*`), arbitrary bracket values (`text-[12.8px]`, `bg-[#ff0000]`),
transforms (`rotate-*`, `scale-*`, `blur-*`), effects, flex, positioning,
overflow, aspect ratio, order, space, divide, outline, list, accent/caret,
and state-conditional modifiers (`hover:`, `focus:`, `focus-visible:`,
`active:`, `disabled:`, `dark:`). Unknown tokens are silently ignored (matching Tailwind's behaviour).

### `style_debug`

When you want to know which tokens had no effect — for typo hunting or test
assertions — use `style_debug`:

```rust,no_run
# use freyacn::prelude::*;
let (style, unknown) = Style::default()
    .style_debug("p-4 bg-primary shdaow-md not-a-thing");

assert_eq!(unknown, vec!["shdaow-md", "not-a-thing"]);
```

### State-aware styling and transforms

State variants (`hover:`, `focus:`, `focus-visible:`, `active:`, `disabled:`,
`dark:`) and transforms (`rotate-45`, `scale-95`, `blur-sm`) parse into
conditional style layers. The `Div` component observes runtime pointer,
focus, disabled, and dark-mode state and folds the active styles
automatically.

### What is not representable in Freya today

CSS Grid layout, CSS keyframe transitions/animations, backdrop
blur/filters, and responsive viewport-width prefixes (`sm:`, `md:`, `lg:`).
For viewport responsiveness and animations, Freya hooks (`use_viewport()`,
`use_animation()`, etc.) can be used directly in Rust code.

---

### `cn!` macro

Utility macro for Tailwind-style class concatenation, skipping empty values
and `None` options:

```rust
# use freyacn::prelude::*;
let is_active = true;
let classes = cn!(
    "px-4 py-2 rounded",
    "bg-primary text-primary-foreground",
    if is_active { "ring-2 ring-ring" } else { "" },
    Some("shadow-md"),
    None::<&str>,
);
assert_eq!(
    classes,
    "px-4 py-2 rounded bg-primary text-primary-foreground ring-2 ring-ring shadow-md",
);
```

---

## `#[extensions(...)]`

Extensions opt a struct into freyacn's built-in bundles. Each name maps to a
field, an initialiser, a render-body local, and a trait impl.

### Supported names

| Name             | Trait                                | Field(s) added                                           | Trait method             |
|------------------|--------------------------------------|----------------------------------------------------------|--------------------------|
| `children`       | `freyacn::prelude::ChildrenExt`      | `children: Vec<Element>`                                 | `get_children`           |
| `key`            | `freyacn::prelude::KeyExt`           | `key: DiffKey`                                           | `write_key`              |
| `style`          | `freyacn::prelude::StyleExt`         | `style: Style`                                           | `get_style`              |
| `event_handlers` | `freyacn::prelude::EventHandlersExt` | `event_handlers: FxHashMap<EventName, EventHandlerType>` | `get_event_handlers`     |
| `accessibility`  | `freyacn::prelude::AccessibilityExt` | `accessibility: AccessibilityData`                       | `get_accessibility_data` |

### Standalone usage

The attribute works on hand-written structs too. The struct must derive or
implement `Default` so the added fields can be initialised.

```rust,no_run
use freyacn::prelude::*;

#[extensions(children, key, style, accessibility)]
#[derive(Clone, PartialEq, Default)]
pub struct PlainBox;
```

---

## Components

`freyacn` ships core primitives built with the macro engine and Tailwind v4
styling.

### `Div`

The foundational, state-aware layout primitive. It wraps Freya's `rect()`,
forwards every `Style` field Freya can render, and owns all runtime state
observation — so consumers never need to compute `StateFlags` themselves.

- **State observation**: `DARK`, `HOVER`, `ACTIVE`, `FOCUS`,
  `FOCUS_VISIBLE`, `DISABLED`. `Div` sets up pointer handlers, reads
  `use_focus`, and folds the matching state-conditional styles at render
  time.
- **Full style surface**: forwards background, foreground, typography, font
  width, text height, text shadows, corner radius, border, padding, margin,
  all size constraints, position, overflow, flex layout, opacity, shadows,
  z-index via layer, text overflow, pointer events, interactive,
  rotate/scale/blur/transform-origin, child offsets, and accessibility.
- **Props**: `disabled: bool`, `focusable: bool`, `on_hover:
  EventHandler<Event<PointerEventData>>`.

```rust,no_run
use freyacn::prelude::*;

Div()
    .style("p-4 bg-card text-card-foreground rounded-lg border shadow-sm")
    .focusable(true)
    .on_hover(|_| println!("hovered"))
    .child(label().text("Card content"))
```

Because `Div` owns state folding, any component that returns a `Div` gets
`hover:` / `focus:` / `active:` / `disabled:` support for free. Just hand it
the merged style and let it observe.

### `Button`

The primary interactive component, composited on top of `Div`.

- **6 variants**: `Default`, `Outline`, `Secondary`, `Ghost`, `Destructive`,
  `Link`. Accepts string coercion: `.variant("outline")`.
- **8 sizes**: `Default` (h-8), `Xs` (h-6), `Sm` (h-7), `Lg` (h-9), `Icon`
  (size-8), `IconXs`, `IconSm`, `IconLg`.
- **Props**: `disabled: bool`, `focusable: bool`, `on_hover:
  EventHandler<Event<PointerEventData>>`.
- **Extensions**: `style`, `children`, `key`, `event_handlers`.

Content is passed as children, matching the shadcn/ui API:

```rust,no_run
use freyacn::prelude::*;

Button()
    .child(label().text("Delete item"))
    .variant(ButtonVariant::Destructive)   // or .variant("destructive")
    .size(ButtonSize::Lg)                  // or .size("lg")
    .on_press(|_| println!("Deleted!"));
```

`Button` is a pure compositor: it builds a `Style` from base classes plus the
variant and size resolutions, and hands it to `Div` via `.merge_style(...)`.
All state observation is `Div`'s job.

---

## Theming

The `Theme` struct holds every semantic colour (`background`, `foreground`,
`primary`, …) plus a `Colors` palette containing all Tailwind colours. It's
`Copy`, so passing it around is free.

### Initialising a theme

```rust,no_run
# use freyacn::prelude::*;
// Fluent light or dark theme with default palette:
let theme = Theme::light().slate();
use_init_cn_theme(theme);

// Or chainable fluent configuration:
let theme = Theme::dark().stone().theme_color("rose");
use_init_cn_theme(theme);

// Or with runtime palette names:
let theme = Theme::from_base("zinc", true).theme_color("blue");
use_init_cn_theme(theme);
```

`.base_color(...)` resets `primary` / `accent` / `ring` to the new base's
defaults, so `.theme_color(...)` must come after it. Per-palette shortcuts (`.stone()`, `.mauve()`, `.olive()`,
`.mist()`, `.taupe()`, `.slate()`,
`.gray()`, `.zinc()`, `.neutral()`) skip the string.

### Reading the theme inside a component

```rust,no_run
# use freyacn::prelude::*;
let theme = use_cn_theme();
let bg      = theme.read().background;
let primary = theme.read().primary;
```

All palette and semantic helpers read the theme at call time, so switching
themes updates every component on the next render without reconstruction.

---

## The CLI — `freyacn-cli`

`freyacn-cli` is the workspace tool for developing, building, and shipping
FreyaCN apps. Its command surface mirrors the Tauri CLI, so familiar
workflows carry over directly:

```bash
# Run the app in dev mode with hot-reload
freyacn dev

# Produce a release build
freyacn build

# Package the app for the current platform (or all targets)
freyacn bundle

# Generate platform icons from a source PNG
freyacn icon path/to/icon.png

# Print environment and version info for bug reports
freyacn info
```

Additional subcommands will land as the workspace tool matures. The CLI is
part of the same workspace as the runtime, so versions are kept in lockstep.

---

## Customisation

Components are builders. Extend them or build your own with the same
primitives.

```rust
use freyacn::prelude::*;

#[component(
    props(label: String),
    ext(style, children),
)]
fn PrimaryButton(label: String) {
    Div()
        .merge_style(style.clone())
        .children(children.clone())
        .child(label().text(label.as_str()))
}
```

Use `#[struct_fields(...)]` to add internal storage for your own trait impls:

```rust
#[component(props(label: String))]
#[struct_fields(tooltip: MyTooltip)]
fn TooltippedLabel(label: String) { /* … */ }

impl TooltipExt for TooltippedLabelComponent {
    fn set_tooltip(&mut self, text: impl Into<String>) {
        let tip = self.tooltip.get_or_insert_with(MyTooltip::default);
        tip.text = text.into();
    }
}
```

---

## Example showcase

The repository includes runnable examples under `freyacn/examples/`
demonstrating every variant, size, icon placement, and interactive state.

```bash
cargo run --example button_showcase -p freyacn
```

---

## Roadmap

- [ ] More components: `Card`, `Input`, `Select`, `Checkbox`, `Switch`,
  `Dialog`, `Tabs`, `Tooltip`, `Popover`, `Dropdown`.
- [ ] `freyacn-cli` — a `dev` / `build` / `bundle` / `icon` flow matching the
  Tauri CLI's ergonomics.
- [ ] Publish `freyacn`, `freyacn-macros`, and `freyacn-cli` to crates.io.
- [ ] Theme overrides at the component level.
- [ ] Additional icon sets.
- [ ] Storybook-style interactive docs.
- [ ] `resolve_with(&Theme)` for theme-explicit variant resolution outside a
  render context.
- [ ] First-class support for the style-surface fields Freya doesn't yet
  render (rings, outlines, per-axis overflow) as soon as the underlying
  primitives land.
- [ ] Migration guide for the 0.1 → 0.2 breaking changes.

---

## Migration from 0.1

The 0.2 release is a substantial rewrite. The most visible changes:

| Before (0.1)                                                                                              | After (0.2)                                |
|-----------------------------------------------------------------------------------------------------------|--------------------------------------------|
| `BackgroundExt`, `ForegroundExt`, `BorderExt`, `SpacingExt`, `SizingExt`, `EffectsExt`, `CornerRadiusExt` | One `StyleExt` trait                       |
| `fg_*` prefix                                                                                             | `text_*` prefix                            |
| Manual `impl MyComponent { fn background(self, …) … }`                                                    | `#[component]` macro                       |
| Hand-rolled variant matching in `render`                                                                  | `#[variant]` enums + `variants(...)` group |
| `#[extensions(...)]` on `#[component]`                                                                    | `ext(...)` group inside `#[component]`     |
| Raw `rect()` in render bodies                                                                             | Styled `Div()` component                   |
| `.class("…")`                                                                                             | `.style("…")`                              |
| `Theme::base_color(name, dark)`                                                                           | `Theme::light()` / `Theme::dark()`         |

The `Style` struct is source-compatible with the old per-trait storage in
shape but adds many new fields. Custom impls that previously implemented
multiple extension traits should implement `StyleExt` once and return
`&mut Style`.

---

## Contributing

Contributions are welcome. Before opening a PR, please open an issue
describing what you want to change and why — the public API is still in flux,
and a short design discussion will save everyone time.

Areas where help is especially useful:

- New components and variants.
- style-string parser coverage — new Tailwind tokens Freya can render.
- Documentation and runnable examples.
- Testing infrastructure for proc-macro output.

---

## License

MIT.

---

## Acknowledgements

- [Freya](https://github.com/marc2332/freya) — for the GUI framework.
- [shadcn/ui](https://ui.shadcn.com) — for design inspiration.
- [Tailwind CSS](https://tailwindcss.com) — for the utility-first philosophy.