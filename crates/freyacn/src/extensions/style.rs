//! # Unified Style Extension Trait
//!
//! Fixed for **Freya 0.4.3**.

use std::borrow::Cow;

use crate::theme::use_cn_theme;
use freya::prelude::{
    Alignment, Border, Content, CornerRadius, Direction, Fill, FontSize, FontSlant, FontWeight,
    Gaps, Overflow, Position, Shadow, Size, TextAlign, TextDecoration,
};

use freya_core::prelude::Color;
use paste::paste;
use torin::values::position::PositionSides;

const SPACING_UNIT: f32 = 4.0;

// ==============================================================
// Local enums
// ==============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    None,
}

// ==============================================================
// Macros — inherent Style methods
// ==============================================================

macro_rules! style_palette_family {
    ($prefix:ident, $field:ident, $family:ident) => {
        style_palette_family! { $prefix, $field, $family; 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950 }
    };
    ($prefix:ident, $field:ident, $family:ident; $($shade:literal),* $(,)?) => {
        paste! {
            $(
                #[doc = concat!("Palette colour helper — `", stringify!($prefix), "_", stringify!($family), "_", stringify!($shade), "`.")]
                pub fn [<$prefix _ $family _ $shade>](mut self) -> Self {
                    let theme = use_cn_theme().read();
                    self.$field = Some(theme.colors.[<$family _ $shade>].into());
                    self
                }
            )*
        }
    };
}

macro_rules! style_semantic_color {
    ($prefix:ident, $field:ident, $($token:ident),* $(,)?) => {
        $(
            paste! {
                pub fn [<$prefix _ $token>](mut self) -> Self {
                    let theme = use_cn_theme().read();
                    self.$field = Some(theme.$token.into());
                    self
                }
            }
        )*
    };
}

/// Spacing scale macro.
///
/// **Fix**: `$name` is the identifier fragment (e.g. `0_5`), `$value` is the
/// numeric value (e.g. `0.5`). `paste!` cannot concatenate float literals,
/// but `0_5` is a valid integer-literal token.
macro_rules! style_spacing_scale {
    ($prefix:ident, $base:ident, $(($name:literal, $value:expr)),* $(,)?) => {
        paste! {
            $(
                #[doc = concat!("Spacing scale helper — `", stringify!($prefix), "_", stringify!($name), "`.")]
                pub fn [<$prefix _ $name>](self) -> Self {
                    self.$base(SPACING_UNIT * ($value as f32))
                }
            )*
        }
    };
}

// ==============================================================
// Macros — StyleExt provided methods
// ==============================================================

macro_rules! ext_palette_family {
    ($prefix:ident, $family:ident) => {
        ext_palette_family! { $prefix, $family; 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950 }
    };
    ($prefix:ident, $family:ident; $($shade:literal),* $(,)?) => {
        paste! {
            $(
                fn [<$prefix _ $family _ $shade>](self) -> Self {
                    self.map_style(|s| s.[<$prefix _ $family _ $shade>]())
                }
            )*
        }
    };
}

macro_rules! ext_semantic_color {
    ($prefix:ident, $($token:ident),* $(,)?) => {
        $(
            paste! {
                fn [<$prefix _ $token>](self) -> Self {
                    self.map_style(|s| s.[<$prefix _ $token>]())
                }
            }
        )*
    };
}

macro_rules! ext_spacing_scale {
    ($prefix:ident, $base:ident, $(($name:literal, $value:expr)),* $(,)?) => {
        paste! {
            $(
                fn [<$prefix _ $name>](self) -> Self {
                    self.$base(SPACING_UNIT * ($value as f32))
                }
            )*
        }
    };
}

// ==============================================================
// Style: the data container
// ==============================================================

#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    // ---- Box styling ----
    pub background: Option<Fill>,
    pub corner_radius: Option<CornerRadius>,
    pub border_width: Option<f32>,
    pub border_top_width: Option<f32>,
    pub border_right_width: Option<f32>,
    pub border_bottom_width: Option<f32>,
    pub border_left_width: Option<f32>,
    pub border_color: Option<Color>,
    pub border_style: Option<BorderStyle>,
    pub shadows: Vec<Shadow>,

    // ---- Ring ----
    pub ring_width: Option<f32>,
    pub ring_color: Option<Color>,
    pub ring_offset: Option<f32>,

    // ---- Text ----
    pub text_color: Option<Color>,
    pub font_size: Option<FontSize>,
    pub font_slant: Option<FontSlant>,
    pub font_weight: Option<FontWeight>,
    pub font_families: Vec<Cow<'static, str>>,
    pub text_align: Option<TextAlign>,
    pub text_decoration: Option<TextDecoration>,
    pub text_transform: Option<TextTransform>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
    pub text_indent: Option<f32>,

    // ---- Spacing ----
    pub padding: Option<Gaps>,
    pub margin: Option<Gaps>,

    // ---- Sizing ----
    pub width: Option<Size>,
    pub height: Option<Size>,
    pub min_width: Option<Size>,
    pub min_height: Option<Size>,
    pub max_width: Option<Size>,
    pub max_height: Option<Size>,

    // ---- Positioning ----
    // **Fix**: position sides are `Option<f32>` and merged into `PositionSides`
    // when a `Position` variant is applied.
    pub position: Option<Position>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,

    // ---- Overflow ----
    // **Fix**: `Overflow` has only `None` and `Clip`.
    pub overflow: Option<Overflow>,

    // ---- Flex ----
    pub direction: Option<Direction>,
    pub main_align: Option<Alignment>,
    pub cross_align: Option<Alignment>,
    pub align_self: Option<Alignment>,
    pub align_content: Option<Alignment>,

    /// **Fix**: replaced `Option<Wrap>` with `Option<Content>` — Freya uses
    /// `Content::Wrap { wrap_spacing }` for wrapping, and `Content::Flex`
    /// for flex content mode.
    pub content: Option<Content>,

    pub spacing: Option<f32>,
    pub gap_x: Option<f32>,
    pub gap_y: Option<f32>,

    // ---- Effects ----
    pub opacity: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            corner_radius: None,
            border_width: None,
            border_top_width: None,
            border_right_width: None,
            border_bottom_width: None,
            border_left_width: None,
            border_color: None,
            border_style: None,
            shadows: Vec::new(),
            ring_width: None,
            ring_color: None,
            ring_offset: None,
            text_color: None,
            font_size: None,
            font_slant: None,
            font_weight: None,
            font_families: Vec::new(),
            text_align: None,
            text_decoration: None,
            text_transform: None,
            line_height: None,
            letter_spacing: None,
            text_indent: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            position: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            z_index: None,
            overflow: None,
            direction: None,
            main_align: None,
            cross_align: None,
            align_self: None,
            align_content: None,
            content: None,
            spacing: None,
            gap_x: None,
            gap_y: None,
            opacity: None,
        }
    }
}

// ==============================================================
// Inherent Style methods
// ==============================================================

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_border(&self) -> Option<Border> {
        let width = self.border_width.or_else(|| {
            [
                self.border_top_width,
                self.border_right_width,
                self.border_bottom_width,
                self.border_left_width,
            ]
            .into_iter()
            .flatten()
            .reduce(f32::max)
        })?;
        let mut b = Border::default().width(width);
        if let Some(colour) = self.border_color {
            b = b.fill(colour);
        }
        Some(b)
    }

    // ==============================================================
    // Merging
    // ==============================================================

    /// Merge `other` into `self`, with **`other`'s set values winning**.
    ///
    /// This is the shape you want when layering styles: base + variant +
    /// size, where each successive layer overrides what the previous one
    /// set and leaves the rest alone.
    ///
    /// # Semantics
    ///
    /// * **Scalar fields** (`Option<T>`): `other.field.or(self.field)`.
    ///   If `other` set the field, it wins. Otherwise `self`'s value stays.
    /// * **Stack fields** (`shadows`, `font_families`): concatenated.
    ///
    /// # Example
    ///
    /// ```
    /// # use freyacn::{Style, StyleExt};
    /// let base = Style::default().p_4().bg_primary();
    /// let variant = Style::default().p_6().bg_destructive();
    ///
    /// let merged = base.merge(variant);
    /// // padding changed to 6·unit — variant won
    /// // background changed to destructive — variant won
    /// ```
    pub fn merge(self, other: Style) -> Style {
        let mut shadows = self.shadows;
        shadows.extend(other.shadows);
        let mut font_families = self.font_families;
        font_families.extend(other.font_families);

        Style {
            background: other.background.or(self.background),
            corner_radius: other.corner_radius.or(self.corner_radius),

            border_width: other.border_width.or(self.border_width),
            border_top_width: other.border_top_width.or(self.border_top_width),
            border_right_width: other.border_right_width.or(self.border_right_width),
            border_bottom_width: other.border_bottom_width.or(self.border_bottom_width),
            border_left_width: other.border_left_width.or(self.border_left_width),
            border_color: other.border_color.or(self.border_color),
            border_style: other.border_style.or(self.border_style),
            shadows,

            ring_width: other.ring_width.or(self.ring_width),
            ring_color: other.ring_color.or(self.ring_color),
            ring_offset: other.ring_offset.or(self.ring_offset),

            text_color: other.text_color.or(self.text_color),
            font_size: other.font_size.or(self.font_size),
            font_slant: other.font_slant.or(self.font_slant),
            font_weight: other.font_weight.or(self.font_weight),
            font_families,
            text_align: other.text_align.or(self.text_align),
            text_decoration: other.text_decoration.or(self.text_decoration),
            text_transform: other.text_transform.or(self.text_transform),
            line_height: other.line_height.or(self.line_height),
            letter_spacing: other.letter_spacing.or(self.letter_spacing),
            text_indent: other.text_indent.or(self.text_indent),

            padding: other.padding.or(self.padding),
            margin: other.margin.or(self.margin),

            width: other.width.or(self.width),
            height: other.height.or(self.height),
            min_width: other.min_width.or(self.min_width),
            min_height: other.min_height.or(self.min_height),
            max_width: other.max_width.or(self.max_width),
            max_height: other.max_height.or(self.max_height),

            position: other.position.or(self.position),
            top: other.top.or(self.top),
            right: other.right.or(self.right),
            bottom: other.bottom.or(self.bottom),
            left: other.left.or(self.left),
            z_index: other.z_index.or(self.z_index),

            overflow: other.overflow.or(self.overflow),

            direction: other.direction.or(self.direction),
            main_align: other.main_align.or(self.main_align),
            cross_align: other.cross_align.or(self.cross_align),
            align_self: other.align_self.or(self.align_self),
            align_content: other.align_content.or(self.align_content),
            content: other.content.or(self.content),
            spacing: other.spacing.or(self.spacing),
            gap_x: other.gap_x.or(self.gap_x),
            gap_y: other.gap_y.or(self.gap_y),

            opacity: other.opacity.or(self.opacity),
        }
    }

    /// Merge `defaults` into `self`, with **`self`'s set values winning**.
    ///
    /// The inverse of [`merge`](Self::merge). `defaults` only fills in
    /// fields that `self` left unset.
    ///
    /// ```
    /// # use freyacn::{Style, StyleExt};
    /// let component_defaults = Style::default().text_sm().bg_background();
    /// let user_override = Style::default().text_lg();
    ///
    /// let final_style = user_override.or(component_defaults);
    /// // font_size = lg           — user won
    /// // background = background  — default filled the gap
    /// ```
    pub fn or(self, defaults: Style) -> Style {
        defaults.merge(self)
    }

    /// In-place variant of [`merge`](Self::merge).
    ///
    /// ```ignore
    /// let mut style = Style::default();
    /// if hovered  { style.merge_from(hover_style); }
    /// if disabled { style.merge_from(disabled_style); }
    /// ```
    pub fn merge_from(&mut self, other: Style) {
        let taken = std::mem::take(self);
        *self = taken.merge(other);
    }

    /// In-place variant of [`or`](Self::or).
    pub fn or_from(&mut self, defaults: Style) {
        let taken = std::mem::take(self);
        *self = taken.or(defaults);
    }

    /// Returns `true` if no field has been set.
    ///
    /// ```
    /// # use freyacn::{Style, StyleExt};
    /// assert!(Style::default().is_empty());
    /// assert!(!Style::default().p_4().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        *self == Style::default()
    }

    /// Parse a Tailwind-style string, returning both the styled value and
    /// the list of unrecognised tokens.
    ///
    /// The regular `style(...)` method silently ignores unknown tokens.
    /// `style_debug` gives you the same result plus the tokens that had no
    /// effect, so you can warn or log during development.
    ///
    /// ```
    /// # use freyacn::Style;
    /// let (style, unknown) = Style::default().style_debug(
    ///     "p-4 bg-primary shdaow-md not-a-thing",
    /// );
    /// // style has p-4 and bg-primary applied
    /// // unknown contains "shdaow-md" and "not-a-thing"
    /// ```
    pub fn style_debug(mut self, styles: impl AsRef<str>) -> (Self, Vec<String>) {
        let mut unknown = Vec::new();
        for token in styles.as_ref().split_whitespace() {
            if !is_known_token(token) {
                unknown.push(token.to_owned());
            }
            self = style_token(self, token);
        }
        (self, unknown)
    }

    // ---- Base setters ----
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color.into());
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
    pub fn corner_radius(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(radius.into());
        self
    }
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = Some(style);
        self
    }
    pub fn font_size(mut self, size: impl Into<FontSize>) -> Self {
        self.font_size = Some(size.into());
        self
    }
    pub fn font_weight(mut self, weight: impl Into<FontWeight>) -> Self {
        self.font_weight = Some(weight.into());
        self
    }
    pub fn font_slant(mut self, slant: impl Into<FontSlant>) -> Self {
        self.font_slant = Some(slant.into());
        self
    }
    pub fn font_family(mut self, family: impl Into<Cow<'static, str>>) -> Self {
        self.font_families.push(family.into());
        self
    }
    pub fn text_align(mut self, align: impl Into<TextAlign>) -> Self {
        self.text_align = Some(align.into());
        self
    }
    pub fn text_decoration(mut self, decoration: impl Into<TextDecoration>) -> Self {
        self.text_decoration = Some(decoration.into());
        self
    }
    pub fn text_transform(mut self, transform: TextTransform) -> Self {
        self.text_transform = Some(transform);
        self
    }
    pub fn line_height(mut self, value: f32) -> Self {
        self.line_height = Some(value);
        self
    }
    pub fn letter_spacing(mut self, value: f32) -> Self {
        self.letter_spacing = Some(value);
        self
    }
    pub fn text_indent(mut self, value: f32) -> Self {
        self.text_indent = Some(value);
        self
    }
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }
    pub fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadows.push(shadow.into());
        self
    }
    pub fn ring_width(mut self, width: f32) -> Self {
        self.ring_width = Some(width);
        self
    }
    pub fn ring_color(mut self, color: Color) -> Self {
        self.ring_color = Some(color);
        self
    }
    pub fn ring_offset(mut self, offset: f32) -> Self {
        self.ring_offset = Some(offset);
        self
    }
    pub fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding = Some(gaps.into());
        self
    }
    pub fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin = Some(gaps.into());
        self
    }
    pub fn width(mut self, size: impl Into<Size>) -> Self {
        self.width = Some(size.into());
        self
    }
    pub fn height(mut self, size: impl Into<Size>) -> Self {
        self.height = Some(size.into());
        self
    }
    pub fn min_width(mut self, size: impl Into<Size>) -> Self {
        self.min_width = Some(size.into());
        self
    }
    pub fn min_height(mut self, size: impl Into<Size>) -> Self {
        self.min_height = Some(size.into());
        self
    }
    pub fn max_width(mut self, size: impl Into<Size>) -> Self {
        self.max_width = Some(size.into());
        self
    }
    pub fn max_height(mut self, size: impl Into<Size>) -> Self {
        self.max_height = Some(size.into());
        self
    }
    pub fn position(mut self, pos: Position) -> Self {
        self.position = Some(pos);
        self
    }
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = Some(z);
        self
    }
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = Some(dir);
        self
    }
    pub fn main_align(mut self, align: Alignment) -> Self {
        self.main_align = Some(align);
        self
    }
    pub fn cross_align(mut self, align: Alignment) -> Self {
        self.cross_align = Some(align);
        self
    }
    pub fn align_self(mut self, align: Alignment) -> Self {
        self.align_self = Some(align);
        self
    }
    pub fn align_content(mut self, align: Alignment) -> Self {
        self.align_content = Some(align);
        self
    }
    pub fn content(mut self, content: Content) -> Self {
        self.content = Some(content);
        self
    }
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }
    pub fn gap_x(mut self, size: f32) -> Self {
        self.gap_x = Some(size);
        self
    }
    pub fn gap_y(mut self, size: f32) -> Self {
        self.gap_y = Some(size);
        self
    }

    // ---- Literal colours ----
    pub fn text_white(self) -> Self {
        let t = use_cn_theme().read();
        self.color(t.colors.white)
    }
    pub fn text_black(self) -> Self {
        let t = use_cn_theme().read();
        self.color(t.colors.black)
    }
    pub fn bg_white(self) -> Self {
        let t = use_cn_theme().read();
        self.background(t.colors.white)
    }
    pub fn bg_black(self) -> Self {
        let t = use_cn_theme().read();
        self.background(t.colors.black)
    }
    pub fn bg_transparent(self) -> Self {
        self.background(Color::TRANSPARENT)
    }
    pub fn text_transparent(self) -> Self {
        self.color(Color::TRANSPARENT)
    }

    // ---- Typography sizes ----
    pub fn text_xs(self) -> Self {
        self.font_size(12.0)
    }
    pub fn text_sm(self) -> Self {
        self.font_size(14.0)
    }
    pub fn text_base(self) -> Self {
        self.font_size(16.0)
    }
    pub fn text_lg(self) -> Self {
        self.font_size(18.0)
    }
    pub fn text_xl(self) -> Self {
        self.font_size(20.0)
    }
    pub fn text_2xl(self) -> Self {
        self.font_size(24.0)
    }
    pub fn text_3xl(self) -> Self {
        self.font_size(30.0)
    }
    pub fn text_4xl(self) -> Self {
        self.font_size(36.0)
    }
    pub fn text_5xl(self) -> Self {
        self.font_size(48.0)
    }
    pub fn text_6xl(self) -> Self {
        self.font_size(60.0)
    }
    pub fn text_7xl(self) -> Self {
        self.font_size(72.0)
    }
    pub fn text_8xl(self) -> Self {
        self.font_size(96.0)
    }
    pub fn text_9xl(self) -> Self {
        self.font_size(128.0)
    }

    // ---- Typography weights ----
    pub fn font_thin(self) -> Self {
        self.font_weight(FontWeight::THIN)
    }
    pub fn font_extralight(self) -> Self {
        self.font_weight(FontWeight::EXTRA_LIGHT)
    }
    pub fn font_light(self) -> Self {
        self.font_weight(FontWeight::LIGHT)
    }
    pub fn font_normal(self) -> Self {
        self.font_weight(FontWeight::NORMAL)
    }
    pub fn font_medium(self) -> Self {
        self.font_weight(FontWeight::MEDIUM)
    }
    pub fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SEMI_BOLD)
    }
    pub fn font_bold(self) -> Self {
        self.font_weight(FontWeight::BOLD)
    }
    pub fn font_extrabold(self) -> Self {
        self.font_weight(FontWeight::EXTRA_BOLD)
    }
    pub fn font_black_weight(self) -> Self {
        self.font_weight(FontWeight::BLACK)
    }

    // ---- Typography slant/family/align/decoration/transform ----
    pub fn italic(self) -> Self {
        self.font_slant(FontSlant::Italic)
    }
    pub fn not_italic(self) -> Self {
        self.font_slant(FontSlant::Upright)
    }
    pub fn font_sans(self) -> Self {
        self.font_family("sans-serif")
    }
    pub fn font_serif(self) -> Self {
        self.font_family("serif")
    }
    pub fn font_mono(self) -> Self {
        self.font_family("monospace")
    }
    pub fn text_left(self) -> Self {
        self.text_align(TextAlign::Left)
    }
    pub fn text_center(self) -> Self {
        self.text_align(TextAlign::Center)
    }
    pub fn text_right(self) -> Self {
        self.text_align(TextAlign::Right)
    }
    pub fn text_justify(self) -> Self {
        self.text_align(TextAlign::Justify)
    }
    pub fn underline(self) -> Self {
        self.text_decoration(TextDecoration::Underline)
    }
    pub fn overline(self) -> Self {
        self.text_decoration(TextDecoration::Overline)
    }
    pub fn line_through(self) -> Self {
        self.text_decoration(TextDecoration::LineThrough)
    }
    pub fn no_underline(self) -> Self {
        self.text_decoration(TextDecoration::None)
    }
    pub fn uppercase(self) -> Self {
        self.text_transform(TextTransform::Uppercase)
    }
    pub fn lowercase(self) -> Self {
        self.text_transform(TextTransform::Lowercase)
    }
    pub fn capitalize(self) -> Self {
        self.text_transform(TextTransform::Capitalize)
    }
    pub fn normal_case(self) -> Self {
        self.text_transform(TextTransform::None)
    }

    // ---- Borders ----
    pub fn border(mut self) -> Self {
        self.border_width = Some(1.0);
        self
    }
    pub fn border_0(self) -> Self {
        self.border_width(0.0)
    }
    pub fn border_2(self) -> Self {
        self.border_width(2.0)
    }
    pub fn border_4(self) -> Self {
        self.border_width(4.0)
    }
    pub fn border_8(self) -> Self {
        self.border_width(8.0)
    }
    pub fn border_primary(self) -> Self {
        let t = use_cn_theme().read();
        self.border_color(t.primary)
    }
    pub fn border_destructive(self) -> Self {
        let t = use_cn_theme().read();
        self.border_color(t.destructive)
    }
    pub fn border_solid(self) -> Self {
        self.border_style(BorderStyle::Solid)
    }
    pub fn border_dashed(self) -> Self {
        self.border_style(BorderStyle::Dashed)
    }
    pub fn border_dotted(self) -> Self {
        self.border_style(BorderStyle::Dotted)
    }
    pub fn border_double(self) -> Self {
        self.border_style(BorderStyle::Double)
    }
    pub fn border_none(self) -> Self {
        self.border_style(BorderStyle::None)
    }
    pub fn border_t(mut self, w: f32) -> Self {
        self.border_top_width = Some(w);
        self
    }
    pub fn border_r(mut self, w: f32) -> Self {
        self.border_right_width = Some(w);
        self
    }
    pub fn border_b(mut self, w: f32) -> Self {
        self.border_bottom_width = Some(w);
        self
    }
    pub fn border_l(mut self, w: f32) -> Self {
        self.border_left_width = Some(w);
        self
    }
    pub fn border_x(mut self, w: f32) -> Self {
        self.border_left_width = Some(w);
        self.border_right_width = Some(w);
        self
    }
    pub fn border_y(mut self, w: f32) -> Self {
        self.border_top_width = Some(w);
        self.border_bottom_width = Some(w);
        self
    }

    // ---- Corner radius ----
    pub fn rounded_none(self) -> Self {
        self.corner_radius(0.0)
    }
    pub fn rounded_sm(self) -> Self {
        self.corner_radius(2.0)
    }
    pub fn rounded(self) -> Self {
        self.corner_radius(4.0)
    }
    pub fn rounded_md(self) -> Self {
        self.corner_radius(6.0)
    }
    pub fn rounded_lg(self) -> Self {
        self.corner_radius(8.0)
    }
    pub fn rounded_xl(self) -> Self {
        self.corner_radius(12.0)
    }
    pub fn rounded_2xl(self) -> Self {
        self.corner_radius(16.0)
    }
    pub fn rounded_3xl(self) -> Self {
        self.corner_radius(24.0)
    }
    pub fn rounded_full(self) -> Self {
        self.corner_radius(9999.0)
    }

    // ---- Effects ----
    pub fn opacity_0(self) -> Self {
        self.opacity(0.0)
    }
    pub fn opacity_25(self) -> Self {
        self.opacity(0.25)
    }
    pub fn opacity_50(self) -> Self {
        self.opacity(0.5)
    }
    pub fn opacity_75(self) -> Self {
        self.opacity(0.75)
    }
    pub fn opacity_100(self) -> Self {
        self.opacity(1.0)
    }
    pub fn shadow_sm(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(1.0)
                .blur(2.0)
                .spread(0.0)
                .color((0, 0, 0, 0.30)),
        )
    }
    pub fn shadow_md(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(4.0)
                .blur(6.0)
                .spread(-1.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    pub fn shadow_lg(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(10.0)
                .blur(15.0)
                .spread(-3.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    pub fn shadow_xl(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(20.0)
                .blur(25.0)
                .spread(-5.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    pub fn shadow_2xl(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(25.0)
                .blur(50.0)
                .spread(-12.0)
                .color((0, 0, 0, 0.25)),
        )
    }
    pub fn shadow_none(mut self) -> Self {
        self.shadows.clear();
        self
    }

    // ---- Ring ----
    pub fn ring(self) -> Self {
        self.ring_width(3.0)
    }
    pub fn ring_0(self) -> Self {
        self.ring_width(0.0)
    }
    pub fn ring_1(self) -> Self {
        self.ring_width(1.0)
    }
    pub fn ring_2(self) -> Self {
        self.ring_width(2.0)
    }
    pub fn ring_4(self) -> Self {
        self.ring_width(4.0)
    }
    pub fn ring_8(self) -> Self {
        self.ring_width(8.0)
    }
    pub fn ring_primary(self) -> Self {
        let t = use_cn_theme().read();
        let c = t.primary;
        drop(t);
        self.ring_color(c)
    }

    // ---- Flex ----
    pub fn flex_col(self) -> Self {
        self.direction(Direction::Vertical)
    }
    pub fn flex_row(self) -> Self {
        self.direction(Direction::Horizontal)
    }

    /// **Fix**: uses `Content::Wrap { wrap_spacing: None }` instead of a
    /// non-existent `Wrap` enum.
    pub fn flex_wrap_content(self) -> Self {
        self.content(Content::Wrap { wrap_spacing: None })
    }

    /// Wrap with a spacing between wrapped lines.
    pub fn flex_wrap_spacing(self, spacing: f32) -> Self {
        self.content(Content::Wrap {
            wrap_spacing: Some(spacing),
        })
    }

    /// Set the content mode to flex.
    pub fn content_flex(self) -> Self {
        self.content(Content::Flex)
    }
    /// Set the content mode to fit.
    pub fn content_fit(self) -> Self {
        self.content(Content::Fit)
    }
    /// Set the content mode to normal (default).
    pub fn content_normal(self) -> Self {
        self.content(Content::Normal)
    }

    /// `flex-1` — grow to fill available space.
    pub fn flex_1(self) -> Self {
        self.width(Size::flex(1.0)).height(Size::flex(1.0))
    }
    /// `grow` — width grows to fill.
    pub fn grow(self) -> Self {
        self.width(Size::flex(1.0))
    }
    /// `grow-0` — no growth.
    pub fn grow_0(self) -> Self {
        self.width(Size::px(0.0))
    }

    pub fn justify_start(self) -> Self {
        self.main_align(Alignment::Start)
    }
    pub fn justify_center(self) -> Self {
        self.main_align(Alignment::Center)
    }
    pub fn justify_end(self) -> Self {
        self.main_align(Alignment::End)
    }
    pub fn justify_between(self) -> Self {
        self.main_align(Alignment::SpaceBetween)
    }
    pub fn justify_around(self) -> Self {
        self.main_align(Alignment::SpaceAround)
    }
    pub fn justify_evenly(self) -> Self {
        self.main_align(Alignment::SpaceEvenly)
    }
    pub fn items_start(self) -> Self {
        self.cross_align(Alignment::Start)
    }
    pub fn items_center(self) -> Self {
        self.cross_align(Alignment::Center)
    }
    pub fn items_end(self) -> Self {
        self.cross_align(Alignment::End)
    }
    pub fn self_start(self) -> Self {
        self.align_self(Alignment::Start)
    }
    pub fn self_center(self) -> Self {
        self.align_self(Alignment::Center)
    }
    pub fn self_end(self) -> Self {
        self.align_self(Alignment::End)
    }
    pub fn content_start(self) -> Self {
        self.align_content(Alignment::Start)
    }
    pub fn content_center(self) -> Self {
        self.align_content(Alignment::Center)
    }
    pub fn content_end(self) -> Self {
        self.align_content(Alignment::End)
    }
    pub fn content_between(self) -> Self {
        self.align_content(Alignment::SpaceBetween)
    }
    pub fn content_around(self) -> Self {
        self.align_content(Alignment::SpaceAround)
    }
    pub fn content_evenly(self) -> Self {
        self.align_content(Alignment::SpaceEvenly)
    }
    pub fn gap(self, size: f32) -> Self {
        self.spacing(size)
    }

    // ---- Sizing ----
    pub fn w(self, px: f32) -> Self {
        self.width(Size::px(px))
    }
    pub fn h(self, px: f32) -> Self {
        self.height(Size::px(px))
    }
    pub fn min_w(self, px: f32) -> Self {
        self.min_width(Size::px(px))
    }
    pub fn min_h(self, px: f32) -> Self {
        self.min_height(Size::px(px))
    }
    pub fn max_w(self, px: f32) -> Self {
        self.max_width(Size::px(px))
    }
    pub fn max_h(self, px: f32) -> Self {
        self.max_height(Size::px(px))
    }
    pub fn w_percent(self, pct: f32) -> Self {
        self.width(Size::percent(pct))
    }
    pub fn h_percent(self, pct: f32) -> Self {
        self.height(Size::percent(pct))
    }
    pub fn w_full(self) -> Self {
        self.width(Size::percent(100.0))
    }
    pub fn w_auto(self) -> Self {
        self.width(Size::auto())
    }
    pub fn h_full(self) -> Self {
        self.height(Size::percent(100.0))
    }
    pub fn h_auto(self) -> Self {
        self.height(Size::auto())
    }
    pub fn size(self, px: f32) -> Self {
        self.w(px).h(px)
    }
    pub fn size_full(self) -> Self {
        self.w_full().h_full()
    }

    // ---- Positioning ----
    //
    // **Fix**: `Position::Stacked`, `Position::Absolute`, `Position::Global`
    // all wrap `Box<PositionSides>`. The sides are merged from the
    // `top/right/bottom/left` fields.

    fn merge_sides(&self) -> PositionSides {
        PositionSides {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
        }
    }

    /// Stacked positioning (Freya's default — the closest to CSS `relative`).
    pub fn relative(mut self) -> Self {
        let sides = self.merge_sides();
        self.position = Some(Position::Stacked(Box::new(sides)));
        self
    }

    /// Absolute positioning relative to the nearest positioned ancestor.
    pub fn absolute(mut self) -> Self {
        let sides = self.merge_sides();
        self.position = Some(Position::Absolute(Box::new(sides)));
        self
    }

    /// Global positioning — relative to the window.
    pub fn global(mut self) -> Self {
        let sides = self.merge_sides();
        self.position = Some(Position::Global(Box::new(sides)));
        self
    }

    pub fn top(mut self, value: f32) -> Self {
        self.top = Some(value);
        self
    }
    pub fn right(mut self, value: f32) -> Self {
        self.right = Some(value);
        self
    }
    pub fn bottom(mut self, value: f32) -> Self {
        self.bottom = Some(value);
        self
    }
    pub fn left(mut self, value: f32) -> Self {
        self.left = Some(value);
        self
    }

    pub fn inset(mut self, value: f32) -> Self {
        self.top = Some(value);
        self.right = Some(value);
        self.bottom = Some(value);
        self.left = Some(value);
        self
    }
    pub fn inset_x(mut self, value: f32) -> Self {
        self.right = Some(value);
        self.left = Some(value);
        self
    }
    pub fn inset_y(mut self, value: f32) -> Self {
        self.top = Some(value);
        self.bottom = Some(value);
        self
    }

    // ---- Overflow ----
    //
    // **Fix**: `Overflow` has only `None` and `Clip`.
    pub fn overflow_none(self) -> Self {
        self.overflow(Overflow::None)
    }
    pub fn overflow_clip(self) -> Self {
        self.overflow(Overflow::Clip)
    }

    // ---- Spacing ----
    pub fn p(self, size: f32) -> Self {
        self.padding(Gaps::new_all(size))
    }
    pub fn px(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, size, 0.0))
    }
    pub fn py(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, size))
    }
    pub fn pt(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, 0.0, 0.0))
    }
    pub fn pr(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, 0.0))
    }
    pub fn pb(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, size, 0.0))
    }
    pub fn pl(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, 0.0, size))
    }
    pub fn m(self, size: f32) -> Self {
        self.margin(Gaps::new_all(size))
    }
    pub fn mx(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, size, 0.0))
    }
    pub fn my(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, size))
    }
    pub fn mt(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, 0.0, 0.0))
    }
    pub fn mr(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, 0.0))
    }
    pub fn mb(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, size, 0.0))
    }
    pub fn ml(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, 0.0, size))
    }

    // ---- Palette / semantic colours ----
    style_palette_family!(text, text_color, slate);
    style_palette_family!(text, text_color, gray);
    style_palette_family!(text, text_color, zinc);
    style_palette_family!(text, text_color, neutral);
    style_palette_family!(text, text_color, stone);
    style_palette_family!(text, text_color, mauve);
    style_palette_family!(text, text_color, olive);
    style_palette_family!(text, text_color, mist);
    style_palette_family!(text, text_color, taupe);
    style_palette_family!(text, text_color, red);
    style_palette_family!(text, text_color, orange);
    style_palette_family!(text, text_color, amber);
    style_palette_family!(text, text_color, yellow);
    style_palette_family!(text, text_color, lime);
    style_palette_family!(text, text_color, green);
    style_palette_family!(text, text_color, emerald);
    style_palette_family!(text, text_color, teal);
    style_palette_family!(text, text_color, cyan);
    style_palette_family!(text, text_color, sky);
    style_palette_family!(text, text_color, blue);
    style_palette_family!(text, text_color, indigo);
    style_palette_family!(text, text_color, violet);
    style_palette_family!(text, text_color, purple);
    style_palette_family!(text, text_color, fuchsia);
    style_palette_family!(text, text_color, pink);
    style_palette_family!(text, text_color, rose);

    style_palette_family!(bg, background, slate);
    style_palette_family!(bg, background, gray);
    style_palette_family!(bg, background, zinc);
    style_palette_family!(bg, background, neutral);
    style_palette_family!(bg, background, stone);
    style_palette_family!(bg, background, mauve);
    style_palette_family!(bg, background, olive);
    style_palette_family!(bg, background, mist);
    style_palette_family!(bg, background, taupe);
    style_palette_family!(bg, background, red);
    style_palette_family!(bg, background, orange);
    style_palette_family!(bg, background, amber);
    style_palette_family!(bg, background, yellow);
    style_palette_family!(bg, background, lime);
    style_palette_family!(bg, background, green);
    style_palette_family!(bg, background, emerald);
    style_palette_family!(bg, background, teal);
    style_palette_family!(bg, background, cyan);
    style_palette_family!(bg, background, sky);
    style_palette_family!(bg, background, blue);
    style_palette_family!(bg, background, indigo);
    style_palette_family!(bg, background, violet);
    style_palette_family!(bg, background, purple);
    style_palette_family!(bg, background, fuchsia);
    style_palette_family!(bg, background, pink);
    style_palette_family!(bg, background, rose);

    style_semantic_color!(
        text,
        text_color,
        background,
        foreground,
        primary,
        primary_foreground,
        secondary,
        secondary_foreground,
        muted,
        muted_foreground,
        accent,
        accent_foreground,
        destructive,
        destructive_foreground,
        card,
        card_foreground,
        popover,
        popover_foreground,
        border,
        input,
        ring,
        chart_1,
        chart_2,
        chart_3,
        chart_4,
        chart_5,
        sidebar,
        sidebar_foreground,
        sidebar_primary,
        sidebar_primary_foreground,
        sidebar_accent,
        sidebar_accent_foreground,
        sidebar_border,
        sidebar_ring,
        disabled,
        disabled_foreground,
    );

    style_semantic_color!(
        bg,
        background,
        background,
        foreground,
        primary,
        primary_foreground,
        secondary,
        secondary_foreground,
        muted,
        muted_foreground,
        accent,
        accent_foreground,
        destructive,
        destructive_foreground,
        card,
        card_foreground,
        popover,
        popover_foreground,
        border,
        input,
        ring,
        chart_1,
        chart_2,
        chart_3,
        chart_4,
        chart_5,
        sidebar,
        sidebar_foreground,
        sidebar_primary,
        sidebar_primary_foreground,
        sidebar_accent,
        sidebar_accent_foreground,
        sidebar_border,
        sidebar_ring,
        disabled,
        disabled_foreground,
    );

    // ---- Spacing / sizing scale ----
    //
    // **Fix**: pairs of `(identifier, value)` — `0_5` → `0.5`, etc.

    style_spacing_scale!(
        gap,
        gap,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );

    style_spacing_scale!(
        p,
        p,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        px,
        px,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        py,
        py,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        pt,
        pt,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        pr,
        pr,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        pb,
        pb,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        pl,
        pl,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );

    style_spacing_scale!(
        m,
        m,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        mx,
        mx,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        my,
        my,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        mt,
        mt,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        mr,
        mr,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        mb,
        mb,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        ml,
        ml,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );

    style_spacing_scale!(
        w,
        w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        h,
        h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        min_w,
        min_w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        min_h,
        min_h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        max_w,
        max_w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    style_spacing_scale!(
        max_h,
        max_h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );

    // ---- style-string API ----
    pub fn style(mut self, styles: impl AsRef<str>) -> Self {
        for token in styles.as_ref().split_whitespace() {
            self = style_token(self, token);
        }
        self
    }
}

// ==============================================================
// StyleExt trait
// ==============================================================

pub trait StyleExt: Sized {
    fn get_style(&mut self) -> &mut Style;

    fn map_style<F: FnOnce(Style) -> Style>(mut self, f: F) -> Self {
        let taken = std::mem::take(self.get_style());
        *self.get_style() = f(taken);
        self
    }

    // Base setters
    fn background(self, color: Color) -> Self {
        self.map_style(|s| s.background(color))
    }
    fn color(self, color: Color) -> Self {
        self.map_style(|s| s.color(color))
    }
    fn corner_radius(self, radius: impl Into<CornerRadius>) -> Self {
        self.map_style(|s| s.corner_radius(radius))
    }
    fn border_width(self, width: f32) -> Self {
        self.map_style(|s| s.border_width(width))
    }
    fn border_color(self, color: Color) -> Self {
        self.map_style(|s| s.border_color(color))
    }
    fn border_style(self, style: BorderStyle) -> Self {
        self.map_style(|s| s.border_style(style))
    }
    fn font_size(self, size: impl Into<FontSize>) -> Self {
        self.map_style(|s| s.font_size(size))
    }
    fn font_weight(self, weight: impl Into<FontWeight>) -> Self {
        self.map_style(|s| s.font_weight(weight))
    }
    fn font_slant(self, slant: impl Into<FontSlant>) -> Self {
        self.map_style(|s| s.font_slant(slant))
    }
    fn font_family(self, family: impl Into<Cow<'static, str>>) -> Self {
        self.map_style(|s| s.font_family(family))
    }
    fn text_align(self, align: impl Into<TextAlign>) -> Self {
        self.map_style(|s| s.text_align(align))
    }
    fn text_decoration(self, decoration: impl Into<TextDecoration>) -> Self {
        self.map_style(|s| s.text_decoration(decoration))
    }
    fn text_transform(self, transform: TextTransform) -> Self {
        self.map_style(|s| s.text_transform(transform))
    }
    fn line_height(self, value: f32) -> Self {
        self.map_style(|s| s.line_height(value))
    }
    fn letter_spacing(self, value: f32) -> Self {
        self.map_style(|s| s.letter_spacing(value))
    }
    fn text_indent(self, value: f32) -> Self {
        self.map_style(|s| s.text_indent(value))
    }
    fn opacity(self, opacity: f32) -> Self {
        self.map_style(|s| s.opacity(opacity))
    }
    fn shadow(self, shadow: impl Into<Shadow>) -> Self {
        self.map_style(|s| s.shadow(shadow))
    }
    fn ring_width(self, width: f32) -> Self {
        self.map_style(|s| s.ring_width(width))
    }
    fn ring_color(self, color: Color) -> Self {
        self.map_style(|s| s.ring_color(color))
    }
    fn ring_offset(self, offset: f32) -> Self {
        self.map_style(|s| s.ring_offset(offset))
    }
    fn padding(self, gaps: impl Into<Gaps>) -> Self {
        self.map_style(|s| s.padding(gaps))
    }
    fn margin(self, gaps: impl Into<Gaps>) -> Self {
        self.map_style(|s| s.margin(gaps))
    }
    fn width(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.width(size))
    }
    fn height(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.height(size))
    }
    fn min_width(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.min_width(size))
    }
    fn min_height(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.min_height(size))
    }
    fn max_width(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.max_width(size))
    }
    fn max_height(self, size: impl Into<Size>) -> Self {
        self.map_style(|s| s.max_height(size))
    }
    fn position(self, pos: Position) -> Self {
        self.map_style(|s| s.position(pos))
    }
    fn z_index(self, z: i32) -> Self {
        self.map_style(|s| s.z_index(z))
    }
    fn overflow(self, overflow: Overflow) -> Self {
        self.map_style(|s| s.overflow(overflow))
    }
    fn direction(self, dir: Direction) -> Self {
        self.map_style(|s| s.direction(dir))
    }
    fn main_align(self, align: Alignment) -> Self {
        self.map_style(|s| s.main_align(align))
    }
    fn cross_align(self, align: Alignment) -> Self {
        self.map_style(|s| s.cross_align(align))
    }
    fn align_self(self, align: Alignment) -> Self {
        self.map_style(|s| s.align_self(align))
    }
    fn align_content(self, align: Alignment) -> Self {
        self.map_style(|s| s.align_content(align))
    }
    fn content(self, content: Content) -> Self {
        self.map_style(|s| s.content(content))
    }
    fn spacing(self, spacing: f32) -> Self {
        self.map_style(|s| s.spacing(spacing))
    }
    fn gap_x(self, size: f32) -> Self {
        self.map_style(|s| s.gap_x(size))
    }
    fn gap_y(self, size: f32) -> Self {
        self.map_style(|s| s.gap_y(size))
    }

    // Literal colours
    fn text_white(self) -> Self {
        self.map_style(|s| s.text_white())
    }
    fn text_black(self) -> Self {
        self.map_style(|s| s.text_black())
    }
    fn bg_white(self) -> Self {
        self.map_style(|s| s.bg_white())
    }
    fn bg_black(self) -> Self {
        self.map_style(|s| s.bg_black())
    }
    fn bg_transparent(self) -> Self {
        self.map_style(|s| s.bg_transparent())
    }
    fn text_transparent(self) -> Self {
        self.map_style(|s| s.text_transparent())
    }

    // Typography sizes
    fn text_xs(self) -> Self {
        self.map_style(|s| s.text_xs())
    }
    fn text_sm(self) -> Self {
        self.map_style(|s| s.text_sm())
    }
    fn text_base(self) -> Self {
        self.map_style(|s| s.text_base())
    }
    fn text_lg(self) -> Self {
        self.map_style(|s| s.text_lg())
    }
    fn text_xl(self) -> Self {
        self.map_style(|s| s.text_xl())
    }
    fn text_2xl(self) -> Self {
        self.map_style(|s| s.text_2xl())
    }
    fn text_3xl(self) -> Self {
        self.map_style(|s| s.text_3xl())
    }
    fn text_4xl(self) -> Self {
        self.map_style(|s| s.text_4xl())
    }
    fn text_5xl(self) -> Self {
        self.map_style(|s| s.text_5xl())
    }
    fn text_6xl(self) -> Self {
        self.map_style(|s| s.text_6xl())
    }
    fn text_7xl(self) -> Self {
        self.map_style(|s| s.text_7xl())
    }
    fn text_8xl(self) -> Self {
        self.map_style(|s| s.text_8xl())
    }
    fn text_9xl(self) -> Self {
        self.map_style(|s| s.text_9xl())
    }

    // Typography weights
    fn font_thin(self) -> Self {
        self.map_style(|s| s.font_thin())
    }
    fn font_extralight(self) -> Self {
        self.map_style(|s| s.font_extralight())
    }
    fn font_light(self) -> Self {
        self.map_style(|s| s.font_light())
    }
    fn font_normal(self) -> Self {
        self.map_style(|s| s.font_normal())
    }
    fn font_medium(self) -> Self {
        self.map_style(|s| s.font_medium())
    }
    fn font_semibold(self) -> Self {
        self.map_style(|s| s.font_semibold())
    }
    fn font_bold(self) -> Self {
        self.map_style(|s| s.font_bold())
    }
    fn font_extrabold(self) -> Self {
        self.map_style(|s| s.font_extrabold())
    }
    fn font_black_weight(self) -> Self {
        self.map_style(|s| s.font_black_weight())
    }

    // Typography slant/family/align/decoration/transform
    fn italic(self) -> Self {
        self.map_style(|s| s.italic())
    }
    fn not_italic(self) -> Self {
        self.map_style(|s| s.not_italic())
    }
    fn font_sans(self) -> Self {
        self.map_style(|s| s.font_sans())
    }
    fn font_serif(self) -> Self {
        self.map_style(|s| s.font_serif())
    }
    fn font_mono(self) -> Self {
        self.map_style(|s| s.font_mono())
    }
    fn text_left(self) -> Self {
        self.map_style(|s| s.text_left())
    }
    fn text_center(self) -> Self {
        self.map_style(|s| s.text_center())
    }
    fn text_right(self) -> Self {
        self.map_style(|s| s.text_right())
    }
    fn text_justify(self) -> Self {
        self.map_style(|s| s.text_justify())
    }
    fn underline(self) -> Self {
        self.map_style(|s| s.underline())
    }
    fn overline(self) -> Self {
        self.map_style(|s| s.overline())
    }
    fn line_through(self) -> Self {
        self.map_style(|s| s.line_through())
    }
    fn no_underline(self) -> Self {
        self.map_style(|s| s.no_underline())
    }
    fn uppercase(self) -> Self {
        self.map_style(|s| s.uppercase())
    }
    fn lowercase(self) -> Self {
        self.map_style(|s| s.lowercase())
    }
    fn capitalize(self) -> Self {
        self.map_style(|s| s.capitalize())
    }
    fn normal_case(self) -> Self {
        self.map_style(|s| s.normal_case())
    }

    // Borders
    fn border(self) -> Self {
        self.map_style(|s| s.border())
    }
    fn border_0(self) -> Self {
        self.map_style(|s| s.border_0())
    }
    fn border_2(self) -> Self {
        self.map_style(|s| s.border_2())
    }
    fn border_4(self) -> Self {
        self.map_style(|s| s.border_4())
    }
    fn border_8(self) -> Self {
        self.map_style(|s| s.border_8())
    }
    fn border_primary(self) -> Self {
        self.map_style(|s| s.border_primary())
    }
    fn border_destructive(self) -> Self {
        self.map_style(|s| s.border_destructive())
    }
    fn border_solid(self) -> Self {
        self.map_style(|s| s.border_solid())
    }
    fn border_dashed(self) -> Self {
        self.map_style(|s| s.border_dashed())
    }
    fn border_dotted(self) -> Self {
        self.map_style(|s| s.border_dotted())
    }
    fn border_double(self) -> Self {
        self.map_style(|s| s.border_double())
    }
    fn border_none(self) -> Self {
        self.map_style(|s| s.border_none())
    }
    fn border_t(self, w: f32) -> Self {
        self.map_style(|s| s.border_t(w))
    }
    fn border_r(self, w: f32) -> Self {
        self.map_style(|s| s.border_r(w))
    }
    fn border_b(self, w: f32) -> Self {
        self.map_style(|s| s.border_b(w))
    }
    fn border_l(self, w: f32) -> Self {
        self.map_style(|s| s.border_l(w))
    }
    fn border_x(self, w: f32) -> Self {
        self.map_style(|s| s.border_x(w))
    }
    fn border_y(self, w: f32) -> Self {
        self.map_style(|s| s.border_y(w))
    }

    // Corner radius
    fn rounded_none(self) -> Self {
        self.map_style(|s| s.rounded_none())
    }
    fn rounded_sm(self) -> Self {
        self.map_style(|s| s.rounded_sm())
    }
    fn rounded(self) -> Self {
        self.map_style(|s| s.rounded())
    }
    fn rounded_md(self) -> Self {
        self.map_style(|s| s.rounded_md())
    }
    fn rounded_lg(self) -> Self {
        self.map_style(|s| s.rounded_lg())
    }
    fn rounded_xl(self) -> Self {
        self.map_style(|s| s.rounded_xl())
    }
    fn rounded_2xl(self) -> Self {
        self.map_style(|s| s.rounded_2xl())
    }
    fn rounded_3xl(self) -> Self {
        self.map_style(|s| s.rounded_3xl())
    }
    fn rounded_full(self) -> Self {
        self.map_style(|s| s.rounded_full())
    }

    // Effects
    fn opacity_0(self) -> Self {
        self.map_style(|s| s.opacity_0())
    }
    fn opacity_25(self) -> Self {
        self.map_style(|s| s.opacity_25())
    }
    fn opacity_50(self) -> Self {
        self.map_style(|s| s.opacity_50())
    }
    fn opacity_75(self) -> Self {
        self.map_style(|s| s.opacity_75())
    }
    fn opacity_100(self) -> Self {
        self.map_style(|s| s.opacity_100())
    }
    fn shadow_sm(self) -> Self {
        self.map_style(|s| s.shadow_sm())
    }
    fn shadow_md(self) -> Self {
        self.map_style(|s| s.shadow_md())
    }
    fn shadow_lg(self) -> Self {
        self.map_style(|s| s.shadow_lg())
    }
    fn shadow_xl(self) -> Self {
        self.map_style(|s| s.shadow_xl())
    }
    fn shadow_2xl(self) -> Self {
        self.map_style(|s| s.shadow_2xl())
    }
    fn shadow_none(self) -> Self {
        self.map_style(|s| s.shadow_none())
    }

    // Ring
    fn ring(self) -> Self {
        self.map_style(|s| s.ring())
    }
    fn ring_0(self) -> Self {
        self.map_style(|s| s.ring_0())
    }
    fn ring_1(self) -> Self {
        self.map_style(|s| s.ring_1())
    }
    fn ring_2(self) -> Self {
        self.map_style(|s| s.ring_2())
    }
    fn ring_4(self) -> Self {
        self.map_style(|s| s.ring_4())
    }
    fn ring_8(self) -> Self {
        self.map_style(|s| s.ring_8())
    }
    fn ring_primary(self) -> Self {
        self.map_style(|s| s.ring_primary())
    }

    // Flex
    fn flex_col(self) -> Self {
        self.map_style(|s| s.flex_col())
    }
    fn flex_row(self) -> Self {
        self.map_style(|s| s.flex_row())
    }
    fn flex_wrap_content(self) -> Self {
        self.map_style(|s| s.flex_wrap_content())
    }
    fn flex_wrap_spacing(self, spacing: f32) -> Self {
        self.map_style(|s| s.flex_wrap_spacing(spacing))
    }
    fn content_flex(self) -> Self {
        self.map_style(|s| s.content_flex())
    }
    fn content_fit(self) -> Self {
        self.map_style(|s| s.content_fit())
    }
    fn content_normal(self) -> Self {
        self.map_style(|s| s.content_normal())
    }
    fn flex_1(self) -> Self {
        self.map_style(|s| s.flex_1())
    }
    fn grow(self) -> Self {
        self.map_style(|s| s.grow())
    }
    fn grow_0(self) -> Self {
        self.map_style(|s| s.grow_0())
    }
    fn justify_start(self) -> Self {
        self.map_style(|s| s.justify_start())
    }
    fn justify_center(self) -> Self {
        self.map_style(|s| s.justify_center())
    }
    fn justify_end(self) -> Self {
        self.map_style(|s| s.justify_end())
    }
    fn justify_between(self) -> Self {
        self.map_style(|s| s.justify_between())
    }
    fn justify_around(self) -> Self {
        self.map_style(|s| s.justify_around())
    }
    fn justify_evenly(self) -> Self {
        self.map_style(|s| s.justify_evenly())
    }
    fn items_start(self) -> Self {
        self.map_style(|s| s.items_start())
    }
    fn items_center(self) -> Self {
        self.map_style(|s| s.items_center())
    }
    fn items_end(self) -> Self {
        self.map_style(|s| s.items_end())
    }
    fn self_start(self) -> Self {
        self.map_style(|s| s.self_start())
    }
    fn self_center(self) -> Self {
        self.map_style(|s| s.self_center())
    }
    fn self_end(self) -> Self {
        self.map_style(|s| s.self_end())
    }
    fn content_start(self) -> Self {
        self.map_style(|s| s.content_start())
    }
    fn content_center(self) -> Self {
        self.map_style(|s| s.content_center())
    }
    fn content_end(self) -> Self {
        self.map_style(|s| s.content_end())
    }
    fn content_between(self) -> Self {
        self.map_style(|s| s.content_between())
    }
    fn content_around(self) -> Self {
        self.map_style(|s| s.content_around())
    }
    fn content_evenly(self) -> Self {
        self.map_style(|s| s.content_evenly())
    }
    fn gap(self, size: f32) -> Self {
        self.map_style(|s| s.gap(size))
    }

    // Sizing
    fn w(self, px: f32) -> Self {
        self.map_style(|s| s.w(px))
    }
    fn h(self, px: f32) -> Self {
        self.map_style(|s| s.h(px))
    }
    fn min_w(self, px: f32) -> Self {
        self.map_style(|s| s.min_w(px))
    }
    fn min_h(self, px: f32) -> Self {
        self.map_style(|s| s.min_h(px))
    }
    fn max_w(self, px: f32) -> Self {
        self.map_style(|s| s.max_w(px))
    }
    fn max_h(self, px: f32) -> Self {
        self.map_style(|s| s.max_h(px))
    }
    fn w_percent(self, pct: f32) -> Self {
        self.map_style(|s| s.w_percent(pct))
    }
    fn h_percent(self, pct: f32) -> Self {
        self.map_style(|s| s.h_percent(pct))
    }
    fn w_full(self) -> Self {
        self.map_style(|s| s.w_full())
    }
    fn w_auto(self) -> Self {
        self.map_style(|s| s.w_auto())
    }
    fn h_full(self) -> Self {
        self.map_style(|s| s.h_full())
    }
    fn h_auto(self) -> Self {
        self.map_style(|s| s.h_auto())
    }
    fn size(self, px: f32) -> Self {
        self.map_style(|s| s.size(px))
    }
    fn size_full(self) -> Self {
        self.map_style(|s| s.size_full())
    }

    // Positioning
    fn relative(self) -> Self {
        self.map_style(|s| s.relative())
    }
    fn absolute(self) -> Self {
        self.map_style(|s| s.absolute())
    }
    fn global(self) -> Self {
        self.map_style(|s| s.global())
    }
    fn top(self, value: f32) -> Self {
        self.map_style(|s| s.top(value))
    }
    fn right(self, value: f32) -> Self {
        self.map_style(|s| s.right(value))
    }
    fn bottom(self, value: f32) -> Self {
        self.map_style(|s| s.bottom(value))
    }
    fn left(self, value: f32) -> Self {
        self.map_style(|s| s.left(value))
    }
    fn inset(self, value: f32) -> Self {
        self.map_style(|s| s.inset(value))
    }
    fn inset_x(self, value: f32) -> Self {
        self.map_style(|s| s.inset_x(value))
    }
    fn inset_y(self, value: f32) -> Self {
        self.map_style(|s| s.inset_y(value))
    }

    // Overflow — **Fix**: only `None` and `Clip` exist.
    fn overflow_none(self) -> Self {
        self.map_style(|s| s.overflow_none())
    }
    fn overflow_clip(self) -> Self {
        self.map_style(|s| s.overflow_clip())
    }

    // Spacing
    fn p(self, size: f32) -> Self {
        self.map_style(|s| s.p(size))
    }
    fn px(self, size: f32) -> Self {
        self.map_style(|s| s.px(size))
    }
    fn py(self, size: f32) -> Self {
        self.map_style(|s| s.py(size))
    }
    fn pt(self, size: f32) -> Self {
        self.map_style(|s| s.pt(size))
    }
    fn pr(self, size: f32) -> Self {
        self.map_style(|s| s.pr(size))
    }
    fn pb(self, size: f32) -> Self {
        self.map_style(|s| s.pb(size))
    }
    fn pl(self, size: f32) -> Self {
        self.map_style(|s| s.pl(size))
    }
    fn m(self, size: f32) -> Self {
        self.map_style(|s| s.m(size))
    }
    fn mx(self, size: f32) -> Self {
        self.map_style(|s| s.mx(size))
    }
    fn my(self, size: f32) -> Self {
        self.map_style(|s| s.my(size))
    }
    fn mt(self, size: f32) -> Self {
        self.map_style(|s| s.mt(size))
    }
    fn mr(self, size: f32) -> Self {
        self.map_style(|s| s.mr(size))
    }
    fn mb(self, size: f32) -> Self {
        self.map_style(|s| s.mb(size))
    }
    fn ml(self, size: f32) -> Self {
        self.map_style(|s| s.ml(size))
    }

    // ==========================================================
    // Merging
    // ==========================================================

    /// Merge `other` into this value's style, with **`other` winning**.
    ///
    /// Equivalent to `self.map_style(|s| s.merge(other))`. Works uniformly
    /// on `Style` and on any component implementing `StyleExt`.
    fn merge_style(self, other: Style) -> Self {
        self.map_style(|s| s.merge(other))
    }

    /// Merge `defaults` into this value's style, with **`self` winning**.
    ///
    /// Equivalent to `self.map_style(|s| s.or(defaults))`.
    fn or_style(self, defaults: Style) -> Self {
        self.map_style(|s| s.or(defaults))
    }

    // Palette colours
    ext_palette_family!(text, slate);
    ext_palette_family!(text, gray);
    ext_palette_family!(text, zinc);
    ext_palette_family!(text, neutral);
    ext_palette_family!(text, stone);
    ext_palette_family!(text, mauve);
    ext_palette_family!(text, olive);
    ext_palette_family!(text, mist);
    ext_palette_family!(text, taupe);
    ext_palette_family!(text, red);
    ext_palette_family!(text, orange);
    ext_palette_family!(text, amber);
    ext_palette_family!(text, yellow);
    ext_palette_family!(text, lime);
    ext_palette_family!(text, green);
    ext_palette_family!(text, emerald);
    ext_palette_family!(text, teal);
    ext_palette_family!(text, cyan);
    ext_palette_family!(text, sky);
    ext_palette_family!(text, blue);
    ext_palette_family!(text, indigo);
    ext_palette_family!(text, violet);
    ext_palette_family!(text, purple);
    ext_palette_family!(text, fuchsia);
    ext_palette_family!(text, pink);
    ext_palette_family!(text, rose);

    ext_palette_family!(bg, slate);
    ext_palette_family!(bg, gray);
    ext_palette_family!(bg, zinc);
    ext_palette_family!(bg, neutral);
    ext_palette_family!(bg, stone);
    ext_palette_family!(bg, mauve);
    ext_palette_family!(bg, olive);
    ext_palette_family!(bg, mist);
    ext_palette_family!(bg, taupe);
    ext_palette_family!(bg, red);
    ext_palette_family!(bg, orange);
    ext_palette_family!(bg, amber);
    ext_palette_family!(bg, yellow);
    ext_palette_family!(bg, lime);
    ext_palette_family!(bg, green);
    ext_palette_family!(bg, emerald);
    ext_palette_family!(bg, teal);
    ext_palette_family!(bg, cyan);
    ext_palette_family!(bg, sky);
    ext_palette_family!(bg, blue);
    ext_palette_family!(bg, indigo);
    ext_palette_family!(bg, violet);
    ext_palette_family!(bg, purple);
    ext_palette_family!(bg, fuchsia);
    ext_palette_family!(bg, pink);
    ext_palette_family!(bg, rose);

    // Semantic colours
    ext_semantic_color!(
        text,
        background,
        foreground,
        primary,
        primary_foreground,
        secondary,
        secondary_foreground,
        muted,
        muted_foreground,
        accent,
        accent_foreground,
        destructive,
        destructive_foreground,
        card,
        card_foreground,
        popover,
        popover_foreground,
        border,
        input,
        ring,
        chart_1,
        chart_2,
        chart_3,
        chart_4,
        chart_5,
        sidebar,
        sidebar_foreground,
        sidebar_primary,
        sidebar_primary_foreground,
        sidebar_accent,
        sidebar_accent_foreground,
        sidebar_border,
        sidebar_ring,
        disabled,
        disabled_foreground,
    );
    ext_semantic_color!(
        bg,
        background,
        foreground,
        primary,
        primary_foreground,
        secondary,
        secondary_foreground,
        muted,
        muted_foreground,
        accent,
        accent_foreground,
        destructive,
        destructive_foreground,
        card,
        card_foreground,
        popover,
        popover_foreground,
        border,
        input,
        ring,
        chart_1,
        chart_2,
        chart_3,
        chart_4,
        chart_5,
        sidebar,
        sidebar_foreground,
        sidebar_primary,
        sidebar_primary_foreground,
        sidebar_accent,
        sidebar_accent_foreground,
        sidebar_border,
        sidebar_ring,
        disabled,
        disabled_foreground,
    );

    // Spacing scale
    ext_spacing_scale!(
        gap,
        gap,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        p,
        p,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        px,
        px,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        py,
        py,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        pt,
        pt,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        pr,
        pr,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        pb,
        pb,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        pl,
        pl,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        m,
        m,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        mx,
        mx,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        my,
        my,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        mt,
        mt,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        mr,
        mr,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        mb,
        mb,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        ml,
        ml,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        w,
        w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        h,
        h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        min_w,
        min_w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        min_h,
        min_h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        max_w,
        max_w,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );
    ext_spacing_scale!(
        max_h,
        max_h,
        (0, 0.0),
        (0_5, 0.5),
        (1, 1.0),
        (1_5, 1.5),
        (2, 2.0),
        (2_5, 2.5),
        (3, 3.0),
        (3_5, 3.5),
        (4, 4.0),
        (5, 5.0),
        (6, 6.0),
        (7, 7.0),
        (8, 8.0),
        (9, 9.0),
        (10, 10.0),
        (11, 11.0),
        (12, 12.0),
        (14, 14.0),
        (16, 16.0),
        (20, 20.0),
        (24, 24.0),
        (28, 28.0),
        (32, 32.0),
        (36, 36.0),
        (40, 40.0),
        (44, 44.0),
        (48, 48.0),
        (52, 52.0),
        (56, 56.0),
        (60, 60.0),
        (64, 64.0),
        (72, 72.0),
        (80, 80.0),
        (96, 96.0)
    );

    // style-string API
    fn style(self, styles: impl AsRef<str>) -> Self {
        let s = styles.as_ref().to_owned();
        self.map_style(|style| style.style(s))
    }
}

impl StyleExt for Style {
    fn get_style(&mut self) -> &mut Style {
        self
    }
}

// ==============================================================
// Palette lookup
// ==============================================================

fn palette_colour(family: &str, shade: &str) -> Option<Color> {
    let theme = use_cn_theme().read();
    let c = &theme.colors;
    match (family, shade) {
        ("slate", "50") => Some(c.slate_50),
        ("slate", "100") => Some(c.slate_100),
        ("slate", "200") => Some(c.slate_200),
        ("slate", "300") => Some(c.slate_300),
        ("slate", "400") => Some(c.slate_400),
        ("slate", "500") => Some(c.slate_500),
        ("slate", "600") => Some(c.slate_600),
        ("slate", "700") => Some(c.slate_700),
        ("slate", "800") => Some(c.slate_800),
        ("slate", "900") => Some(c.slate_900),
        ("slate", "950") => Some(c.slate_950),
        ("gray", "50") => Some(c.gray_50),
        ("gray", "100") => Some(c.gray_100),
        ("gray", "200") => Some(c.gray_200),
        ("gray", "300") => Some(c.gray_300),
        ("gray", "400") => Some(c.gray_400),
        ("gray", "500") => Some(c.gray_500),
        ("gray", "600") => Some(c.gray_600),
        ("gray", "700") => Some(c.gray_700),
        ("gray", "800") => Some(c.gray_800),
        ("gray", "900") => Some(c.gray_900),
        ("gray", "950") => Some(c.gray_950),
        ("zinc", "50") => Some(c.zinc_50),
        ("zinc", "100") => Some(c.zinc_100),
        ("zinc", "200") => Some(c.zinc_200),
        ("zinc", "300") => Some(c.zinc_300),
        ("zinc", "400") => Some(c.zinc_400),
        ("zinc", "500") => Some(c.zinc_500),
        ("zinc", "600") => Some(c.zinc_600),
        ("zinc", "700") => Some(c.zinc_700),
        ("zinc", "800") => Some(c.zinc_800),
        ("zinc", "900") => Some(c.zinc_900),
        ("zinc", "950") => Some(c.zinc_950),
        ("neutral", "50") => Some(c.neutral_50),
        ("neutral", "100") => Some(c.neutral_100),
        ("neutral", "200") => Some(c.neutral_200),
        ("neutral", "300") => Some(c.neutral_300),
        ("neutral", "400") => Some(c.neutral_400),
        ("neutral", "500") => Some(c.neutral_500),
        ("neutral", "600") => Some(c.neutral_600),
        ("neutral", "700") => Some(c.neutral_700),
        ("neutral", "800") => Some(c.neutral_800),
        ("neutral", "900") => Some(c.neutral_900),
        ("neutral", "950") => Some(c.neutral_950),
        ("stone", "50") => Some(c.stone_50),
        ("stone", "100") => Some(c.stone_100),
        ("stone", "200") => Some(c.stone_200),
        ("stone", "300") => Some(c.stone_300),
        ("stone", "400") => Some(c.stone_400),
        ("stone", "500") => Some(c.stone_500),
        ("stone", "600") => Some(c.stone_600),
        ("stone", "700") => Some(c.stone_700),
        ("stone", "800") => Some(c.stone_800),
        ("stone", "900") => Some(c.stone_900),
        ("stone", "950") => Some(c.stone_950),
        ("mauve", "50") => Some(c.mauve_50),
        ("mauve", "100") => Some(c.mauve_100),
        ("mauve", "200") => Some(c.mauve_200),
        ("mauve", "300") => Some(c.mauve_300),
        ("mauve", "400") => Some(c.mauve_400),
        ("mauve", "500") => Some(c.mauve_500),
        ("mauve", "600") => Some(c.mauve_600),
        ("mauve", "700") => Some(c.mauve_700),
        ("mauve", "800") => Some(c.mauve_800),
        ("mauve", "900") => Some(c.mauve_900),
        ("mauve", "950") => Some(c.mauve_950),
        ("olive", "50") => Some(c.olive_50),
        ("olive", "100") => Some(c.olive_100),
        ("olive", "200") => Some(c.olive_200),
        ("olive", "300") => Some(c.olive_300),
        ("olive", "400") => Some(c.olive_400),
        ("olive", "500") => Some(c.olive_500),
        ("olive", "600") => Some(c.olive_600),
        ("olive", "700") => Some(c.olive_700),
        ("olive", "800") => Some(c.olive_800),
        ("olive", "900") => Some(c.olive_900),
        ("olive", "950") => Some(c.olive_950),
        ("mist", "50") => Some(c.mist_50),
        ("mist", "100") => Some(c.mist_100),
        ("mist", "200") => Some(c.mist_200),
        ("mist", "300") => Some(c.mist_300),
        ("mist", "400") => Some(c.mist_400),
        ("mist", "500") => Some(c.mist_500),
        ("mist", "600") => Some(c.mist_600),
        ("mist", "700") => Some(c.mist_700),
        ("mist", "800") => Some(c.mist_800),
        ("mist", "900") => Some(c.mist_900),
        ("mist", "950") => Some(c.mist_950),
        ("taupe", "50") => Some(c.taupe_50),
        ("taupe", "100") => Some(c.taupe_100),
        ("taupe", "200") => Some(c.taupe_200),
        ("taupe", "300") => Some(c.taupe_300),
        ("taupe", "400") => Some(c.taupe_400),
        ("taupe", "500") => Some(c.taupe_500),
        ("taupe", "600") => Some(c.taupe_600),
        ("taupe", "700") => Some(c.taupe_700),
        ("taupe", "800") => Some(c.taupe_800),
        ("taupe", "900") => Some(c.taupe_900),
        ("taupe", "950") => Some(c.taupe_950),
        ("red", "50") => Some(c.red_50),
        ("red", "100") => Some(c.red_100),
        ("red", "200") => Some(c.red_200),
        ("red", "300") => Some(c.red_300),
        ("red", "400") => Some(c.red_400),
        ("red", "500") => Some(c.red_500),
        ("red", "600") => Some(c.red_600),
        ("red", "700") => Some(c.red_700),
        ("red", "800") => Some(c.red_800),
        ("red", "900") => Some(c.red_900),
        ("red", "950") => Some(c.red_950),
        ("orange", "50") => Some(c.orange_50),
        ("orange", "100") => Some(c.orange_100),
        ("orange", "200") => Some(c.orange_200),
        ("orange", "300") => Some(c.orange_300),
        ("orange", "400") => Some(c.orange_400),
        ("orange", "500") => Some(c.orange_500),
        ("orange", "600") => Some(c.orange_600),
        ("orange", "700") => Some(c.orange_700),
        ("orange", "800") => Some(c.orange_800),
        ("orange", "900") => Some(c.orange_900),
        ("orange", "950") => Some(c.orange_950),
        ("amber", "50") => Some(c.amber_50),
        ("amber", "100") => Some(c.amber_100),
        ("amber", "200") => Some(c.amber_200),
        ("amber", "300") => Some(c.amber_300),
        ("amber", "400") => Some(c.amber_400),
        ("amber", "500") => Some(c.amber_500),
        ("amber", "600") => Some(c.amber_600),
        ("amber", "700") => Some(c.amber_700),
        ("amber", "800") => Some(c.amber_800),
        ("amber", "900") => Some(c.amber_900),
        ("amber", "950") => Some(c.amber_950),
        ("yellow", "50") => Some(c.yellow_50),
        ("yellow", "100") => Some(c.yellow_100),
        ("yellow", "200") => Some(c.yellow_200),
        ("yellow", "300") => Some(c.yellow_300),
        ("yellow", "400") => Some(c.yellow_400),
        ("yellow", "500") => Some(c.yellow_500),
        ("yellow", "600") => Some(c.yellow_600),
        ("yellow", "700") => Some(c.yellow_700),
        ("yellow", "800") => Some(c.yellow_800),
        ("yellow", "900") => Some(c.yellow_900),
        ("yellow", "950") => Some(c.yellow_950),
        ("lime", "50") => Some(c.lime_50),
        ("lime", "100") => Some(c.lime_100),
        ("lime", "200") => Some(c.lime_200),
        ("lime", "300") => Some(c.lime_300),
        ("lime", "400") => Some(c.lime_400),
        ("lime", "500") => Some(c.lime_500),
        ("lime", "600") => Some(c.lime_600),
        ("lime", "700") => Some(c.lime_700),
        ("lime", "800") => Some(c.lime_800),
        ("lime", "900") => Some(c.lime_900),
        ("lime", "950") => Some(c.lime_950),
        ("green", "50") => Some(c.green_50),
        ("green", "100") => Some(c.green_100),
        ("green", "200") => Some(c.green_200),
        ("green", "300") => Some(c.green_300),
        ("green", "400") => Some(c.green_400),
        ("green", "500") => Some(c.green_500),
        ("green", "600") => Some(c.green_600),
        ("green", "700") => Some(c.green_700),
        ("green", "800") => Some(c.green_800),
        ("green", "900") => Some(c.green_900),
        ("green", "950") => Some(c.green_950),
        ("emerald", "50") => Some(c.emerald_50),
        ("emerald", "100") => Some(c.emerald_100),
        ("emerald", "200") => Some(c.emerald_200),
        ("emerald", "300") => Some(c.emerald_300),
        ("emerald", "400") => Some(c.emerald_400),
        ("emerald", "500") => Some(c.emerald_500),
        ("emerald", "600") => Some(c.emerald_600),
        ("emerald", "700") => Some(c.emerald_700),
        ("emerald", "800") => Some(c.emerald_800),
        ("emerald", "900") => Some(c.emerald_900),
        ("emerald", "950") => Some(c.emerald_950),
        ("teal", "50") => Some(c.teal_50),
        ("teal", "100") => Some(c.teal_100),
        ("teal", "200") => Some(c.teal_200),
        ("teal", "300") => Some(c.teal_300),
        ("teal", "400") => Some(c.teal_400),
        ("teal", "500") => Some(c.teal_500),
        ("teal", "600") => Some(c.teal_600),
        ("teal", "700") => Some(c.teal_700),
        ("teal", "800") => Some(c.teal_800),
        ("teal", "900") => Some(c.teal_900),
        ("teal", "950") => Some(c.teal_950),
        ("cyan", "50") => Some(c.cyan_50),
        ("cyan", "100") => Some(c.cyan_100),
        ("cyan", "200") => Some(c.cyan_200),
        ("cyan", "300") => Some(c.cyan_300),
        ("cyan", "400") => Some(c.cyan_400),
        ("cyan", "500") => Some(c.cyan_500),
        ("cyan", "600") => Some(c.cyan_600),
        ("cyan", "700") => Some(c.cyan_700),
        ("cyan", "800") => Some(c.cyan_800),
        ("cyan", "900") => Some(c.cyan_900),
        ("cyan", "950") => Some(c.cyan_950),
        ("sky", "50") => Some(c.sky_50),
        ("sky", "100") => Some(c.sky_100),
        ("sky", "200") => Some(c.sky_200),
        ("sky", "300") => Some(c.sky_300),
        ("sky", "400") => Some(c.sky_400),
        ("sky", "500") => Some(c.sky_500),
        ("sky", "600") => Some(c.sky_600),
        ("sky", "700") => Some(c.sky_700),
        ("sky", "800") => Some(c.sky_800),
        ("sky", "900") => Some(c.sky_900),
        ("sky", "950") => Some(c.sky_950),
        ("blue", "50") => Some(c.blue_50),
        ("blue", "100") => Some(c.blue_100),
        ("blue", "200") => Some(c.blue_200),
        ("blue", "300") => Some(c.blue_300),
        ("blue", "400") => Some(c.blue_400),
        ("blue", "500") => Some(c.blue_500),
        ("blue", "600") => Some(c.blue_600),
        ("blue", "700") => Some(c.blue_700),
        ("blue", "800") => Some(c.blue_800),
        ("blue", "900") => Some(c.blue_900),
        ("blue", "950") => Some(c.blue_950),
        ("indigo", "50") => Some(c.indigo_50),
        ("indigo", "100") => Some(c.indigo_100),
        ("indigo", "200") => Some(c.indigo_200),
        ("indigo", "300") => Some(c.indigo_300),
        ("indigo", "400") => Some(c.indigo_400),
        ("indigo", "500") => Some(c.indigo_500),
        ("indigo", "600") => Some(c.indigo_600),
        ("indigo", "700") => Some(c.indigo_700),
        ("indigo", "800") => Some(c.indigo_800),
        ("indigo", "900") => Some(c.indigo_900),
        ("indigo", "950") => Some(c.indigo_950),
        ("violet", "50") => Some(c.violet_50),
        ("violet", "100") => Some(c.violet_100),
        ("violet", "200") => Some(c.violet_200),
        ("violet", "300") => Some(c.violet_300),
        ("violet", "400") => Some(c.violet_400),
        ("violet", "500") => Some(c.violet_500),
        ("violet", "600") => Some(c.violet_600),
        ("violet", "700") => Some(c.violet_700),
        ("violet", "800") => Some(c.violet_800),
        ("violet", "900") => Some(c.violet_900),
        ("violet", "950") => Some(c.violet_950),
        ("purple", "50") => Some(c.purple_50),
        ("purple", "100") => Some(c.purple_100),
        ("purple", "200") => Some(c.purple_200),
        ("purple", "300") => Some(c.purple_300),
        ("purple", "400") => Some(c.purple_400),
        ("purple", "500") => Some(c.purple_500),
        ("purple", "600") => Some(c.purple_600),
        ("purple", "700") => Some(c.purple_700),
        ("purple", "800") => Some(c.purple_800),
        ("purple", "900") => Some(c.purple_900),
        ("purple", "950") => Some(c.purple_950),
        ("fuchsia", "50") => Some(c.fuchsia_50),
        ("fuchsia", "100") => Some(c.fuchsia_100),
        ("fuchsia", "200") => Some(c.fuchsia_200),
        ("fuchsia", "300") => Some(c.fuchsia_300),
        ("fuchsia", "400") => Some(c.fuchsia_400),
        ("fuchsia", "500") => Some(c.fuchsia_500),
        ("fuchsia", "600") => Some(c.fuchsia_600),
        ("fuchsia", "700") => Some(c.fuchsia_700),
        ("fuchsia", "800") => Some(c.fuchsia_800),
        ("fuchsia", "900") => Some(c.fuchsia_900),
        ("fuchsia", "950") => Some(c.fuchsia_950),
        ("pink", "50") => Some(c.pink_50),
        ("pink", "100") => Some(c.pink_100),
        ("pink", "200") => Some(c.pink_200),
        ("pink", "300") => Some(c.pink_300),
        ("pink", "400") => Some(c.pink_400),
        ("pink", "500") => Some(c.pink_500),
        ("pink", "600") => Some(c.pink_600),
        ("pink", "700") => Some(c.pink_700),
        ("pink", "800") => Some(c.pink_800),
        ("pink", "900") => Some(c.pink_900),
        ("pink", "950") => Some(c.pink_950),
        ("rose", "50") => Some(c.rose_50),
        ("rose", "100") => Some(c.rose_100),
        ("rose", "200") => Some(c.rose_200),
        ("rose", "300") => Some(c.rose_300),
        ("rose", "400") => Some(c.rose_400),
        ("rose", "500") => Some(c.rose_500),
        ("rose", "600") => Some(c.rose_600),
        ("rose", "700") => Some(c.rose_700),
        ("rose", "800") => Some(c.rose_800),
        ("rose", "900") => Some(c.rose_900),
        ("rose", "950") => Some(c.rose_950),
        _ => None,
    }
}

/// Returns `true` if `token` looks like a class the parser recognises.
///
/// Checks the token against the fixed keyword set and the prefix list the
/// `style_token` dispatcher handles. Does **not** verify that the value
/// after the prefix is valid — `bg-invalid-999` reports as known because
/// the `bg-` prefix is recognised.
///
/// Used by [`Style::style_debug`].
fn is_known_token(token: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "italic",
        "not-italic",
        "underline",
        "overline",
        "line-through",
        "no-underline",
        "uppercase",
        "lowercase",
        "capitalize",
        "normal-case",
        "flex-row",
        "flex-col",
        "flex-1",
        "grow",
        "grow-0",
        "flex-wrap",
        "content-flex",
        "content-fit",
        "content-normal",
        "relative",
        "absolute",
        "global",
        "overflow-clip",
        "overflow-none",
        "bg-transparent",
        "text-transparent",
        "border",
        "rounded",
        "ring",
    ];

    const PREFIXES: &[&str] = &[
        "min-w-",
        "min-h-",
        "max-w-",
        "max-h-",
        "rounded-",
        "bg-",
        "text-",
        "font-",
        "border-",
        "shadow-",
        "opacity-",
        "leading-",
        "tracking-",
        "gap-",
        "justify-",
        "items-",
        "self-",
        "content-",
        "z-",
        "px-",
        "py-",
        "pt-",
        "pr-",
        "pb-",
        "pl-",
        "p-",
        "mx-",
        "my-",
        "mt-",
        "mr-",
        "mb-",
        "ml-",
        "m-",
        "w-",
        "h-",
    ];

    KEYWORDS.contains(&token) || PREFIXES.iter().any(|p| token.starts_with(p))
}

// ==============================================================
// style-string parser (private)
// ==============================================================

fn parse_len(s: &str) -> Option<f32> {
    let s = s.trim();
    let s = s
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(s);
    let s = s.strip_suffix("px").unwrap_or(s);
    s.parse::<f32>().ok()
}

/// **Fix**: `$name` is the identifier fragment (e.g. `0_5`), and
/// `stringify!($name).replace('_', ".")` produces the Tailwind token
/// (`"0.5"`).
macro_rules! style_scale_dispatch {
    ($this:expr, $rest:expr, $prefix:ident; $($name:literal),* $(,)?) => {
        paste! {
            $(
                if $rest == stringify!($name).replace('_', ".") {
                    return $this.[<$prefix _ $name>]();
                }
            )*
        }
    };
}

fn style_token<T: StyleExt>(this: T, token: &str) -> T {
    match token {
        "italic" => return this.italic(),
        "not-italic" => return this.not_italic(),
        "underline" => return this.underline(),
        "overline" => return this.overline(),
        "line-through" => return this.line_through(),
        "no-underline" => return this.no_underline(),
        "uppercase" => return this.uppercase(),
        "lowercase" => return this.lowercase(),
        "capitalize" => return this.capitalize(),
        "normal-case" => return this.normal_case(),
        "flex-row" => return this.flex_row(),
        "flex-col" => return this.flex_col(),
        "flex-1" => return this.flex_1(),
        "grow" => return this.grow(),
        "grow-0" => return this.grow_0(),
        "flex-wrap" => return this.flex_wrap_content(),
        "content-flex" => return this.content_flex(),
        "content-fit" => return this.content_fit(),
        "content-normal" => return this.content_normal(),
        "relative" => return this.relative(),
        "absolute" => return this.absolute(),
        "global" => return this.global(),
        "overflow-clip" => return this.overflow_clip(),
        "overflow-none" => return this.overflow_none(),
        "bg-transparent" => return this.bg_transparent(),
        "text-transparent" => return this.text_transparent(),
        "border" => return this.border(),
        "rounded" => return this.rounded(),
        "ring" => return this.ring(),
        _ => {}
    }

    if let Some(r) = token.strip_prefix("min-w-") {
        return style_min_w(this, r);
    }
    if let Some(r) = token.strip_prefix("min-h-") {
        return style_min_h(this, r);
    }
    if let Some(r) = token.strip_prefix("max-w-") {
        return style_max_w(this, r);
    }
    if let Some(r) = token.strip_prefix("max-h-") {
        return style_max_h(this, r);
    }
    if let Some(r) = token.strip_prefix("rounded-") {
        return style_rounded(this, r);
    }

    if let Some(r) = token.strip_prefix("bg-") {
        return style_bg(this, r);
    }
    if let Some(r) = token.strip_prefix("text-") {
        return style_text(this, r);
    }
    if let Some(r) = token.strip_prefix("font-") {
        return style_font(this, r);
    }
    if let Some(r) = token.strip_prefix("border-") {
        return style_border(this, r);
    }
    if let Some(r) = token.strip_prefix("shadow-") {
        return style_shadow(this, r);
    }
    if let Some(r) = token.strip_prefix("opacity-") {
        return style_opacity(this, r);
    }
    if let Some(r) = token.strip_prefix("leading-") {
        return style_leading(this, r);
    }
    if let Some(r) = token.strip_prefix("tracking-") {
        return style_tracking(this, r);
    }
    if let Some(r) = token.strip_prefix("gap-") {
        return style_gap(this, r);
    }
    if let Some(r) = token.strip_prefix("justify-") {
        return style_justify(this, r);
    }
    if let Some(r) = token.strip_prefix("items-") {
        return style_items(this, r);
    }
    if let Some(r) = token.strip_prefix("self-") {
        return style_self_(this, r);
    }
    if let Some(r) = token.strip_prefix("content-") {
        return style_content(this, r);
    }
    if let Some(r) = token.strip_prefix("z-") {
        return style_z(this, r);
    }
    if let Some(r) = token.strip_prefix("px-") {
        return style_px(this, r);
    }
    if let Some(r) = token.strip_prefix("py-") {
        return style_py(this, r);
    }
    if let Some(r) = token.strip_prefix("pt-") {
        return style_pt(this, r);
    }
    if let Some(r) = token.strip_prefix("pr-") {
        return style_pr(this, r);
    }
    if let Some(r) = token.strip_prefix("pb-") {
        return style_pb(this, r);
    }
    if let Some(r) = token.strip_prefix("pl-") {
        return style_pl(this, r);
    }
    if let Some(r) = token.strip_prefix("p-") {
        return style_p(this, r);
    }
    if let Some(r) = token.strip_prefix("mx-") {
        return style_mx(this, r);
    }
    if let Some(r) = token.strip_prefix("my-") {
        return style_my(this, r);
    }
    if let Some(r) = token.strip_prefix("mt-") {
        return style_mt(this, r);
    }
    if let Some(r) = token.strip_prefix("mr-") {
        return style_mr(this, r);
    }
    if let Some(r) = token.strip_prefix("mb-") {
        return style_mb(this, r);
    }
    if let Some(r) = token.strip_prefix("ml-") {
        return style_ml(this, r);
    }
    if let Some(r) = token.strip_prefix("m-") {
        return style_m(this, r);
    }
    if let Some(r) = token.strip_prefix("w-") {
        return style_w(this, r);
    }
    if let Some(r) = token.strip_prefix("h-") {
        return style_h(this, r);
    }

    this
}

// ---------- Colour dispatchers ----------

fn style_bg<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "background" => return this.bg_background(),
        "foreground" => return this.bg_foreground(),
        "primary" => return this.bg_primary(),
        "primary-foreground" => return this.bg_primary_foreground(),
        "secondary" => return this.bg_secondary(),
        "secondary-foreground" => return this.bg_secondary_foreground(),
        "muted" => return this.bg_muted(),
        "muted-foreground" => return this.bg_muted_foreground(),
        "accent" => return this.bg_accent(),
        "accent-foreground" => return this.bg_accent_foreground(),
        "destructive" => return this.bg_destructive(),
        "destructive-foreground" => return this.bg_destructive_foreground(),
        "card" => return this.bg_card(),
        "card-foreground" => return this.bg_card_foreground(),
        "popover" => return this.bg_popover(),
        "popover-foreground" => return this.bg_popover_foreground(),
        "border" => return this.bg_border(),
        "input" => return this.bg_input(),
        "ring" => return this.bg_ring(),
        "sidebar" => return this.bg_sidebar(),
        "sidebar-foreground" => return this.bg_sidebar_foreground(),
        "sidebar-primary" => return this.bg_sidebar_primary(),
        "sidebar-primary-foreground" => return this.bg_sidebar_primary_foreground(),
        "sidebar-accent" => return this.bg_sidebar_accent(),
        "sidebar-accent-foreground" => return this.bg_sidebar_accent_foreground(),
        "sidebar-border" => return this.bg_sidebar_border(),
        "sidebar-ring" => return this.bg_sidebar_ring(),
        "disabled" => return this.bg_disabled(),
        "disabled-foreground" => return this.bg_disabled_foreground(),
        "chart-1" => return this.bg_chart_1(),
        "chart-2" => return this.bg_chart_2(),
        "chart-3" => return this.bg_chart_3(),
        "chart-4" => return this.bg_chart_4(),
        "chart-5" => return this.bg_chart_5(),
        "white" => return this.bg_white(),
        "black" => return this.bg_black(),
        _ => {}
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.background(c);
        }
    }
    this
}

fn style_text<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "xs" => return this.text_xs(),
        "sm" => return this.text_sm(),
        "base" => return this.text_base(),
        "lg" => return this.text_lg(),
        "xl" => return this.text_xl(),
        "2xl" => return this.text_2xl(),
        "3xl" => return this.text_3xl(),
        "4xl" => return this.text_4xl(),
        "5xl" => return this.text_5xl(),
        "6xl" => return this.text_6xl(),
        "7xl" => return this.text_7xl(),
        "8xl" => return this.text_8xl(),
        "9xl" => return this.text_9xl(),
        "left" => return this.text_left(),
        "center" => return this.text_center(),
        "right" => return this.text_right(),
        "justify" => return this.text_justify(),
        _ => {}
    }
    match rest {
        "background" => return this.text_background(),
        "foreground" => return this.text_foreground(),
        "primary" => return this.text_primary(),
        "primary-foreground" => return this.text_primary_foreground(),
        "secondary" => return this.text_secondary(),
        "secondary-foreground" => return this.text_secondary_foreground(),
        "muted" => return this.text_muted(),
        "muted-foreground" => return this.text_muted_foreground(),
        "accent" => return this.text_accent(),
        "accent-foreground" => return this.text_accent_foreground(),
        "destructive" => return this.text_destructive(),
        "destructive-foreground" => return this.text_destructive_foreground(),
        "card" => return this.text_card(),
        "card-foreground" => return this.text_card_foreground(),
        "popover" => return this.text_popover(),
        "popover-foreground" => return this.text_popover_foreground(),
        "border" => return this.text_border(),
        "input" => return this.text_input(),
        "ring" => return this.text_ring(),
        "sidebar" => return this.text_sidebar(),
        "sidebar-foreground" => return this.text_sidebar_foreground(),
        "sidebar-primary" => return this.text_sidebar_primary(),
        "sidebar-primary-foreground" => return this.text_sidebar_primary_foreground(),
        "sidebar-accent" => return this.text_sidebar_accent(),
        "sidebar-accent-foreground" => return this.text_sidebar_accent_foreground(),
        "sidebar-border" => return this.text_sidebar_border(),
        "sidebar-ring" => return this.text_sidebar_ring(),
        "disabled" => return this.text_disabled(),
        "disabled-foreground" => return this.text_disabled_foreground(),
        "chart-1" => return this.text_chart_1(),
        "chart-2" => return this.text_chart_2(),
        "chart-3" => return this.text_chart_3(),
        "chart-4" => return this.text_chart_4(),
        "chart-5" => return this.text_chart_5(),
        "white" => return this.text_white(),
        "black" => return this.text_black(),
        _ => {}
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.color(c);
        }
    }
    this
}

fn style_border<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "" => return this.border(),
        "0" => return this.border_0(),
        "2" => return this.border_2(),
        "4" => return this.border_4(),
        "8" => return this.border_8(),
        "primary" => return this.border_primary(),
        "destructive" => return this.border_destructive(),
        "solid" => return this.border_solid(),
        "dashed" => return this.border_dashed(),
        "dotted" => return this.border_dotted(),
        "double" => return this.border_double(),
        "none" => return this.border_none(),
        _ => {}
    }
    match rest {
        "t" => return this.border_t(1.0),
        "r" => return this.border_r(1.0),
        "b" => return this.border_b(1.0),
        "l" => return this.border_l(1.0),
        "x" => return this.border_x(1.0),
        "y" => return this.border_y(1.0),
        _ => {}
    }
    if let Some((side, val)) = rest.split_once('-') {
        if let Some(px) = parse_len(val) {
            match side {
                "t" => return this.border_t(px),
                "r" => return this.border_r(px),
                "b" => return this.border_b(px),
                "l" => return this.border_l(px),
                "x" => return this.border_x(px),
                "y" => return this.border_y(px),
                _ => {}
            }
        }
    }
    if let Some(px) = parse_len(rest) {
        return this.border_width(px);
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.border_color(c);
        }
    }
    this
}

fn style_rounded<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "none" => return this.rounded_none(),
        "sm" => return this.rounded_sm(),
        "md" => return this.rounded_md(),
        "lg" => return this.rounded_lg(),
        "xl" => return this.rounded_xl(),
        "2xl" => return this.rounded_2xl(),
        "3xl" => return this.rounded_3xl(),
        "full" => return this.rounded_full(),
        _ => {}
    }
    if let Some(px) = parse_len(rest) {
        return this.corner_radius(px);
    }
    this
}

fn style_font<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "thin" => this.font_thin(),
        "extralight" => this.font_extralight(),
        "light" => this.font_light(),
        "normal" => this.font_normal(),
        "medium" => this.font_medium(),
        "semibold" => this.font_semibold(),
        "bold" => this.font_bold(),
        "extrabold" => this.font_extrabold(),
        "black" => this.font_black_weight(),
        "sans" => this.font_sans(),
        "serif" => this.font_serif(),
        "mono" => this.font_mono(),
        _ => this,
    }
}

fn style_leading<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "none" => this.line_height(1.0),
        "tight" => this.line_height(1.25),
        "snug" => this.line_height(1.375),
        "normal" => this.line_height(1.5),
        "relaxed" => this.line_height(1.625),
        "loose" => this.line_height(2.0),
        _ => {
            if let Some(v) = parse_len(rest) {
                this.line_height(v)
            } else {
                this
            }
        }
    }
}

fn style_tracking<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "tighter" => this.letter_spacing(-0.05),
        "tight" => this.letter_spacing(-0.025),
        "normal" => this.letter_spacing(0.0),
        "wide" => this.letter_spacing(0.025),
        "wider" => this.letter_spacing(0.05),
        "widest" => this.letter_spacing(0.1),
        _ => {
            if let Some(v) = parse_len(rest) {
                this.letter_spacing(v)
            } else {
                this
            }
        }
    }
}

fn style_shadow<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "sm" => this.shadow_sm(),
        "md" => this.shadow_md(),
        "lg" => this.shadow_lg(),
        "xl" => this.shadow_xl(),
        "2xl" => this.shadow_2xl(),
        "none" => this.shadow_none(),
        _ => this,
    }
}

fn style_opacity<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(v) = rest.parse::<f32>() {
        let v = if v > 1.0 { v / 100.0 } else { v };
        this.opacity(v.clamp(0.0, 1.0))
    } else {
        this
    }
}

fn style_justify<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "start" => this.justify_start(),
        "center" => this.justify_center(),
        "end" => this.justify_end(),
        "between" => this.justify_between(),
        "around" => this.justify_around(),
        "evenly" => this.justify_evenly(),
        _ => this,
    }
}

fn style_items<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "start" => this.items_start(),
        "center" => this.items_center(),
        "end" => this.items_end(),
        _ => this,
    }
}

fn style_self_<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "start" => this.self_start(),
        "center" => this.self_center(),
        "end" => this.self_end(),
        _ => this,
    }
}

fn style_content<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "start" => this.content_start(),
        "center" => this.content_center(),
        "end" => this.content_end(),
        "between" => this.content_between(),
        "around" => this.content_around(),
        "evenly" => this.content_evenly(),
        _ => this,
    }
}

fn style_z<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(v) = rest.parse::<i32>() {
        this.z_index(v)
    } else {
        this
    }
}

// ---------- Scale dispatchers ----------

macro_rules! style_scale {
    ($fn_name:ident, $base:ident; $($name:literal),* $(,)?) => {
        fn $fn_name<T: StyleExt>(this: T, rest: &str) -> T {
            paste! {
                $(
                    if rest == stringify!($name).replace('_', ".") {
                        return this.[<$base _ $name>]();
                    }
                )*
            }
            if let Some(px) = parse_len(rest) { return this.$base(px); }
            this
        }
    };
}

style_scale!(style_gap, gap; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_p, p; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_px, px; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_py, py; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_pt, pt; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_pr, pr; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_pb, pb; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_pl, pl; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_m, m; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_mx, mx; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_my, my; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_mt, mt; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_mr, mr; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_mb, mb; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_ml, ml; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_min_w, min_w; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_min_h, min_h; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_max_w, max_w; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
style_scale!(style_max_h, max_h; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);

fn style_w<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "full" | "screen" => return this.w_full(),
        "auto" => return this.w_auto(),
        _ => {}
    }
    if let Some((num, den)) = rest.split_once('/') {
        if let (Ok(n), Ok(d)) = (num.parse::<f32>(), den.parse::<f32>()) {
            if d != 0.0 {
                return this.w_percent((n / d) * 100.0);
            }
        }
    }
    style_scale_dispatch!(this, rest, w; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(px) = parse_len(rest) {
        return this.w(px);
    }
    if let Some(p) = rest.strip_suffix('%') {
        if let Ok(pct) = p.parse::<f32>() {
            return this.w_percent(pct);
        }
    }
    this
}

fn style_h<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "full" | "screen" => return this.h_full(),
        "auto" => return this.h_auto(),
        _ => {}
    }
    if let Some((num, den)) = rest.split_once('/') {
        if let (Ok(n), Ok(d)) = (num.parse::<f32>(), den.parse::<f32>()) {
            if d != 0.0 {
                return this.h_percent((n / d) * 100.0);
            }
        }
    }
    style_scale_dispatch!(this, rest, h; 0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(px) = parse_len(rest) {
        return this.h(px);
    }
    if let Some(p) = rest.strip_suffix('%') {
        if let Ok(pct) = p.parse::<f32>() {
            return this.h_percent(pct);
        }
    }
    this
}
