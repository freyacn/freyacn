//! # Button Showcase
//!
//! A runnable example demonstrating every `Button` variant, every size,
//! icon-only layouts, disabled states, and interactive hover behaviour,
//! inside a scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example button_showcase
//! ```

use freyacn::prelude::*;

// ==============================================================
// Demo assets
// ==============================================================

const CHECK_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>"##;

const PLUS_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>"##;

const TRASH_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/></svg>"##;

// ==============================================================
// Layout helpers
// ==============================================================

fn section(title: &str, description: &str, body: impl IntoElement) -> impl IntoElement {
    Div()
        .style("flex flex-col gap-3 p-6 bg-card rounded-xl border border-border shadow-sm")
        .child(
            Label()
                .text(title)
                .style("text-lg font-semibold text-card-foreground"),
        )
        .child(
            Label()
                .text(description)
                .style("text-sm text-muted-foreground"),
        )
        .child(body)
}

fn row(children: impl IntoIterator<Item = Element>) -> impl IntoElement {
    Div()
        .style("flex flex-row flex-wrap items-center gap-3")
        .children(children)
}

fn divider() -> impl IntoElement {
    Div().style("h-px w-full bg-border")
}

// ==============================================================
// Showcase sections
// ==============================================================

fn variants_section() -> impl IntoElement {
    section(
        "Variants",
        "Every ButtonVariant at the default size.",
        row([
            Button()
                .variant(ButtonVariant::Default)
                .child(Label().text("Default"))
                .on_press(|_| println!("pressed: default"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Secondary)
                .child(Label().text("Secondary"))
                .on_press(|_| println!("pressed: secondary"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Destructive)
                .child(Label().text("Destructive"))
                .on_press(|_| println!("pressed: destructive"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Outline)
                .child(Label().text("Outline"))
                .on_press(|_| println!("pressed: outline"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Ghost)
                .child(Label().text("Ghost"))
                .on_press(|_| println!("pressed: ghost"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Link)
                .child(Label().text("Link"))
                .on_press(|_| println!("pressed: link"))
                .into_element(),
        ]),
    )
}

fn sizes_section() -> impl IntoElement {
    section(
        "Sizes",
        "Every ButtonSize at the default variant.",
        row([
            Button()
                .size(ButtonSize::Xs)
                .child(Label().text("Extra small"))
                .into_element(),
            Button()
                .size(ButtonSize::Sm)
                .child(Label().text("Small"))
                .into_element(),
            Button()
                .size(ButtonSize::Default)
                .child(Label().text("Default"))
                .into_element(),
            Button()
                .size(ButtonSize::Lg)
                .child(Label().text("Large"))
                .into_element(),
        ]),
    )
}

fn with_icons_section() -> impl IntoElement {
    section(
        "With icons",
        "Icons sit inline with the label and inherit the button's colour.",
        row([
            Button()
                .variant(ButtonVariant::Default)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Confirm"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Secondary)
                .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4())
                .child(Label().text("New item"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Destructive)
                .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_4())
                .child(Label().text("Delete"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Outline)
                .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4())
                .child(Label().text("Add"))
                .into_element(),
        ]),
    )
}

fn icon_only_section() -> impl IntoElement {
    section(
        "Icon only",
        "Square icon buttons with accessibility labels.",
        row([
            Button()
                .size(ButtonSize::IconXs)
                .variant(ButtonVariant::Ghost)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_3())
                .into_element(),
            Button()
                .size(ButtonSize::IconSm)
                .variant(ButtonVariant::Outline)
                .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_3_5())
                .into_element(),
            Button()
                .size(ButtonSize::Icon)
                .variant(ButtonVariant::Default)
                .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4())
                .into_element(),
            Button()
                .size(ButtonSize::IconLg)
                .variant(ButtonVariant::Destructive)
                .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_5())
                .into_element(),
        ]),
    )
}

fn variant_icon_matrix_section() -> impl IntoElement {
    section(
        "Every variant, with icon",
        "The icon inherits each variant's foreground colour.",
        row([
            Button()
                .variant(ButtonVariant::Default)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Default"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Secondary)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Secondary"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Destructive)
                .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_4())
                .child(Label().text("Destructive"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Outline)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Outline"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Ghost)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Ghost"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Link)
                .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                .child(Label().text("Link"))
                .into_element(),
        ]),
    )
}

fn disabled_section() -> impl IntoElement {
    section(
        "Disabled",
        "Same variants, disabled. Hover and press are suppressed.",
        row([
            Button()
                .variant(ButtonVariant::Default)
                .disabled(true)
                .child(Label().text("Default"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Secondary)
                .disabled(true)
                .child(Label().text("Secondary"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Destructive)
                .disabled(true)
                .child(Label().text("Destructive"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Outline)
                .disabled(true)
                .child(Label().text("Outline"))
                .into_element(),
        ]),
    )
}

fn stacking_section() -> impl IntoElement {
    section(
        "Within a column",
        "The container stacks them vertically; each button's own contents stay horizontal.",
        Div()
            .style("flex flex-col items-start gap-2")
            .child(
                Button()
                    .variant(ButtonVariant::Default)
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                    .child(Label().text("First"))
                    .into_element(),
            )
            .child(
                Button()
                    .variant(ButtonVariant::Secondary)
                    .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4())
                    .child(Label().text("Second"))
                    .into_element(),
            )
            .child(
                Button()
                    .variant(ButtonVariant::Outline)
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                    .child(Label().text("Third"))
                    .into_element(),
            ),
    )
}

fn overrides_section() -> impl IntoElement {
    section(
        "Overrides",
        "Variant + user overrides. User intent always wins.",
        row([
            Button()
                .variant(ButtonVariant::Default)
                .bg_emerald_500()
                .text_white()
                .child(Label().text("Emerald override"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Outline)
                .style("border-2 border-violet-500 text-violet-500 hover:bg-violet-500 hover:text-white")
                .child(Label().text("Style string"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Secondary)
                .corner_radius(9999.0)
                .p_6()
                .child(Label().text("Pill"))
                .into_element(),
            Button()
                .variant(ButtonVariant::Default)
                .shadow_lg()
                .child(Label().text("Elevated"))
                .into_element(),
        ]),
    )
}

/// A long list of buttons, used to make sure the page scrolls.
fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Many buttons stacked vertically — enough content to overflow the viewport and require scrolling.",
        Div()
            .style("flex flex-col gap-2")
            .children((0..40).map(|i| {
                Button()
                    .variant(if i % 2 == 0 {
                        ButtonVariant::Default
                    } else {
                        ButtonVariant::Outline
                    })
                    .child(Label().text(format!("Button {}", i + 1)))
                    .on_press(move |_| println!("pressed button {}", i + 1))
                    .into_element()
            })),
    )
}

// ==============================================================
// App
// ==============================================================

fn app() -> impl IntoElement {
    // Initialise the theme once at the root.
    use_init_cn_theme(Theme::dark().neutral().theme_color("neutral"));

    // The page is a single scrollable `Div`.
    //
    //   - `h-screen` gives it a definite height equal to the viewport,
    //     which is what makes `.scrollable(true)` actually scroll.
    //     Without a bounded height the container grows to fit its
    //     content and no scrollbar appears.
    //   - `overflow-scroll` sets `Style::scroll_axis`, which `Div`
    //     turns into `.scrollable(true)` plus `Overflow::Clip`.
    Div()
        .style("flex flex-col w-full h-screen bg-background overflow-scroll")
        .child(
            Div()
                .style("flex flex-col gap-8 max-w-3xl w-full mx-auto p-8")
                // Header
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label()
                                .text("Button")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Every variant, size, and composition, rendered live.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                // Sections
                .child(variants_section())
                .child(sizes_section())
                .child(with_icons_section())
                .child(icon_only_section())
                .child(variant_icon_matrix_section())
                .child(disabled_section())
                .child(stacking_section())
                .child(overrides_section())
                // Long tail of content so the page overflows
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
