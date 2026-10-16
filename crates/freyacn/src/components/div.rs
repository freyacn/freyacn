//! # Div — styled primitive wrapper
//! …(docs unchanged)…

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

    let style = style.clone().with_active_states(active);

    // ── Single-value fields ────────────────────────────────────────
    // `with_event_handlers` comes FIRST — it REPLACES the map, so
    // anything registered inline (the pointer handlers below) has to
    // run after it to survive.
    let mut primitive = rect()
        .with_event_handlers(event_handlers.clone())
        .accessibility(accessibility.clone())
        .a11y_id(a11y_id)
        .a11y_focusable(*focusable && !disabled)
        .background(style.background.clone().unwrap_or_default())
        .color(style.text_color.unwrap_or_default())
        .font_size(style.font_size.unwrap_or_default())
        .font_slant(style.font_slant.unwrap_or_default())
        .font_weight(style.font_weight.unwrap_or_default())
        .text_align(style.text_align.unwrap_or_default())
        .text_decoration(style.text_decoration.unwrap_or_default())
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
        .direction(style.direction.unwrap_or_default())
        .main_align(style.main_align.unwrap_or_default())
        .cross_align(style.cross_align.unwrap_or_default())
        .content(style.content.unwrap_or_default())
        .spacing(style.spacing.unwrap_or_default())
        .opacity(style.opacity.unwrap_or(1.0))
        .interactive(style.pointer_events.unwrap_or(true));

    // ── Text overflow ──────────────────────────────────────────────
    if let Some(ellipsis) = style.text_ellipsis {
        primitive = primitive.text_overflow(match ellipsis {
            TextEllipsis::Clip => TextOverflow::Clip,
            TextEllipsis::Ellipsis => TextOverflow::Ellipsis,
        });
    }

    // ── Layer / z-index ────────────────────────────────────────────
    if let Some(z) = style.z_index {
        let clamped = z.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        primitive = primitive.layer(clamped);
    }

    // ── Vector fields ──────────────────────────────────────────────
    for family in &style.font_families {
        primitive = primitive.font_family(family.clone());
    }
    for shadow in &style.shadows {
        primitive = primitive.shadow(shadow.clone());
    }
    for shadow in &style.text_shadows {
        primitive = primitive.text_shadow(shadow.clone());
    }

    // ── Extended typography ────────────────────────────────────────
    if let Some(w) = style.font_width {
        primitive = primitive.font_width(w);
    }
    if let Some(h) = style.text_height {
        primitive = primitive.text_height(h);
    }
    if let Some(w) = style.visible_width {
        primitive = primitive.visible_width(w);
    }
    if let Some(h) = style.visible_height {
        primitive = primitive.visible_height(h);
    }

    // ── Transforms / effects ──────────────────────────────────────
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

    // ── Child offsets ─────────────────────────────────────────────
    if let Some(x) = style.offset_x {
        primitive = primitive.offset_x(x);
    }
    if let Some(y) = style.offset_y {
        primitive = primitive.offset_y(y);
    }

    // ── Pointer handlers ───────────────────────────────────────────
    // These INSERT into the map that `with_event_handlers` populated
    // above. Order matters: they must run after that call, since it
    // replaces the map wholesale.
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

    // ── Children ──────────────────────────────────────────────────
    primitive.children(children.clone())
}

impl DivComponent {
    pub fn on_hover(mut self, on_hover: impl Into<EventHandler<Event<PointerEventData>>>) -> Self {
        self.on_hover = Some(on_hover.into());
        self
    }
}
