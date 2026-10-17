//! # Grid Showcase
//!
//! A runnable example demonstrating `Grid` and `GridItem`, plus the
//! `grid grid-cols-N` style-string form on a plain `Div`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example grid_showcase
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

/// A short numeric label used as a cell's content.
fn cell_label(text: &str) -> impl IntoElement {
    Label().text(text).style("text-sm font-medium")
}

// ==============================================================
// Cases
// ==============================================================

/// Baseline: three columns, no gap. Each cell is exactly `1/3` wide,
/// so three fit per row before wrapping.
fn three_columns_section() -> impl IntoElement {
    section(
        "Three columns, no gap",
        "A 3-column grid where every cell spans one column.",
        Grid()
            .cols(3)
            .style("w-full")
            .child(
                GridItem()
                    .style("p-4 bg-primary text-primary-foreground")
                    .child(cell_label("1")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-primary text-primary-foreground")
                    .child(cell_label("2")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-primary text-primary-foreground")
                    .child(cell_label("3")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("4")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("5")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("6")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-accent text-accent-foreground")
                    .child(cell_label("7")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-accent text-accent-foreground")
                    .child(cell_label("8")),
            ),
    )
}

/// Column spans. `span(2)` in a 3-column grid takes two-thirds of the
/// row, so a span-2 cell followed by a span-1 cell fills exactly one
/// row.
fn spans_section() -> impl IntoElement {
    section(
        "Column spans",
        "A span-2 cell takes 2/3 of the row; a span-3 cell takes the whole row.",
        Grid()
            .cols(3)
            .style("w-full")
            .child(
                GridItem()
                    .span(2)
                    .style("p-4 bg-primary text-primary-foreground")
                    .child(cell_label("span 2")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("span 1")),
            )
            .child(
                GridItem()
                    .span(3)
                    .style("p-4 bg-accent text-accent-foreground")
                    .child(cell_label("span 3")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("span 1")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("span 1")),
            )
            .child(
                GridItem()
                    .style("p-4 bg-secondary text-secondary-foreground")
                    .child(cell_label("span 1")),
            ),
    )
}

/// Gaps via padding on each cell. This is the Option A workaround: cell
/// widths stay exact at `100 / cols`, and the visual gap comes from the
/// cell's own padding plus a matching margin.
fn gaps_via_padding_section() -> impl IntoElement {
    section(
        "Gaps via padding",
        "Cell widths stay exact when the gap comes from the cell, not \
         the container.",
        Grid()
            .cols(3)
            .style("w-full")
            .child(
                GridItem().style("pr-2 pb-2").child(
                    Div()
                        .style("p-4 bg-primary text-primary-foreground rounded")
                        .child(cell_label("1")),
                ),
            )
            .child(
                GridItem().style("px-2 pb-2").child(
                    Div()
                        .style("p-4 bg-primary text-primary-foreground rounded")
                        .child(cell_label("2")),
                ),
            )
            .child(
                GridItem().style("pl-2 pb-2").child(
                    Div()
                        .style("p-4 bg-primary text-primary-foreground rounded")
                        .child(cell_label("3")),
                ),
            )
            .child(
                GridItem().style("pr-2 pb-2").child(
                    Div()
                        .style("p-4 bg-secondary text-secondary-foreground rounded")
                        .child(cell_label("4")),
                ),
            )
            .child(
                GridItem().style("px-2 pb-2").child(
                    Div()
                        .style("p-4 bg-secondary text-secondary-foreground rounded")
                        .child(cell_label("5")),
                ),
            )
            .child(
                GridItem().style("pl-2 pb-2").child(
                    Div()
                        .style("p-4 bg-secondary text-secondary-foreground rounded")
                        .child(cell_label("6")),
                ),
            ),
    )
}

/// The style-string form on a plain `Div`. Each plain child gets an
/// equal-width cell, but spans aren't expressible here — use `Grid` +
/// `GridItem` when you need them.
fn style_string_section() -> impl IntoElement {
    section(
        "Via style string",
        "`grid grid-cols-3` on a plain `Div`. Each child gets an equal cell.",
        Div()
            .style("grid grid-cols-3 w-full")
            .child(
                Div()
                    .style("p-4 bg-muted-foreground text-background")
                    .child(cell_label("A")),
            )
            .child(
                Div()
                    .style("p-4 bg-muted text-foreground")
                    .child(cell_label("B")),
            )
            .child(
                Div()
                    .style("p-4 bg-muted text-foreground")
                    .child(cell_label("C")),
            )
            .child(
                Div()
                    .style("p-4 bg-muted text-foreground")
                    .child(cell_label("D")),
            )
            .child(
                Div()
                    .style("p-4 bg-muted text-foreground")
                    .child(cell_label("E")),
            )
            .child(
                Div()
                    .style("p-4 bg-muted text-foreground")
                    .child(cell_label("F")),
            ),
    )
}

/// Different column counts, side by side, to check the math.
fn column_counts_section() -> impl IntoElement {
    section(
        "Column counts",
        "Two grids stacked, one with 2 columns and one with 4.",
        Div()
            .style("flex flex-col gap-4 w-full")
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("cols(2)")
                            .style("text-xs text-muted-foreground"),
                    )
                    .child(
                        Grid()
                            .cols(2)
                            .style("w-full")
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("1")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("2")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("3")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("4")),
                            ),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("cols(4)")
                            .style("text-xs text-muted-foreground"),
                    )
                    .child(
                        Grid()
                            .cols(4)
                            .style("w-full")
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("1")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("2")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("3")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-primary text-primary-foreground")
                                    .child(cell_label("4")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("5")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("6")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("7")),
                            )
                            .child(
                                GridItem()
                                    .style("p-3 bg-secondary text-secondary-foreground")
                                    .child(cell_label("8")),
                            ),
                    ),
            ),
    )
}

// ==============================================================
// App
// ==============================================================

fn app() -> impl IntoElement {
    use_init_cn_theme(Theme::dark().neutral().theme_color("neutral"));

    Div()
        .style("flex flex-col w-full h-screen bg-background overflow-scroll")
        .child(
            Div()
                .style("flex flex-col gap-6 max-w-3xl w-full mx-auto p-8")
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label()
                                .text("Grid")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Fixed-column layout via flex-wrap.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(three_columns_section())
                .child(spans_section())
                .child(gaps_via_padding_section())
                .child(style_string_section())
                .child(column_counts_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
