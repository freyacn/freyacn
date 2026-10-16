# freyacn

> ⚠️ **Work in progress.** `freyacn` is under active development and its public
> API is **not yet stable**. Breaking changes may land in any release, and the
> crate has not been published to crates.io yet.
>
> **Status:** pre-alpha. Expect rough edges, missing features, and occasional
> doc/code drift. If you want to try it, use a `git` dependency pinned to a
> specific commit. If you find a bug or want to discuss the design, please
> open an issue.

A modern, shadcn/ui-inspired component library for
[Freya](https://github.com/marc2332/freya), built around a single styling
trait, a theme system, and a layered variant model.

`freyacn` is the runtime crate: it provides `Style`, `StyleExt`, `Variant`,
the theme, the styled `Div` component, the built-in extension bundles, and
re-exports the `#[component]`, `#[variant]`, `#[extensions]`, and
`#[struct_fields]` macros. It also re-exports Freya's own prelude, so you
don't need a direct `freya` dependency.

---

## Installation

```toml
[dependencies]
freyacn = "0.2.0-alpha.1"
```

> **Not published to crates.io yet.** Use a git dependency pinned to a
> specific commit for reproducible builds.

Then in your code:

```rust
use freyacn::prelude::*;
```

That single import brings in the macros, `Style` / `StyleExt`, the `Div`
component, `Theme`, `StateFlags`, and Freya's own prelude — `label()`,
`EventHandler`, `Event`, `PointerEventData`, `LaunchConfig`, `WindowConfig`,
and `launch` are all in scope.

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

#[component(
    props(tooltip: String),
    ext(children, style),
    variants(variant: ButtonVariant = ButtonVariant::Default),
)]
#[derive(Clone, PartialEq)]
fn CustomButton(text: String) {
    Div()
        .merge_style(style.clone())
        .children(children.clone())
        .child(label().text(text.clone()))
}

fn app() -> impl IntoElement {
    let theme = Theme::light().slate();
    use_init_cn_theme(theme);

    CustomButton()
        .text("Save")
        .variant(ButtonVariant::Destructive)
        .bg_card()
        .p_6()
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
```

---

## The two halves of the API

Everything in `freyacn` hangs off two abstractions:

| Abstraction    | Purpose                                                                                                                                                                                                                                |
|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **`Style`**    | A plain data struct holding every styling field: background, foreground, typography, borders, effects, flex, sizing, spacing, positioning, overflow. All fields are `Option<T>` or `Vec<T>`, so `Style::default()` is the empty style. |
| **`StyleExt`** | A trait with one required method, `fn get_style(&mut self) -> &mut Style`. Every Tailwind-style helper is a provided method that mutates the returned `Style`.                                                                         |

`Style` implements its own trait, so bare `Style` values chain the same way components do:

```rust,no_run
use freyacn::prelude::*;

let s = Style::default()
    .bg_card()
    .text_card_foreground()
    .p_6()
    .corner_radius(12.0)
    .shadow_md()
    .style("font-semibold");
```

Any component participates by implementing one method:

```rust
use freyacn::prelude::*;

struct MyWidget {
    style: Style,
}

impl StyleExt for MyWidget {
    fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}
```

Once that impl exists, every `StyleExt` helper becomes available on
`MyWidget`. There's also a `map_style` provided method that handles the
borrow dance for you:

```rust
fn get_style(&mut self) -> &mut Style { &mut self.style }
// Every provided helper is defined as `self.map_style(|s| s.some_setter(...))`.
```

---

## The styling vocabulary

`StyleExt` exposes the full Tailwind subset Freya can render. Every helper
funneis through a base setter — `background(Color)`, `color(Color)`,
`corner_radius(…)`, `padding(…)`, `width(…)`, and so on — so you can drop to
the raw form whenever you have a concrete value.

### Colours

- **Palette** — every family × shade, as `bg_*` and `text_*`:
  `slate`, `gray`, `zinc`, `neutral`, `stone`, `mauve`, `olive`, `mist`,
  `taupe`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`,
  `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`,
  `pink`, `rose` — shades `50, 100, 200, …, 900, 950`.
- **Semantic tokens** — `background`, `foreground`, `primary`,
  `primary_foreground`, `secondary`, `secondary_foreground`, `muted`,
  `muted_foreground`, `accent`, `accent_foreground`, `destructive`,
  `destructive_foreground`, `card`, `card_foreground`, `popover`,
  `popover_foreground`, `border`, `input`, `ring`, `chart_1` … `chart_5`,
  the `sidebar_*` family, and `disabled` / `disabled_foreground`.
- **Literal** — `text_white`, `text_black`, `bg_white`, `bg_black`,
  `bg_transparent`, `text_transparent`.

### Typography

- Sizes: `text_xs` … `text_9xl`
- Weights: `font_thin`, `font_extralight`, `font_light`, `font_normal`,
  `font_medium`, `font_semibold`, `font_bold`, `font_extrabold`,
  `font_black_weight`
- Slant: `italic`, `not_italic`
- Families: `font_sans`, `font_serif`, `font_mono`
- Alignment: `text_left`, `text_center`, `text_right`, `text_justify`
- Decoration: `underline`, `overline`, `line_through`, `no_underline`, plus
  `decoration_color`, `decoration_thickness`, `underline_offset`
- Transform: `uppercase`, `lowercase`, `capitalize`, `normal_case`
- Extras: `whitespace_*`, `break_words`, `break_all`, `break_keep`,
  `text_ellipsis_set`, `text_clip`, `line_clamp`, list style helpers,
  `accent_color`, `caret_color`, `selection_color`, `placeholder_color`,
  `line_height`, `letter_spacing`, `text_indent`

### Borders

- Width: `border`, `border_0`, `border_2`, `border_4`, `border_8`, and the
  per-side `border_t` / `border_r` / `border_b` / `border_l` / `border_x` /
  `border_y`
- Colour: `border_primary`, `border_destructive`, every palette token
- Style: `border_solid`, `border_dashed`, `border_dotted`, `border_double`,
  `border_none`
- Divide: `divide_x`, `divide_y`, `divide_color`
- Corner radius: `rounded_none`, `rounded_sm`, `rounded`, `rounded_md`,
  `rounded_lg`, `rounded_xl`, `rounded_2xl`, `rounded_3xl`, `rounded_full`

### Effects

- Opacity: `opacity_0`, `opacity_25`, `opacity_50`, `opacity_75`, `opacity_100`
- Shadows: `shadow_sm`, `shadow_md`, `shadow_lg`, `shadow_xl`, `shadow_2xl`,
  `shadow_none`
- Ring (Tailwind v4): `ring`, `ring_0` … `ring_8`, `ring_primary`,
  `ring_offset`, `inset_ring`, plus palette and semantic colour variants
- Outline: `outline_width`, `outline_color`, `outline_offset`, `outline_none`
- Transforms: `rotate(deg)`, `scale(s)`, `transform_origin(o)`, `blur(radius)`

### Flex

- Direction: `flex_row`, `flex_col`
- Wrap: `flex_wrap_content`, `flex_wrap_spacing`
- Content mode: `content_flex`, `content_fit`, `content_normal`
- Sizing: `flex_1`, `grow`, `grow_0`, `shrink`, `shrink_0`, `shrink_1`
- Justify: `justify_start`, `justify_center`, `justify_end`,
  `justify_between`, `justify_around`, `justify_evenly`
- Cross axis: `items_start`, `items_center`, `items_end`
- Self: `self_start`, `self_center`, `self_end`
- Content align: `content_start`, `content_center`, `content_end`,
  `content_between`, `content_around`, `content_evenly`
- Gaps: `gap`, `gap_x`, `gap_y`, `space_x`, `space_y`
- Order: `order`, `order_first`, `order_last`, `order_none`

### Sizing

- Raw: `w`, `h`, `min_w`, `min_h`, `max_w`, `max_h`, `size`
- Percent: `w_percent`, `h_percent`
- Keywords: `w_full`, `w_auto`, `h_full`, `h_auto`, `size_full`
- Tailwind scale: `w_0` … `w_96`, `h_0` … `h_96`, and the min/max variants
- Aspect ratio: `aspect`, `aspect_square`, `aspect_video`, `aspect_auto`

### Spacing

`p_*`, `px_*`, `py_*`, `pt_*`, `pr_*`, `pb_*`, `pl_*` and `m_*`, `mx_*`,
`my_*`, `mt_*`, `mr_*`, `mb_*`, `ml_*` on the Tailwind scale (1 unit = 4 px),
including fractional steps (`p_0_5`, `p_1_5`, `p_2_5`, `p_3_5`).

### Positioning and overflow

`relative`, `absolute`, `global`, `top`, `right`, `bottom`, `left`, `inset`,
`inset_x`, `inset_y`, `z_index`, `overflow_none`, `overflow_clip`, and the
per-axis overflow helpers.

### Layout extras

`hidden`, `visible`, `invisible`, `sr_only`, `not_sr_only`,
`pointer_events_none`, `pointer_events_auto`, `select_none`, `select_text`,
`select_all`, `select_auto`, `resize`, `resize_none`, `resize_x`, `resize_y`,
`table_auto`, `table_fixed`, `border_collapse`, `border_separate`, `columns`,
`container`.

---

## Style strings

Every `StyleExt` implementor accepts a Tailwind-style style string:

```rust,no_run
use freyacn::prelude::*;

let _ = Style::default().style("p-4 bg-primary text-white shadow-md font-bold");
```

Unknown tokens are silently ignored, matching Tailwind's behaviour. The
parser understands the full vocabulary above — palette and semantic colours,
opacity modifiers (`/80`), spacing, sizing (including viewport `w-screen`),
typography, borders, rings (`ring`, `ring-offset-*`, `inset-ring-*`),
arbitrary bracket values (`text-[12.8px]`, `bg-[#ff0000]`), transforms (`rotate-*`, `scale-*`, `blur-*`), effects, flex,
positioning, overflow,
aspect ratio, order, space, divide, outline, list, accent/caret, and
state-conditional modifiers (`hover:`, `focus:`, `focus-visible:`, `active:`,
`disabled:`, `dark:`).

For typo hunting, `style_debug` reports unrecognised tokens:

```rust,no_run
use freyacn::prelude::*;

let (style, unknown) = Style::default()
    .style_debug("p-4 bg-primary shdaow-md not-a-thing");

assert_eq!(unknown, vec!["shdaow-md", "not-a-thing"]);
```

### State-aware styling and transforms

State variants (`hover:`, `focus:`, `focus-visible:`, `active:`, `disabled:`,
`dark:`) and transforms (`rotate-45`, `scale-95`, `blur-sm`) are parsed into
conditional style layers. The `Div` component observes runtime pointer,
focus, disabled, and dark-mode state and folds the active styles
automatically.

### What is not representable in Freya today

CSS Grid layout, CSS keyframe transitions/animations, backdrop
blur/filters, and responsive viewport-width prefixes (`sm:`, `md:`, `lg:`).
For viewport responsiveness and animations, Freya hooks (`use_viewport()`,
`use_animation()`, etc.) can be used directly in Rust code.

---

## Merging

`Style` has explicit merging primitives used by the component macro to
combine variant styles, size styles, and user overrides.

```rust,no_run
use freyacn::prelude::*;

let base    = Style::default().p_4().bg_primary();
let variant = Style::default().p_6().bg_destructive();
let merged  = base.merge(variant);
// merged.padding    == Some(6·unit)  — variant won
// merged.background == destructive   — variant won
```

| Method                                                | Effect                                                        |
|-------------------------------------------------------|---------------------------------------------------------------|
| `Style::merge(self, other) -> Style`                  | `other` wins on every field it set. Stack fields concatenate. |
| `Style::or(self, defaults) -> Style`                  | `self` wins; `defaults` fills gaps.                           |
| `Style::merge_from(&mut self, other)` / `or_from`     | In-place variants.                                            |
| `Style::is_empty(&self) -> bool`                      | `true` iff the style equals the default.                      |
| `Style::with_active_states(self, flags) -> Style`     | Fold state-conditional styles matching `flags`.               |
| `StyleExt::merge_style(other)` / `or_style(defaults)` | Lift the above onto any component.                            |

### Padding and margin merge per side

`padding` and `margin` are `Gaps` values, which merge field-by-field rather
than wholesale. That means a variant that sets only `px-4` overrides the
horizontal sides and leaves the base class's `py-2` intact. The same logic
applies across `.merge(...)` and `.merge_style(...)`.

---

## Variants

`#[variant]` turns an enum into a style-resolving type.

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

The macro generates:

- `impl MyEnum { fn style_str(&self) -> &'static str; }`
- `impl MyEnum { fn resolve(&self) -> freyacn::Style; }`
- `impl freyacn::Variant for MyEnum`
- `impl From<&str>` and `impl From<String>`, matching kebab-cased variant
  names with a fallback to the first declared variant.

Kebab conversion handles simple (`Md` → `md`), acronym (`XS` → `xs`), and
compound (`SecondaryDestructive` → `secondary-destructive`) names.

The `Variant` trait itself is one method:

```rust
use freyacn::prelude::Style;

pub trait Variant {
    fn resolve(&self) -> Style;
}
```

`.resolve()` reads the active theme, so it must be called inside a Freya
render context. When a variant is used as a prop inside a component that
declares `variants(...)` and `ext(style)`, the component macro calls
`.resolve()` for you and merges the result into the render body's `style`
local.

---

## Extensions

Extensions opt a struct into freyacn's built-in bundles. Each name maps to a
field, an initialiser, a render-body local, and a trait impl.

| Name             | Trait                                | Field(s) added                                           | Trait method             |
|------------------|--------------------------------------|----------------------------------------------------------|--------------------------|
| `children`       | `freyacn::prelude::ChildrenExt`      | `children: Vec<Element>`                                 | `get_children`           |
| `key`            | `freyacn::prelude::KeyExt`           | `key: DiffKey`                                           | `write_key`              |
| `style`          | `freyacn::prelude::StyleExt`         | `style: Style`                                           | `get_style`              |
| `event_handlers` | `freyacn::prelude::EventHandlersExt` | `event_handlers: FxHashMap<EventName, EventHandlerType>` | `get_event_handlers`     |
| `accessibility`  | `freyacn::prelude::AccessibilityExt` | `accessibility: AccessibilityData`                       | `get_accessibility_data` |

Used inside `#[component]` via the `ext(...)` group, or standalone:

```rust
use freyacn::prelude::*;

#[extensions(children, key, style)]
#[derive(Clone, PartialEq, Default)]
pub struct PlainBox {
    // Custom user fields
}
```

Declaring `variants(...)` inside `#[component]` requires `ext(style)` so that
variant styles can be folded into the effective style.

---

## The macros

`freyacn` re-exports four proc-macros from `freyacn-macros`:

- **`#[component(...)]`** — turns a function into a component struct with
  props, extensions, and variants.
- **`#[variant]`** — turns an enum into a style-resolving variant type.
- **`#[struct_fields(...)]`** — adds internal storage fields (no setters,
  separate docs).
- **`#[extensions(...)]`** — standalone form of `ext(...)`.

The full grammar, field sources, collision rules, generated code, and every
compile error are documented in the [`freyacn-macros`](../freyacn-macros)
README.

### The `#[component]` groups at a glance

```rust,no_run
# use freyacn::prelude::*;
# #[variant]
# #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
# pub enum ButtonVariant { #[default] #[variant("")] Default }
# #[derive(Clone, Default, PartialEq)]
# pub struct MyTooltip { pub text: String }
#[component(
    props(    subtitle: String, badge: String = "new".to_string()),
    ext(children, style),
    variants( variant: ButtonVariant = ButtonVariant::Default),
)]
#[struct_fields( tooltip: MyTooltip, hover_count: usize = 0)]
#[derive(Clone, PartialEq)]
fn Card(name: String) {
    // fn params  → `name: &String`   (required, panics if unset)
    // props      → `subtitle: &Option<String>`, `badge: &String`
    // struct     → `tooltip: &Option<MyTooltip>`, `hover_count: &usize`
    // variants   → `variant: &ButtonVariant`
    // extensions → `children: &Vec<Element>`, `style: &Style`

    Div()
        .merge_style(style.clone())
        .children(children.clone())
        .child(label().text(name.clone()))
}
```

The generated `CardComponent` has setters for props (`subtitle`, `badge`,
`variant`), a `new()`, a `Default` impl, a free function `Card()`, and a
companion macro `Card!(...)`.

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

```rust,no_run
# use freyacn::prelude::*;
# #[variant]
# #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
# pub enum ButtonVariant { #[default] #[variant("")] Default }
# #[component(
#     props(disabled: bool = false),
#     ext(style, children, key, event_handlers, accessibility),
#     variants(variant: ButtonVariant = ButtonVariant::Default),
# )]
# fn MyButton() {
Div()
    .merge_style(style.clone())
    .disabled(*disabled)
    .focusable(true)
    .accessibility(accessibility.clone())
    .with_event_handlers(event_handlers.clone())
    .children(children.clone())
# }
```

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

### `cn!` macro

Utility macro for Tailwind-style class concatenation. Skips empty strings and
`None` options:

```rust
use freyacn::prelude::*;

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

## Style layering at render time

When a component declares `variants(...)` alongside `ext(style)`, the
generated `render` body computes the effective style before user code runs:

```text
Style::default()
    .merge(variant.resolve())      // declared order
    .merge(size.resolve())
    .merge(self.style.clone())     // user overrides win
```

The render body then binds `style: &Style` to the merged value.

**Consequences:**

- **Overrides persist across variant changes.** `.variant("outline")`
  recomputes the base layer but leaves user intent intact. If a caller wrote
  `.bg_green_50()`, that override still wins.
- **Theme changes propagate automatically.** `.resolve()` reads
  `use_cn_theme()` on every render, not at construction. A dark-mode toggle
  re-renders the whole tree with the new theme.
- **State styles fold last.** Any pending `hover:` / `focus:` entries in the
  merged style are folded by `Div` at render time using runtime state.

---

## Theming

`Theme` holds every semantic colour plus a palette containing all Tailwind
colours. It's `Copy`, so passing it around is free.

```rust,no_run
use freyacn::prelude::*;

// Fluent entry points:
let theme = Theme::light().slate();           // light, slate neutral base
let theme = Theme::dark().stone().theme_color("rose");
let theme = Theme::dark().zinc();             // base default accent
let theme = Theme::from_base("zinc", true).theme_color("blue");

use_init_cn_theme(theme);
```

`.base_color(...)` resets `primary` / `accent` / `ring` to the new base's
defaults, so `.theme_color(...)` must come after it. Per-palette shortcuts (`.stone()`, `.mauve()`, `.olive()`,
`.mist()`, `.taupe()`, `.slate()`,
`.gray()`, `.zinc()`, `.neutral()`) skip the string.

Reading the theme from a component:

```rust,no_run
use freyacn::prelude::*;

fn MyComponent() -> impl IntoElement {
    let theme = use_cn_theme().read();

    // Theme carries is_dark plus every semantic token.
    let bg      = theme.background;
    let fg      = theme.foreground;
    let primary = theme.primary;

    Div().background(bg).color(fg).style("p-4")
}
```

`use_init_cn_theme` reuses an existing context if one exists. For a scoped
override that always creates a fresh context, use `use_provide_cn_theme`.
`get_cn_theme_or_default()` returns a `Readable<Theme>` and never panics —
useful for built-in components that should work without an explicit provider.

Every palette and semantic helper reads the theme at call time, so switching
light/dark or changing the base colour updates every component on the next
render without reconstruction.

---

## Compile-time safety

The `#[component]` macro refuses to compile on:

- Duplicate field names across `props`, `struct_fields`, `variants`, and
  function parameters.
- A field whose name matches an extension field from `ext(...)`.
- A variant prop without a default.
- `variants(...)` without `ext(style)`.
- The removed `required` keyword or a missing type on an attribute prop.
- An unknown group (only `props`, `ext`, `variants` are valid).
- A misplaced `#[struct_fields(...)]` (it must come after `#[component]`).

Each error names the offending item and points at the fix.

---

## Workspace

`freyacn` is part of a Cargo workspace:

| Crate                      | Purpose                                                                                                                                                                                            |
|----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **`freyacn`** (this crate) | Runtime library: `Style`, `StyleExt`, `Variant`, theme, extensions, components. Re-exports all macros and Freya's prelude.                                                                         |
| `freyacn-macros`           | Proc-macros (`#[component]`, `#[variant]`, `#[extensions]`, `#[struct_fields]`). Internal — never depend on it directly.                                                                           |
| `freyacn-cli`              | Command-line tool for bundling and packaging `freyacn` apps. Its command surface mirrors the Tauri CLI — `dev`, `build`, `bundle`, `icon`, `info`, and friends — so familiar workflows carry over. |

---

## Roadmap

- More components: `Card`, `Input`, `Select`, `Checkbox`, `Switch`, `Dialog`,
  `Tabs`, `Tooltip`, `Popover`, `Dropdown`.
- `freyacn-cli` — an initial `dev` / `build` / `bundle` flow, mirroring the
  Tauri CLI's ergonomics.
- Publish to crates.io.
- Theme overrides at the component level.
- Additional icon sets.
- `resolve_with(&Theme)` for theme-explicit variant resolution outside a
  render context.
- First-class support for the style-surface fields Freya doesn't yet render (rings, outlines, per-axis overflow) as soon
  as the underlying primitives
  land.

---

## License

MIT.

---

## Acknowledgements

- [Freya](https://github.com/marc2332/freya) — GUI framework.
- [shadcn/ui](https://ui.shadcn.com) — design inspiration.
- [Tailwind CSS](https://tailwindcss.com) — utility-first philosophy.