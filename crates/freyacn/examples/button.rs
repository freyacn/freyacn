use freyacn::components::Button;
use freyacn::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    let theme = Theme::light().neutral();
    use_init_cn_theme(theme);

    ScrollView::new()
        .width(Size::fill())
        .height(Size::fill())
        .show_scrollbar(true)
        .child(
            rect()
                .width(Size::fill())
                .height(Size::fill())
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
                        // Default Button
                        .child(
                            label()
                                .font_size(18.0)
                                .font_weight(FontWeight::MEDIUM)
                                .color(theme.foreground)
                                .text("Default Size"),
                        )
                        .child(Button().child("Button").variant("destructive")),
                ),
        )
}
