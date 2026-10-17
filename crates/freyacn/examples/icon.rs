//! # Icon Showcase
//!
//! A runnable example demonstrating the `Icon` component: default
//! sizing, every size step, single-axis sizing, colour inheritance
//! from a parent, explicit colour overrides, hover states, icon
//! composition inside buttons, and a gallery of common outline SVGs —
//! all inside a scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example icon_showcase
//! ```

use freyacn::prelude::*;

// ==============================================================
// Demo assets — every SVG uses `currentColor` so the icon inherits
// its parent's text colour.
// ==============================================================

const CHECK_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>"##;

const PLUS_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>"##;

const TRASH_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/></svg>"##;

const HEART_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>"##;

const STAR_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>"##;

const SEARCH_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>"##;

const USER_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##;

const ARROW_RIGHT_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>"##;

const BELL_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>"##;

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

/// A labelled cell: an icon above a small monospace caption.
fn cell(caption: &str, icon: impl IntoElement) -> impl IntoElement {
    Div()
        .style("flex flex-col items-center gap-2 p-3 rounded-md border border-border bg-muted/40")
        .child(icon)
        .child(
            Label()
                .text(caption)
                .style("text-[10px] font-mono text-muted-foreground"),
        )
}

// ==============================================================
// Showcase sections
// ==============================================================

fn basic_section() -> impl IntoElement {
    section(
        "Basic",
        "The smallest possible usage — an SVG and nothing else. Defaults to 16×16.",
        Div()
            .style("flex flex-row flex-wrap items-center gap-4")
            .child(cell("check", Icon().svg(Bytes::from_static(CHECK_SVG))))
            .child(cell("plus", Icon().svg(Bytes::from_static(PLUS_SVG))))
            .child(cell("trash", Icon().svg(Bytes::from_static(TRASH_SVG))))
            .child(cell("heart", Icon().svg(Bytes::from_static(HEART_SVG))))
            .child(cell("star", Icon().svg(Bytes::from_static(STAR_SVG))))
            .child(cell("search", Icon().svg(Bytes::from_static(SEARCH_SVG)))),
    )
}

fn sizes_section() -> impl IntoElement {
    section(
        "Sizes",
        "`size_N()` sets both axes. Sizes are inherited from the FreyaCN spacing scale.",
        Div()
            .style("flex flex-col gap-6")
            .child(
                Div()
                    .style("flex flex-row flex-wrap items-end gap-4")
                    .child(cell(
                        "size_3",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_3(),
                    ))
                    .child(cell(
                        "size_4",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_4(),
                    ))
                    .child(cell(
                        "size_5",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_5(),
                    ))
                    .child(cell(
                        "size_6",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_6(),
                    ))
                    .child(cell(
                        "size_8",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_8(),
                    ))
                    .child(cell(
                        "size_10",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_10(),
                    ))
                    .child(cell(
                        "size_12",
                        Icon().svg(Bytes::from_static(HEART_SVG)).size_12(),
                    )),
            )
            .child(
                Div()
                    .style("flex flex-col gap-2")
                    .child(
                        Label()
                            .text("single-axis (mirrored automatically)")
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .child(
                        Div()
                            .style("flex flex-row flex-wrap items-end gap-4")
                            .child(cell("w_6", Icon().svg(Bytes::from_static(STAR_SVG)).w_6()))
                            .child(cell("h_6", Icon().svg(Bytes::from_static(STAR_SVG)).h_6()))
                            .child(cell(
                                "w_8 h_4",
                                Icon().svg(Bytes::from_static(STAR_SVG)).w_8().h_4(),
                            )),
                    ),
            ),
    )
}

fn colour_inheritance_section() -> impl IntoElement {
    section(
        "Colour inheritance",
        "Icons don't set a colour of their own — they inherit from the nearest ancestor that does.",
        Div()
            .style("flex flex-row flex-wrap gap-4")
            .child(
                Div()
                    .style("flex flex-col items-center gap-2 p-4 rounded-lg bg-muted text-foreground")
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_8())
                    .child(
                        Label()
                            .text("text-foreground")
                            .style("text-xs font-mono"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col items-center gap-2 p-4 rounded-lg bg-muted text-primary")
                    .child(Icon().svg(Bytes::from_static(HEART_SVG)).size_8())
                    .child(Label().text("text-primary").style("text-xs font-mono")),
            )
            .child(
                Div()
                    .style("flex flex-col items-center gap-2 p-4 rounded-lg bg-muted text-destructive")
                    .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_8())
                    .child(
                        Label()
                            .text("text-destructive")
                            .style("text-xs font-mono"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col items-center gap-2 p-4 rounded-lg bg-muted text-muted-foreground")
                    .child(Icon().svg(Bytes::from_static(STAR_SVG)).size_8())
                    .child(
                        Label()
                            .text("text-muted-foreground")
                            .style("text-xs font-mono"),
                    ),
            ),
    )
}

fn explicit_colour_section() -> impl IntoElement {
    section(
        "Explicit colours",
        "Setting a text colour on the icon itself wins over whatever the parent set.",
        Div()
            .style("flex flex-row flex-wrap gap-4 text-foreground")
            .child(cell(
                "default",
                Icon().svg(Bytes::from_static(HEART_SVG)).size_8(),
            ))
            .child(cell(
                "text_primary",
                Icon()
                    .svg(Bytes::from_static(HEART_SVG))
                    .size_8()
                    .text_primary(),
            ))
            .child(cell(
                "text_destructive",
                Icon()
                    .svg(Bytes::from_static(HEART_SVG))
                    .size_8()
                    .text_destructive(),
            ))
            .child(cell(
                "text_muted_foreground",
                Icon()
                    .svg(Bytes::from_static(HEART_SVG))
                    .size_8()
                    .text_muted_foreground(),
            )),
    )
}

fn hover_section() -> impl IntoElement {
    section(
        "Hover states",
        "The wrapper `Div` tracks hover, so `hover:` utilities on the icon work. Hover each cell directly.",
        Div()
            .style("flex flex-row flex-wrap gap-4")
            .child(cell(
                "hover:text-primary",
                Icon()
                    .svg(Bytes::from_static(HEART_SVG))
                    .size_8()
                    .style("hover:text-primary transition-colors cursor-pointer"),
            ))
            .child(cell(
                "hover:text-destructive",
                Icon()
                    .svg(Bytes::from_static(TRASH_SVG))
                    .size_8()
                    .style("hover:text-destructive transition-colors cursor-pointer"),
            ))
            .child(cell(
                "hover:scale-110",
                Icon()
                    .svg(Bytes::from_static(STAR_SVG))
                    .size_8()
                    .style("hover:scale-110 transition-transform cursor-pointer"),
            )),
    )
}

fn inside_buttons_section() -> impl IntoElement {
    section(
        "Inside buttons",
        "The icon inherits the button's foreground colour automatically — no extra styling needed.",
        Div()
            .style("flex flex-row flex-wrap items-center gap-3")
            .child(
                Button()
                    .variant(ButtonVariant::Default)
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                    .child(Label().text("Confirm"))
                    .into_element(),
            )
            .child(
                Button()
                    .variant(ButtonVariant::Secondary)
                    .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4())
                    .child(Label().text("New item"))
                    .into_element(),
            )
            .child(
                Button()
                    .variant(ButtonVariant::Destructive)
                    .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_4())
                    .child(Label().text("Delete"))
                    .into_element(),
            )
            .child(
                Button()
                    .variant(ButtonVariant::Outline)
                    .child(Icon().svg(Bytes::from_static(SEARCH_SVG)).size_4())
                    .child(Label().text("Search"))
                    .into_element(),
            ),
    )
}

fn gallery_section() -> impl IntoElement {
    const GALLERY: &[(&str, &[u8])] = &[
        ("check", CHECK_SVG),
        ("plus", PLUS_SVG),
        ("trash", TRASH_SVG),
        ("heart", HEART_SVG),
        ("star", STAR_SVG),
        ("search", SEARCH_SVG),
        ("user", USER_SVG),
        ("arrow", ARROW_RIGHT_SVG),
        ("bell", BELL_SVG),
    ];

    section(
        "Gallery",
        "A handful of common outline icons rendered at `size_8`, all using `currentColor`.",
        Div()
            .style("grid-cols-3 gap-3 w-full")
            .children(GALLERY.iter().map(|&(name, svg)| {
                Div()
                    .style("flex flex-col items-center gap-2 p-4 rounded-md bg-muted/40 border border-border text-foreground")
                    .child(Icon().svg(Bytes::from_static(svg)).size_8())
                    .child(
                        Label()
                            .text(name)
                            .style("text-xs font-mono text-muted-foreground"),
                    )
                    .into_element()
            })),
    )
}

fn composition_section() -> impl IntoElement {
    section(
        "Composition",
        "Real UI is just icons paired with text, laid out with `Div`s.",
        Div()
            .style("flex flex-col gap-4")
            // Card header with icon
            .child(
                Div()
                    .style("flex flex-row items-center gap-2 p-4 rounded-lg border border-border bg-card")
                    .child(
                        Icon()
                            .svg(Bytes::from_static(BELL_SVG))
                            .size_5()
                            .text_primary(),
                    )
                    .child(Label().text("Notifications").style("font-medium")),
            )
            // Toolbar
            .child(
                Div()
                    .style("flex flex-row items-center gap-1 p-2 rounded-lg border border-border bg-card w-fit")
                    .child(
                        Div()
                            .style("p-2 rounded-md hover:bg-muted cursor-pointer text-foreground")
                            .child(Icon().svg(Bytes::from_static(SEARCH_SVG)).size_4()),
                    )
                    .child(
                        Div()
                            .style("p-2 rounded-md hover:bg-muted cursor-pointer text-foreground")
                            .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_4()),
                    )
                    .child(
                        Div()
                            .style("p-2 rounded-md hover:bg-muted cursor-pointer text-destructive")
                            .child(Icon().svg(Bytes::from_static(TRASH_SVG)).size_4()),
                    ),
            )
            // Vertical navigation rail
            .child(
                Div()
                    .style("flex flex-col gap-1 p-2 rounded-lg border border-border bg-card w-56")
                    .child(
                        Div()
                            .style("flex flex-row items-center gap-3 p-2 rounded-md bg-primary text-primary-foreground")
                            .child(Icon().svg(Bytes::from_static(HEART_SVG)).size_4())
                            .child(Label().text("Favorites")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row items-center gap-3 p-2 rounded-md text-muted-foreground hover:bg-muted")
                            .child(Icon().svg(Bytes::from_static(STAR_SVG)).size_4())
                            .child(Label().text("Starred")),
                    )
                    .child(
                        Div()
                            .style("flex flex-row items-center gap-3 p-2 rounded-md text-muted-foreground hover:bg-muted")
                            .child(Icon().svg(Bytes::from_static(USER_SVG)).size_4())
                            .child(Label().text("Profile")),
                    ),
            ),
    )
}

fn density_section() -> impl IntoElement {
    section(
        "Density",
        "The same three icons rendered at three sizes, to show how icon scale tracks with the surrounding UI.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Div()
                    .style("flex flex-row items-center gap-2 p-2 rounded-md border border-border")
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_3())
                    .child(Label().text("Compact").style("text-xs")),
            )
            .child(
                Div()
                    .style("flex flex-row items-center gap-3 p-3 rounded-md border border-border")
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_4())
                    .child(Label().text("Default").style("text-sm")),
            )
            .child(
                Div()
                    .style("flex flex-row items-center gap-4 p-4 rounded-md border border-border")
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_6())
                    .child(Label().text("Comfortable").style("text-base")),
            ),
    )
}

/// A long row of icons, used to make sure the page scrolls.
fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Many icons stacked vertically — enough content to overflow the viewport and require scrolling.",
        Div()
            .style("flex flex-col gap-2")
            .children((0..40).map(|i| {
            Div()
                .style(
                    "flex flex-row items-center gap-3 p-2 rounded-md border border-border bg-card",
                )
                .child(
                    Icon()
                        .svg(Bytes::from_static(PLUS_SVG))
                        .size_4()
                        .text_primary(),
                )
                .child(Label().text(format!("Item {}", i + 1)))
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
                                .text("Icon")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Current-colour SVG icons that inherit their parent's text colour.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                // Sections
                .child(basic_section())
                .child(sizes_section())
                .child(colour_inheritance_section())
                .child(explicit_colour_section())
                .child(hover_section())
                .child(inside_buttons_section())
                .child(gallery_section())
                .child(composition_section())
                .child(density_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
