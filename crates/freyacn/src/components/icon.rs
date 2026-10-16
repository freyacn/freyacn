//! # FreyaCN Icon Component
//!
//! A flexible SVG icon component that integrates with the FreyaCN theming
//! system and all extension traits.
//!
//! ## Overview
//!
//! `Icon` wraps Freya's `SvgViewer` in a `Div`, so every `StyleExt` helper
//! applies to the icon's container and state-conditional layers (`hover:`,
//! `focus:`, `active:`, `disabled:`, `dark:`) fold automatically.
//!
//! ## Colour inheritance
//!
//! `Icon` does **not** set a colour of its own unless you explicitly ask
//! for one. When nested inside a parent that sets a text colour — a
//! `Button`, a `Card`, any `Div` with `.text_*()` — the icon inherits
//! that colour through Freya's text-style cascade. An SVG that uses
//! `currentColor` will pick it up.
//!
//! ```ignore
//! Button()
//!     .variant("default")
//!     .child(Icon().svg(bytes).size_4())
//!     .child(label().text("Save"));
//! // Icon renders in primary-foreground, matching the label.
//! ```
//!
//! ## Sizing
//!
//! Icons default to `16×16` when no explicit size is set. If only one
//! axis is set, the other mirrors it so the icon stays square.

use crate::prelude::*;

/// Default icon edge length in pixels when no explicit size is set.
const DEFAULT_ICON_SIZE: f32 = 16.0;

#[component(
    props(
        svg: Bytes = Bytes::default(),
    ),
    ext(style, accessibility, key),
)]
#[derive(PartialEq, Clone)]
pub fn Icon() {
    let mut style = style.clone();

    // Icons are square by convention. If only one axis was set, mirror it
    // on the other; if neither was set, use the built-in default.
    let (width, height) = match (&style.width, &style.height) {
        (Some(w), Some(h)) => (w.clone(), h.clone()),
        (Some(w), None) => (w.clone(), w.clone()),
        (None, Some(h)) => (h.clone(), h.clone()),
        (None, None) => {
            let s = Size::px(DEFAULT_ICON_SIZE);
            (s.clone(), s)
        }
    };

    let svg_viewer = SvgViewer::new(svg.clone()).width(width).height(height);

    // `style.text_color` is left as-is. When it's `None`, `Div` skips
    // the `.color(...)` call entirely, which lets Freya's text-style
    // cascade hand the parent's colour down to the SVG.
    Div()
        .merge_style(style)
        .accessibility(accessibility.clone())
        .child(svg_viewer)
}
