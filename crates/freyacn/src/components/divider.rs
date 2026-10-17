//! # Divider component — a thin separating line.
//!
//! A horizontal or vertical rule. Horizontal by default, spanning the
//! full width of its parent. Vertical dividers fill their parent's
//! height, which means the parent needs a bounded height for the
//! divider to be visible.
//!
//! ## Orientation
//!
//! `DividerOrientation::Horizontal` takes `h-px w-full`, and
//! `DividerOrientation::Vertical` takes `w-px h-full`. Both are
//! `shrink-0`, so a divider inside a flex row or column won't be
//! squeezed by its neighbours.
//!
//! ## Labelled dividers
//!
//! Set `label(...)` on a horizontal divider to centre a caption on the
//! line:
//!
//! ```ignore
//! Divider().label("OR")
//! // ────────────── OR ──────────────
//! ```
//!
//! The component switches to a flexible layout — two `flex-1` line
//! segments flanking a `shrink-0` label — so the label is genuinely
//! centred regardless of the text's length. A user `style(...)`
//! override is ignored in this mode; for custom label colours or
//! spacing, compose manually with two `Divider`s and a `Label` between
//! them.
//!
//! ## Overrides
//!
//! Everything a `Divider` renders is a `Div`, so any `StyleExt` helper
//! applies. Custom thickness, colour, or spacing all fold in cleanly:
//!
//! ```ignore
//! Divider().bg_primary()                      // recolour
//! Divider().h_0_5()                           // thicker horizontal rule
//! Divider().style("bg-gradient-to-r ...")     // arbitrary style
//! ```

use crate::prelude::*;

// ==============================================================
// Orientation
// ==============================================================

/// Orientation of a [`Divider`].
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    /// Spans the full width of its parent.
    #[variant("h-0.5 w-full")]
    #[default]
    Horizontal,

    /// Spans the full height of its parent.
    #[variant("w-0.5 h-full")]
    Vertical,
}

// ==============================================================
// Component
// ==============================================================

/// A thin separating line.
///
/// # Examples
///
/// ```ignore
/// // Horizontal (default)
/// Div()
///     .style("flex flex-col gap-4")
///     .child(Label().text("Above"))
///     .child(Divider())
///     .child(Label().text("Below"));
///
/// // Vertical, inside a horizontal row
/// Div()
///     .style("flex flex-row items-center gap-4 h-6")
///     .child(Label().text("Left"))
///     .child(Divider().orientation(DividerOrientation::Vertical))
///     .child(Label().text("Right"));
///
/// // Horizontal, with a centred label
/// Divider().label("OR");
/// ```
#[component(
    props(
        label: String = String::new(),
    ),
    variants(
        orientation: DividerOrientation = DividerOrientation::Horizontal,
    ),
    ext(style, accessibility, key),
)]
#[derive(PartialEq, Clone)]
pub fn Divider() {
    if !label.is_empty() {
        // ──────── Label ────────
        //
        // Two flexible line segments flank a `shrink-0` label. Because
        // both segments are `flex-1`, they split the remaining width
        // equally, which centres the label regardless of its length.
        return Div()
            .style("flex flex-row items-center gap-3 w-full shrink-0")
            .child(Div().style("flex-1 h-0.5 bg-border"))
            .child(
                Label()
                    .text(label)
                    .style("text-xs text-muted-foreground shrink-0 items-center"),
            )
            .child(Div().style("flex-1 h-0.5 bg-border"))
            .accessibility(accessibility.clone())
            .into_element();
    }

    let style = style
        .clone()
        .style("shrink-0 bg-border")
        .merge(orientation.resolve());

    Div()
        .merge_style(style)
        .accessibility(accessibility.clone())
        .into_element()
}
