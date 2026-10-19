//! # Label Showcase
//!
//! Every Label feature: sizes, weights, colours, families, italic,
//! alignment (needs `w-full`), decoration, own hover, group hover,
//! disabled, truncation, wrapping, inheritance, composition.
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
        .style("flex flex-col gap-4 p-6 bg-card rounded-xl border border-border shadow-sm")
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
    Div().style("h-[1px] w-full bg-border")
}

// ==============================================================
// Sections
// ==============================================================

fn basic_section() -> impl IntoElement {
    section(
        "Basic",
        "Minimal usage — `Label().text(...)` with no styling.",
        Div()
            .style("flex flex-col gap-3")
            .child(Label().text("Hello, world."))
            .child(Label().text("A second line at the default style.")),
    )
}

fn sizes_section() -> impl IntoElement {
    section(
        "Sizes",
        "`text_xs` through `text_3xl`.",
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
        "`font_light` through `font_extrabold`.",
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

fn theme_colors_section() -> impl IntoElement {
    section(
        "Theme colours",
        "Every theme colour via `text_*`.",
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

fn palette_section() -> impl IntoElement {
    section(
        "Palette colours",
        "Every Tailwind palette shade via `text_{family}_{shade}`.",
        Div()
            .style("flex flex-col gap-2")
            .child(Label().text("text_blue_500").style("text-blue-500"))
            .child(Label().text("text_emerald_400").style("text-emerald-400"))
            .child(Label().text("text_violet_500").style("text-violet-500"))
            .child(Label().text("text_rose_500").style("text-rose-500"))
            .child(Label().text("text_amber_400").style("text-amber-400")),
    )
}

fn inheritance_section() -> impl IntoElement {
    section(
        "Inheritance",
        "Label picks up text style from the nearest ancestor that set it.",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-foreground")
                    .child(Label().text("Parent: text-foreground")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-primary")
                    .child(Label().text("Parent: text-primary")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-destructive")
                    .child(Label().text("Parent: text-destructive")),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted text-muted-foreground")
                    .child(Label().text("Parent: text-muted-foreground")),
            ),
    )
}

fn override_section() -> impl IntoElement {
    section(
        "Explicit override",
        "A colour on the label itself wins over the parent's cascade.",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted flex flex-col gap-1 text-foreground")
                    .child(Label().text("Parent: foreground").style("text-foreground"))
                    .child(
                        Label()
                            .text("Child: overridden to primary")
                            .style("text-primary"),
                    ),
            )
            .child(
                Div()
                    .style("p-4 rounded-lg bg-muted flex flex-col gap-1 text-foreground")
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
        "`font_sans`, `font_serif`, `font_mono`.",
        Div()
            .style("flex flex-col gap-2")
            .child(
                Label()
                    .text("The quick brown fox — sans")
                    .style("font-sans text-base text-foreground"),
            )
            .child(
                Label()
                    .text("The quick brown fox — serif")
                    .style("font-serif text-base text-foreground"),
            )
            .child(
                Label()
                    .text("The quick brown fox — mono")
                    .style("font-mono text-base text-foreground"),
            ),
    )
}

fn italic_section() -> impl IntoElement {
    section(
        "Italic",
        "`italic` sets font-slant.",
        Div()
            .style("flex flex-col gap-2")
            .child(Label().text("Upright").style("text-base text-foreground"))
            .child(
                Label()
                    .text("Italic")
                    .style("italic text-base text-foreground"),
            ),
    )
}

fn alignment_section() -> impl IntoElement {
    section(
        "Alignment",
        "`text_align` needs bounded width. The `w-full` on each label gives it room to align.",
        Div()
            .style("flex flex-col gap-2 w-full")
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Left")
                        .style("text-left text-sm text-foreground w-full"),
                ),
            )
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Center")
                        .style("text-center text-sm text-foreground w-full"),
                ),
            )
            .child(
                Div().style("w-full p-2 rounded-md bg-muted/40").child(
                    Label()
                        .text("Right")
                        .style("text-right text-sm text-foreground w-full"),
                ),
            ),
    )
}

fn decoration_section() -> impl IntoElement {
    section(
        "Decoration",
        "`underline`, `overline`, `line-through`.",
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
                    .style("underline text-base text-foreground"),
            )
            .child(
                Label()
                    .text("Overlined")
                    .style("overline text-base text-foreground"),
            )
            .child(
                Label()
                    .text("Struck through")
                    .style("line-through text-base text-foreground"),
            ),
    )
}

fn hover_section() -> impl IntoElement {
    section(
        "Own hover state",
        "`Label` tracks its own hover, so `hover:underline` fires when the pointer is over the label text.",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-md border border-border bg-muted/40")
                    .child(
                        Label()
                            .text("Hover this text")
                            .style("text-sm text-foreground hover:underline cursor-pointer"),
                    ),
            )
            .child(
                Div()
                    .style("p-4 rounded-md border border-border bg-muted/40")
                    .child(Label().text("Or this").style(
                        "text-sm text-foreground hover:text-primary hover:underline cursor-pointer",
                    )),
            ),
    )
}

fn inherited_hover_section() -> impl IntoElement {
    section(
        "Inherited hover",
        "Hover anywhere on the parent row — the label's `hover:underline` fires because the parent `Div`'s hover state cascades via `DivActiveState`.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Div()
                    .style("flex flex-row items-center gap-3 p-4 rounded-lg border border-border bg-muted/40 cursor-pointer")
                    .child(
                        Label()
                            .text("Hover the whole row, not just the text")
                            .style("flex-1 h-auto text-sm font-medium hover:underline hover:text-primary"),
                    )
                    .child(Label().text("→").style("text-muted-foreground")),
            )
            .child(
                Div()
                    .style("flex flex-row items-center justify-between p-4 rounded-lg border border-border bg-muted/40 cursor-pointer")
                    .child(Label().text("Row title").style("text-base font-semibold hover:underline"))
                    .child(Label().text("12").style("text-xs text-muted-foreground")),
            ),
    )
}

fn group_hover_section() -> impl IntoElement {
    section(
        "Group hover",
        "Wrap the row in `Group()`. `group-hover:underline` fires when the *group* is hovered, independent of any single child.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Group()
                    .style("flex flex-row items-center justify-between gap-3 p-4 rounded-lg border border-border bg-muted/40 cursor-pointer")
                    .child(
                        Label()
                            .text("Group-hovered title")
                            .style("text-sm font-medium group-hover:underline group-hover:text-primary"),
                    )
                    .child(
                        Label()
                            .text("→")
                            .style("text-muted-foreground group-hover:text-primary"),
                    ),
            )
            .child(
                Group()
                    .style("flex flex-col gap-1 p-4 rounded-lg border border-border bg-muted/40 cursor-pointer")
                    .child(
                        Label()
                            .text("Grouped card title")
                            .style("text-base font-semibold group-hover:text-primary group-hover:underline"),
                    )
                    .child(
                        Label()
                            .text("Supporting description also reacts.")
                            .style("text-sm text-muted-foreground group-hover:text-foreground"),
                    ),
            ),
    )
}

fn disabled_section() -> impl IntoElement {
    section(
        "Disabled",
        "`disabled(true)` suppresses hover/press/focus and dims the label.",
        Div()
            .style("flex flex-row flex-wrap gap-3")
            .child(
                Div()
                    .style("p-4 rounded-md border border-border bg-muted/40")
                    .child(
                        Label()
                            .text("Enabled")
                            .style("text-sm hover:underline cursor-pointer"),
                    ),
            )
            .child(
                Div()
                    .style("p-4 rounded-md border border-border bg-muted/40")
                    .child(
                        Label()
                            .text("Disabled")
                            .disabled(true)
                            .style("text-sm hover:underline"),
                    ),
            ),
    )
}

fn truncation_section() -> impl IntoElement {
    section(
        "Truncation",
        "`truncate` needs bounded width — either on the label or on its parent.",
        Div()
            .style("flex flex-col gap-3")
            .child(
                Div()
                    .style("flex flex-col gap-1")
                    .child(Label().text("max-w-64 + truncate on label").style("text-[10px] font-mono text-muted-foreground"))
                    .child(
                        Label()
                            .text("This is a very long piece of text and it gets cut off with an ellipsis thanks to max_w_64 on the label itself.")
                            .style("text-sm text-foreground truncate max-w-64"),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col gap-1")
                    .child(Label().text("w-48 parent + truncate on label").style("text-[10px] font-mono text-muted-foreground"))
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
        "Without `truncate`, long text wraps when its parent is bounded.",
        Div()
            .style("w-full max-w-sm p-3 rounded-md bg-muted/40")
            .child(
                Label()
                    .text("The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.")
                    .style("text-sm text-foreground"),
            ),
    )
}

fn composition_section() -> impl IntoElement {
    section(
        "In composition",
        "Labels inherit text styling from buttons, cards, and any styled ancestor.",
        Div()
            .style("flex flex-col gap-4")
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
            .child(
                Div()
                    .style("w-full max-w-sm rounded-xl border border-border bg-card overflow-hidden")
                    .child(
                        Div()
                            .style("p-4 flex flex-col gap-1")
                            .child(Label().text("Card title").style("text-base font-semibold text-card-foreground"))
                            .child(
                                Label()
                                    .text("Supporting text in a muted colour, matching the card's hierarchy.")
                                    .style("text-sm text-muted-foreground"),
                            ),
                    ),
            ),
    )
}

fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Enough content to force the page to scroll.",
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
    use_init_cn_theme(Theme::light().neutral().theme_color("neutral"));

    Div()
        .style("flex flex-col w-full h-screen bg-background overflow-scroll text-foreground")
        .child(
            Div()
                .style("flex flex-col gap-8 max-w-3xl w-full mx-auto p-8")
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label().text("Label").style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Text primitive with cascade inheritance, own hover/press/focus, group hover, and inherited hover from parent Div.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                .child(basic_section())
                .child(sizes_section())
                .child(weights_section())
                .child(theme_colors_section())
                .child(palette_section())
                .child(inheritance_section())
                .child(override_section())
                .child(families_section())
                .child(italic_section())
                .child(alignment_section())
                .child(decoration_section())
                .child(hover_section())
                .child(inherited_hover_section())
                .child(group_hover_section())
                .child(disabled_section())
                .child(truncation_section())
                .child(wrapping_section())
                .child(composition_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
