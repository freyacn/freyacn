/*
//! # Label Component
//!
//! A flexible, theme‑aware label component that supports all styling extensions.

use crate::extensions::*;
use crate::icon_context::IconColorContext;
use crate::theme::use_cn_theme;
use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CNLabel {
    label: String,
    elements: Vec<Element>,

    // ---- Core styling state ----
    background: Option<Color>,
    color: Option<Color>,
    padding: Option<Gaps>,
    margin: Option<Gaps>,
    width: Option<Size>,
    height: Option<Size>,
    min_width: Option<Size>,
    min_height: Option<Size>,
    max_width: Option<Size>,
    max_height: Option<Size>,
    border_width: Option<f32>,
    border_color: Option<Color>,
    corner_radius: Option<CornerRadius>,
    font_size: Option<f32>,
    font_weight: Option<FontWeight>,
    text_align: Option<TextAlign>,
    text_decoration: Option<TextDecoration>,
    opacity: Option<f32>,
    shadow: Option<Shadow>,

    // ---- Key ----
    key: DiffKey,
}

impl CNLabel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            elements: Vec::new(),
            background: None,
            color: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            border_width: None,
            border_color: None,
            corner_radius: None,
            font_size: None,
            font_weight: None,
            text_align: None,
            text_decoration: None,
            opacity: None,
            shadow: None,
            key: DiffKey::None,
        }
    }
}

// ---- Extension trait implementations ----
impl ChildrenExt for CNLabel {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CNLabel {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl BackgroundExt for CNLabel {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
}

impl ForegroundExt for CNLabel {
    fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl SpacingExt for CNLabel {
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding = Some(gaps.into());
        self
    }

    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin = Some(gaps.into());
        self
    }
}

impl SizingExt for CNLabel {
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

impl BorderExt for CNLabel {
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

impl TypographyExt for CNLabel {
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

    fn text_decoration(mut self, decoration: TextDecoration) -> Self {
        self.text_decoration = Some(decoration);
        self
    }
}

impl EffectsExt for CNLabel {
    fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }
}

// ---- Component implementation ----
impl Component for CNLabel {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();
        let icon_color_context = try_consume_context::<IconColorContext>();

        // Build the label element.
        let mut label = label().text(self.label.clone());

        // Apply typography.
        if let Some(size) = self.font_size {
            label = label.font_size(size);
        }
        if let Some(weight) = self.font_weight {
            label = label.font_weight(weight);
        }
        if let Some(align) = self.text_align {
            label = label.text_align(align);
        }
        if let Some(decoration) = self.text_decoration {
            label = label.text_decoration(decoration);
        }

        // Apply foreground color: prefer explicit color, then icon context, then theme foreground.
        let text_color = self
            .color
            .or(icon_color_context.map(|c| c.0))
            .unwrap_or(theme.foreground);
        label = label.color(text_color);

        // Build the container rectangle.
        let mut rect = rect().horizontal().cross_align(Alignment::Center);

        // Apply background.
        if let Some(bg) = self.background {
            rect = rect.background(bg);
        }

        // Apply padding.
        if let Some(padding) = self.padding {
            rect = rect.padding(padding);
        }

        // Apply margin.
        if let Some(margin) = self.margin {
            rect = rect.margin(margin);
        }

        // Apply sizing – clone to avoid moving.
        if let Some(width) = self.width.clone() {
            rect = rect.width(width);
        }
        if let Some(height) = self.height.clone() {
            rect = rect.height(height);
        }
        if let Some(min_width) = self.min_width.clone() {
            rect = rect.min_width(min_width);
        }
        if let Some(min_height) = self.min_height.clone() {
            rect = rect.min_height(min_height);
        }
        if let Some(max_width) = self.max_width.clone() {
            rect = rect.max_width(max_width);
        }
        if let Some(max_height) = self.max_height.clone() {
            rect = rect.max_height(max_height);
        }

        // Apply border (combining width and color) – use Border::new().
        if let (Some(width), Some(color)) = (self.border_width, self.border_color) {
            let border = Border::new().fill(color).width(width);
            rect = rect.border(border);
        }

        // Apply corner radius.
        if let Some(radius) = self.corner_radius {
            rect = rect.corner_radius(radius);
        }

        // Apply opacity.
        if let Some(opacity) = self.opacity {
            rect = rect.opacity(opacity);
        }

        // Apply shadow.
        if let Some(shadow) = self.shadow.clone() {
            rect = rect.shadow(shadow);
        }

        // Add the label as a child.
        rect = rect.child(label);

        // Add any user children.
        for child in &self.elements {
            rect = rect.child(child.clone());
        }

        rect.key(self.key.clone())
    }
}

// ---- Component constructor ----
#[allow(non_snake_case)]
pub fn Label(label: impl Into<String>) -> CNLabel {
    CNLabel::new(label)
}

 */
