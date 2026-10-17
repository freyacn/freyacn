//! # Div — styled primitive wrapper
//!
//! A `#[component]` that renders a Freya `rect()` after folding the
//! `ext`-provided [`Style`] into concrete values.
//!
//! ## Layout modes
//!
//! `Div` picks one of two shapes at render time, based on what the
//! `Style` asks for:
//!
//! | `Style` state         | Rendered as                                                  |
//! |-----------------------|--------------------------------------------------------------|
//! | `scroll_axis = None`  | styled `rect()` with the requested layout                     |
//! | `scroll_axis = Some`  | `ScrollView` wrapping a content-sized `rect()`                |
//!
//! When scrolling is active, the outer `ScrollView` takes the size
//! (`w-*`, `h-*`, `max-*`) and the inner styled rect grows to fit its
//! content. Without that split, the inner rect would be bounded and
//! the scroll view would never overflow.
//!
//! ## State handling
//!
//! `Div` is the last hop before the primitive, so it owns runtime
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
//! ancestor provided.
//!
//! Non-inheritable fields (`background`, `padding`, `margin`, sizes,
//! borders, position, flex layout, opacity) are forwarded with their
//! `Default` when unset.

use crate::extensions::TextEllipsis;
use crate::prelude::*;

/// Percentage of a single grid column.
fn grid_cell_pct(cols: u32) -> f32 {
    100.0 / cols.max(1) as f32
}

/// Apply every size-related field of `style` to a `ScrollView`.
fn apply_size_to_scroll_view(style: &Style, mut sv: ScrollView) -> ScrollView {
    if let Some(w) = style.width.clone() {
        sv = sv.width(w);
    }
    if let Some(h) = style.height.clone() {
        sv = sv.height(h);
    }
    if let Some(w) = style.max_width.clone() {
        sv = sv.max_width(w);
    }
    if let Some(h) = style.max_height.clone() {
        sv = sv.max_height(h);
    }
    sv
}

/// Apply every size-related field of `style` to a `rect()`.
fn apply_size_to_rect(style: &Style, mut r: Rect) -> Rect {
    if let Some(w) = style.width.clone() {
        r = r.width(w);
    }
    if let Some(h) = style.height.clone() {
        r = r.height(h);
    }
    if let Some(w) = style.min_width.clone() {
        r = r.min_width(w);
    }
    if let Some(h) = style.min_height.clone() {
        r = r.min_height(h);
    }
    if let Some(w) = style.max_width.clone() {
        r = r.max_width(w);
    }
    if let Some(h) = style.max_height.clone() {
        r = r.max_height(h);
    }
    r
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

    // ── Build the styled rect ──────────────────────────────────────
    //
    // This rect always carries the visual styling, layout, and children.
    // Its size behaviour depends on whether we're scrolling:
    //
    //   - Non-scrolling: the size fields go straight on the rect.
    //   - Scrolling: the rect grows to fit its content (`Size::auto()`),
    //     and the size fields move to the outer `ScrollView` so it has
    //     a bounded viewport to scroll inside.

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
            .direction(style.direction.clone().unwrap_or_default())
            .main_align(style.main_align.clone().unwrap_or_default())
            .cross_align(style.cross_align.clone().unwrap_or_default())
            .content(style.content.clone().unwrap_or_default())
            .spacing(style.spacing.clone().unwrap_or_default());

        for child in children.iter() {
            visual = visual.child(child.clone());
        }
    }

    // ── Size ───────────────────────────────────────────────────────
    //
    // Only applied to the inner rect when *not* scrolling. Scrolling
    // sizes the ScrollView instead.
    if !is_scroll {
        visual = apply_size_to_rect(&style, visual);
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
    if let Some(z) = style.z_index.clone() {
        let clamped = z.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        visual = visual.layer(clamped);
    }
    if let Some(w) = style.visible_width.clone() {
        visual = visual.visible_width(w);
    }
    if let Some(h) = style.visible_height.clone() {
        visual = visual.visible_height(h);
    }
    if let Some(deg) = style.rotate.clone() {
        visual = visual.rotate(deg);
    }
    if let Some(s) = style.scale.clone() {
        visual = visual.scale(s);
    }
    if let Some(origin) = style.transform_origin.clone() {
        visual = visual.transform_origin(origin);
    }
    if let Some(radius) = style.blur.clone() {
        visual = visual.blur(radius);
    }
    if let Some(x) = style.offset_x.clone() {
        visual = visual.offset_x(x);
    }
    if let Some(y) = style.offset_y.clone() {
        visual = visual.offset_y(y);
    }
    for shadow in &style.shadows.clone() {
        visual = visual.shadow(shadow.clone());
    }

    // ── Accessibility and events on the inner rect ─────────────────
    //
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

    // ── Scrolling branch ───────────────────────────────────────────
    //
    // When the style asks for scrolling, hand the content-sized rect to
    // a `ScrollView`. The scroll view takes the outer size so it has a
    // bounded viewport; the inner rect grows to fit its content and
    // overflows, which is what makes the scrollbar appear.
    if let Some(axis) = style.scroll_axis {
        let direction = match axis {
            ScrollAxis::Vertical => Direction::Vertical,
            ScrollAxis::Horizontal => Direction::Horizontal,
            // `ScrollView` doesn't expose a both-axes direction — it
            // picks the scroll axis from whichever content overflows.
            // Default to vertical here.
            ScrollAxis::Both => Direction::Vertical,
        };

        let sv = ScrollView::new()
            .direction(direction)
            .show_scrollbar(style.scrollbar.unwrap_or(true))
            .child(visual);

        let sv = apply_size_to_scroll_view(&style, sv);

        return sv.into_element();
    }

    visual.into_element()
}

impl DivComponent {
    pub fn on_hover(mut self, on_hover: impl Into<EventHandler<Event<PointerEventData>>>) -> Self {
        self.on_hover = Some(on_hover.into());
        self
    }
}
