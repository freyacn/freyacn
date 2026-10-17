//! # FreyaCN Label Component
//!
//! A text primitive that inherits text styling from its ancestors through
//! Freya's text-style cascade, and overrides only what you explicitly set.
//!
//! ## Inheritance
//!
//! `Label` never sets a text-style field on its own. If the surrounding
//! `Button`, `Card`, or `Div` set a color, size, weight, or font family,
//! the label renders with that value. Whatever you write on the `Label`
//! itself wins, and everything else cascades down.
//!
//! ```ignore
//! // Red, bold, 18px:
//! Div()
//!     .text_red_500()
//!     .font_bold()
//!     .text_lg()
//!     .child(Label().text("Warning"));
//!
//! // Red, bold, 18px, then overridden to blue by the label:
//! Div()
//!     .text_red_500()
//!     .font_bold()
//!     .text_lg()
//!     .child(Label().text("Info").text_blue_500());
//! ```
//!
//! ## Sizing
//!
//! A `Label` sizes to its content by default. Every size field set on
//! the style is forwarded — `width`, `height`, and the `min_*` /
//! `max_*` variants — so truncation inside a fixed-width column works
//! out of the box:
//!
//! ```ignore
//! Label()
//!     .text("A very long headline that should be cut off")
//!     .max_w_64()
//!     .truncate();
//! ```

use crate::extensions::TextEllipsis;
use crate::prelude::*;

#[component(
    props(
        text: String = String::new(),
    ),
    ext(style, accessibility, key),
)]
#[derive(PartialEq, Clone)]
pub fn Label() {
    let mut el = label().text(text.clone());

    // ── Inheritable text styles ────────────────────────────────────
    // Apply only when the caller set them, so the value the parent
    // provided survives otherwise.

    if let Some(colour) = style.text_color {
        el = el.color(colour);
    }
    if let Some(size) = style.font_size {
        el = el.font_size(size);
    }
    if let Some(slant) = style.font_slant {
        el = el.font_slant(slant);
    }
    if let Some(weight) = style.font_weight {
        el = el.font_weight(weight);
    }
    if let Some(align) = style.text_align {
        el = el.text_align(align);
    }
    if let Some(deco) = style.text_decoration {
        el = el.text_decoration(deco);
    }
    if let Some(width) = style.font_width {
        el = el.font_width(width);
    }
    if let Some(height) = style.text_height {
        el = el.text_height(height);
    }

    // `Vec` fields already behave — an empty list leaves the parent's
    // value alone, a non-empty one layers on top.
    for family in &style.font_families {
        el = el.font_family(family.clone());
    }
    for shadow in &style.text_shadows {
        el = el.text_shadow(shadow.clone());
    }

    // ── Non-inheritable ────────────────────────────────────────────
    // Truncation is per-element, not cascading.

    if let Some(ellipsis) = style.text_ellipsis {
        el = el.text_overflow(match ellipsis {
            TextEllipsis::Clip => TextOverflow::Clip,
            TextEllipsis::Ellipsis => TextOverflow::Ellipsis,
        });
    }

    // ── Accessibility ──────────────────────────────────────────────
    // `.accessibility(...)` replaces the whole struct, so any per-field
    // overrides must come after it.

    let mut el = el.accessibility(accessibility.clone());

    // ── Layout ─────────────────────────────────────────────────────
    // A `Label` sizes to its content by default; the parent usually
    // controls layout. Forward every size field the caller set so
    // truncation inside a fixed-width column works, but never impose
    // a default size — `Div` is the layout workhorse, `Label` is a
    // leaf.

    if let Some(size) = style.width.clone() {
        el = el.width(size);
    }
    if let Some(size) = style.height.clone() {
        el = el.height(size);
    }
    if let Some(size) = style.min_width.clone() {
        el = el.min_width(size);
    }
    if let Some(size) = style.min_height.clone() {
        el = el.min_height(size);
    }
    if let Some(size) = style.max_width.clone() {
        el = el.max_width(size);
    }
    if let Some(size) = style.max_height.clone() {
        el = el.max_height(size);
    }

    el
}
