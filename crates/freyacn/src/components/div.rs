//! # Div — styled primitive wrapper
//!
//! A `#[component]` that renders a Freya `rect()` after folding the
//! `ext`-provided [`Style`] into concrete values.
//!
//! ## Layout modes
//!
//! `Div` picks one of two shapes at render time, based on the `Style`:
//!
//! | `Style` state           | Rendered as                                          |
//! |-------------------------|------------------------------------------------------|
//! | `scroll_axis = None`    | styled `rect()` with the requested layout            |
//! | `scroll_axis = Some(_)` | the same rect, plus `.scrollable(true)` for scrolling |
//!
//! Scrolling and grid combine freely: a grid with `overflow-scroll`
//! becomes a scrollable grid.
//!
//! ## State handling
//!
//! `Div` is the last hop before the primitive, so it owns the runtime
//! observation of every state that `Style::states` can be conditioned
//! on:
//!
//! | Flag              | Source                                             |
//! |-------------------|----------------------------------------------------|
//! | `DARK`            | `use_cn_theme().is_dark`                           |
//! | `HOVER`           | `on_pointer_over` / `on_pointer_out`               |
//! | `ACTIVE`          | `on_pointer_down` / `on_pointer_out`               |
//! | `FOCUS`           | `use_focus(a11y_id)`                               |
//! | `FOCUS_VISIBLE`   | `use_focus(a11y_id)`                               |
//! | `DISABLED`        | the `disabled` prop                                |
//!
//! ## Inheritable vs. non-inheritable fields
//!
//! Freya's `Rect` cascades: `color`, `font_size`, `font_slant`,
//! `font_weight`, `font_families`, `text_align`, `text_decoration`,
//! `font_width`, `text_height`, `text_shadows`. Those are forwarded
//! only when the `Style` set them, so descendants inherit whatever an
//! ancestor provided. Forcing them with `unwrap_or_default()` would
//! clobber inheritance.
//!
//! Non-inheritable fields (`background`, `padding`, `margin`, sizes,
//! borders, position, flex layout, opacity) are forwarded with their
//! `Default` when unset — those defaults match Freya's neutral state.

use crate::extensions::TextEllipsis;
use crate::prelude::*;

/// Percentage of a single grid column.
fn grid_cell_pct(cols: u32) -> f32 {
    100.0 / cols.max(1) as f32
}

#[component(
    props(
        disabled: bool = false,
        focusable: bool = false,
    ),
    ext(style, children, event_handlers, accessibility),
)]
#[struct_fields(on_hover: EventHandler<Event<PointerEventData>>)]
#[derive(PartialEq, Clone)]
pub fn Div() {
    // ── Runtime state tracking ─────────────────────────────────────
    let mut hovering = use_state(|| false);
    let mut pressed = use_state(|| false);
    let a11y_id = use_a11y();
    let focus = use_focus(a11y_id);

    let theme = use_cn_theme().read();
    let mut active = StateFlags::NONE;
    if theme.is_dark {
        active = active.or(StateFlags::DARK);
    }
    drop(theme);

    if *hovering.read() {
        active = active.or(StateFlags::HOVER);
    }
    if *pressed.read() {
        active = active.or(StateFlags::ACTIVE);
    }
    if focus().is_focused() {
        active = active.or(StateFlags::FOCUS).or(StateFlags::FOCUS_VISIBLE);
    }
    if *disabled {
        active = active.or(StateFlags::DISABLED);
    }

    let style = style.clone().with_active_states(active);
    let is_scroll = style.scroll_axis.is_some();

    // ── Build the single rect ──────────────────────────────────────
    //
    // Everything — styling, layout, children, scroll — applies to one
    // primitive. Freya's `scrollable` flag turns the same rect into a
    // scroll container; no outer wrapper is needed.

    let mut visual = rect()
        // Non-inheritable
        .background(style.background.clone().unwrap_or_default())
        .corner_radius(style.corner_radius.unwrap_or_default())
        .border(style.to_border())
        .padding(style.padding.unwrap_or_default())
        .margin(style.margin.unwrap_or_default())
        .position(style.to_position().unwrap_or_default())
        .overflow(style.overflow.unwrap_or_default())
        .opacity(style.opacity.unwrap_or(1.0))
        .interactive(style.pointer_events.unwrap_or(true));

    // ── Layout axis ────────────────────────────────────────────────
    if let Some(cols) = style.grid_cols {
        // Grid: horizontal flex with wrap. Cell widths are applied to
        // each child below.
        visual = visual
            .content(Content::Wrap { wrap_spacing: None })
            .direction(Direction::Horizontal)
            .spacing(style.spacing.unwrap_or(0.0));

        let cell_pct = grid_cell_pct(cols);
        for child in children.iter() {
            visual = visual.child(
                rect()
                    .width(Size::percent(cell_pct))
                    .content(Content::Fit)
                    .child(child.clone()),
            );
        }
    } else {
        visual = visual
            .direction(style.direction.unwrap_or_default())
            .main_align(style.main_align.unwrap_or_default())
            .cross_align(style.cross_align.unwrap_or_default())
            .content(style.content.unwrap_or_default())
            .spacing(style.spacing.unwrap_or_default());

        for child in children.iter() {
            visual = visual.child(child.clone());
        }
    }

    // ── Size ───────────────────────────────────────────────────────
    // Applied whether scrolling or not. A scroll container with an
    // unbounded size grows to fit its content and never scrolls —
    // always pair `overflow-scroll` with a fixed or max size.
    if let Some(w) = style.width.clone() {
        visual = visual.width(w);
    }
    if let Some(h) = style.height.clone() {
        visual = visual.height(h);
    }
    if let Some(w) = style.min_width.clone() {
        visual = visual.min_width(w);
    }
    if let Some(h) = style.min_height.clone() {
        visual = visual.min_height(h);
    }
    if let Some(w) = style.max_width.clone() {
        visual = visual.max_width(w);
    }
    if let Some(h) = style.max_height.clone() {
        visual = visual.max_height(h);
    }

    // ── Scroll ─────────────────────────────────────────────────────
    // Freya's `scrollable` flag: on for any `overflow-scroll` variant.
    // Freya decides which axis to scroll based on which content
    // overflows — it doesn't expose a direction parameter.
    if is_scroll {
        visual = visual.scrollable(true);
    }

    // ── Inheritable text styles ────────────────────────────────────
    if let Some(c) = style.text_color {
        visual = visual.color(c);
    }
    if let Some(s) = style.font_size {
        visual = visual.font_size(s);
    }
    if let Some(s) = style.font_slant {
        visual = visual.font_slant(s);
    }
    if let Some(s) = style.font_weight {
        visual = visual.font_weight(s);
    }
    if let Some(a) = style.text_align {
        visual = visual.text_align(a);
    }
    if let Some(d) = style.text_decoration {
        visual = visual.text_decoration(d);
    }
    if let Some(w) = style.font_width {
        visual = visual.font_width(w);
    }
    if let Some(h) = style.text_height {
        visual = visual.text_height(h);
    }
    for family in &style.font_families {
        visual = visual.font_family(family.clone());
    }
    for shadow in &style.text_shadows {
        visual = visual.text_shadow(shadow.clone());
    }

    // ── Non-inheritable, single-shot overrides ─────────────────────
    if let Some(ellipsis) = style.text_ellipsis {
        visual = visual.text_overflow(match ellipsis {
            TextEllipsis::Clip => TextOverflow::Clip,
            TextEllipsis::Ellipsis => TextOverflow::Ellipsis,
        });
    }
    if let Some(z) = style.z_index {
        let clamped = z.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        visual = visual.layer(clamped);
    }
    if let Some(w) = style.visible_width {
        visual = visual.visible_width(w);
    }
    if let Some(h) = style.visible_height {
        visual = visual.visible_height(h);
    }
    if let Some(deg) = style.rotate {
        visual = visual.rotate(deg);
    }
    if let Some(s) = style.scale {
        visual = visual.scale(s);
    }
    if let Some(origin) = style.transform_origin {
        visual = visual.transform_origin(origin);
    }
    if let Some(radius) = style.blur {
        visual = visual.blur(radius);
    }
    if let Some(x) = style.offset_x {
        visual = visual.offset_x(x);
    }
    if let Some(y) = style.offset_y {
        visual = visual.offset_y(y);
    }
    for shadow in &style.shadows {
        visual = visual.shadow(shadow.clone());
    }

    // ── Accessibility and events ───────────────────────────────────
    // `with_event_handlers` replaces the map, so it runs before the
    // inline pointer handlers below.
    visual = visual
        .accessibility(accessibility.clone())
        .a11y_id(a11y_id)
        .a11y_focusable(*focusable && !*disabled)
        .with_event_handlers(event_handlers.clone());

    if !*disabled {
        let on_hover = on_hover.clone();
        visual = visual
            .on_pointer_over(move |e: Event<PointerEventData>| {
                hovering.set(true);
                if let Some(handler) = &on_hover {
                    handler.call(e);
                }
            })
            .on_pointer_out(move |_| {
                hovering.set_if_modified(false);
                pressed.set_if_modified(false);
            })
            .on_pointer_down(move |_| pressed.set(true));
    }

    visual.into_element()
}

impl DivComponent {
    pub fn on_hover(mut self, on_hover: impl Into<EventHandler<Event<PointerEventData>>>) -> Self {
        self.on_hover = Some(on_hover.into());
        self
    }
}
