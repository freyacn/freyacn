//! # Label Showcase
//!
//! A runnable example demonstrating the `Label` primitive: every
//! inheritable text style, cascade from a parent, explicit overrides,
//! alignment, decoration, line-height, truncation, and composition
//! inside buttons and cards — all inside a scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example label_showcase
//! ```

use freyacn::prelude::*;

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

fn divider() -> impl IntoElement {
    Div().style("h-px w-full bg-border")
}

/// A labelled example cell: monospace caption above the sample.
fn cell(caption: &str, sample: impl IntoElement) -> impl IntoElement {
    Div()
        .style("flex flex-col gap-2 p-3 rounded-md border border-border bg-muted/40")
        .child(
            Label()
                .text(caption)
                .style("text-[10px] font-mono text-muted-foreground"),
        )
        .child(sample)
}

// ==============================================================
// Showcase sections
// ==============================================================

fn basic_section() -> impl IntoElement {
    section(
        "Basic",
        "The smallest possible usage — a `Label` with text and nothing else.",
        Div()
            .style("flex flex-col gap-3")
            .child(Label().text("Hello, world."))
            .child(
                Label()
                    .text("A second line at the same default style.")
                    .style("text-foreground"),
            ),
    )
}

fn sizes_section() -> impl IntoElement {
    section(
        "Sizes",
        "The `text_*` helpers set font-size. Each is a leaf, so nothing cascades further.",
        Div()
            .style("flex flex-col gap-2")
            .child(Label().text("text_xs").style("text-xs text-foreground"))
            .child(Label().text("text_sm").style("text-sm text-foreground"))
            .child(Label().text("text_base").style("text-base text-foreground"))
            .child(Label().text("text_lg").style("text-lg text-foreground"))
            .child(Label().text("text_xl").style("text-xl text-foreground"))
            .child(Label().text("text_2xl").style("text-2xl text-foreground"))
            .child(Label().text("text_3xl").style("text-3xl text-foreground")),
    )
}

fn weights_section() -> impl IntoElement {
    section(
        "Weights",
        "The `font_*` helpers set font-weight.",
        Div()
            .style("flex flex-col gap-2")
            .child(
                Label()
                    .text("font_light — 300")
                    .style("font-light text-base text-foreground"),
            )
            .child(
                Label()
                    .text("font_normal — 400")
                    .style("font-normal text-base text-foreground"),
            )
            .child(
                Label()
                    .text("font_medium — 500")
                    .style("font-medium text-base text-foreground"),
            )
            .child(
                Label()
                    .text("font_semibold — 600")
                    .style("font-semibold text-base text-foreground"),
            )
            .child(
                Label()
                    .text("font_bold — 700")
                    .style("font-bold text-base text-foreground"),
            )
            .child(
                Label()
                    .text("font_extrabold — 800")
                    .style("font-extrabold text-base text-foreground"),
            ),
    )
}

fn theme_colours_section() -> impl IntoElement {
    section(
        "Theme colours",
        "Every theme colour is available via `text_*`.",
        Div()
            .style("flex flex-col gap-2")
            .child(Label().text("text_foreground").style("text-foreground"))
            .child(
                Label()
                    .text("text_muted_foreground")
                    .style("text-muted-foreground"),
            )
            .child(Label().text("text_primary").style("text-primary"))
            .child(
                Label()
                    .text("text_secondary_foreground")
                    .style("text-secondary-foreground"),
            )
            .child(Label().text("text_destructive").style("text-destructive")),
    )
}

fn inherited_colour_section() -> impl IntoElement {
    section(
        "Inherited colour",
        "A `Label` picks up the colour of whatever ancestor set it.",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-foreground")
                    .child(Label().text("text-foreground")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-primary")
                    .child(Label().text("text-primary")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-destructive")
                    .child(Label().text("text-destructive")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-muted-foreground")
                    .child(Label().text("text-muted-foreground")),
            ),
    )
}

fn explicit_override_section() -> impl IntoElement {
    section(
        "Explicit override",
        "A colour set on the label itself wins over the parent's cascade.",
        Div()
            .style("flex flex-row flex-wrap gap-3 text-foreground")
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted flex flex-col gap-1")
                    .child(Label().text("Parent: foreground").style("text-foreground"))
                    .child(
                        Label()
                            .text("Child: overridden to primary")
                            .style("text-primary"),
                    ),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted flex flex-col gap-1")
                    .child(Label().text("Parent: foreground").style("text-foreground"))
                    .child(
                        Label()
                            .text("Child: overridden to destructive")
                            .style("text-destructive"),
                    ),
            ),
    )
}

fn families_section() -> impl IntoElement {
    section(
        "Font families",
        "`font_sans`, `font_serif`, and `font_mono` set the font family.",
        Div()
            .style("flex flex-col gap-2")
            .child(
                Label()
                    .text("The quick brown fox — font_sans")
                    .style("font-sans text-base text-foreground"),
            )
            .child(
                Label()
                    .text("The quick brown fox — font_serif")
                    .style("font-serif text-base text-foreground"),
            )
            .child(
                Label()
                    .text("The quick brown fox — font_mono")
                    .style("font-mono text-base text-foreground"),
            ),
    )
}

fn slant_section() -> impl IntoElement {
    section(
        "Slant",
        "`italic` sets font-slant.",
        Div()
            .style("flex flex-col gap-2")
            .child(Label().text("Upright").style("text-base text-foreground"))
            .child(
                Label()
                    .text("Italic")
                    .style("text-base text-foreground italic"),
            ),
    )
}

fn alignment_section() -> impl IntoElement {
    section(
        "Alignment",
        "`text_left`, `text_center`, and `text_right` set text-align. Each label needs a bounded width for the alignment to be visible.",
        Div()
            .style("flex flex-col gap-2 w-full")
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Left-aligned text")
                        .style("text-left text-sm w-full"),
                ),
            )
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Centered text")
                        .style("text-center text-sm w-full"),
                ),
            )
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Right-aligned text")
                        .style("text-right text-sm w-full"),
                ),
            ),
    )
}

fn decoration_section() -> impl IntoElement {
    section(
        "Decoration",
        "`underline` and `line-through` set text-decoration.",
        Div()
            .style("flex flex-col gap-2")
            .child(
                Label()
                    .text("No decoration")
                    .style("text-base text-foreground"),
            )
            .child(
                Label()
                    .text("Underlined")
                    .style("text-base text-foreground underline"),
            )
            .child(
                Label()
                    .text("Struck through")
                    .style("text-base text-foreground line-through"),
            ),
    )
}

fn line_height_section() -> impl IntoElement {
    section(
        "Line height",
        "`leading_*` sets line-height. It only shows on multi-line text.",
        Div()
            .style("flex flex-col gap-3 max-w-md")
            .child(
                Div()
                    .style("flex flex-col gap-1 p-3 rounded-md bg-muted/40")
                    .child(
                        Label()
                            .text("leading-tight")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Label()
                            .text("The quick brown fox jumps over the lazy dog, then does it again and again to force wrapping.")
                            .style("text-sm text-foreground leading-tight"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-1 p-3 rounded-md bg-muted/40")
                    .child(
                        Label()
                            .text("leading-normal")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Label()
                            .text("The quick brown fox jumps over the lazy dog, then does it again and again to force wrapping.")
                            .style("text-sm text-foreground leading-normal"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-1 p-3 rounded-md bg-muted/40")
                    .child(
                        Label()
                            .text("leading-loose")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Label()
                            .text("The quick brown fox jumps over the lazy dog, then does it again and again to force wrapping.")
                            .style("text-sm text-foreground leading-loose"),
                    ),
            ),
    )
}

fn truncation_section() -> impl IntoElement {
    section(
        "Truncation",
        "`truncate` needs a bounded width — either on the label itself or on its parent.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Div()
                    .style("flex flex-col gap-1")
                    .child(
                        Label()
                            .text("label truncate, no width — no visible effect")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Label()
                            .text("This is a very long piece of text that would normally overflow its container if there were one.")
                            .style("text-sm text-foreground truncate"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-1")
                    .child(
                        Label()
                            .text("max_w_64 + truncate on the label")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Label()
                            .text("This is a very long piece of text and it gets cut off cleanly with an ellipsis thanks to max_w_64 on the label itself.")
                            .style("text-sm text-foreground truncate max-w-64"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-1")
                    .child(
                        Label()
                            .text("w-48 parent + truncate on the label")
                            .style("text-[10px] font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("w-48")
                            .child(
                                Label()
                                    .text("This is another long piece of text and the parent controls where it gets cut off.")
                                    .style("text-sm text-foreground truncate"),
                            ),
                    ),
            ),
    )
}

fn wrapping_section() -> impl IntoElement {
    section(
        "Wrapping",
        "Without `truncate`, long text wraps to the next line when its parent is bounded.",
        Div()
            .style("w-full max-w-sm p-3 rounded-md bg-muted/40")
            .child(
                Label()
                    .text("The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.")
                    .style("text-sm text-foreground"),
            ),
    )
}

fn in_composition_section() -> impl IntoElement {
    section(
        "In composition",
        "Labels inherit text styling from buttons, cards, and any other styled ancestor.",
        Div()
            .style("flex flex-col gap-4")
            // In buttons
            .child(
                Div()
                    .style("flex flex-row flex-wrap items-center gap-3")
                    .child(
                        Button()
                            .variant(ButtonVariant::Default)
                            .child(Label().text("Default"))
                            .into_element(),
                    )
                    .child(
                        Button()
                            .variant(ButtonVariant::Destructive)
                            .child(Label().text("Destructive"))
                            .into_element(),
                    )
                    .child(
                        Button()
                            .variant(ButtonVariant::Ghost)
                            .child(Label().text("Ghost"))
                            .into_element(),
                    ),
            )
            // In a card
            .child(
                Div()
                    .style("w-full max-w-sm rounded-xl border border-border bg-card overflow-hidden")
                    .child(
                        Div()
                            .style("p-4 flex flex-col gap-1")
                            .child(
                                Label()
                                    .text("Card title")
                                    .style("text-base font-semibold text-card-foreground"),
                            )
                            .child(
                                Label()
                                    .text("Supporting text in a muted colour, matching the card's foreground hierarchy.")
                                    .style("text-sm text-muted-foreground"),
                            ),
                    )
                    .child(divider())
                    .child(
                        Div()
                            .style("p-4 flex flex-col gap-2")
                            .child(
                                Div()
                                    .style("flex flex-row items-center justify-between")
                                    .child(Label().text("Plan"))
                                    .child(Label().text("Pro")),
                            )
                            .child(
                                Div()
                                    .style("flex flex-row items-center justify-between")
                                    .child(Label().text("Renews"))
                                    .child(
                                        Label()
                                            .text("2026-10-17")
                                            .style("text-muted-foreground"),
                                    ),
                            ),
                    ),
            ),
    )
}

/// A long list of labelled rows, used to make sure the page scrolls.
fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Many labelled rows — enough content to overflow the viewport and require scrolling.",
        Div()
            .style("flex flex-col gap-2")
            .children((0..40).map(|i| {
                Div()
                    .style("flex flex-row items-center justify-between p-3 rounded-md border border-border bg-card")
                    .child(Label().text(format!("Item {}", i + 1)))
                    .child(
                        Label()
                            .text("→")
                            .style("text-muted-foreground"),
                    )
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
    // `h-screen` gives it a definite height equal to the viewport, which
    // is what makes `overflow-scroll` actually scroll. Without a bounded
    // height the container grows to fit its content and no scrollbar
    // appears.
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
                                .text("Label")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("A text primitive that inherits everything it doesn't set.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                // Sections
                .child(basic_section())
                .child(sizes_section())
                .child(weights_section())
                .child(theme_colours_section())
                .child(inherited_colour_section())
                .child(explicit_override_section())
                .child(families_section())
                .child(slant_section())
                .child(alignment_section())
                .child(decoration_section())
                .child(line_height_section())
                .child(truncation_section())
                .child(wrapping_section())
                .child(in_composition_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
