//! # Div Showcase
//!
//! A runnable example demonstrating everything the `Div` primitive
//! supports — flexbox layout, alignment, spacing, sizing, grid,
//! colours, radii, borders, shadows, opacity, transforms, positioning,
//! overflow, scrolling, interaction states, text overflow, layering,
//! and composition — inside a scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example div_showcase
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

/// A small labelled tile used to visualise layout behaviour.
/// Defaults are overridable by the `style` argument.
fn tile(label: &str, style: &str) -> Element {
    Div()
        .style(format!(
            "flex items-center justify-center rounded-md border border-border bg-muted text-xs text-muted-foreground {style}"
        ))
        .child(Label().text(label))
        .into_element()
}

/// A colour swatch: a coloured tile with a caption beneath it.
fn swatch(caption: &str, tile_style: &str) -> Element {
    Div()
        .style("flex flex-col gap-2 items-start")
        .child(Div().style(format!(
            "w-24 h-16 rounded-lg border border-border {tile_style}"
        )))
        .child(Label().text(caption).style("text-xs text-muted-foreground"))
        .into_element()
}

/// A row of three numbered tiles placed inside the given container style.
fn demo_row(label: &str, container_style: &str) -> impl IntoElement {
    Div()
        .style("flex flex-col gap-1")
        .child(
            Label()
                .text(label)
                .style("text-xs font-mono text-muted-foreground"),
        )
        .child(
            Div()
                .style(container_style)
                .child(tile("1", "w-10 h-6"))
                .child(tile("2", "w-10 h-10"))
                .child(tile("3", "w-10 h-8")),
        )
}

// ==============================================================
// Showcase sections
// ==============================================================

fn flex_direction_section() -> impl IntoElement {
    section(
        "Flex direction",
        "The `flex-*` utilities control the main axis and its direction.",
        Div()
            .style("flex flex-col gap-4")
            .child(demo_row("flex-row", "flex flex-row gap-2"))
            .child(demo_row("flex-row-reverse", "flex flex-row-reverse gap-2"))
            .child(demo_row("flex-col", "flex flex-col gap-2 items-start"))
            .child(demo_row(
                "flex-col-reverse",
                "flex flex-col-reverse gap-2 items-start",
            )),
    )
}

fn alignment_section() -> impl IntoElement {
    section(
        "Alignment",
        "`justify-*` distributes children along the main axis, `items-*` along the cross axis.",
        Div()
            .style("flex flex-col gap-4")
            .child(demo_row(
                "justify-start",
                "flex flex-row justify-start gap-2 w-full h-16 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "justify-center",
                "flex flex-row justify-center gap-2 w-full h-16 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "justify-end",
                "flex flex-row justify-end gap-2 w-full h-16 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "justify-between",
                "flex flex-row justify-between gap-2 w-full h-16 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "items-start",
                "flex flex-row items-start gap-2 w-full h-20 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "items-center",
                "flex flex-row items-center gap-2 w-full h-20 p-2 bg-muted rounded-md",
            ))
            .child(demo_row(
                "items-end",
                "flex flex-row items-end gap-2 w-full h-20 p-2 bg-muted rounded-md",
            )),
    )
}

fn spacing_section() -> impl IntoElement {
    section(
        "Spacing",
        "`gap-*` spaces children; `p-*` pads the inside of a box; `m-*` pushes it away from neighbours.",
        Div()
            .style("flex flex-col gap-6")
            // gap
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("gap")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-1")
                            .child(tile("g1", "w-10 h-10"))
                            .child(tile("g1", "w-10 h-10"))
                            .child(tile("g1", "w-10 h-10")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-2")
                            .child(tile("g2", "w-10 h-10"))
                            .child(tile("g2", "w-10 h-10"))
                            .child(tile("g2", "w-10 h-10")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-4")
                            .child(tile("g4", "w-10 h-10"))
                            .child(tile("g4", "w-10 h-10"))
                            .child(tile("g4", "w-10 h-10")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-8")
                            .child(tile("g8", "w-10 h-10"))
                            .child(tile("g8", "w-10 h-10"))
                            .child(tile("g8", "w-10 h-10")),
                    ),
            )
            // padding
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("padding")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-3 items-start")
                            .child(
                                Div()
                                    .style("p-2 bg-muted rounded-md")
                                    .child(tile("p-2", "w-16 h-8")),
                            )
                            .child(
                                Div()
                                    .style("p-4 bg-muted rounded-md")
                                    .child(tile("p-4", "w-16 h-8")),
                            )
                            .child(
                                Div()
                                    .style("p-6 bg-muted rounded-md")
                                    .child(tile("p-6", "w-16 h-8")),
                            )
                            .child(
                                Div()
                                    .style("px-6 py-2 bg-muted rounded-md")
                                    .child(tile("px-6 py-2", "w-16 h-8")),
                            ),
                    ),
            )
            // margin
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("margin")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row gap-4 p-3 bg-muted rounded-md items-start")
                            .child(tile("m-0", "w-14 h-14 m-0"))
                            .child(tile("m-2", "w-14 h-14 m-2"))
                            .child(tile("m-4", "w-14 h-14 m-4")),
                    ),
            ),
    )
}

fn sizing_section() -> impl IntoElement {
    section(
        "Sizing",
        "`w-*`, `h-*`, `size-*`, `min-*`, and `max-*` control the box model.",
        Div()
            .style("flex flex-col gap-6")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("fixed")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row flex-wrap items-end gap-3")
                            .child(tile("w-16", "w-16 h-16"))
                            .child(tile("w-24", "w-24 h-16"))
                            .child(tile("w-32", "w-32 h-16"))
                            .child(tile("size-16", "size-16")),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("full width")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(tile("w-full", "w-full h-10")),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("max width")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("w-full max-w-xs bg-muted rounded-md p-3")
                            .child(
                                Label()
                                    .text("Capped at max-w-xs")
                                    .style("text-xs text-muted-foreground"),
                            ),
                    ),
            ),
    )
}

fn grid_section() -> impl IntoElement {
    section(
        "Grid",
        "`grid-cols-*` splits children into equal-width columns that wrap.",
        Div()
            .style("grid-cols-3 gap-2 w-full")
            .children((1..=9).map(|i| {
                Div()
                    .style("flex items-center justify-center h-12 rounded-md bg-muted border border-border text-xs text-muted-foreground")
                    .child(Label().text(format!("{i}")))
                    .into_element()
            })),
    )
}

fn colours_section() -> impl IntoElement {
    section(
        "Colours",
        "Theme colours adapt to the active light/dark theme.",
        Div()
            .style("flex flex-col gap-6")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("backgrounds")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row flex-wrap gap-3")
                            .child(swatch("background", "bg-background"))
                            .child(swatch("card", "bg-card"))
                            .child(swatch("muted", "bg-muted"))
                            .child(swatch("primary", "bg-primary"))
                            .child(swatch("secondary", "bg-secondary"))
                            .child(swatch("accent", "bg-accent"))
                            .child(swatch("destructive", "bg-destructive")),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("text colours")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row flex-wrap gap-4")
                            .child(Label().text("foreground").style("text-foreground"))
                            .child(
                                Label()
                                    .text("muted-foreground")
                                    .style("text-muted-foreground"),
                            )
                            .child(Label().text("primary").style("text-primary"))
                            .child(Label().text("destructive").style("text-destructive")),
                    ),
            ),
    )
}

fn radius_section() -> impl IntoElement {
    section(
        "Corner radius",
        "`rounded-*` sets the corner radius on a box.",
        Div()
            .style("flex flex-row flex-wrap gap-3 items-end")
            .child(tile("none", "w-16 h-16 rounded-none"))
            .child(tile("sm", "w-16 h-16 rounded-sm"))
            .child(tile("md", "w-16 h-16 rounded-md"))
            .child(tile("lg", "w-16 h-16 rounded-lg"))
            .child(tile("xl", "w-16 h-16 rounded-xl"))
            .child(tile("2xl", "w-16 h-16 rounded-2xl"))
            .child(tile("full", "w-16 h-16 rounded-full")),
    )
}

fn border_section() -> impl IntoElement {
    section(
        "Borders",
        "Border width, colour, and style.",
        Div()
            .style("flex flex-row flex-wrap gap-3 items-end")
            .child(tile("border", "w-20 h-16 border"))
            .child(tile("border-2", "w-20 h-16 border-2"))
            .child(tile("border-4", "w-20 h-16 border-4"))
            .child(tile("primary", "w-20 h-16 border-2 border-primary"))
            .child(tile("destructive", "w-20 h-16 border-2 border-destructive"))
            .child(tile("dashed", "w-20 h-16 border-2 border-dashed")),
    )
}

fn shadow_section() -> impl IntoElement {
    section(
        "Shadows",
        "`shadow-*` sets the box shadow. Colours follow the theme.",
        Div()
            .style("flex flex-row flex-wrap gap-4 p-4")
            .child(tile("shadow-sm", "w-20 h-16 bg-card shadow-sm"))
            .child(tile("shadow", "w-20 h-16 bg-card shadow"))
            .child(tile("shadow-md", "w-20 h-16 bg-card shadow-md"))
            .child(tile("shadow-lg", "w-20 h-16 bg-card shadow-lg"))
            .child(tile("shadow-xl", "w-20 h-16 bg-card shadow-xl"))
            .child(tile("shadow-2xl", "w-20 h-16 bg-card shadow-2xl")),
    )
}

fn opacity_section() -> impl IntoElement {
    section(
        "Opacity",
        "`opacity-*` ranges from 0 to 100.",
        Div()
            .style("flex flex-row flex-wrap gap-3 items-end")
            .child(tile("100", "w-16 h-16 bg-primary text-white opacity-100"))
            .child(tile("75", "w-16 h-16 bg-primary text-white opacity-75"))
            .child(tile("50", "w-16 h-16 bg-primary text-white opacity-50"))
            .child(tile("25", "w-16 h-16 bg-primary text-white opacity-25"))
            .child(tile("10", "w-16 h-16 bg-primary text-white opacity-10")),
    )
}

fn transforms_section() -> impl IntoElement {
    section(
        "Transforms",
        "`rotate-*`, `scale-*`, `translate-*`, and `blur-*` apply GPU transforms without affecting layout.",
        Div()
            .style("flex flex-row flex-wrap items-center gap-10 p-8")
            .child(tile(
                "rotate-45",
                "w-16 h-16 bg-primary text-white rotate-45",
            ))
            .child(tile(
                "scale-110",
                "w-16 h-16 bg-secondary text-secondary-foreground scale-110",
            ))
            .child(tile("-translate-y-2", "w-16 h-16 bg-accent -translate-y-2"))
            .child(tile("blur-sm", "w-16 h-16 bg-muted blur-sm")),
    )
}

fn positioning_section() -> impl IntoElement {
    section(
        "Positioning",
        "A `relative` parent with `absolute` children.",
        Div()
            .style("relative w-full h-56 bg-muted rounded-lg overflow-hidden")
            .child(
                Div()
                    .style("absolute top-4 left-4 px-3 py-1 rounded-md bg-primary text-primary-foreground text-xs")
                    .child(Label().text("top-4 left-4")),
            )
            .child(
                Div()
                    .style("absolute top-4 right-4 px-3 py-1 rounded-md bg-secondary text-secondary-foreground text-xs")
                    .child(Label().text("top-4 right-4")),
            )
            .child(
                Div()
                    .style("absolute bottom-4 left-4 px-3 py-1 rounded-md bg-destructive text-white text-xs")
                    .child(Label().text("bottom-4 left-4")),
            )
            .child(
                Div()
                    .style("absolute bottom-4 right-4 px-3 py-1 rounded-md bg-accent text-xs")
                    .child(Label().text("bottom-4 right-4")),
            )
            .child(
                Div()
                    .style("absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 px-3 py-1 rounded-md bg-card border border-border text-xs")
                    .child(Label().text("centered")),
            ),
    )
}

fn overflow_section() -> impl IntoElement {
    section(
        "Overflow",
        "`overflow-hidden` clips content that doesn't fit inside the box.",
        Div()
            .style("flex flex-row flex-wrap gap-4")
            .child(
                Div()
                    .style("w-56 h-24 rounded-md bg-muted overflow-hidden p-3")
                    .child(
                        Label()
                            .text("This content is much longer than the container and is clipped by overflow-hidden.")
                            .style("text-xs text-muted-foreground"),
                    ),
            )
            .child(
                Div()
                    .style("w-56 h-24 rounded-md bg-muted p-3 overflow-hidden")
                    .child(
                        Label()
                            .text("Same content, also clipped, showing a different crop.")
                            .style("text-xs text-muted-foreground"),
                    ),
            ),
    )
}

fn scrolling_section() -> impl IntoElement {
    section(
        "Scrolling",
        "`overflow-scroll` turns a `Div` into a scroll view. The element needs a bounded height for the scrollbar to appear.",
        Div().style("flex flex-row gap-4 w-full").child(
            Div()
                .style("flex flex-col gap-2 w-full h-48 overflow-scroll p-3 bg-muted rounded-md")
                .children((1..=20).map(|i| {
                    Div()
                        .style(
                            "p-2 rounded-md bg-card border border-border text-xs text-foreground",
                        )
                        .child(Label().text(format!("Row {i}")))
                        .into_element()
                })),
        ),
    )
}

fn interaction_section() -> impl IntoElement {
    let mut count = use_state(|| 0i32);
    let displayed = *count.read();

    section(
        "Interaction",
        "`Div` tracks hover, press, and focus; `on_hover` fires on pointer-enter. `disabled(true)` suppresses both.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Div()
                    .style("p-6 rounded-lg border border-border bg-card text-center transition-all hover:bg-muted cursor-pointer select-none")
                    .on_hover(move |_| {
                        let n = *count.read();
                        count.set(n + 1);
                    })
                    .child(
                        Label()
                            .text("Hover me")
                            .style("text-sm text-foreground"),
                    ),
            )
            .child(
                Label()
                    .text(format!("Hover count: {displayed}"))
                    .style("text-xs text-muted-foreground"),
            )
            .child(
                Div()
                    .style("p-6 rounded-lg border border-border bg-muted text-center opacity-50")
                    .disabled(true)
                    .child(
                        Label()
                            .text("Disabled — hover and press are suppressed")
                            .style("text-sm text-muted-foreground"),
                    ),
            ),
    )
}

fn text_overflow_section() -> impl IntoElement {
    section(
        "Text overflow",
        "`truncate` clips long text with an ellipsis at the container's edge.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Div()
                    .style("w-64 truncate text-sm text-foreground")
                    .child(Label().text(
                    "This is a very long piece of text that will be truncated with an ellipsis.",
                )),
            )
            .child(
                Div()
                    .style("w-64 truncate text-sm text-foreground")
                    .child(Label().text("Short text")),
            ),
    )
}

fn layering_section() -> impl IntoElement {
    section(
        "Layering",
        "`z-*` controls stacking order between siblings.",
        Div()
            .style("relative w-full h-40")
            .child(
                Div()
                    .style("absolute top-0 left-0 w-40 h-24 rounded-lg bg-primary z-10 p-2")
                    .child(Label().text("z-10").style("text-white text-xs")),
            )
            .child(
                Div()
                    .style("absolute top-8 left-16 w-40 h-24 rounded-lg bg-secondary z-20 p-2")
                    .child(Label().text("z-20").style("text-xs")),
            )
            .child(
                Div()
                    .style("absolute top-16 left-32 w-40 h-24 rounded-lg bg-accent z-30 p-2")
                    .child(Label().text("z-30").style("text-xs")),
            ),
    )
}

fn composition_section() -> impl IntoElement {
    section(
        "Composition",
        "Real layouts are just nested `Div`s.",
        Div()
            .style("w-full max-w-sm rounded-xl border border-border bg-card shadow-sm overflow-hidden")
            .child(
                Div()
                    .style("p-4 border-b border-border")
                    .child(
                        Label()
                            .text("Card title")
                            .style("text-base font-semibold text-card-foreground"),
                    ),
            )
            .child(
                Div()
                    .style("p-4 flex flex-col gap-3")
                    .child(
                        Label()
                            .text("Nested `Div`s let you build arbitrary layouts — headers, footers, sidebars, toolbars, and more.")
                            .style("text-sm text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row justify-end gap-2 pt-2")
                            .child(
                                Div()
                                    .style("px-3 py-1 rounded-md bg-secondary text-secondary-foreground text-xs")
                                    .child(Label().text("Cancel")),
                            )
                            .child(
                                Div()
                                    .style("px-3 py-1 rounded-md bg-primary text-primary-foreground text-xs")
                                    .child(Label().text("Confirm")),
                            ),
                    ),
            ),
    )
}

/// A long list of rows, used to make sure the page scrolls.
fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Many rows stacked vertically — enough content to overflow the viewport and require scrolling.",
        Div()
            .style("flex flex-col gap-2")
            .children((0..40).map(|i| {
                Div()
                    .style("flex flex-row items-center justify-between p-3 rounded-md border border-border bg-card")
                    .child(Label().text(format!("Item {}", i + 1)))
                    .child(Label().text("→").style("text-muted-foreground"))
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
                                .text("Div")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("The styled primitive that powers every other component.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                // Sections
                .child(flex_direction_section())
                .child(alignment_section())
                .child(spacing_section())
                .child(sizing_section())
                .child(grid_section())
                .child(colours_section())
                .child(radius_section())
                .child(border_section())
                .child(shadow_section())
                .child(opacity_section())
                .child(transforms_section())
                .child(positioning_section())
                .child(overflow_section())
                .child(scrolling_section())
                .child(interaction_section())
                .child(text_overflow_section())
                .child(layering_section())
                .child(divider())
                .child(composition_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
