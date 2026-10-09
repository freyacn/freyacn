/*
//! # Input Component
//!
//! A themed input component inspired by shadcn/ui with full extension support.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::components::input::Input;
//! use freyacn::extensions::*;
//!
//! let value = use_state(String::new);
//! let input = Input::new(value)
//!     .placeholder("Type here...")
//!     .bg_background()
//!     .border_primary()
//!     .rounded(8)
//!     .p_2();
//! ```

use crate::extensions::*;
use crate::theme::use_cn_theme;
use freya::components::{Input as FreyaInput, InputMode};
use freya::prelude::*;

/// A themed input component wrapping Freya's Input with shadcn/ui styling.
#[derive(Clone, PartialEq)]
pub struct CNInput {
    // ---- Core state ----
    value: Writable<String>,
    placeholder: Option<String>,
    mode: Option<InputMode>,
    enabled: bool,
    auto_focus: bool,
    text_align: Option<TextAlign>,
    font_size: Option<f32>,
    font_weight: Option<FontWeight>,
    width: Option<Size>,
    height: Option<Size>,
    min_width: Option<Size>,
    min_height: Option<Size>,
    max_width: Option<Size>,
    max_height: Option<Size>,

    // ---- Extension state ----
    background: Option<Color>,
    color: Option<Color>,
    padding: Option<Gaps>,
    margin: Option<Gaps>,
    border_width: Option<f32>,
    border_color: Option<Color>,
    corner_radius: Option<CornerRadius>,
    opacity: Option<f32>,
    shadow: Option<Shadow>,

    // ---- Events ----
    on_submit: Option<EventHandler<String>>,
    on_validate: Option<EventHandler<InputValidator>>,
    on_pre_key_down: Option<Callback<Event<KeyboardEventData>, bool>>,

    // ---- Key ----
    key: DiffKey,
}

impl CNInput {
    /// Creates a new input bound to the given state.
    pub fn new(value: impl Into<Writable<String>>) -> Self {
        Self {
            value: value.into(),
            placeholder: None,
            mode: None,
            enabled: true,
            auto_focus: false,
            text_align: None,
            font_size: None,
            font_weight: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            background: None,
            color: None,
            padding: None,
            margin: None,
            border_width: None,
            border_color: None,
            corner_radius: None,
            opacity: None,
            shadow: None,
            on_submit: None,
            on_validate: None,
            on_pre_key_down: None,
            key: DiffKey::None,
        }
    }

    // ---- Configuration methods ----
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn auto_focus(mut self, auto_focus: impl Into<bool>) -> Self {
        self.auto_focus = auto_focus.into();
        self
    }

    pub fn on_submit(mut self, on_submit: impl Into<EventHandler<String>>) -> Self {
        self.on_submit = Some(on_submit.into());
        self
    }

    pub fn on_validate(mut self, on_validate: impl Into<EventHandler<InputValidator>>) -> Self {
        self.on_validate = Some(on_validate.into());
        self
    }

    pub fn on_pre_key_down(
        mut self,
        on_pre_key_down: impl Into<Callback<Event<KeyboardEventData>, bool>>,
    ) -> Self {
        self.on_pre_key_down = Some(on_pre_key_down.into());
        self
    }
}

// ---- Extension trait implementations ----

impl BackgroundExt for CNInput {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
}

impl ForegroundExt for CNInput {
    fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl SpacingExt for CNInput {
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding = Some(gaps.into());
        self
    }

    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin = Some(gaps.into());
        self
    }
}

impl SizingExt for CNInput {
    fn width(mut self, size: impl Into<Size>) -> Self {
        self.width = Some(size.into());
        self
    }

    fn height(mut self, size: impl Into<Size>) -> Self {
        self.height = Some(size.into());
        self
    }

    fn min_width(mut self, size: impl Into<Size>) -> Self {
        self.min_width = Some(size.into());
        self
    }

    fn min_height(mut self, size: impl Into<Size>) -> Self {
        self.min_height = Some(size.into());
        self
    }

    fn max_width(mut self, size: impl Into<Size>) -> Self {
        self.max_width = Some(size.into());
        self
    }

    fn max_height(mut self, size: impl Into<Size>) -> Self {
        self.max_height = Some(size.into());
        self
    }
}

impl BorderExt for CNInput {
    fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    fn corner_radius(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(radius.into());
        self
    }
}

impl EffectsExt for CNInput {
    fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }
}

impl TypographyExt for CNInput {
    fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    fn text_align(mut self, align: TextAlign) -> Self {
        self.text_align = Some(align);
        self
    }

    fn text_decoration(self, _decoration: TextDecoration) -> Self {
        self
    }
}

impl KeyExt for CNInput {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl CornerRadiusExt for CNInput {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

// ---- Component implementation ----

impl Component for CNInput {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        // Build the Freya input.
        let mut input = FreyaInput::new(self.value.clone());

        // Apply basic properties.
        input = input.enabled(self.enabled).auto_focus(self.auto_focus);

        if let Some(placeholder) = &self.placeholder {
            input = input.placeholder(placeholder.clone());
        }
        if let Some(mode) = self.mode.clone() {
            input = input.mode(mode);
        }
        if let Some(text_align) = self.text_align {
            input = input.text_align(text_align);
        }
        // Set input width to fill the wrapper.
        input = input.width(Size::fill());

        // Apply shadcn-like default styling:
        // - Height: 32px (h-8) via 8px top/bottom padding + 16px font.
        // - Horizontal padding: 10px (px-2.5).
        // - Rounded-lg: 8px corner radius.
        let default_bg = Color::TRANSPARENT;
        let default_border_color = theme.border;
        let default_text_color = theme.foreground;
        let default_placeholder_color = theme.muted_foreground;
        let default_corner_radius = CornerRadius::new_all(8.0); // rounded-lg
        // 8px vertical padding gives 16px total + 16px font = 32px height.
        let default_inner_margin = Gaps::new(8.0, 10.0, 8.0, 10.0); // py-2 (8px), px-2.5 (10px)

        // Apply overrides.
        let bg = self.background.unwrap_or(default_bg);
        let border_color = self.border_color.unwrap_or(default_border_color);
        let text_color = self.color.unwrap_or(default_text_color);
        let corner_radius = self.corner_radius.unwrap_or(default_corner_radius);
        let inner_margin = self.padding.unwrap_or(default_inner_margin);

        // Apply styles to the input.
        input = input
            .background(bg)
            .border_fill(border_color)
            .color(text_color)
            .placeholder_color(default_placeholder_color)
            .corner_radius(corner_radius)
            .inner_margin(inner_margin);

        // Apply events.
        if let Some(on_submit) = self.on_submit.clone() {
            input = input.on_submit(on_submit);
        }
        if let Some(on_validate) = self.on_validate.clone() {
            input = input.on_validate(on_validate);
        }
        if let Some(on_pre_key_down) = self.on_pre_key_down.clone() {
            input = input.on_pre_key_down(on_pre_key_down);
        }

        // ---- Wrap input in a container for additional styles ----
        // Set font size to 16px (text-base) on the wrapper so the input inherits it.
        let font_size = self.font_size.unwrap_or(16.0);

        let mut wrapper = rect()
            .child(input)
            .font_size(font_size)
            .width(self.width.clone().unwrap_or(Size::auto()));

        // If a custom height is provided, apply it; otherwise let the content determine height.
        if let Some(height) = self.height.clone() {
            wrapper = wrapper.height(height);
        }

        // Apply min/max sizing.
        if let Some(min_w) = self.min_width.clone() {
            wrapper = wrapper.min_width(min_w);
        }
        if let Some(min_h) = self.min_height.clone() {
            wrapper = wrapper.min_height(min_h);
        }
        if let Some(max_w) = self.max_width.clone() {
            wrapper = wrapper.max_width(max_w);
        }
        if let Some(max_h) = self.max_height.clone() {
            wrapper = wrapper.max_height(max_h);
        }

        if let Some(margin) = self.margin {
            wrapper = wrapper.margin(margin);
        }

        if let Some(opacity) = self.opacity {
            wrapper = wrapper.opacity(opacity);
        }

        if let Some(shadow) = self.shadow.clone() {
            wrapper = wrapper.shadow(shadow);
        }

        wrapper = wrapper.corner_radius(corner_radius);

        if let Some(width) = self.border_width {
            let color = self.border_color.unwrap_or(theme.border);
            wrapper = wrapper.border(Border::new().fill(color).width(width));
        }

        wrapper.key(self.key.clone())
    }
}

/// Constructor for the Input component.
#[allow(non_snake_case)]
pub fn Input(value: impl Into<Writable<String>>) -> CNInput {
    CNInput::new(value)
}

 */
