//! # Alert — shadcn/ui styled alert for Freya.
//!
//! ## Action slot
//!
//! Pass the action via `.action(...)`, not `.child(...)`:
//!
//! ```ignore
//! Alert()
//!     .child(AlertTitle().text("Heads up!"))
//!     .child(AlertDescription().text("You can add components."))
//!     .action(AlertAction().child(Button().child(Label().text("Enable"))))
//! ```
//!
//! `AlertAction` is **absolutely positioned** at `top-2 right-2` (8px,
//! 8px) inside a `Position::Stacked` wrapper. Absolute positioning
//! removes the button from the flex flow — no shrinking, no label
//! wrapping, natural intrinsic width.
//!
//! ## The containing-block wrapper
//!
//! `Div` wraps every child in a `WithPosition` rect. If the alert
//! itself were a `Div` and `AlertAction` a child, `AlertAction` would
//! position against that `WithPosition` wrapper — a content-fit box
//! around `AlertAction` itself, so `top-2 right-2` would resolve to
//! nowhere meaningful.
//!
//! The wrapper `rect()` here is a `Position::Stacked` box (relative at
//! 0,0) sized to match the alert's own explicit width / height / min /
//! max. `AlertAction` is a direct child of that `rect()` — no
//! `WithPosition` layer — so its absolute insets resolve against the
//! wrapper's box, which is exactly the alert's visible box.
//!
//! ## Reserved padding
//!
//! The action doesn't take layout space. Long descriptions that would
//! slide under the action need right padding — `.style("pr-24")` on
//! `AlertDescription`, or on `AlertTitle` when the title is long. This
//! matches shadcn: the reference implementation also expects callers
//! to reserve space when both share horizontal extent.
//!
//! ## Icon layout
//!
//! The icon uses a `flex_row` column layout (`shrink_0` + `offset_y-2`),
//! NOT absolute positioning. Icons are fixed 16×16, so `shrink_0` is a
//! no-op in practice and the row works correctly. Buttons need
//! absolute positioning because they're content-sized and would
//! otherwise be shrunk by flex.

use crate::prelude::*;
use torin::values::position::PositionSides;

// ==============================================================
// Variants
// ==============================================================

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[variant("bg-card text-card-foreground")]
    #[default]
    Default,

    #[variant("bg-card text-destructive")]
    Destructive,
}

// ==============================================================
// Context
// ==============================================================

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct AlertVariantContext(pub AlertVariant);

// ==============================================================
// Alert
// ==============================================================

#[component(
    variants(
        "relative w-full border border-border text-sm",
        variant: AlertVariant = AlertVariant::Default,
    ),
    ext(style, children, key, event_handlers, accessibility),
)]
#[struct_fields(
    icon: Element,
    action: Element,
)]
#[derive(Clone, PartialEq)]
pub fn Alert() {
    let theme_state = use_cn_theme();
    let (card, radius_lg) = {
        let t = theme_state.read();
        (t.card, t.radius_lg)
    };

    use_provide_context(|| AlertVariantContext(*variant));

    let has_icon = icon.is_some();

    // Content — the visible card with padding, radius, and surface.
    let mut content = if has_icon {
        Div()
            .flex_row()
            .items_start()
            .gap(12.0) // gap-x-3
            .child(
                Div()
                    .shrink_0()
                    .offset_y(2.0) // translate-y-0.5, aligns with title
                    .child(icon.as_ref().unwrap().clone()),
            )
            .child(
                Div()
                    .flex_col()
                    .gap(2.0) // gap-y-0.5
                    .width(Size::flex(1.0))
                    .children(children.clone()),
            )
    } else {
        Div().flex_col().gap(2.0).children(children.clone())
    };

    content = content
        .padding(Gaps::new(12.0, 16.0, 12.0, 16.0)) // px-4 py-3
        .corner_radius(radius_lg)
        .background(card)
        .merge_style(style.clone())
        .event_handlers(event_handlers.clone())
        .accessibility(accessibility.clone())
        .a11y_role(AccessibilityRole::Alert);

    // With an action, wrap in a `Position::Stacked` rect so the action
    // positions against the alert's visible box, not `WithPosition`.
    if let Some(action_el) = action.as_ref() {
        let mut wrapper = rect().position(Position::Stacked(Box::new(PositionSides {
            top: None,
            right: None,
            bottom: None,
            left: None,
        })));

        // Mirror sizing so the wrapper is exactly the alert's box and
        // `content`'s `w-full` has a parent width to fill.
        if let Some(w) = style.width.clone() {
            wrapper = wrapper.width(w);
        }
        if let Some(h) = style.height.clone() {
            wrapper = wrapper.height(h);
        }
        if let Some(w) = style.min_width.clone() {
            wrapper = wrapper.min_width(w);
        }
        if let Some(h) = style.min_height.clone() {
            wrapper = wrapper.min_height(h);
        }
        if let Some(w) = style.max_width.clone() {
            wrapper = wrapper.max_width(w);
        }
        if let Some(h) = style.max_height.clone() {
            wrapper = wrapper.max_height(h);
        }

        wrapper
            .child(content)
            .child(action_el.clone())
            .into_element()
    } else {
        content.into_element()
    }
}

impl AlertComponent {
    /// Attach a leading icon. Expected to be 16×16 (`size_4`).
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    /// Attach a top-right action.
    ///
    /// The action renders absolutely at `top-2 right-2`. Use
    /// `.action(AlertAction().child(...))` so the positioning applies.
    /// Long descriptions that share horizontal extent with the action
    /// need `.style("pr-24")` (or similar) on `AlertDescription`.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_element());
        self
    }
}

// ==============================================================
// AlertTitle
// ==============================================================

#[component(
    props(text: String = String::new()),
    ext(style, children, key, event_handlers, accessibility),
)]
#[derive(Clone, PartialEq)]
pub fn AlertTitle() {
    let mut container = Div()
        .style("flex items-center font-medium tracking-tight")
        .min_h(20.0);

    if !text.is_empty() {
        container = container.child(Label().text(text.as_str()).text_sm().tracking_tight());
    }

    container
        .merge_style(style.clone())
        .event_handlers(event_handlers.clone())
        .accessibility(accessibility.clone())
        .children(children.clone())
}

// ==============================================================
// AlertDescription
// ==============================================================

#[component(
    props(text: String = String::new()),
    ext(style, children, key, event_handlers, accessibility),
)]
#[derive(Clone, PartialEq)]
pub fn AlertDescription() {
    let ctx: Option<AlertVariantContext> = try_consume_context();
    let desc_color = match ctx.unwrap_or_default().0 {
        AlertVariant::Destructive => "text-destructive/90",
        AlertVariant::Default => "text-muted-foreground",
    };

    let mut container = Div()
        .style("flex flex-col justify-center gap-1")
        .style(desc_color)
        .min_h(20.0);

    if !text.is_empty() {
        container = container.child(Label().text(text.as_str()).text_sm().style(desc_color));
    }

    container
        .merge_style(style.clone())
        .event_handlers(event_handlers.clone())
        .accessibility(accessibility.clone())
        .children(children.clone())
}

// ==============================================================
// AlertAction
// ==============================================================

/// The action slot. Absolutely positioned at `top-2 right-2` (8px,
/// 8px) against the alert's visible box.
///
/// Must be passed via `Alert().action(...)`, not `.child(...)` —
/// `.child(...)` would place it inside `Div`'s `WithPosition` wrapper,
/// which becomes the containing block and breaks the positioning.
///
/// Absolute positioning means the action is out of the flex flow:
/// buttons render at their natural intrinsic width, never shrunk,
/// never wrapping. Reserve horizontal space via `.style("pr-24")` on
/// the description when text would overlap.
#[component(ext(style, children, key, event_handlers, accessibility))]
#[derive(Clone, PartialEq)]
pub fn AlertAction() {
    Div()
        .absolute()
        .top(8.0) // top-2
        .right(8.0) // right-2
        .z_index(10)
        .merge_style(style.clone())
        .event_handlers(event_handlers.clone())
        .accessibility(accessibility.clone())
        .children(children.clone())
}
