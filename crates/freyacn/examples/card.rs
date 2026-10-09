//! Card component showcase with the FreyaCN theming system.
//!
//! Displays various card configurations including different sizes,
//! headers, titles, descriptions, actions, content, and footers.

use freya::components::ScrollView;
use freya::prelude::*;
use freya_icons::lucide::{bell, settings, user};

use freyacn::components::Button;
use freyacn::components::Icon;
use freyacn::components::Label;
use freyacn::components::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
use freyacn::extensions::*;
use freyacn::theme::{Theme, use_init_cn_theme};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::neutral(false, "neutral");
    use_init_cn_theme(theme);

    // Main scrollable container.
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
                                .text("Card Showcase"),
                        )
                        // Default size card
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Default Size"),
                        )
                        .child(
                            Card()
                                .child(
                                    CardHeader()
                                        .child(CardTitle("Payment Methods"))
                                        .child(CardDescription(
                                            "Manage your payment settings and methods",
                                        ))
                                        .child(
                                            CardAction()
                                                .child(
                                                    Button()
                                                        .size_icon()
                                                        .child(Icon(settings()))
                                                        .bg_muted(),
                                                )
                                                .child(
                                                    Button()
                                                        .size_icon()
                                                        .child(Icon(bell()))
                                                        .bg_muted(),
                                                ),
                                        ),
                                )
                                .child(
                                    CardContent()
                                        .child(
                                            Label("Your current payment method is set to:")
                                                .text_sm()
                                                .text_muted_foreground(),
                                        )
                                        .child(Label("•••• 4242").text_base().font_bold())
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.0)
                                                .child(
                                                    Button()
                                                        .child(Label("Update"))
                                                        .bg_primary()
                                                        .text_primary_foreground()
                                                        .size_sm(),
                                                )
                                                .child(
                                                    Button()
                                                        .child(Label("Remove"))
                                                        .destructive()
                                                        .size_sm(),
                                                ),
                                        ),
                                )
                                .child(
                                    CardFooter()
                                        .child(
                                            Label("Last updated: Today")
                                                .text_sm()
                                                .text_muted_foreground(),
                                        )
                                        .child(Button().child(Label("Refresh")).ghost().size_sm()),
                                ),
                        )
                        // Small size card
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Small Size"),
                        )
                        .child(
                            Card()
                                .size(CardSize::Sm)
                                .child(
                                    CardHeader()
                                        .child(CardTitle("User Profile"))
                                        .child(CardDescription("Manage your account details"))
                                        .child(CardAction().child(
                                            Button().size_icon_sm().child(Icon(user())).bg_muted(),
                                        )),
                                )
                                .child(
                                    CardContent()
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.0)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    Label("Name:")
                                                        .text_sm()
                                                        .text_muted_foreground(),
                                                )
                                                .child(Label("John Doe").text_sm().font_bold()),
                                        )
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.0)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    Label("Email:")
                                                        .text_sm()
                                                        .text_muted_foreground(),
                                                )
                                                .child(
                                                    Label("john@example.com").text_sm().font_bold(),
                                                ),
                                        ),
                                )
                                .child(
                                    CardFooter()
                                        .child(
                                            Button()
                                                .child(Label("Edit Profile"))
                                                .secondary()
                                                .size_sm(),
                                        )
                                        .child(Button().child(Label("Save")).default().size_sm()),
                                ),
                        ),
                ),
        )
}
