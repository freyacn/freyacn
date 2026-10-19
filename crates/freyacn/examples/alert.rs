//! # Alert Showcase
//!
//! Run with: `cargo run --example alert_showcase`

use freyacn::prelude::*;

// Sections sit on `bg-background` (the page surface) — the alert's
// `bg-card` then reads as a distinct surface, matching shadcn docs.
fn section(title: &str, description: &str, body: impl IntoElement) -> impl IntoElement {
    Div()
        .style("flex flex-col gap-4 p-6 rounded-xl border border-border bg-background")
        .child(
            Label()
                .text(title)
                .style("text-lg font-semibold text-foreground"),
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

fn basic_section() -> impl IntoElement {
    section(
        "Basic",
        "Title and description, no icon.",
        Alert()
            .child(AlertTitle().child(Label().text("Heads up!")))
            .child(
                AlertDescription()
                    .child(Label().text("You can add components to your app using the CLI.")),
            ),
    )
}

fn variants_section() -> impl IntoElement {
    section(
        "Variants",
        "`Default` and `Destructive` — description softens to destructive/90 automatically.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Alert()
                    .child(AlertTitle().child(Label().text("Heads up!")))
                    .child(AlertDescription().child(Label().text("This is a default alert."))),
            )
            .child(
                Alert()
                    .variant(AlertVariant::Destructive)
                    .child(AlertTitle().child(Label().text("Error")))
                    .child(
                        AlertDescription()
                            .child(Label().text("Your session has expired. Please log in again.")),
                    ),
            ),
    )
}

fn icon_section() -> impl IntoElement {
    section(
        "With icons",
        "`.icon(...)` — content-fit icon column + 1fr content column, 12px gap.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Alert()
                    .icon(Icon().svg(info()))
                    .child(AlertTitle().child(Label().text("Heads up!")))
                    .child(
                        AlertDescription().child(
                            Label().text("You can add components to your app using the CLI."),
                        ),
                    ),
            )
            .child(
                Alert()
                    .variant(AlertVariant::Destructive)
                    .icon(Icon().svg(circle_alert()))
                    .child(AlertTitle().child(Label().text("Error")))
                    .child(
                        AlertDescription()
                            .child(Label().text("Your session has expired. Please log in again.")),
                    ),
            ),
    )
}

fn rich_section() -> impl IntoElement {
    section(
        "Rich content",
        "Multiple paragraphs inside AlertDescription — `gap-1` between them.",
        Alert()
            .icon(Icon().svg(info()))
            .child(AlertTitle().child(Label().text("Multiple paragraphs")))
            .child(
                AlertDescription()
                    .child(Label().text("First paragraph of the description."))
                    .child(Label().text("Second paragraph, separated by gap-1 (4px).")),
            ),
    )
}

fn styled_section() -> impl IntoElement {
    section(
        "Style overrides",
        "Overrides on the alert wrapper win over the variant.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Alert()
                    .variant(AlertVariant::Destructive)
                    .style("border-destructive/40")
                    .child(AlertTitle().child(Label().text("Custom border")))
                    .child(AlertDescription().child(Label().text("1px destructive border."))),
            )
            .child(
                Alert()
                    .style("shadow-md")
                    .child(AlertTitle().child(Label().text("Shadowed")))
                    .child(AlertDescription().child(Label().text("Drop shadow on the wrapper."))),
            ),
    )
}

fn stacked_section() -> impl IntoElement {
    section(
        "Stacked",
        "Three alerts in a column with `gap-3` on the parent.",
        Div().style("flex flex-col gap-3").children((0..3).map(|i| {
            let (variant, icon) = match i % 3 {
                0 => (AlertVariant::Default, Icon().svg(info())),
                1 => (AlertVariant::Default, Icon().svg(circle_check())),
                _ => (AlertVariant::Destructive, Icon().svg(circle_alert())),
            };
            Alert()
                .variant(variant)
                .icon(icon)
                .child(AlertTitle().child(Label().text(format!("Alert #{}", i + 1))))
                .child(AlertDescription().child(Label().text("Stacked alert body.")))
                .into_element()
        })),
    )
}

fn action_section() -> impl IntoElement {
    section(
        "With action",
        "The action is a direct flex sibling of the content — same treatment as the icon. No wrapping, no shrinking, no reserved padding.",
        Div()
            .style("flex flex-col gap-4")
            .child(
                Alert()
                    .child(AlertTitle().text("Heads up!"))
                    .child(
                        AlertDescription()
                            .text("You can add components to your app using the CLI."),
                    )
                    .action(
                        AlertAction().child(
                            Button()
                                .size(ButtonSize::Default)
                                .child(Label().text("Enable")),
                        ),
                    ),
            )
            .child(
                Alert()
                    .variant(AlertVariant::Destructive)
                    .icon(Icon().svg(circle_alert()).size_4())
                    .child(AlertTitle().text("Error"))
                    .child(AlertDescription().text("Your session has expired."))
                    .action(
                        AlertAction().child(
                            Button()
                                .variant(ButtonVariant::Destructive)
                                .size(ButtonSize::Default)
                                .child(Label().text("Retry")),
                        ),
                    ),
            )
            .child(
                Alert()
                    .child(AlertTitle().text("Update available"))
                    .child(
                        AlertDescription()
                            .text("A new version of the application is ready to install."),
                    )
                    .action(
                        AlertAction().child(
                            Button()
                                .size(ButtonSize::Default)
                                .child(Label().text("Update")),
                        ),
                    ),
            ),
    )
}

fn app() -> impl IntoElement {
    use_init_cn_theme(Theme::dark().neutral().theme_color("neutral"));

    Div()
        .style("flex flex-col w-full h-screen bg-background overflow-scroll text-foreground")
        .child(
            Div()
                .style("flex flex-col gap-8 max-w-3xl w-full mx-auto p-8")
                .child(
                    Div()
                        .style("flex flex-col gap-2")
                        .child(
                            Label()
                                .text("Alert")
                                .style("text-3xl font-bold text-foreground"),
                        )
                        .child(
                            Label()
                                .text("shadcn-compatible alerts — bg-card surface, 16/12px padding, 14px text, 2px vertical gap, 12px icon gap.")
                                .style("text-base text-muted-foreground"),
                        ),
                )
                .child(divider())
                .child(basic_section())
                .child(variants_section())
                .child(icon_section())
                .child(rich_section())
                .child(styled_section())
                .child(action_section())
                .child(stacked_section()),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
