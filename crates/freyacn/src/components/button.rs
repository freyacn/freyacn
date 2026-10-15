//! A fully-featured button component inspired by shadcn/ui.
//!
//! The button supports multiple variants (default, destructive, outline, secondary, ghost, link)
//! and sizes (default, xs, sm, lg, icon, icon-xs, icon-sm, icon-lg). It integrates with the
//! FreyaCN theming system and all extension traits.

use crate::*;

/// Available visual variants for the button, matching shadcn/ui styles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Available size presets for the button, matching shadcn/ui sizes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Default, // h-8, px-2.5
    Xs,      // h-6, px-2
    Sm,      // h-7, px-2.5
    Lg,      // h-9, px-2.5
    Icon,    // size-8
    IconXs,  // size-6
    IconSm,  // size-7
    IconLg,  // size-9
}

#[component(
    _variant: ButtonVariant = ButtonVariant::Default,
    _size: ButtonSize = ButtonSize::Default,
    disabled: bool = false
)]
#[extensions(style, children)]
#[derive(PartialEq, Clone)]
pub fn Button() {
    let theme = use_cn_theme().read();

    let is_icon_only = matches!(
        _size,
        ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg
    );

    let mut button = ButtonPrimitive::new();

    // ---- Content container ----
    let mut content = rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .width(Size::auto())
        .height(Size::auto());

    // ---- Wrap the button in a container to apply extra styles ----
    let mut wrapper = rect();

    // Compute variant colors
    button = match _variant {
        ButtonVariant::Default => {
            let hover_bg = color_with_alpha(theme.primary, 0.8);
            content = content.color(theme.primary_foreground);

            button
                .background(theme.primary)
                .color(theme.primary_foreground)
                .hover_background(hover_bg)
                .border_fill(Color::TRANSPARENT)
        }

        ButtonVariant::Secondary => {
            let hover_bg = blend_colors(theme.secondary, theme.foreground, 0.05);
            content = content.color(theme.secondary_foreground);

            button
                .background(theme.secondary)
                .color(theme.secondary_foreground)
                .hover_background(hover_bg)
                .border_fill(Color::TRANSPARENT)
        }

        ButtonVariant::Ghost => {
            content = content.color(theme.foreground);

            button
                .background(Color::TRANSPARENT)
                .hover_background(theme.muted)
                .color(theme.foreground)
                .border_fill(Color::TRANSPARENT)
        }

        ButtonVariant::Destructive => {
            let bg = color_with_alpha(theme.destructive, 0.1);
            let hover_bg = color_with_alpha(theme.destructive, 0.2);
            content = content.color(theme.destructive);

            button
                .background(bg)
                .hover_background(hover_bg)
                .color(theme.destructive)
                .border_fill(Color::TRANSPARENT)
        }

        ButtonVariant::Link => {
            content = content.color(theme.primary);

            button
                .background(Color::TRANSPARENT)
                .hover_background(Color::TRANSPARENT)
                .color(theme.primary)
                .border_fill(Color::TRANSPARENT)
        }

        ButtonVariant::Outline => {
            let mut b = Border::default().fill(theme.border);
            wrapper = wrapper.border(b);
            content = content.color(theme.foreground);

            button
                .background(theme.background)
                .hover_background(theme.muted)
                .color(theme.foreground)
                .border_fill(Color::TRANSPARENT)
        }
    };

    // Size metrics (matching shadcn React exactly).
    button = match _size {
        ButtonSize::Default => {
            content = content.spacing(6.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(32.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(32.0));

            button
                .height(Size::px(32.0))
                .padding(if is_icon_only {
                    Gaps::new_all(10.0)
                } else {
                    Gaps::new(0.0, 10.0, 0.0, 10.0)
                })
                .corner_radius(8.0)
        }

        ButtonSize::Xs => {
            content = content.spacing(4.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(24.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(24.0));

            button
                .height(Size::px(24.0))
                .padding(if is_icon_only {
                    Gaps::new_all(8.0)
                } else {
                    Gaps::new(0.0, 8.0, 0.0, 8.0)
                })
                .corner_radius(6.0)
        }

        ButtonSize::Sm => {
            content = content.spacing(4.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(28.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(28.0));

            button
                .height(Size::px(28.0))
                .padding(if is_icon_only {
                    Gaps::new_all(10.0)
                } else {
                    Gaps::new(0.0, 10.0, 0.0, 10.0)
                })
                .corner_radius(6.0)
        }

        ButtonSize::Lg => {
            content = content.spacing(6.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(36.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(36.0));

            button
                .height(Size::px(36.0))
                .padding(if is_icon_only {
                    Gaps::new_all(10.0)
                } else {
                    Gaps::new(0.0, 10.0, 0.0, 10.0)
                })
                .corner_radius(8.0)
        }

        ButtonSize::Icon => {
            content = content.spacing(0.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(32.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(32.0));

            button
                .height(Size::px(32.0))
                .padding(if is_icon_only {
                    Gaps::new_all(4.0)
                } else {
                    Gaps::new(0.0, 4.0, 0.0, 4.0)
                })
                .corner_radius(8.0)
        }

        ButtonSize::IconLg => {
            content = content.spacing(0.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(36.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(36.0));

            button
                .height(Size::px(36.0))
                .padding(if is_icon_only {
                    Gaps::new_all(4.0)
                } else {
                    Gaps::new(0.0, 4.0, 0.0, 4.0)
                })
                .corner_radius(8.0)
        }

        ButtonSize::IconSm => {
            content = content.spacing(0.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(28.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(28.0));

            button
                .height(Size::px(28.0))
                .padding(if is_icon_only {
                    Gaps::new_all(4.0)
                } else {
                    Gaps::new(0.0, 4.0, 0.0, 4.0)
                })
                .corner_radius(6.0)
        }

        ButtonSize::IconXs => {
            content = content.spacing(0.0);

            wrapper = wrapper
                .width(if is_icon_only {
                    Size::px(24.0)
                } else {
                    Size::auto()
                })
                .height(Size::px(24.0));

            button
                .height(Size::px(24.0))
                .padding(if is_icon_only {
                    Gaps::new_all(4.0)
                } else {
                    Gaps::new(0.0, 4.0, 0.0, 4.0)
                })
                .corner_radius(6.0)
        }
    };

    // Add children
    for child in children {
        content = content.child(child.clone());
    }

    // Check if some styles are set and then override the ones from the variants if set
    if let Some(cross_align) = &style.cross_align {
        content = content.cross_align(cross_align.clone());
    }

    if let Some(main_align) = &style.main_align {
        content = content.main_align(main_align.clone())
    }

    if let Some(direction) = &style.direction {
        content = content.direction(direction.clone());
    }

    if let Some(spacing) = &style.spacing {
        content = content.spacing(spacing.clone());
    }

    if let Some(text_color) = &style.text_color {
        content = content.color(text_color.clone());
        button = button.color(text_color.clone());
    }

    if let Some(border_color) = &style.border_color {
        wrapper = wrapper.border(style.clone().to_border());
    }

    if let Some(border_width) = &style.border_width {
        wrapper = wrapper.border(style.clone().to_border());
    }

    if let Some(font_size) = &style.font_size {
        content = content.font_size(font_size.clone())
    }

    if let Some(font_slant) = &style.font_slant {
        content = content.font_slant(font_slant.clone())
    }

    if let Some(font_weight) = &style.font_weight {
        content = content.font_weight(font_weight.clone())
    }

    if let Some(text_align) = &style.text_align {
        content = content.text_align(text_align.clone());
    }

    if let Some(text_decoration) = &style.text_decoration {
        content = content.text_decoration(text_decoration.clone());
    }

    if let Some(width) = &style.width {
        wrapper = wrapper.width(width.clone());
    }

    if let Some(height) = &style.height {
        wrapper = wrapper.height(height.clone());
    }

    if let Some(min_width) = &style.min_width {
        wrapper = wrapper.min_width(min_width.clone());
    }

    if let Some(min_height) = &style.min_height {
        wrapper = wrapper.min_height(min_height.clone());
    }

    if let Some(max_width) = &style.max_width {
        wrapper = wrapper.max_width(max_width.clone());
    }

    if let Some(max_height) = &style.max_height {
        wrapper = wrapper.max_height(max_height.clone());
    }

    if let Some(opacity) = &style.opacity {
        wrapper = wrapper.opacity(opacity.clone());
    }

    // confirm background is not default and is not the same with any variant
    if style.background.clone() != Fill::default() && &style.background != &button.background {}

    /*
    // Add children.
        for element in &self.elements {
            content = content.child(element.clone());
        }

        button = button.child(content);

        // ---- Wrap the button in a container to apply extra styles ----
        let mut wrapper = rect()
            .width(self.width_override.clone().unwrap_or_else(|| {
                if is_icon_only {
                    Size::px(height)
                } else {
                    Size::auto()
                }
            }))
            .height(self.height_override.clone().unwrap_or(Size::px(height)));

        // Apply min/max sizing.
        if let Some(min_w) = self.min_width_override.clone() {
            wrapper = wrapper.min_width(min_w);
        }
        if let Some(min_h) = self.min_height_override.clone() {
            wrapper = wrapper.min_height(min_h);
        }
        if let Some(max_w) = self.max_width_override.clone() {
            wrapper = wrapper.max_width(max_w);
        }
        if let Some(max_h) = self.max_height_override.clone() {
            wrapper = wrapper.max_height(max_h);
        }

        if let Some(margin) = self.margin_override {
            wrapper = wrapper.margin(margin);
        }

        if let Some(opacity) = self.opacity {
            wrapper = wrapper.opacity(opacity);
        }

        if let Some(shadow) = self.shadow.clone() {
            wrapper = wrapper.shadow(shadow);
        }

        // ---- Assemble ----
        wrapper.child(button)
     */

    button.child("Labek")
}

/// impl custome methods for the ButtonComponent struct
impl ButtonComponent {
    /// sugar-coated methods
    pub fn variant(self, variant: impl Into<ButtonVariant>) -> Self {
        self._variant(variant.into())
    }
    pub fn size(self, size: impl Into<ButtonSize>) -> Self {
        self._size(size.into())
    }
}
/*
//! A fully-featured button component inspired by shadcn/ui.
//!
//! The button supports multiple variants (default, destructive, outline, secondary, ghost, link)
//! and sizes (default, xs, sm, lg, icon, icon-xs, icon-sm, icon-lg). It integrates with the
//! FreyaCN theming system and all extension traits.

use crate::StyleState;
use crate::extensions::TextStyleExt;
use crate::extensions::*;
use crate::icon_context::IconColorContext;
use crate::theme::use_cn_theme;
use freya::prelude::{Button as ButtonPrimitive, *};

/// Available visual variants for the button, matching shadcn/ui styles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Available size presets for the button, matching shadcn/ui sizes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Default, // h-8, px-2.5
    Xs,      // h-6, px-2
    Sm,      // h-7, px-2.5
    Lg,      // h-9, px-2.5
    Icon,    // size-8
    IconXs,  // size-6
    IconSm,  // size-7
    IconLg,  // size-9
}

#[derive(PartialEq, Clone)]
pub struct CNButton {
    variant: ButtonVariant,
    size: ButtonSize,
    corner_radius: f32,
    elements: Vec<Element>,

    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_secondary_down: Option<EventHandler<Event<PressEventData>>>,
    on_pointer_down: Option<EventHandler<Event<PointerEventData>>>,

    key: DiffKey,
    enabled: bool,
    focusable: bool,
    cursor_icon: CursorIcon,

    style: StyleState,
    text_color: Option<Color>,
    padding_override: Option<Gaps>,
    margin_override: Option<Gaps>,
    width_override: Option<Size>,
    height_override: Option<Size>,
    min_width_override: Option<Size>,
    min_height_override: Option<Size>,
    max_width_override: Option<Size>,
    max_height_override: Option<Size>,
    border_width: Option<f32>,
    border_color: Option<Color>,
    opacity: Option<f32>,
    shadow: Option<Shadow>,
    text_style_data: TextStyleData,
}

impl Default for CNButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CNButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CNButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl CNButton {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            corner_radius: 8.0,
            elements: Vec::new(),
            on_press: None,
            on_secondary_down: None,
            on_pointer_down: None,
            key: DiffKey::None,
            enabled: true,
            focusable: true,
            cursor_icon: CursorIcon::default(),
            style: StyleState::default(),
            text_color: None,
            padding_override: None,
            margin_override: None,
            width_override: None,
            height_override: None,
            min_width_override: None,
            min_height_override: None,
            max_width_override: None,
            max_height_override: None,
            border_width: None,
            border_color: None,
            opacity: None,
            shadow: None,
            text_style_data: Default::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn focusable(mut self, focusable: impl Into<bool>) -> Self {
        self.focusable = focusable.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn on_secondary_down(
        mut self,
        on_secondary_down: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        self.on_secondary_down = Some(on_secondary_down.into());
        self
    }

    pub fn on_pointer_down(
        mut self,
        on_pointer_down: impl Into<EventHandler<Event<PointerEventData>>>,
    ) -> Self {
        self.on_pointer_down = Some(on_pointer_down.into());
        self
    }

    pub fn default(self) -> Self {
        self.variant(ButtonVariant::Default)
    }
    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }
    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }
    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }
    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }
    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }

    pub fn size_default(self) -> Self {
        self.size(ButtonSize::Default)
    }
    pub fn size_xs(self) -> Self {
        self.size(ButtonSize::Xs)
    }
    pub fn size_sm(self) -> Self {
        self.size(ButtonSize::Sm)
    }
    pub fn size_lg(self) -> Self {
        self.size(ButtonSize::Lg)
    }
    pub fn size_icon(self) -> Self {
        self.size(ButtonSize::Icon)
    }
    pub fn size_icon_xs(self) -> Self {
        self.size(ButtonSize::IconXs)
    }
    pub fn size_icon_sm(self) -> Self {
        self.size(ButtonSize::IconSm)
    }
    pub fn size_icon_lg(self) -> Self {
        self.size(ButtonSize::IconLg)
    }

    pub fn cursor_icon(mut self, cursor_icon: impl Into<CursorIcon>) -> Self {
        self.cursor_icon = cursor_icon.into();
        self
    }

    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

// ---- Extension trait implementations ----

impl BackgroundExt for CNButton {
    fn get_style(&mut self) -> &mut StyleState {
        &mut self.style
    }
}

impl TextStyleExt for CNButton {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.text_style_data
    }
}

impl SpacingExt for CNButton {
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding_override = Some(gaps.into());
        self
    }

    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin_override = Some(gaps.into());
        self
    }
}

impl SizingExt for CNButton {
    fn width(mut self, size: impl Into<Size>) -> Self {
        self.width_override = Some(size.into());
        self
    }

    fn height(mut self, size: impl Into<Size>) -> Self {
        self.height_override = Some(size.into());
        self
    }

    fn min_width(mut self, size: impl Into<Size>) -> Self {
        self.min_width_override = Some(size.into());
        self
    }

    fn min_height(mut self, size: impl Into<Size>) -> Self {
        self.min_height_override = Some(size.into());
        self
    }

    fn max_width(mut self, size: impl Into<Size>) -> Self {
        self.max_width_override = Some(size.into());
        self
    }

    fn max_height(mut self, size: impl Into<Size>) -> Self {
        self.max_height_override = Some(size.into());
        self
    }
}

impl BorderExt for CNButton {
    fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    fn corner_radius(mut self, radius: impl Into<CornerRadius>) -> Self {
        let radius = radius.into();
        let uniform = radius
            .top_left
            .max(radius.top_right)
            .max(radius.bottom_left)
            .max(radius.bottom_right);
        self.corner_radius = uniform;
        self
    }
}

impl EffectsExt for CNButton {
    fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }
}

impl CornerRadiusExt for CNButton {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

// ---- Component ----
impl Component for CNButton {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        // Compute variant colors.
        let (bg, hover_bg, text_color) = match self.variant {
            ButtonVariant::Default => {
                let bg = theme.primary;
                let hover = color_with_alpha(bg, 0.8);
                (bg, hover, theme.primary_foreground)
            }
            ButtonVariant::Destructive => {
                let destructive = theme.destructive;
                let bg = color_with_alpha(destructive, 0.1);
                let hover = color_with_alpha(destructive, 0.2);
                (bg, hover, destructive)
            }
            ButtonVariant::Outline => (theme.background, theme.muted, theme.foreground),
            ButtonVariant::Secondary => {
                let bg = theme.secondary;
                let hover = blend_colors(bg, theme.foreground, 0.05);
                (bg, hover, theme.secondary_foreground)
            }
            ButtonVariant::Ghost => (Color::TRANSPARENT, theme.muted, theme.foreground),
            ButtonVariant::Link => (Color::TRANSPARENT, Color::TRANSPARENT, theme.primary),
        };

        let final_bg = bg;
        let final_text_color = self.text_color.unwrap_or(text_color);

        // Provide icon color context.
        provide_context(IconColorContext(final_text_color));

        // Size metrics (matching shadcn React exactly).
        let (height, pad_h, gap, radius) = match self.size {
            ButtonSize::Default => (32.0, 10.0, 6.0, self.corner_radius),
            ButtonSize::Xs => (24.0, 8.0, 4.0, self.corner_radius.min(6.0)),
            ButtonSize::Sm => (28.0, 10.0, 4.0, self.corner_radius.min(6.0)),
            ButtonSize::Lg => (36.0, 10.0, 6.0, self.corner_radius),
            ButtonSize::Icon => (32.0, 4.0, 0.0, self.corner_radius.min(8.0)),
            ButtonSize::IconXs => (24.0, 4.0, 0.0, self.corner_radius.min(6.0)),
            ButtonSize::IconSm => (28.0, 4.0, 0.0, self.corner_radius.min(6.0)),
            ButtonSize::IconLg => (36.0, 4.0, 0.0, self.corner_radius.min(8.0)),
        };

        let is_icon_only = matches!(
            self.size,
            ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg
        );

        // ---- Build the button primitive ----
        let mut button = ButtonPrimitive::new()
            .enabled(self.enabled)
            .focusable(self.focusable)
            .cursor_icon(self.cursor_icon)
            .corner_radius(radius)
            .background(final_bg)
            .hover_background(hover_bg)
            .height(Size::px(height))
            .padding(self.padding_override.unwrap_or_else(|| {
                if is_icon_only {
                    Gaps::new_all(pad_h)
                } else {
                    Gaps::new(0.0, pad_h, 0.0, pad_h)
                }
            }));

        // Apply border_fill only for outline variant.
        if self.variant == ButtonVariant::Outline {
            let border_color = self.border_color.unwrap_or(theme.border);
            button = button.border_fill(border_color);
        } else {
            // For non-outline, ensure no visible border by setting fill to transparent.
            button = button.border_fill(Color::TRANSPARENT);
        }

        // ---- Content container ----
        let mut content = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .main_align(Alignment::Center)
            .spacing(gap)
            .width(Size::auto())
            .height(Size::auto());

        // Add children.
        for element in &self.elements {
            content = content.child(element.clone());
        }

        button = button.child(content);

        // ---- Wrap the button in a container to apply extra styles ----
        let mut wrapper = rect()
            .width(self.width_override.clone().unwrap_or_else(|| {
                if is_icon_only {
                    Size::px(height)
                } else {
                    Size::auto()
                }
            }))
            .height(self.height_override.clone().unwrap_or(Size::px(height)));

        // Apply min/max sizing.
        if let Some(min_w) = self.min_width_override.clone() {
            wrapper = wrapper.min_width(min_w);
        }
        if let Some(min_h) = self.min_height_override.clone() {
            wrapper = wrapper.min_height(min_h);
        }
        if let Some(max_w) = self.max_width_override.clone() {
            wrapper = wrapper.max_width(max_w);
        }
        if let Some(max_h) = self.max_height_override.clone() {
            wrapper = wrapper.max_height(max_h);
        }

        if let Some(margin) = self.margin_override {
            wrapper = wrapper.margin(margin);
        }

        if let Some(opacity) = self.opacity {
            wrapper = wrapper.opacity(opacity);
        }

        if let Some(shadow) = self.shadow.clone() {
            wrapper = wrapper.shadow(shadow);
        }

        // ---- Assemble ----
        wrapper.child(button)
    }
}

#[allow(non_snake_case)]
pub fn Button() -> CNButton {
    CNButton::new()
}

 */
