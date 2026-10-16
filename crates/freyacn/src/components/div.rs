//! # Div — styled primitive wrapper
//!
//! A `#[component]` that renders a Freya `rect()` after folding the
//! `ext`-provided [`Style`] into concrete values.
//!
//! `Div` owns runtime state observation (`DARK`, `HOVER`, `ACTIVE`,
//! `FOCUS`, `FOCUS_VISIBLE`, `DISABLED`), so any component that returns a
//! `Div` gets `hover:` / `focus:` / `active:` / `disabled:` / `dark:`
//! folding for free.
//!
//! ## Inheritable vs. non-inheritable fields
//!
//! Freya's `Rect` behaves like CSS for the fields that cascade:
//! `color`, `font_size`, `font_slant`, `font_weight`, `font_families`,
//! `text_align`, `text_decoration`, `font_width`, `text_height`,
//! `text_shadows`. Those are forwarded only when the `Style` actually
//! set them, so a nested element inherits whatever an ancestor provided.
//! Forcing them with `unwrap_or_default()` would clobber inheritance and
//! is what caused icons inside buttons to lose the button's text colour.
//!
//! Fields that don't inherit (`background`, `padding`, `margin`, sizes,
//! borders, position, flex layout, opacity, etc.) are still forwarded
//! with their `Default` when unset — those defaults match Freya's own
//! neutral state, so writing them is harmless.

use crate::extensions::TextEllipsis;
use crate::prelude::*;

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

    // ── Compute the active flag set ────────────────────────────────
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

    // ── Fold state-conditional styles into the base ────────────────
    let style = style.clone().with_active_states(active);

    // ── Non-inheritable fields ─────────────────────────────────────
    // These either don't cascade in CSS or have a Freya default that
    // matches "unset", so forwarding them with `unwrap_or_default()` is
    // safe and produces the same visual result as leaving them off.
    let mut primitive = rect()
        // Event handlers first — `with_event_handlers` REPLACES the map,
        // so inline `.on_pointer_*` registrations below must run after it.
        .with_event_handlers(event_handlers.clone())
        // Accessibility: `.accessibility(...)` replaces the whole struct,
        // so the individual `a11y_*` setters come after it.
        .accessibility(accessibility.clone())
        .a11y_id(a11y_id)
        .a11y_focusable(*focusable && !*disabled)
        // Background — not inherited; forcing transparent matches the
        // renderer's default.
        .background(style.background.clone().unwrap_or_default())
        // Layout / box
        .corner_radius(style.corner_radius.unwrap_or_default())
        .border(style.to_border())
        .padding(style.padding.unwrap_or_default())
        .margin(style.margin.unwrap_or_default())
        .width(style.width.clone().unwrap_or_default())
        .height(style.height.clone().unwrap_or_default())
        .min_width(style.min_width.clone().unwrap_or_default())
        .min_height(style.min_height.clone().unwrap_or_default())
        .max_width(style.max_width.clone().unwrap_or_default())
        .max_height(style.max_height.clone().unwrap_or_default())
        .position(style.to_position().unwrap_or_default())
        .overflow(style.overflow.unwrap_or_default())
        // Flex layout — direction / alignments default to whatever Freya
        // considers neutral, so writing them when unset is fine.
        .direction(style.direction.unwrap_or_default())
        .main_align(style.main_align.unwrap_or_default())
        .cross_align(style.cross_align.unwrap_or_default())
        .content(style.content.unwrap_or_default())
        .spacing(style.spacing.unwrap_or_default())
        // Effects. `opacity` defaults to 1.0 (fully opaque), not 0.0.
        .opacity(style.opacity.unwrap_or(1.0))
        .interactive(style.pointer_events.unwrap_or(true));

    // ── Inheritable fields ─────────────────────────────────────────
    // Only written when the `Style` actually set them. This is what lets
    // a nested element inherit the ancestor's text colour and typography
    // instead of being clobbered with a neutral default.

    if let Some(colour) = style.text_color {
        primitive = primitive.color(colour);
    }
    if let Some(size) = style.font_size {
        primitive = primitive.font_size(size);
    }
    if let Some(slant) = style.font_slant {
        primitive = primitive.font_slant(slant);
    }
    if let Some(weight) = style.font_weight {
        primitive = primitive.font_weight(weight);
    }
    if let Some(align) = style.text_align {
        primitive = primitive.text_align(align);
    }
    if let Some(deco) = style.text_decoration {
        primitive = primitive.text_decoration(deco);
    }
    if let Some(width) = style.font_width {
        primitive = primitive.font_width(width);
    }
    if let Some(height) = style.text_height {
        primitive = primitive.text_height(height);
    }

    // `Vec` fields only emit calls when non-empty, so they already
    // behave correctly — an empty list leaves the inherited value alone.
    for family in &style.font_families {
        primitive = primitive.font_family(family.clone());
    }
    for shadow in &style.text_shadows {
        primitive = primitive.text_shadow(shadow.clone());
    }

    // ── Non-inheritable, single-shot overrides ─────────────────────
    // These don't cascade and have no CSS default; skip when unset.

    if let Some(ellipsis) = style.text_ellipsis {
        primitive = primitive.text_overflow(match ellipsis {
            TextEllipsis::Clip => TextOverflow::Clip,
            TextEllipsis::Ellipsis => TextOverflow::Ellipsis,
        });
    }
    if let Some(z) = style.z_index {
        let clamped = z.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        primitive = primitive.layer(clamped);
    }
    if let Some(w) = style.visible_width {
        primitive = primitive.visible_width(w);
    }
    if let Some(h) = style.visible_height {
        primitive = primitive.visible_height(h);
    }
    if let Some(deg) = style.rotate {
        primitive = primitive.rotate(deg);
    }
    if let Some(s) = style.scale {
        primitive = primitive.scale(s);
    }
    if let Some(origin) = style.transform_origin {
        primitive = primitive.transform_origin(origin);
    }
    if let Some(radius) = style.blur {
        primitive = primitive.blur(radius);
    }
    if let Some(x) = style.offset_x {
        primitive = primitive.offset_x(x);
    }
    if let Some(y) = style.offset_y {
        primitive = primitive.offset_y(y);
    }

    for shadow in &style.shadows {
        primitive = primitive.shadow(shadow.clone());
    }

    // ── Pointer handlers ───────────────────────────────────────────
    if !*disabled {
        let on_hover = on_hover.clone();
        primitive = primitive
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

    primitive.children(children.clone())
}

impl DivComponent {
    pub fn on_hover(mut self, on_hover: impl Into<EventHandler<Event<PointerEventData>>>) -> Self {
        self.on_hover = Some(on_hover.into());
        self
    }
}
