//! Input component showcase with the FreyaCN theming system.
//!
//! Displays various input configurations including default, filled, flat,
//! and themed inputs with different sizes and states.

use freya::components::ScrollView;
use freya::prelude::*;

use freyacn::components::input::Input;
use freyacn::extensions::*;
use freyacn::theme::{Theme, use_init_cn_theme};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::neutral(false, "slate");
    use_init_cn_theme(theme);

    // State values for each input.
    let name_state = use_state(|| String::new());
    let email_state = use_state(|| String::new());
    let password_state = use_state(|| String::new());
    let search_state = use_state(|| String::new());
    let disabled_state = use_state(|| String::from("Disabled"));

    ScrollView::new()
        .width(Size::fill())
        .height(Size::fill())
        .show_scrollbar(true)
        .child(
            rect()
                .width(Size::fill())
                .padding(Gaps::new_all(32.0))
                .background(theme.background)
                .child(
                    rect()
                        .vertical()
                        .spacing(32.0)
                        .width(Size::fill())
                        // Header
                        .child(
                            label()
                                .font_size(28.0)
                                .font_weight(FontWeight::BOLD)
                                .color(theme.foreground)
                                .text("Input Showcase"),
                        )
                        // Default input
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Default Input"),
                        )
                        .child(
                            rect()
                                .width(Size::px(300.0))
                                .child(Input(name_state).placeholder("Enter your name...")),
                        )
                        // Filled input
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Filled Input"),
                        )
                        .child(
                            rect()
                                .width(Size::px(300.0))
                                .child(Input(email_state).placeholder("Enter your email...")),
                        )
                        // Flat input (no border)
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Flat Input"),
                        )
                        .child(
                            rect().width(Size::px(300.0)).child(
                                Input(search_state)
                                    .placeholder("Search...")
                                    .background(Color::TRANSPARENT)
                                    .border_color(Color::TRANSPARENT)
                                    .corner_radius(8.0)
                                    .padding(Gaps::new(0.0, 10.0, 0.0, 10.0)),
                            ),
                        )
                        // Input with icon
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Input with Icon"),
                        )
                        .child(
                            rect().width(Size::px(300.0)).child(
                                Input(password_state)
                                    .placeholder("Enter password...")
                                    .background(theme.background)
                                    .border_color(theme.border)
                                    .corner_radius(8.0)
                                    .padding(Gaps::new(0.0, 10.0, 0.0, 10.0)),
                            ),
                        )
                        // Disabled input
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Disabled Input"),
                        )
                        .child(
                            rect().width(Size::px(300.0)).child(
                                Input(disabled_state)
                                    .placeholder("Disabled input...")
                                    .background(theme.muted)
                                    .border_color(theme.border)
                                    .corner_radius(8.0)
                                    .padding(Gaps::new(0.0, 10.0, 0.0, 10.0))
                                    .enabled(false),
                            ),
                        )
                        // Primary themed input
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Primary Themed Input"),
                        )
                        .child(
                            rect().width(Size::px(300.0)).child(
                                Input(use_state(|| String::new()))
                                    .placeholder("Primary input...")
                                    .background(theme.primary)
                                    .color(theme.primary_foreground)
                                    .border_color(theme.primary)
                                    .corner_radius(8.0)
                                    .padding(Gaps::new(0.0, 10.0, 0.0, 10.0)),
                            ),
                        )
                        // Destructive themed input
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Destructive Themed Input"),
                        )
                        .child(
                            rect().width(Size::px(300.0)).child(
                                Input(use_state(|| String::new()))
                                    .placeholder("Destructive input...")
                                    .background(theme.destructive)
                                    .color(theme.destructive_foreground)
                                    .border_color(theme.destructive)
                                    .corner_radius(8.0)
                                    .padding(Gaps::new(0.0, 10.0, 0.0, 10.0)),
                            ),
                        ),
                ),
        )
}
