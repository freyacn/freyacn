/*
//! # FreyaCN Icon Component
//!
//! A flexible SVG icon component that integrates with the FreyaCN theming system
//! and all extension traits.

use freya::components::SvgViewer;
use freya::prelude::*;

use crate::extensions::*;
use crate::icon_context::IconColorContext;
use crate::theme::use_cn_theme;

/// A flexible SVG icon component.
#[derive(PartialEq, Clone)]
pub struct CNIcon {
    svg_data: Bytes,
    width: f32,
    height: f32,
    color: Option<Color>,

    // ---- Extension state ----
    background: Option<Color>,
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
    corner_radius: Option<CornerRadius>,
    opacity: Option<f32>,
    shadow: Option<Shadow>,

    // ---- Key ----
    key: DiffKey,
}

impl CNIcon {
    pub fn new(icon: Bytes) -> Self {
        Self {
            svg_data: icon,
            width: 20.0,
            height: 20.0,
            color: None,
            background: None,
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
            corner_radius: None,
            opacity: None,
            shadow: None,
            key: DiffKey::None,
        }
    }

    // ---- Sizing ----
    pub fn size(mut self, size: f32) -> Self {
        self.width = size;
        self.height = size;
        self
    }

    pub fn width_inner(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height_inner(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    // ---- Color ----
    pub fn color_inner(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn color_theme(mut self) -> Self {
        self.color = None;
        self
    }

    // ---- Size presets ----
    pub fn size_1(self) -> Self {
        self.size(12.0)
    }
    pub fn size_2(self) -> Self {
        self.size(14.0)
    }
    pub fn size_3(self) -> Self {
        self.size(16.0)
    }
    pub fn size_4(self) -> Self {
        self.size(18.0)
    }
    pub fn size_5(self) -> Self {
        self.size(20.0)
    }
    pub fn size_6(self) -> Self {
        self.size(24.0)
    }
    pub fn size_7(self) -> Self {
        self.size(28.0)
    }
    pub fn size_8(self) -> Self {
        self.size(32.0)
    }
    pub fn size_9(self) -> Self {
        self.size(36.0)
    }

    pub fn size_xs(self) -> Self {
        self.size_1()
    }
    pub fn size_sm(self) -> Self {
        self.size_3()
    }
    pub fn size_md(self) -> Self {
        self.size_5()
    }
    pub fn size_lg(self) -> Self {
        self.size_6()
    }
    pub fn size_xl(self) -> Self {
        self.size_8()
    }
    pub fn size_2xl(self) -> Self {
        self.size(40.0)
    }
    pub fn size_3xl(self) -> Self {
        self.size(48.0)
    }

    pub fn size_12(self) -> Self {
        self.size(12.0)
    }
    pub fn size_14(self) -> Self {
        self.size(14.0)
    }
    pub fn size_16(self) -> Self {
        self.size(16.0)
    }
    pub fn size_18(self) -> Self {
        self.size(18.0)
    }
    pub fn size_20(self) -> Self {
        self.size(20.0)
    }
    pub fn size_24(self) -> Self {
        self.size(24.0)
    }
    pub fn size_28(self) -> Self {
        self.size(28.0)
    }
    pub fn size_32(self) -> Self {
        self.size(32.0)
    }
    pub fn size_36(self) -> Self {
        self.size(36.0)
    }
    pub fn size_40(self) -> Self {
        self.size(40.0)
    }
    pub fn size_48(self) -> Self {
        self.size(48.0)
    }
    pub fn size_64(self) -> Self {
        self.size(64.0)
    }
    pub fn size_96(self) -> Self {
        self.size(96.0)
    }
}

impl KeyExt for CNIcon {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

// ---- Extension implementations ----
impl BackgroundExt for CNIcon {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
}

impl ForegroundExt for CNIcon {
    fn color(self, color: Color) -> Self {
        self.color_inner(color)
    }
}

impl SpacingExt for CNIcon {
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding_override = Some(gaps.into());
        self
    }

    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin_override = Some(gaps.into());
        self
    }
}

impl SizingExt for CNIcon {
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

impl BorderExt for CNIcon {
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

impl EffectsExt for CNIcon {
    fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }
}

// ---- Component ----
impl Component for CNIcon {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        // Determine the icon color.
        let icon_color = if let Some(color) = self.color {
            color
        } else if let Some(ctx) = try_consume_context::<IconColorContext>() {
            ctx.0
        } else {
            theme.foreground
        };

        let svg = SvgViewer::new(self.svg_data.clone())
            .width(Size::px(self.width))
            .height(Size::px(self.height))
            .color(icon_color);

        // Build the container – clone the Options to avoid moves.
        let mut container = rect()
            .child(svg)
            .width(self.width_override.clone().unwrap_or(Size::auto()))
            .height(self.height_override.clone().unwrap_or(Size::auto()));

        // Apply min/max sizing – clone the Option to avoid moving.
        if let Some(min_w) = self.min_width_override.clone() {
            container = container.min_width(min_w);
        }
        if let Some(min_h) = self.min_height_override.clone() {
            container = container.min_height(min_h);
        }
        if let Some(max_w) = self.max_width_override.clone() {
            container = container.max_width(max_w);
        }
        if let Some(max_h) = self.max_height_override.clone() {
            container = container.max_height(max_h);
        }

        // Apply background.
        if let Some(bg) = self.background {
            container = container.background(bg);
        }

        // Apply padding.
        if let Some(padding) = self.padding_override {
            container = container.padding(padding);
        }

        // Apply margin.
        if let Some(margin) = self.margin_override {
            container = container.margin(margin);
        }

        // Apply border (combine width and color).
        if let (Some(width), Some(color)) = (self.border_width, self.border_color) {
            // Border::new() expects BorderWidth, which can be created via .into().
            let border = Border::new().fill(color).width(width);
            container = container.border(border);
        }

        // Apply corner radius.
        if let Some(radius) = self.corner_radius {
            container = container.corner_radius(radius);
        }

        // Apply opacity.
        if let Some(opacity) = self.opacity {
            container = container.opacity(opacity);
        }

        // Apply shadow.
        if let Some(shadow) = self.shadow.clone() {
            container = container.shadow(shadow);
        }

        container.key(self.key.clone())
    }
}

/// Constructor for the Icon component.
#[allow(non_snake_case)]
pub fn Icon(icon: Bytes) -> CNIcon {
    CNIcon::new(icon)
}

 */
