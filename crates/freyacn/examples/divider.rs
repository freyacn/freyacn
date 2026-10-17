//! # Divider Showcase
//!
//! A runnable example demonstrating the `Divider` component: both
//! orientations, colour and thickness overrides, dividers inside
//! lists, toolbars, and cards, a "divider with label" composition, and
//! interactive usage — all inside a scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example divider_showcase
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

/// A small chip used to show what's happening.
fn chip(text: &str) -> impl IntoElement {
    Div()
        .style("px-2 py-1 rounded-md bg-muted text-xs text-muted-foreground")
        .child(Label().text(text))
}

// ==============================================================
// Showcase sections
// ==============================================================

fn basic_section() -> impl IntoElement {
    section(
        "Basic",
        "The default orientation is Horizontal. It spans the full width of its parent.",
        Div()
            .style("flex flex-col gap-3 w-full")
            .child(Label().text("Above"))
            .child(Divider())
            .child(Label().text("Below")),
    )
}

fn orientation_section() -> impl IntoElement {
    section(
        "Orientation",
        "Horizontal spans the width; Vertical spans the height of its parent. Vertical dividers need a bounded parent height to be visible.",
        Div()
            .style("flex flex-col gap-6")
            // Horizontal
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Horizontal")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-col gap-3 p-3 rounded-md bg-muted/40")
                            .child(Label().text("First"))
                            .child(Divider())
                            .child(Label().text("Second"))
                            .child(Divider())
                            .child(Label().text("Third")),
                    ),
            )
            // Vertical
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Vertical")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style(
                                "flex flex-row items-center gap-4 h-10 px-3 rounded-md bg-muted/40",
                            )
                            .child(Label().text("Left"))
                            .child(Divider().orientation(DividerOrientation::Vertical))
                            .child(Label().text("Middle"))
                            .child(Divider().orientation(DividerOrientation::Vertical))
                            .child(Label().text("Right")),
                    ),
            ),
    )
}

fn colour_section() -> impl IntoElement {
    section(
        "Colour",
        "The divider defaults to `bg-border`; a `bg_*` helper on the component overrides it.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("default (border)")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider()),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("primary")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().bg_primary()),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("secondary")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().bg_secondary()),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("destructive")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().bg_destructive()),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("muted-foreground")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().bg_muted_foreground()),
            ),
    )
}

fn thickness_section() -> impl IntoElement {
    section(
        "Thickness",
        "Override `h-*` on a horizontal divider (or `w-*` on a vertical one) to make it thicker.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("h-0.5 (default)")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider()),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("h-0.5")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().style("h-0.5")),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("h-1")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().style("h-1")),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("h-2 rounded-full bg-primary")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().style("h-2 rounded-full bg-primary")),
            ),
    )
}

fn with_label_section() -> impl IntoElement {
    section(
        "With label",
        "Pass `label(...)` to centre a caption on the divider line.",
        Div()
            .style("flex flex-col gap-6")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Short label")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().label("OR")),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Longer label")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().label("Continue with email")),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Section header")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().label("Section 2")),
            )
            // Composition still works if you need custom label styling
            // — the trick is giving each `Divider` `flex-1 w-auto` so
            // it shrinks to share the row with the label.
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Manual composition (custom label style)")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row items-center gap-3 w-full")
                            .child(Divider().style("flex-1 w-auto"))
                            .child(
                                Label()
                                    .text("Custom")
                                    .style("text-sm font-semibold text-primary shrink-0"),
                            )
                            .child(Divider().style("flex-1 w-auto")),
                    ),
            ),
    )
}

fn in_a_list_section() -> impl IntoElement {
    let items = [
        ("Inbox", "12"),
        ("Drafts", "3"),
        ("Sent", "—"),
        ("Archive", "—"),
    ];

    section(
        "In a list",
        "A common use: splitting menu items with a hairline rule.",
        Div()
            .style("flex flex-col rounded-lg border border-border bg-card overflow-hidden")
            .children(items.iter().enumerate().flat_map(|(i, (name, count))| {
                let mut row: Vec<Element> = Vec::new();
                if i > 0 {
                    row.push(Divider().into_element());
                }
                row.push(
                    Div()
                        .style("flex flex-row items-center justify-between px-4 py-3")
                        .child(Label().text(*name))
                        .child(Label().text(*count).style("text-xs text-muted-foreground"))
                        .into_element(),
                );
                row
            })),
    )
}

fn in_a_toolbar_section() -> impl IntoElement {
    section(
        "In a toolbar",
        "Vertical dividers split groups of controls in a horizontal row.",
        Div()
            .style("flex flex-row items-center gap-1 p-1 rounded-lg border border-border bg-card w-fit h-10")
            .child(chip("Bold"))
            .child(chip("Italic"))
            .child(chip("Underline"))
            .child(Divider().orientation(DividerOrientation::Vertical))
            .child(chip("Left"))
            .child(chip("Center"))
            .child(chip("Right"))
            .child(Divider().orientation(DividerOrientation::Vertical))
            .child(chip("Link")),
    )
}

fn in_a_card_section() -> impl IntoElement {
    section(
        "In a card",
        "Dividers split a card into header, body, and footer without any extra borders.",
        Div()
            .style("w-full max-w-sm rounded-xl border border-border bg-card overflow-hidden")
            .child(
                Div()
                    .style("p-4 flex flex-col gap-1")
                    .child(
                        Label()
                            .text("Account")
                            .style("text-base font-semibold text-card-foreground"),
                    )
                    .child(
                        Label()
                            .text("Manage your plan and billing.")
                            .style("text-sm text-muted-foreground"),
                    ),
            )
            .child(Divider())
            .child(
                Div()
                    .style("p-4 flex flex-col gap-3")
                    .child(
                        Div()
                            .style("flex flex-row items-center justify-between")
                            .child(Label().text("Plan"))
                            .child(Label().text("Pro").style("text-sm font-medium")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row items-center justify-between")
                            .child(Label().text("Renews"))
                            .child(
                                Label()
                                    .text("2026-10-17")
                                    .style("text-sm text-muted-foreground"),
                            ),
                    ),
            )
            .child(Divider())
            .child(
                Div()
                    .style("p-4 flex flex-row justify-end gap-2 bg-muted/40")
                    .child(
                        Div()
                            .style("px-3 py-1 rounded-md bg-secondary text-secondary-foreground text-xs")
                            .child(Label().text("Cancel")),
                    )
                    .child(
                        Div()
                            .style("px-3 py-1 rounded-md bg-primary text-primary-foreground text-xs")
                            .child(Label().text("Update")),
                    ),
            ),
    )
}

fn spacing_section() -> impl IntoElement {
    section(
        "Spacing",
        "A divider is a `Div`, so margin and padding helpers all work.",
        Div()
            .style("flex flex-col w-full")
            .child(Label().text("No margin"))
            .child(Divider())
            .child(Label().text("my_2"))
            .child(Divider().my_2())
            .child(Label().text("my_4"))
            .child(Divider().my_4())
            .child(Label().text("my_8"))
            .child(Divider().my_8())
            .child(Label().text("End")),
    )
}

fn gradient_section() -> impl IntoElement {
    section(
        "Gradient",
        "Arbitrary style strings work: a divider can fade out at the edges.",
        Div()
            .style("flex flex-col gap-6")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Faded at both ends")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().style(
                        "h-px w-full bg-gradient-to-r from-transparent via-border to-transparent",
                    )),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("Primary, faded")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(Divider().style(
                        "h-px w-full bg-gradient-to-r from-transparent via-primary to-transparent",
                    )),
            ),
    )
}

/// A long list of rows separated by dividers, used to make sure the
/// page scrolls.
fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Many rows separated by dividers — enough content to overflow the viewport.",
        Div()
            .style("flex flex-col rounded-lg border border-border bg-card overflow-hidden")
            .children((0..40).flat_map(|i| {
                let mut row: Vec<Element> = Vec::new();
                if i > 0 {
                    row.push(Divider().into_element());
                }
                row.push(
                    Div()
                        .style("flex flex-row items-center justify-between px-4 py-2")
                        .child(Label().text(format!("Item {}", i + 1)))
                        .child(Label().text("→").style("text-muted-foreground"))
                        .into_element(),
                );
                row
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
                                .text("Divider")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Horizontal and vertical rules for splitting content.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                // Sections
                .child(basic_section())
                .child(orientation_section())
                .child(colour_section())
                .child(thickness_section())
                .child(with_label_section())
                .child(in_a_list_section())
                .child(in_a_toolbar_section())
                .child(in_a_card_section())
                .child(spacing_section())
                .child(gradient_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
