//! Button component — shadcn/ui styling on top of Freya's `Button`.

use freya::prelude::*;
use freya_core::integration::Overflow::Clip;
use freyacn::*;

// ==============================================================
// Variants
// ==============================================================

/// Visual variant of a [`Button`].
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground")]
    #[default]
    Default,

    #[variant("bg-background border border-border")]
    Outline,

    #[variant("bg-secondary text-secondary-foreground")]
    Secondary,

    #[variant("text-foreground")]
    Ghost,

    #[variant("bg-destructive/10 text-destructive-foreground")]
    Destructive,

    #[variant("text-primary underline underline-offset-4")]
    Link,
}

/// Size of a [`Button`].
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[variant("h-8 gap-1_5 px-2_5 text-sm")]
    #[default]
    Default,

    #[variant("h-6 gap-1 rounded-md px-2 text-xs")]
    Xs,

    #[variant("h-7 gap-1 rounded-md px-2_5 text-sm")]
    Sm,

    #[variant("h-9 gap-1_5 px-2_5 text-sm")]
    Lg,

    #[variant("w-8 h-8")]
    Icon,

    #[variant("w-6 h-6 rounded-md")]
    IconXs,

    #[variant("w-7 h-7 rounded-md")]
    IconSm,

    #[variant("w-9 h-9")]
    IconLg,
}

// ==============================================================
// Component
// ==============================================================

/// A button.
///
/// # Examples
///
/// ```ignore
/// Button().text("Save").on_press(|_| save());
///
/// Button()
///     .text("Delete")
///     .variant(ButtonVariant::Destructive)
///     .size(ButtonSize::Lg)
///     .on_press(|_| delete());
/// ```
#[component(
    props(
        disabled: bool = false,
        focusable: bool = true,
        on_hover: String,
    ),
    variants(
        variant: ButtonVariant = ButtonVariant::Default,
        size:    ButtonSize    = ButtonSize::Default,
    ),
    ext(style, children, key, event_handlers),
)]
#[derive(Clone, PartialEq)]
pub fn Button() {
    let theme = use_cn_theme().read();
    let mut hovering = use_state(|| false);
    let a11y_id = use_a11y();
    let focus = use_focus(a11y_id);
    let disabled = use_reactive(disabled);

    /*
    let merged = Style::default()
        .merge(variant.resolve())
        .merge(size.resolve())
        .merge(style.clone());

    // ── Compute the active flag set from runtime state ──
    let mut active = StateFlags::NONE;
    if theme.is_dark           { active = active.or(StateFlags::DARK); }
    if *hovering.read()        { active = active.or(StateFlags::HOVER); }
    if focus.is_focused()      { active = active.or(StateFlags::FOCUS_VISIBLE); }
    if *disabled               { active = active.or(StateFlags::DISABLED); }

    // ── Fold matching state styles into the base ──
    let effective = merged.with_active_states(active);

    // ── From here, `effective` is exactly what you had before ──
    let bg         = effective.background.clone().and_then(fill_to_color)
                       .unwrap_or(Color::TRANSPARENT);
    let fg         = effective.text_color.unwrap_or(theme.foreground);
    let border     = effective.border_color.unwrap_or(Color::TRANSPARENT);
    let focus_ring = effective.ring_color.unwrap_or(fg);
    let radius     = effective.corner_radius.unwrap_or_default();

    drop(theme);

     */

    let mut hover_style = Style::default();
    let mut focus_style = Style::default()
        .ring_width(3.0)
        .ring_color(apply_opacity(theme.ring, 0.5));

    // ── Merge variant + size + user overrides ──
    let mut style = Style::default()
        .overflow(Clip)
        .merge(variant.resolve())
        .merge(size.resolve())
        .merge(style.clone());

    match variant {
        ButtonVariant::Default => {
            hover_style = hover_style.background(apply_opacity(theme.primary, 0.8));
        }
        ButtonVariant::Outline => {
            hover_style = hover_style.bg_muted().text_foreground();

            if theme.is_dark {
                style = style.background(apply_opacity(theme.input, 0.3));
                hover_style = hover_style.background(apply_opacity(theme.input, 0.5));
            }
        }
        ButtonVariant::Secondary => {
            hover_style =
                hover_style.background(blend_colors(theme.secondary, theme.foreground, 0.05));
        }
        ButtonVariant::Ghost => {
            hover_style = hover_style.bg_muted().text_foreground();

            if theme.is_dark {
                hover_style = hover_style.background(apply_opacity(theme.muted, 0.5));
            }
        }
        ButtonVariant::Destructive => {
            style = style.background(apply_opacity(theme.destructive, 0.1));
            hover_style = hover_style.background(apply_opacity(theme.destructive, 0.2));
            focus_style = focus_style
                .border_color(apply_opacity(theme.destructive, 0.4))
                .ring_color(apply_opacity(theme.destructive, 0.2));

            if theme.is_dark {
                style = style.background(apply_opacity(theme.destructive, 0.2));
                hover_style = hover_style.background(apply_opacity(theme.destructive, 0.3));
                focus_style = focus_style.ring_color(apply_opacity(theme.destructive, 0.4));
            }
        }
        ButtonVariant::Link => {
            hover_style = hover_style.underline();
        }
        _ => {}
    }

    rect()
        .overflow(style.overflow.unwrap().clone())
        .a11y_id(a11y_id)
}
