# FreyaCN

A modern, shadcn/ui‑inspired component library for [Freya](https://github.com/marc2332/freya) built with Rust and
Tailwind‑style extension traits.

---

## Overview

**FreyaCN** is a component library that brings the elegance and consistency of [shadcn/ui](https://ui.shadcn.com) to
Freya, a Rust‑based GUI framework. It provides a set of theme‑aware, highly composable components with a
Tailwind‑inspired API.

The library is built around three core ideas:

1. **Theme‑first**: All components automatically consume the current theme via `use_cn_theme()`.
2. **Tailwind‑inspired extensions**: A suite of extension traits lets you style any component using familiar utility
   names (`p_4()`, `bg_primary()`, `flex_col()`, etc.).
3. **shadcn/ui semantics**: Components follow the design language and variants of shadcn/ui, making them immediately
   recognisable and easy to use.

---

## Features

- **Full theming support** – Light/dark modes, custom palettes (slate, stone, neutral, etc.), and shadcn‑style
  `base_color`/`theme_color` configuration.
- **Tailwind extension traits** – Over 100 helper methods for backgrounds, foregrounds, spacing, sizing, borders,
  flexbox, typography, and effects.
- **Flexible component API** – Every component is a builder, allowing deep customisation through method chaining.
- **Icon integration** – Works seamlessly with `freya_icons` and inherits parent text colour via context.
- **Accessibility** – Built‑in support for focus, keyboard navigation, and ARIA attributes (via Freya’s accessibility
  traits).
- **Composable** – All components implement `ChildrenExt`, so you can nest them naturally.
- **Performant** – Leverages Freya’s diffing and layout engine; only re‑renders when necessary.

---

## Getting Started

Add FreyaCN to your `Cargo.toml`:

```toml
[dependencies]
freyacn = { git = "https://github.com/Caldek/freyacn" }
freya = "0.4.1"
```

In your `main.rs`:

```rust
use freyacn::components::{Button, Label};
use freyacn::extensions::*;
use freyacn::theme::{Theme, use_init_cn_theme};
use freya::prelude::*;

fn app() -> impl IntoElement {
    let theme = Theme::neutral(false, "slate"); // or Theme::stone(dark, theme)
    use_init_cn_theme(theme);

    Button()
        .child(Label("Click me"))
        .on_press(|| println!("Clicked!"))
        .bg_primary()
        .fg_primary_foreground()
        .p_4()
        .rounded(8)
}

fn main() {
    launch(app);
}
```

---

## Extensions

All extension traits are defined in the `freyacn::ext` module and are automatically implemented for any component that
provides the core methods (`background()`, `color()`, `padding()`, `width()`, etc.).

| Trait             | Methods                                                      | Description                                            |
|-------------------|--------------------------------------------------------------|--------------------------------------------------------|
| **BackgroundExt** | `bg_*`, `bg_primary`, `bg_destructive`, etc.                 | Set background colour from palette or theme.           |
| **ForegroundExt** | `fg_*`, `fg_primary`, `fg_white`, etc.                       | Set text/icon colour.                                  |
| **SpacingExt**    | `p_4()`, `m_2()`, `px_*`, `py_*`, etc.                       | Padding and margin with Tailwind scale (1 unit = 4px). |
| **SizingExt**     | `w_full()`, `h_auto()`, `w_4()`, etc.                        | Width, height, and min/max constraints.                |
| **BorderExt**     | `border_width()`, `border_color()`, `corner_radius()`        | Border styling.                                        |
| **FlexExt**       | `flex_col()`, `justify_center()`, `items_start()`, `gap_4()` | Flexbox layout control.                                |
| **TypographyExt** | `text_xl()`, `font_bold()`, `underline()`, etc.              | Font size, weight, alignment, decoration.              |
| **EffectsExt**    | `opacity_50()`, `shadow_md()`, etc.                          | Opacity and drop shadows.                              |

All methods are chainable and use the theme context automatically.

---

## Components

### Button

The primary interactive component – supports 6 variants and 8 sizes.

**Variants**:  
`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`

**Sizes**:  
`default` (h‑8), `xs` (h‑6), `sm` (h‑7), `lg` (h‑9), `icon` (size‑8), `icon-xs`, `icon-sm`, `icon-lg`

```rust
Button()
.destructive()
.size_lg()
.child(Label("Delete"))
.on_press( | | delete())
.rounded(8)
```

### Label

A simple text label with full typography and styling support.

```rust
Label("Hello World")
.text_2xl()
.font_bold()
.fg_primary()
.p_2()
```

### Icon

An SVG icon component that inherits foreground colour from its parent context.

```rust
Icon(heart())
.size_24()
.fg_red_500()
```

---

## Theming

Theming is central to FreyaCN. The `Theme` struct holds all semantic colours (`background`, `foreground`, `primary`,
etc.) and a `Colors` palette with all Tailwind colours.

### Initialising a theme

```rust
let theme = Theme::neutral(false, "slate"); // light mode, slate base
use_init_cn_theme(theme);
```

### Shadcn‑style theming

```rust
let theme = Theme::base_color("stone", true).theme_color("rose");
use_init_cn_theme(theme);
```

### Accessing the theme inside a component

```rust
let theme = use_cn_theme().read();
let bg = theme.background;
let primary = theme.primary;
```

---

## Customisation

All components are built as builders, so you can easily create your own variants or extend them.

```rust
// Custom primary button
fn primary_button(label: &str) -> impl IntoElement {
    Button()
        .child(Label(label))
        .bg_primary()
        .fg_primary_foreground()
        .rounded(8)
}
```

You can also combine extension traits to build complex layouts without writing CSS.

---

## Example Showcase

The repository includes a full showcase app in `examples/button_showcase.rs` that demonstrates every variant, size, icon
placement, and state in a vertically scrollable card.

---

## Roadmap

- [ ] Add more components: `Card`, `Input`, `Select`, `Checkbox`, `Switch`, `Dialog`
- [ ] Support for theme overrides at component level
- [ ] Additional icon sets
- [ ] More comprehensive documentation and storybook

---

## License

MIT OR Apache-2.0

---

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub.

---

## Acknowledgements

- Freya – for providing the wonderful Rust GUI framework
- shadcn/ui – for design inspiration
- Tailwind CSS – for the utility‑first philosophy

```

---

This version removes all emoji icons while preserving the content and formatting.