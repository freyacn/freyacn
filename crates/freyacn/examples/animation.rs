//! # Animation & Selectors Showcase
//!
//! Demonstrates the animation system (`animate-*`), structural
//! selectors (`first:`, `last:`, `odd:`, `even:`, `only:`),
//! negation (`not-*`), and `Group`-based `group-hover:` /
//! `has-hover:` all in one scrollable page.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example animation_showcase
//! ```

use freyacn::prelude::*;

// ==============================================================
// Demo assets
// ==============================================================

const PLUS_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>"##;

const CHECK_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>"##;

const ARROW_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>"##;

// ==============================================================
// Helpers
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
    // `h-px` doesn't parse (`parse_size` requires a value before `px`);
    // `h-0.5` resolves to 2px and is visible.
    Div().style("h-0.5 w-full bg-border")
}

// ==============================================================
// Sections
// ==============================================================

fn spin_section() -> impl IntoElement {
    section(
        "animate-spin",
        "The whole element rotates. Icons make the rotation visible: the + passes through × on the way round.",
        Div()
            .style("flex flex-row items-center gap-6")
            // Ring spinner — the border stays circular, but the
            // centred icon visibly rotates through + → × → +.
            .child(
                Div()
                    .style("flex items-center justify-center size-12 rounded-full border-4 border-primary animate-spin")
                    .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_5()),
            )
            // Filled spinner — icon in the foreground colour.
            .child(
                Div()
                    .style("flex items-center justify-center size-12 rounded-full bg-primary text-primary-foreground animate-spin")
                    .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_5()),
            )
            // Arrow spinner — most visible, since the arrow is fully
            // asymmetric.
            .child(
                Div()
                    .style("flex items-center justify-center size-12 rounded-full bg-muted text-primary animate-spin")
                    .child(Icon().svg(Bytes::from_static(ARROW_SVG)).size_5()),
            ),
    )
}

fn pulse_section() -> impl IntoElement {
    section(
        "animate-pulse",
        "Opacity fades between 0.5 and 1.0, in and out.",
        Div()
            .style("flex flex-row items-center gap-6")
            .child(
                Div()
                    .style("size-12 rounded-md bg-primary animate-pulse"),
            )
            .child(
                Div()
                    .style("flex flex-row items-center gap-2 p-3 rounded-md border border-border animate-pulse")
                    .child(Div().style("size-8 rounded-full bg-muted"))
                    .child(
                        Div()
                            .style("flex flex-col gap-1")
                            .child(Div().style("w-24 h-2 rounded bg-muted"))
                            .child(Div().style("w-16 h-2 rounded bg-muted")),
                    ),
            ),
    )
}

fn ping_section() -> impl IntoElement {
    // `inset-0` doesn't parse (`parse_size` rejects a bare `px` token),
    // so the ping elements are sized with `size-12` and centred with
    // flex instead. The wrapper is `relative` and the same size, so the
    // absolute children stack on top of it at 0,0.
    section(
        "animate-ping",
        "Scale up and fade out — classic notification badge ripple.",
        Div()
            .style("flex flex-row items-center gap-6 text-foreground")
            .child(
                Div()
                    .style("relative size-12")
                    // Expanding, fading ring.
                    .child(Div().style("absolute size-12 rounded-full bg-primary animate-ping"))
                    // Solid dot at the centre.
                    .child(
                        Div()
                            .style("absolute size-12 flex flex-row items-center justify-center")
                            .child(Div().style("size-6 rounded-full bg-primary")),
                    ),
            )
            .child(
                Div()
                    .style("relative size-12")
                    .child(Div().style("absolute size-12 rounded-full bg-destructive animate-ping"))
                    .child(
                        Div()
                            .style("absolute size-12 flex flex-row items-center justify-center")
                            .child(Div().style("size-6 rounded-full bg-destructive")),
                    ),
            ),
    )
}

fn bounce_section() -> impl IntoElement {
    // A thin rule beneath each bouncing element acts as the "floor" —
    // without it the up-and-down translate reads as noise. The bouncing
    // element genuinely translates (the animation wraps it in a
    // Size::auto() rect with `offset_y`), so with a reference line the
    // motion reads unambiguously as a bounce.
    section(
        "animate-bounce",
        "Vertical oscillation — the element genuinely translates up and down against the floor line.",
        Div()
            .style("flex flex-row items-end gap-10 p-4 h-32")
            .child(
                Div()
                    .style("flex flex-col items-center gap-3")
                    .child(Div().style("size-8 rounded-full bg-primary animate-bounce"))
                    .child(Div().style("w-12 h-0.5 bg-border")),
            )
            .child(
                Div()
                    .style("flex flex-col items-center gap-3")
                    .child(Div().style("size-8 rounded-md bg-secondary animate-bounce"))
                    .child(Div().style("w-12 h-0.5 bg-border")),
            )
            .child(
                Div()
                    .style("flex flex-col items-center gap-3")
                    .child(
                        Div()
                            .style("flex items-center justify-center size-10 rounded-full bg-card border border-border animate-bounce")
                            .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_5()),
                    )
                    .child(Div().style("w-12 h-0.5 bg-border")),
            ),
    )
}

fn first_last_section() -> impl IntoElement {
    section(
        "first: / last: / only:",
        "Structural selectors fire automatically based on sibling position.",
        Div()
            .style("flex flex-col gap-2")
            .child(
                Div()
                    .style("flex flex-col gap-1 rounded-md overflow-hidden border border-border")
                    .child(
                        Div()
                            .style("px-3 py-2 bg-muted first:bg-primary first:text-primary-foreground first:rounded-t-md")
                            .child(Label().text("First row — receives first:")),
                    )
                    .child(
                        Div()
                            .style("px-3 py-2 bg-muted")
                            .child(Label().text("Middle row — plain")),
                    )
                    .child(
                        Div()
                            .style("px-3 py-2 bg-muted last:bg-destructive last:text-white last:rounded-b-md")
                            .child(Label().text("Last row — receives last:")),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-row gap-2 mt-4")
                    .child(
                        Div()
                            .style("flex-1 p-2 rounded-md bg-muted only:bg-primary only:text-primary-foreground text-xs text-center")
                            .child(Label().text("Only child")),
                    ),
            ),
    )
}

fn odd_even_section() -> impl IntoElement {
    section(
        "odd: / even:",
        "Alternating row striping, exactly like a table.",
        Div()
            .style("flex flex-col rounded-md border border-border overflow-hidden")
            .children((0..8).map(|_| {
                Div()
                    .style("px-3 py-2 text-sm odd:bg-card even:bg-muted first:rounded-t-md last:rounded-b-md")
                    .child(Label().text("Row"))
                    .into_element()
            })),
    )
}

fn not_hover_section() -> impl IntoElement {
    section(
        "not-hover:",
        "The inverse of hover. (Note: `transition-colors` is parsed but inert — see the module docs.)",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-md bg-primary text-primary-foreground hover:bg-primary/80 not-hover:bg-muted not-hover:text-muted-foreground")
                    .child(Label().text("Hover me")),
            )
            .child(
                Div()
                    .style("p-4 rounded-md border border-border not-hover:text-muted-foreground hover:border-primary hover:text-primary")
                    .child(Label().text("Hover me too")),
            ),
    )
}

fn group_hover_section() -> impl IntoElement {
    section(
        "group-hover:",
        "Wrap the row in `Group()`. Any descendant with `group-hover:` reacts when the group is hovered — even if the hover is over a sibling.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Group()
                    .style("flex flex-row items-center gap-3 p-4 rounded-lg border border-border bg-card cursor-pointer")
                    .child(
                        Div()
                            .style("flex items-center justify-center size-10 rounded-md bg-muted group-hover:bg-primary group-hover:text-primary-foreground")
                            .child(Icon().svg(Bytes::from_static(PLUS_SVG)).size_5()),
                    )
                    .child(
                        Div()
                            .style("flex flex-col flex-1")
                            .child(
                                Label()
                                    .text("Add a teammate")
                                    .style("text-sm font-medium group-hover:text-primary"),
                            )
                            .child(
                                Label()
                                    .text("Hover anywhere in this row")
                                    .style("text-xs text-muted-foreground"),
                            ),
                    )
                    // The arrow is wrapped in a `Div` so it can use
                    // `group-hover:scale-*` — `Label` doesn't forward
                    // `scale` or `margin` to its primitive, but `Div`
                    // does.
                    .child(
                        Div()
                            .style("text-muted-foreground group-hover:text-primary group-hover:scale-125")
                            .child(Label().text("→")),
                    ),
            )
            .child(
                Group()
                    .style("flex flex-row items-center gap-3 p-4 rounded-lg border border-border bg-card cursor-pointer")
                    .child(
                        Div()
                            .style("flex items-center justify-center size-10 rounded-md bg-muted group-hover:bg-destructive group-hover:text-white")
                            .child(Icon().svg(Bytes::from_static(CHECK_SVG)).size_5()),
                    )
                    .child(
                        Label()
                            .text("Confirm selection")
                            .style("text-sm font-medium group-hover:text-destructive"),
                    ),
            ),
    )
}

fn has_hover_section() -> impl IntoElement {
    section(
        "has-hover:",
        "Aliased to `group-hover:` — same effect, different name.",
        Group()
            .style("flex flex-row items-center gap-3 p-4 rounded-lg border border-border bg-card")
            .child(Div().style("size-10 rounded-full bg-muted has-hover:bg-primary"))
            .child(
                Label()
                    .text("The dot lights up when this card is hovered.")
                    .style("text-sm has-hover:text-primary"),
            ),
    )
}

fn combo_section() -> impl IntoElement {
    section(
        "Combining",
        "Structural + interactive + negation can be chained.",
        Div()
            .style("flex flex-col rounded-md border border-border overflow-hidden")
            .children((0..5).map(|_| {
                Div()
                    .style("px-3 py-2 odd:bg-card even:bg-muted hover:bg-primary hover:text-primary-foreground not-hover:text-foreground first:rounded-t-md last:rounded-b-md")
                    .child(Label().text("Hover any row"))
                    .into_element()
            })),
    )
}

fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Enough content to force the page to scroll.",
        Div()
            .style("flex flex-col rounded-md border border-border overflow-hidden")
            .children((0..40).map(|i| {
                Div()
                    .style("px-3 py-2 odd:bg-card even:bg-muted")
                    .child(Label().text(format!("Item {}", i + 1)))
                    .into_element()
            })),
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
                .style("flex flex-col gap-8 max-w-3xl w-full mx-auto p-8")
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label()
                                .text("Animations & Selectors")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("animate-*, first:/last:/odd:/even:/only:, not-*, group-hover:, has-hover:")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                .child(spin_section())
                .child(pulse_section())
                .child(ping_section())
                .child(bounce_section())
                .child(first_last_section())
                .child(odd_even_section())
                .child(not_hover_section())
                .child(group_hover_section())
                .child(has_hover_section())
                .child(combo_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
