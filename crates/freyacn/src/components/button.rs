//! Button component — shadcn/ui styling on top of Freya's `Button`.

use crate::prelude::*;
// ==============================================================
// Variants
// ==============================================================

/// Visual variant of a [`Button`].
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[variant("bg-primary text-primary-foreground hover:bg-primary/80")]
    #[default]
    Default,

    #[variant(
        "border-border bg-background hover:bg-muted hover:text-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
    )]
    Outline,

    #[variant("bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,

    #[variant("hover:bg-muted hover:text-foreground dark:hover:bg-muted/50")]
    Ghost,

    #[variant(
        "bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40"
    )]
    Destructive,

    #[variant("text-primary underline underline-offset-4")]
    Link,
}

/// Size of a [`Button`].
#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[variant("h-8 gap-1.5 px-2.5")]
    #[default]
    Default,

    #[variant("h-6 gap-1 rounded-md px-2 text-xs")]
    Xs,

    #[variant("h-7 gap-1 rounded-md px-2.5 text-[0.8rem]")]
    Sm,

    #[variant("h-9 gap-1.5 px-2.5 text-sm")]
    Lg,

    #[variant("size-8")]
    Icon,

    #[variant("size-6 rounded-md")]
    IconXs,

    #[variant("size-7 rounded-md")]
    IconSm,

    #[variant("size-9")]
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
        on_hover: EventHandler<Event<PointerEventData>> = EventHandler::from(|_: Event<PointerEventData>| {}),
    ),
    variants(
        variant: ButtonVariant = ButtonVariant::Default,
        size:    ButtonSize    = ButtonSize::Default,
    ),
    ext(style, children, key, event_handlers),
)]
#[derive(Clone, PartialEq)]
pub fn Button() {
    let style = style
        .clone()
        .style(
            "inline-flex shrink-0 items-center justify-center rounded-lg border border-transparent bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:opacity-50"
        )
        .merge(variant.resolve())
        .merge(size.resolve());

    Div()
        .children(children.clone())
        .with_event_handlers(event_handlers.clone())
        .merge_style(style)
        .disabled(disabled.clone())
        .on_hover(on_hover.clone())
        .focusable(focusable.clone())
}
