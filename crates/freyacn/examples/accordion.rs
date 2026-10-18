//! # Accordion Showcase
//!
//! Every configuration: single, collapsible, multiple, default-open,
//! disabled (per-item and root-level), per-item overrides, icons,
//! FAQ, inheritance, and a long scrolling list.
//!
//! Every trigger's title label uses `flex-1 hover:underline`, so
//! hovering anywhere on the trigger row underlines the title.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example accordion_showcase
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

/// Trigger with a leading icon and optional trailing badge. The title
/// label carries `flex-1 hover:underline` so it fills the trigger row
/// and underlines on hover anywhere on the row.
fn icon_trigger(
    title: &str,
    icon_svg: Bytes,
    badge: Option<&'static str>,
) -> AccordionTriggerComponent {
    let mut trigger = AccordionTrigger()
        .style("flex-row items-center gap-3")
        .child(
            // `pointer-events-none` so this Div doesn't steal hover
            // from the trigger. The trigger is the interactive unit;
            // its children are decoration.
            Div()
                .style("pointer-events-none shrink-0 text-muted-foreground")
                .child(Icon().svg(icon_svg).size_4()),
        )
        .child(
            Label()
                .text(title)
                .style("font-medium flex-1 hover:underline"),
        );

    if let Some(badge_text) = badge {
        trigger = trigger.child(
            Div()
                .style(
                    "pointer-events-none shrink-0 px-2 py-0.5 rounded-full bg-muted \
                     text-xs text-muted-foreground",
                )
                .child(Label().text(badge_text)),
        );
    }

    trigger
}

// ==============================================================
// Sections
// ==============================================================

fn inheritance_section() -> impl IntoElement {
    section(
        "Inheritance",
        "The same `.item()` helper on three different backgrounds. Each accordion inherits its container's surface.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Div()
                    .style("flex flex-col p-4 rounded-md bg-muted")
                    .child(
                        Label()
                            .text("Parent: bg-muted")
                            .style("text-xs font-mono text-muted-foreground mb-2"),
                    )
                    .child(
                        Accordion()
                            .type_(AccordionType::Single)
                            .collapsible(true)
                            .item(
                                "inh-muted-1",
                                "First item on bg-muted",
                                "Background is inherited from the container.",
                            )
                            .item(
                                "inh-muted-2",
                                "Second item on bg-muted",
                                "Same background — nothing is baked in.",
                            ),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col p-4 rounded-md bg-background border border-border")
                    .child(
                        Label()
                            .text("Parent: bg-background")
                            .style("text-xs font-mono text-muted-foreground mb-2"),
                    )
                    .child(
                        Accordion()
                            .type_(AccordionType::Single)
                            .collapsible(true)
                            .item(
                                "inh-bg-1",
                                "First item on bg-background",
                                "Background is inherited from the container.",
                            )
                            .item(
                                "inh-bg-2",
                                "Second item on bg-background",
                                "Same background — the container wins.",
                            ),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col p-4 rounded-md bg-secondary text-secondary-foreground")
                    .child(
                        Label()
                            .text("Parent: bg-secondary")
                            .style("text-xs font-mono text-muted-foreground mb-2"),
                    )
                    .child(
                        Accordion()
                            .type_(AccordionType::Single)
                            .collapsible(true)
                            .item(
                                "inh-secondary-1",
                                "Item on bg-secondary",
                                "Background is bg-secondary — inherited, not forced.",
                            )
                            .item(
                                "inh-secondary-2",
                                "Second item on bg-secondary",
                                "Same background, same inheritance.",
                            ),
                    ),
            ),
    )
}

fn single_section() -> impl IntoElement {
    section(
        "Single",
        "One item open at a time. Hover any trigger — the whole row underlines.",
        Accordion()
            .type_(AccordionType::Single)
            .item(
                "single-1",
                "Is it accessible?",
                "Triggers are role=Button and focusable; content is role=Region, labelled by the trigger and hidden while closed.",
            )
            .item(
                "single-2",
                "Is it styled?",
                "Yes — padding, hover underline, focus ring, and chevron rotation all match shadcn.",
            )
            .item(
                "single-3",
                "Is it animated?",
                "Yes — 300ms linear height animation, 200ms linear chevron rotation.",
            ),
    )
}

fn collapsible_section() -> impl IntoElement {
    section(
        "Single (collapsible)",
        "`collapsible(true)` lets the user close the currently open item.",
        Accordion()
            .type_(AccordionType::Single)
            .collapsible(true)
            .item(
                "collapsible-1",
                "Click to open",
                "Click the trigger again to close it.",
            )
            .item(
                "collapsible-2",
                "Or click me instead",
                "Opening a different item closes this one.",
            ),
    )
}

fn multiple_section() -> impl IntoElement {
    section(
        "Multiple",
        "Any number of items open simultaneously.",
        Accordion()
            .type_(AccordionType::Multiple)
            .item(
                "multi-1",
                "Section one",
                "Click another item to open alongside this one.",
            )
            .item("multi-2", "Section two", "Every item is independent.")
            .item(
                "multi-3",
                "Section three",
                "All three can be open simultaneously.",
            ),
    )
}

fn default_open_section() -> impl IntoElement {
    section(
        "Default open",
        "`default_value(vec![...])` starts specific items open. Initial-open items do not animate on mount.",
        Accordion()
            .type_(AccordionType::Multiple)
            .default_value(vec!["default-1".to_string(), "default-3".to_string()])
            .item(
                "default-1",
                "This one starts open",
                "Its content is visible and its chevron is already rotated.",
            )
            .item("default-2", "This one starts closed", "Click to open.")
            .item(
                "default-3",
                "Also starts open",
                "Driven by OnCreation::Finish.",
            ),
    )
}

fn disabled_section() -> impl IntoElement {
    section(
        "Disabled",
        "Per-item `disabled(true)` dims the trigger and blocks interaction. Root-level `disabled(true)` disables every item.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Div()
                    .style("flex flex-col")
                    .child(
                        Label()
                            .text("Per-item disabled")
                            .style("text-xs font-mono text-muted-foreground mb-2"),
                    )
                    .child(
                        Accordion()
                            .type_(AccordionType::Single)
                            .item("disabled-1", "Enabled item", "This one works normally.")
                            .child(
                                AccordionItem()
                                    .value("disabled-2")
                                    .disabled(true)
                                    .child(
                                        AccordionTrigger().child(
                                            Label()
                                                .text("Disabled item")
                                                .style("font-medium flex-1 hover:underline"),
                                        ),
                                    )
                                    .child(
                                        AccordionContent().child(
                                            Label()
                                                .text("This content cannot be opened.")
                                                .style("text-sm text-muted-foreground"),
                                        ),
                                    ),
                            )
                            .item("disabled-3", "Another enabled item", "Still interactive."),
                    ),
            )
            .child(
                Div()
                    .style("flex flex-col")
                    .child(
                        Label()
                            .text("Root-level disabled")
                            .style("text-xs font-mono text-muted-foreground mb-2"),
                    )
                    .child(
                        Accordion()
                            .type_(AccordionType::Single)
                            .disabled(true)
                            .item("root-1", "One", "Fully disabled via the root.")
                            .item("root-2", "Two", "Every item is disabled."),
                    ),
            ),
    )
}

fn override_section() -> impl IntoElement {
    section(
        "Per-item overrides",
        "Items inherit by default. Set `bg-*` or `border-*` on individual items to override. Card-style items carry their own full border.",
        Accordion()
            .type_(AccordionType::Single)
            .collapsible(true)
            .child(
                AccordionItem()
                    .value("override-primary")
                    .style("bg-primary/10 border border-primary/40 rounded-lg mb-2 px-4")
                    .child(
                        AccordionTrigger().child(
                            Label()
                                .text("Primary-tinted")
                                .style("font-medium text-primary flex-1 hover:underline"),
                        ),
                    )
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Overrides the parent background with bg-primary/10.")
                                .style("text-sm text-muted-foreground"),
                        ),
                    ),
            )
            .child(
                AccordionItem()
                    .value("override-neutral")
                    .style("bg-background border border-border rounded-lg px-4")
                    .child(
                        AccordionTrigger().child(
                            Label()
                                .text("Neutral card")
                                .style("font-medium flex-1 hover:underline"),
                        ),
                    )
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Overrides with bg-background.")
                                .style("text-sm text-muted-foreground"),
                        ),
                    ),
            ),
    )
}

fn icons_section() -> impl IntoElement {
    section(
        "With icons and badges",
        "The compound API lets the trigger contain anything. Only the title label carries `hover:underline`; the badge stays unaffected.",
        Accordion()
            .type_(AccordionType::Single)
            .child(
                AccordionItem()
                    .value("icon-1")
                    .child(icon_trigger("Notifications", bell(), Some("3 new")))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Push, email, and in-app settings.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            )
            .child(
                AccordionItem()
                    .value("icon-2")
                    .child(icon_trigger("Privacy", lock(), None))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Profile visibility and read receipts.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            )
            .child(
                AccordionItem()
                    .value("icon-3")
                    .child(icon_trigger("Appearance", palette(), Some("Dark")))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Theme, density, and accent color.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            ),
    )
}

fn faq_section() -> impl IntoElement {
    section(
        "FAQ",
        "Canonical use case: single + collapsible, with an icon on every row. Hover any row — the question underlines.",
        Accordion()
            .type_(AccordionType::Single)
            .collapsible(true)
            .child(
                AccordionItem()
                    .value("faq-1")
                    .child(icon_trigger(
                        "Can I open several items at once?",
                        circle_question_mark(),
                        None,
                    ))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Set `type_(AccordionType::Multiple)` on the root.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            )
            .child(
                AccordionItem()
                    .value("faq-2")
                    .child(icon_trigger(
                        "How does the animation work?",
                        circle_question_mark(),
                        None,
                    ))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Content height animates via `VisibleSize::inner_percent` with a 300ms linear curve; the chevron rotates 200ms linear.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            )
            .child(
                AccordionItem()
                    .value("faq-3")
                    .child(icon_trigger(
                        "Is it keyboard accessible?",
                        circle_question_mark(),
                        None,
                    ))
                    .child(
                        AccordionContent().child(
                            Label()
                                .text("Yes — the trigger is role=Button + focusable, so Enter/Space activate it.")
                                .style("text-sm text-muted-foreground leading-relaxed"),
                        ),
                    ),
            ),
    )
}

fn long_list_section() -> impl IntoElement {
    section(
        "A long list",
        "Enough items to force the page to scroll. Only the opening and closing items re-render per click.",
        Accordion()
            .type_(AccordionType::Single)
            .collapsible(true)
            .children((0..30).map(|i| {
                AccordionItem()
                    .value(format!("long-{}", i))
                    .child(
                        AccordionTrigger().child(
                            Label()
                                .text(format!("Section {}", i + 1))
                                .style("font-medium flex-1 hover:underline"),
                        ),
                    )
                    .child(
                        AccordionContent().child(
                            Label()
                                .text(format!(
                                    "Body of section {}. Click another trigger to switch.",
                                    i + 1
                                ))
                                .style("text-sm text-muted-foreground"),
                        ),
                    )
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
        .style("flex flex-col w-full h-screen bg-background overflow-scroll text-foreground")
        .child(
            Div()
                .style("flex flex-col gap-8 max-w-2xl w-full mx-auto p-8")
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label().text("Accordion").style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("Shadcn-shaped. `.item()` for plain text, compound API for rich content. Hover any trigger row to see `hover:underline` fire on the title label.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                .child(inheritance_section())
                .child(single_section())
                .child(collapsible_section())
                .child(multiple_section())
                .child(default_open_section())
                .child(disabled_section())
                .child(override_section())
                .child(icons_section())
                .child(faq_section())
                .child(divider())
                .child(long_list_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
