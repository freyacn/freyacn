//! # Unified Style Extension Trait
//!
//! A single, cohesive surface for every Tailwind‑inspired styling helper the
//! crate offers: background, foreground colour, typography, borders, effects,
//! flex, sizing and spacing — plus a [`style`](StyleExt::style) method that
//! accepts Tailwind‑style style strings.
//!
//! ## Design
//!
//! There is **one** required method:
//!
//! ```ignore
//! fn get_style(&mut self) -> &mut Style;
//! ```
//!
//! Everything else — including [`style`](StyleExt::style) — is a provided
//! method. Your component stores a [`Style`] field and returns a mutable
//! reference to it, and it inherits every helper for free.
//!
//! ## Quick reference
//!
//! | Category | Prefix | Examples |
//! | --- | --- | --- |
//! | Background colour | `bg_` | `bg_primary()`, `bg_slate_500()`, `bg_white()` |
//! | Text colour | `text_` | `text_primary()`, `text_red_500()`, `text_black()` |
//! | Font size | `text_` | `text_xs()` … `text_9xl()` |
//! | Font weight | `font_` | `font_thin()` … `font_black_weight()` |
//! | Font slant | — | `italic()`, `not_italic()` |
//! | Font family | `font_` | `font_sans()`, `font_serif()`, `font_mono()` |
//! | Alignment | `text_` | `text_left()`, `text_center()`, `text_right()`, `text_justify()` |
//! | Decoration | — | `underline()`, `overline()`, `line_through()`, `no_underline()` |
//! | Border width | `border_` | `border_0()`, `border_2()`, `border_4()`, `border_8()` |
//! | Border colour | `border_` | `border_primary()`, `border_destructive()` |
//! | Corner radius | — | `corner_radius(8.0)` |
//! | Opacity | `opacity_` | `opacity_0()` … `opacity_100()` |
//! | Shadow | `shadow_` | `shadow_sm()` … `shadow_xl()`, `shadow_none()` |
//! | Flex direction | `flex_` | `flex_row()`, `flex_col()` |
//! | Main axis | `justify_` | `justify_start()` … `justify_evenly()` |
//! | Cross axis | `items_` | `items_start()`, `items_center()`, `items_end()` |
//! | Gap | `gap_` | `gap(8.0)`, `gap_0()` … `gap_96()` |
//! | Sizing | `w_`, `h_`, `min_w_`, … | `w_4()`, `h_full()`, `max_w_64()` |
//! | Padding | `p_`, `px_`, … | `p_4()`, `px_6()`, `pt_2()` |
//! | Margin | `m_`, `mx_`, … | `m_4()`, `my_2()` |
//!
//! ## Examples
//!
//! ### Minimal — the one-method impl
//!
//! ```no_run
//! use freyacn::extensions::{Style, StyleExt};
//!
//! struct Card { style: Style }
//!
//! impl StyleExt for Card {
//!     fn get_style(&mut self) -> &mut Style {
//!         &mut self.style
//!     }
//! }
//!
//! // Every helper is now available:
//! let card = Card { style: Style::default() }
//!     .bg_card()
//!     .text_card_foreground()
//!     .p_6()
//!     .gap_4()
//!     .border_2()
//!     .border_primary()
//!     .corner_radius(8.0)
//!     .shadow_md();
//! ```
//!
//! ### A CTA button
//!
//! ```no_run
//! # use freyacn::extensions::{Style, StyleExt};
//! # struct Button { style: Style }
//! # impl StyleExt for Button {
//! #     fn get_style(&mut self) -> &mut Style { &mut self.style }
//! # }
//! let cta = Button { style: Style::default() }
//!     .bg_primary()
//!     .text_primary_foreground()
//!     .text_sm()
//!     .font_semibold()
//!     .px_4()
//!     .py_2()
//!     .corner_radius(6.0)
//!     .shadow_sm();
//! ```
//!
//! ### Using style strings
//!
//! ```no_run
//! # use freyacn::extensions::{Style, StyleExt};
//! let s = Style::default()
//!     .style("bg-red-500 text-white p-4 shadow-md");
//! ```
//!
//! See [`style`](StyleExt::style) for the full list of recognised tokens.
//!
//! ### Mixing style strings with typed helpers
//!
//! ```no_run
//! # use freyacn::extensions::{Style, StyleExt};
//! let s = Style::default()
//!     .style("bg-slate-800 text-slate-100 p-6 gap-4")
//!     .corner_radius(12.0)     // typed, outside the style API
//!     .shadow_lg();
//! ```
//!
//! ### Rendering the style
//!
//! When you're ready to render, forward the fields to the relevant Freya
//! builder methods:
//!
//! ```no_run
//! # use freyacn::extensions::{Style, StyleExt};
//! use freya::prelude::*;
//!
//! fn render_card(style: &Style) -> IntoElement {
//!     let mut rect = rect()
//!         .background(style.background)
//!         .corner_radius(style.corner_radius)
//!         .padding(style.padding);
//!     if let Some(w) = style.width  { rect = rect.width(w); }
//!     if let Some(h) = style.height { rect = rect.height(h); }
//!     rect.into()
//! }
//! ```

use std::borrow::Cow;

use crate::theme::use_cn_theme;
use freya::prelude::{
    Alignment, Border, CornerRadius, Direction, Fill, FontSize, FontSlant, FontWeight, Gaps,
    Shadow, Size, StyleState, TextAlign, TextDecoration,
};
use freya_core::prelude::Color;
use paste::paste;

/// The spacing unit used across all Tailwind‑style scales (1 unit = 4 px).
const SPACING_UNIT: f32 = 4.0;

// ==============================================================
// Style: the data container
// ==============================================================

/// A single, cohesive bag of styling state consumed by [`StyleExt`] helpers.
///
/// Every helper on `StyleExt` mutates one field of this struct. When you are
/// ready to render, walk the fields and forward them to your underlying
/// element (typically a Freya `Rect`, `Label`, `Paragraph`, …).
///
/// # Construction
///
/// ```
/// # use freyacn::extensions::Style;
/// let style = Style::default();
/// ```
///
/// # Reading back a `Border`
///
/// ```
/// # use freyacn::extensions::{Style, StyleExt};
/// let style = Style::default().border_2().border_primary();
/// assert!(style.to_border().is_some());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    // ---- Box styling (background, corners, borders, shadows) ----
    /// Fill painted behind the element.
    pub background: Fill,
    /// Corner radius applied to the element's box.
    pub corner_radius: CornerRadius,
    /// Border width in pixels, if a border is set.
    pub border_width: Option<f32>,
    /// Border colour, if a border is set.
    pub border_color: Option<Color>,
    /// Shadows cast by the element (stackable).
    pub shadows: Vec<Shadow>,

    // ---- Text styling ----
    /// Foreground (text/icon) colour.
    pub text_color: Option<Color>,
    /// Font size.
    pub font_size: Option<FontSize>,
    /// Font slant (normal or italic).
    pub font_slant: Option<FontSlant>,
    /// Font weight.
    pub font_weight: Option<FontWeight>,
    /// Font families, in order of preference.
    pub font_families: Vec<Cow<'static, str>>,
    /// Horizontal text alignment.
    pub text_align: Option<TextAlign>,
    /// Text decoration (underline, overline, line-through).
    pub text_decoration: Option<TextDecoration>,

    // ---- Layout: spacing ----
    /// Padding on each side.
    pub padding: Gaps,
    /// Margin on each side.
    pub margin: Gaps,

    // ---- Layout: sizing ----
    /// Width.
    pub width: Option<Size>,
    /// Height.
    pub height: Option<Size>,
    /// Minimum width.
    pub min_width: Option<Size>,
    /// Minimum height.
    pub min_height: Option<Size>,
    /// Maximum width.
    pub max_width: Option<Size>,
    /// Maximum height.
    pub max_height: Option<Size>,

    // ---- Layout: flex ----
    /// Flex direction.
    pub direction: Option<Direction>,
    /// Main axis alignment.
    pub main_align: Option<Alignment>,
    /// Cross axis alignment.
    pub cross_align: Option<Alignment>,
    /// Gap between flex children.
    pub spacing: Option<f32>,

    // ---- Effects ----
    /// Opacity, in `[0.0, 1.0]`.
    pub opacity: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        let default_style = StyleState::default();
        Self {
            background: default_style.background,
            corner_radius: default_style.corner_radius,
            border_width: None,
            border_color: None,
            shadows: Vec::new(),
            text_color: None,
            font_size: None,
            font_slant: None,
            font_weight: None,
            font_families: Vec::new(),
            text_align: None,
            text_decoration: None,
            padding: Gaps::new_all(0.0),
            margin: Gaps::new_all(0.0),
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            direction: None,
            main_align: None,
            cross_align: None,
            spacing: None,
            opacity: None,
        }
    }
}

impl Style {
    /// Build a fresh, empty `Style` with all fields reset to defaults.
    ///
    /// Equivalent to [`Style::default`].
    ///
    /// ```
    /// # use freyacn::extensions::Style;
    /// let s = Style::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert the accumulated border width / colour into a [`Border`], if a
    /// border width has been set.
    ///
    /// ```
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default().border_2().border_primary();
    /// assert!(s.to_border().is_some());
    ///
    /// let s = Style::default();
    /// assert!(s.to_border().is_none());
    /// ```
    pub fn to_border(&self) -> Option<Border> {
        self.border_width.map(|width| {
            let mut b = Border::default().width(width);
            if let Some(colour) = self.border_color {
                b = b.fill(colour);
            }
            b
        })
    }
}

// ==============================================================
// Macros
// ==============================================================

/// Generates every shade of one palette family for a given prefix.
/// Generates every shade of one palette family for a given prefix.
///
/// Two call forms:
///
/// * `palette_family!(text, color, slate)` — expands all 11 shades.
/// * `palette_family!(text, color, slate; 50, 500, 950)` — explicit shades.
///
/// The first form forwards to the second with the standard
/// `50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950` list.
macro_rules! palette_family {
    ($prefix:ident, $setter:ident, $family:ident) => {
        palette_family! {
            $prefix, $setter, $family;
            50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950
        }
    };
    ($prefix:ident, $setter:ident, $family:ident; $($shade:literal),* $(,)?) => {
        paste! {
            $(
                #[doc = concat!("Palette colour helper — `", stringify!($prefix), "_", stringify!($family), "_", stringify!($shade), "`.")]
                fn [<$prefix _ $family _ $shade>](self) -> Self {
                    let theme = use_cn_theme().read();
                    self.$setter(theme.colors.[<$family _ $shade>])
                }
            )*
        }
    };
}

/// Generates semantic-token helpers.
macro_rules! semantic_color {
    ($prefix:ident, $setter:ident, $($field:ident),* $(,)?) => {
        $(
            paste! {
                #[doc = concat!("Semantic colour helper — `", stringify!($prefix), "_", stringify!($field), "`.")]
                fn [<$prefix _ $field>](self) -> Self {
                    let theme = use_cn_theme().read();
                    self.$setter(theme.$field)
                }
            }
        )*
    };
}

/// Generates `gap_N`, `w_N`, `p_N`, … scale helpers.
macro_rules! spacing_scale {
    ($prefix:ident, $base:ident, $($value:literal),* $(,)?) => {
        paste! {
            $(
                fn [<$prefix _ $value>](self) -> Self {
                    self.$base(SPACING_UNIT * ($value as f32))
                }
            )*
        }
    };
}

// ==============================================================
// style-string parser  (private)
// ==============================================================
//
// Every function below takes ownership of a `T: StyleExt`, dispatches a
// single token, and returns the possibly-mutated `T`. Unknown tokens are
// silently ignored.

/// Parse and apply one whitespace-separated token.
fn style_token<T: StyleExt>(this: T, token: &str) -> T {
    // 1. Fixed-literal stylees.
    match token {
        "italic" => return this.italic(),
        "not-italic" => return this.not_italic(),
        "underline" => return this.underline(),
        "overline" => return this.overline(),
        "line-through" => return this.line_through(),
        "no-underline" => return this.no_underline(),
        "flex-row" => return this.flex_row(),
        "flex-col" => return this.flex_col(),
        _ => {}
    }

    // 2. Multi-word prefixes must come before their single-word suffixes.
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

    // 3. Single-word prefixes.
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
    if let Some(r) = token.strip_prefix("gap-") {
        return style_gap(this, r);
    }
    if let Some(r) = token.strip_prefix("justify-") {
        return style_justify(this, r);
    }
    if let Some(r) = token.strip_prefix("items-") {
        return style_items(this, r);
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
    // Sizes
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
    // Semantic
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
        "chart-1" => return this.text_chart_1(),
        "chart-2" => return this.text_chart_2(),
        "chart-3" => return this.text_chart_3(),
        "chart-4" => return this.text_chart_4(),
        "chart-5" => return this.text_chart_5(),
        "white" => return this.text_white(),
        "black" => return this.text_black(),
        _ => {}
    }
    // Palette
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.color(c);
        }
    }
    this
}

fn style_border<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => return this.border_0(),
        "2" => return this.border_2(),
        "4" => return this.border_4(),
        "8" => return this.border_8(),
        "primary" => return this.border_primary(),
        "destructive" => return this.border_destructive(),
        _ => {}
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.border_color(c);
        }
    }
    this
}

// ---------- Typography dispatchers ----------

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

// ---------- Effect dispatchers ----------

fn style_shadow<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "sm" => this.shadow_sm(),
        "md" => this.shadow_md(),
        "lg" => this.shadow_lg(),
        "xl" => this.shadow_xl(),
        "none" => this.shadow_none(),
        _ => this,
    }
}

fn style_opacity<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => this.opacity_0(),
        "25" => this.opacity_25(),
        "50" => this.opacity_50(),
        "75" => this.opacity_75(),
        "100" => this.opacity_100(),
        _ => {
            if let Ok(v) = rest.parse::<f32>() {
                this.opacity(v.clamp(0.0, 1.0))
            } else {
                this
            }
        }
    }
}

// ---------- Layout dispatchers ----------

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

// ---------- Scale dispatchers ----------

/// Generates a `<name>_N()`-aware style dispatcher that falls back to raw
/// pixel parsing.
macro_rules! style_scale {
    ($fn_name:ident, $base:ident; $($n:literal),* $(,)?) => {
        fn $fn_name<T: StyleExt>(this: T, rest: &str) -> T {
            paste! {
                $(
                    if rest == stringify!($n) {
                        return this.[<$base _ $n>]();
                    }
                )*
            }
            if let Ok(px) = rest.parse::<f32>() {
                return this.$base(px);
            }
            this
        }
    };
}

style_scale!(style_gap, gap; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);

style_scale!(style_p,  p;  0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_px, px; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_py, py; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_pt, pt; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_pr, pr; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_pb, pb; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_pl, pl; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);

style_scale!(style_m,  m;  0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_mx, mx; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_my, my; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_mt, mt; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_mr, mr; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_mb, mb; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_ml, ml; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);

style_scale!(style_min_w, min_w; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_min_h, min_h; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_max_w, max_w; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);
style_scale!(style_max_h, max_h; 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96);

// `w-` and `h-` need special handling for `full` / `screen` / `auto` / `%`.
fn style_w<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "full" | "screen" => return this.w_full(),
        "auto" => return this.w_auto(),
        _ => {}
    }
    paste! {
        if rest == "0"   { return this.w_0(); }
        if rest == "1"   { return this.w_1(); }
        if rest == "2"   { return this.w_2(); }
        if rest == "3"   { return this.w_3(); }
        if rest == "4"   { return this.w_4(); }
        if rest == "5"   { return this.w_5(); }
        if rest == "6"   { return this.w_6(); }
        if rest == "8"   { return this.w_8(); }
        if rest == "10"  { return this.w_10(); }
        if rest == "12"  { return this.w_12(); }
        if rest == "16"  { return this.w_16(); }
        if rest == "20"  { return this.w_20(); }
        if rest == "24"  { return this.w_24(); }
        if rest == "32"  { return this.w_32(); }
        if rest == "48"  { return this.w_48(); }
        if rest == "64"  { return this.w_64(); }
        if rest == "96"  { return this.w_96(); }
    }
    if let Ok(px) = rest.parse::<f32>() {
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
    paste! {
        if rest == "0"   { return this.h_0(); }
        if rest == "1"   { return this.h_1(); }
        if rest == "2"   { return this.h_2(); }
        if rest == "3"   { return this.h_3(); }
        if rest == "4"   { return this.h_4(); }
        if rest == "5"   { return this.h_5(); }
        if rest == "6"   { return this.h_6(); }
        if rest == "8"   { return this.h_8(); }
        if rest == "10"  { return this.h_10(); }
        if rest == "12"  { return this.h_12(); }
        if rest == "16"  { return this.h_16(); }
        if rest == "20"  { return this.h_20(); }
        if rest == "24"  { return this.h_24(); }
        if rest == "32"  { return this.h_32(); }
        if rest == "48"  { return this.h_48(); }
        if rest == "64"  { return this.h_64(); }
        if rest == "96"  { return this.h_96(); }
    }
    if let Ok(px) = rest.parse::<f32>() {
        return this.h(px);
    }
    if let Some(p) = rest.strip_suffix('%') {
        if let Ok(pct) = p.parse::<f32>() {
            return this.h_percent(pct);
        }
    }
    this
}

// ==============================================================
// Palette lookup
// ==============================================================

/// Look up a Tailwind-palette colour by family and shade from the active theme.
///
/// Returns `None` if the `(family, shade)` pair is not a valid palette entry.
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

// ==============================================================
// The StyleExt trait
// ==============================================================

/// Unified Tailwind‑inspired styling extension.
///
/// Implement this on any component by returning a mutable reference to a
/// [`Style`] field. Every helper the crate offers then becomes available on
/// your type.
///
/// # Required method
///
/// [`get_style`](StyleExt::get_style) — return a mutable reference to the
/// element's [`Style`].
///
/// # Examples
///
/// ## A minimal component
///
/// ```no_run
/// use freyacn::extensions::{Style, StyleExt};
///
/// struct Card { style: Style }
///
/// impl StyleExt for Card {
///     fn get_style(&mut self) -> &mut Style {
///         &mut self.style
///     }
/// }
///
/// let card = Card { style: Style::default() }
///     .bg_card()
///     .corner_radius(12.0);
/// ```
///
/// ## A `#[component]`-based button
///
/// ```no_run
/// use freyacn::extensions::{Style, StyleExt};
/// use freya::prelude::*;
///
/// #[component]
/// fn PrimaryButton(label_text: String) -> IntoElement {
///     let style = Style::default()
///         .bg_primary()
///         .text_primary_foreground()
///         .text_sm()
///         .font_semibold()
///         .px_4()
///         .py_2()
///         .corner_radius(6.0)
///         .shadow_sm();
///
///     rect()
///         .background(style.background)
///         .corner_radius(style.corner_radius)
///         .padding(style.padding)
///         .child(label().text(label_text))
///         .into()
/// }
/// ```
///
/// ## Layered overrides (last write wins)
///
/// ```no_run
/// # use freyacn::extensions::{Style, StyleExt};
/// let style = Style::default()
///     .bg_slate_800()      // applied first…
///     .bg_emerald_400()    // …replaced by this
///     .text_white();       // foreground, independent
/// ```
///
/// ## style strings
///
/// ```no_run
/// # use freyacn::extensions::{Style, StyleExt};
/// let style = Style::default()
///     .style("bg-primary text-white p-4 gap-2 shadow-md font-bold text-sm");
/// ```
pub trait StyleExt: Sized {
    /// Return a mutable reference to the element's [`Style`].
    ///
    /// This is the only method you must implement.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// struct Card { style: Style }
    ///
    /// impl StyleExt for Card {
    ///     fn get_style(&mut self) -> &mut Style {
    ///         &mut self.style
    ///     }
    /// }
    /// ```
    fn get_style(&mut self) -> &mut Style;

    // ==========================================================
    // Base setters
    // ==========================================================

    /// Set the background with any [`Color`]. Every `bg_*` helper funnels
    /// through here.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// # use freya::prelude::Color;
    /// let s = Style::default().background(Color::from_rgb(30, 41, 59));
    /// ```
    fn background(mut self, color: Color) -> Self {
        self.get_style().background = color.into();
        self
    }

    /// Set the foreground (text / icon) colour. Every `text_*` colour helper
    /// funnels through here.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// # use freya::prelude::Color;
    /// let s = Style::default().color(Color::from_rgb(255, 255, 255));
    /// ```
    fn color(mut self, color: Color) -> Self {
        self.get_style().text_color = Some(color);
        self
    }

    /// Set the corner radius. Accepts `f32`, `[f32; 4]`, or `CornerRadius`.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default().corner_radius(8.0);
    /// ```
    fn corner_radius(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.get_style().corner_radius = radius.into();
        self
    }

    /// Set the border width in pixels.
    fn border_width(mut self, width: f32) -> Self {
        self.get_style().border_width = Some(width);
        self
    }

    /// Set the border colour.
    fn border_color(mut self, color: Color) -> Self {
        self.get_style().border_color = Some(color);
        self
    }

    /// Set the font size.
    fn font_size(mut self, size: impl Into<FontSize>) -> Self {
        self.get_style().font_size = Some(size.into());
        self
    }

    /// Set the font weight.
    fn font_weight(mut self, weight: impl Into<FontWeight>) -> Self {
        self.get_style().font_weight = Some(weight.into());
        self
    }

    /// Set the font slant.
    fn font_slant(mut self, slant: impl Into<FontSlant>) -> Self {
        self.get_style().font_slant = Some(slant.into());
        self
    }

    /// Add a font family, in order of preference.
    fn font_family(mut self, family: impl Into<Cow<'static, str>>) -> Self {
        self.get_style().font_families.push(family.into());
        self
    }

    /// Set the horizontal text alignment.
    fn text_align(mut self, align: impl Into<TextAlign>) -> Self {
        self.get_style().text_align = Some(align.into());
        self
    }

    /// Set the text decoration.
    fn text_decoration(mut self, decoration: impl Into<TextDecoration>) -> Self {
        self.get_style().text_decoration = Some(decoration.into());
        self
    }

    /// Set opacity, in the range `[0.0, 1.0]`.
    fn opacity(mut self, opacity: f32) -> Self {
        self.get_style().opacity = Some(opacity);
        self
    }

    /// Push a shadow (stackable).
    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.get_style().shadows.push(shadow.into());
        self
    }

    /// Set padding.
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.get_style().padding = gaps.into();
        self
    }

    /// Set margin.
    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.get_style().margin = gaps.into();
        self
    }

    /// Set width.
    fn width(mut self, size: impl Into<Size>) -> Self {
        self.get_style().width = Some(size.into());
        self
    }

    /// Set height.
    fn height(mut self, size: impl Into<Size>) -> Self {
        self.get_style().height = Some(size.into());
        self
    }

    /// Set minimum width.
    fn min_width(mut self, size: impl Into<Size>) -> Self {
        self.get_style().min_width = Some(size.into());
        self
    }

    /// Set minimum height.
    fn min_height(mut self, size: impl Into<Size>) -> Self {
        self.get_style().min_height = Some(size.into());
        self
    }

    /// Set maximum width.
    fn max_width(mut self, size: impl Into<Size>) -> Self {
        self.get_style().max_width = Some(size.into());
        self
    }

    /// Set maximum height.
    fn max_height(mut self, size: impl Into<Size>) -> Self {
        self.get_style().max_height = Some(size.into());
        self
    }

    /// Set flex direction.
    fn direction(mut self, dir: Direction) -> Self {
        self.get_style().direction = Some(dir);
        self
    }

    /// Set main-axis alignment.
    fn main_align(mut self, align: Alignment) -> Self {
        self.get_style().main_align = Some(align);
        self
    }

    /// Set cross-axis alignment.
    fn cross_align(mut self, align: Alignment) -> Self {
        self.get_style().cross_align = Some(align);
        self
    }

    /// Set gap between flex children (raw pixel value).
    fn spacing(mut self, spacing: f32) -> Self {
        self.get_style().spacing = Some(spacing);
        self
    }

    // ==========================================================
    // Palette colours
    // ==========================================================
    // Every family × shade pair is generated below.
    //
    // ```
    // # use freyacn::extensions::{Style, StyleExt};
    // let s = Style::default()
    //     .bg_slate_900()
    //     .text_slate_100();
    // ```

    palette_family!(text, color, slate);
    palette_family!(text, color, gray);
    palette_family!(text, color, zinc);
    palette_family!(text, color, neutral);
    palette_family!(text, color, stone);
    palette_family!(text, color, mauve);
    palette_family!(text, color, olive);
    palette_family!(text, color, mist);
    palette_family!(text, color, taupe);
    palette_family!(text, color, red);
    palette_family!(text, color, orange);
    palette_family!(text, color, amber);
    palette_family!(text, color, yellow);
    palette_family!(text, color, lime);
    palette_family!(text, color, green);
    palette_family!(text, color, emerald);
    palette_family!(text, color, teal);
    palette_family!(text, color, cyan);
    palette_family!(text, color, sky);
    palette_family!(text, color, blue);
    palette_family!(text, color, indigo);
    palette_family!(text, color, violet);
    palette_family!(text, color, purple);
    palette_family!(text, color, fuchsia);
    palette_family!(text, color, pink);
    palette_family!(text, color, rose);

    palette_family!(bg, background, slate);
    palette_family!(bg, background, gray);
    palette_family!(bg, background, zinc);
    palette_family!(bg, background, neutral);
    palette_family!(bg, background, stone);
    palette_family!(bg, background, mauve);
    palette_family!(bg, background, olive);
    palette_family!(bg, background, mist);
    palette_family!(bg, background, taupe);
    palette_family!(bg, background, red);
    palette_family!(bg, background, orange);
    palette_family!(bg, background, amber);
    palette_family!(bg, background, yellow);
    palette_family!(bg, background, lime);
    palette_family!(bg, background, green);
    palette_family!(bg, background, emerald);
    palette_family!(bg, background, teal);
    palette_family!(bg, background, cyan);
    palette_family!(bg, background, sky);
    palette_family!(bg, background, blue);
    palette_family!(bg, background, indigo);
    palette_family!(bg, background, violet);
    palette_family!(bg, background, purple);
    palette_family!(bg, background, fuchsia);
    palette_family!(bg, background, pink);
    palette_family!(bg, background, rose);

    // ==========================================================
    // Semantic colours
    // ==========================================================
    // Every semantic token maps to a field on the active theme.
    //
    // ```
    // # use freyacn::extensions::{Style, StyleExt};
    // let s = Style::default()
    //     .bg_card()
    //     .text_card_foreground();
    // ```

    semantic_color!(
        text,
        color,
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
    );

    semantic_color!(
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
    );

    // ==========================================================
    // Literal colours
    // ==========================================================

    /// Set the foreground to pure white (`#ffffff`).
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default().text_white();
    /// ```
    fn text_white(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.colors.white)
    }

    /// Set the foreground to pure black (`#000000`).
    fn text_black(self) -> Self {
        let theme = use_cn_theme().read();
        self.color(theme.colors.black)
    }

    /// Set the background to pure white (`#ffffff`).
    fn bg_white(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.colors.white)
    }

    /// Set the background to pure black (`#000000`).
    fn bg_black(self) -> Self {
        let theme = use_cn_theme().read();
        self.background(theme.colors.black)
    }

    // ==========================================================
    // Typography — sizes
    // ==========================================================
    // Tailwind scale, in CSS pixels.
    //
    // ```
    // # use freyacn::extensions::{Style, StyleExt};
    // let heading = Style::default().text_3xl().font_bold();
    // let body    = Style::default().text_base();
    // let hint    = Style::default().text_xs().text_muted_foreground();
    // ```

    /// 12 px — `text-xs`.
    fn text_xs(self) -> Self {
        self.font_size(12.0)
    }
    /// 14 px — `text-sm`.
    fn text_sm(self) -> Self {
        self.font_size(14.0)
    }
    /// 16 px — `text-base`.
    fn text_base(self) -> Self {
        self.font_size(16.0)
    }
    /// 18 px — `text-lg`.
    fn text_lg(self) -> Self {
        self.font_size(18.0)
    }
    /// 20 px — `text-xl`.
    fn text_xl(self) -> Self {
        self.font_size(20.0)
    }
    /// 24 px — `text-2xl`.
    fn text_2xl(self) -> Self {
        self.font_size(24.0)
    }
    /// 30 px — `text-3xl`.
    fn text_3xl(self) -> Self {
        self.font_size(30.0)
    }
    /// 36 px — `text-4xl`.
    fn text_4xl(self) -> Self {
        self.font_size(36.0)
    }
    /// 48 px — `text-5xl`.
    fn text_5xl(self) -> Self {
        self.font_size(48.0)
    }
    /// 60 px — `text-6xl`.
    fn text_6xl(self) -> Self {
        self.font_size(60.0)
    }
    /// 72 px — `text-7xl`.
    fn text_7xl(self) -> Self {
        self.font_size(72.0)
    }
    /// 96 px — `text-8xl`.
    fn text_8xl(self) -> Self {
        self.font_size(96.0)
    }
    /// 128 px — `text-9xl`.
    fn text_9xl(self) -> Self {
        self.font_size(128.0)
    }

    // ==========================================================
    // Typography — weights
    // ==========================================================
    // ```
    // # use freyacn::extensions::{Style, StyleExt};
    // let bold    = Style::default().font_bold();
    // let regular = Style::default().font_normal();
    // ```

    /// Weight 100 — `font-thin`.
    fn font_thin(self) -> Self {
        self.font_weight(FontWeight::THIN)
    }
    /// Weight 200 — `font-extralight`.
    fn font_extralight(self) -> Self {
        self.font_weight(FontWeight::EXTRA_LIGHT)
    }
    /// Weight 300 — `font-light`.
    fn font_light(self) -> Self {
        self.font_weight(FontWeight::LIGHT)
    }
    /// Weight 400 — `font-normal`.
    fn font_normal(self) -> Self {
        self.font_weight(FontWeight::NORMAL)
    }
    /// Weight 500 — `font-medium`.
    fn font_medium(self) -> Self {
        self.font_weight(FontWeight::MEDIUM)
    }
    /// Weight 600 — `font-semibold`.
    fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SEMI_BOLD)
    }
    /// Weight 700 — `font-bold`.
    fn font_bold(self) -> Self {
        self.font_weight(FontWeight::BOLD)
    }
    /// Weight 800 — `font-extrabold`.
    fn font_extrabold(self) -> Self {
        self.font_weight(FontWeight::EXTRA_BOLD)
    }
    /// Weight 900 — `font-black`. Renamed to avoid clashing with `text_black`.
    fn font_black_weight(self) -> Self {
        self.font_weight(FontWeight::BLACK)
    }

    // ==========================================================
    // Typography — slant / family / align / decoration
    // ==========================================================

    /// Apply italic slant — `italic`.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default().italic();
    /// ```
    fn italic(self) -> Self {
        self.font_slant(FontSlant::Italic)
    }
    /// Reset slant to upright — `not-italic`.
    fn not_italic(self) -> Self {
        self.font_slant(FontSlant::Upright)
    }

    /// Use a sans‑serif family — `font-sans`.
    fn font_sans(self) -> Self {
        self.font_family("sans-serif")
    }
    /// Use a serif family — `font-serif`.
    fn font_serif(self) -> Self {
        self.font_family("serif")
    }
    /// Use a monospace family — `font-mono`.
    fn font_mono(self) -> Self {
        self.font_family("monospace")
    }

    /// Align text to the left.
    fn text_left(self) -> Self {
        self.text_align(TextAlign::Left)
    }
    /// Center text.
    fn text_center(self) -> Self {
        self.text_align(TextAlign::Center)
    }
    /// Align text to the right.
    fn text_right(self) -> Self {
        self.text_align(TextAlign::Right)
    }
    /// Justify text.
    fn text_justify(self) -> Self {
        self.text_align(TextAlign::Justify)
    }

    /// Underline the text.
    fn underline(self) -> Self {
        self.text_decoration(TextDecoration::Underline)
    }
    /// Overline the text.
    fn overline(self) -> Self {
        self.text_decoration(TextDecoration::Overline)
    }
    /// Strike through the text.
    fn line_through(self) -> Self {
        self.text_decoration(TextDecoration::LineThrough)
    }
    /// Remove any decoration.
    fn no_underline(self) -> Self {
        self.text_decoration(TextDecoration::None)
    }

    // ==========================================================
    // Borders
    // ==========================================================

    /// No border (0 px).
    fn border_0(self) -> Self {
        self.border_width(0.0)
    }
    /// 2 px border.
    fn border_2(self) -> Self {
        self.border_width(2.0)
    }
    /// 4 px border.
    fn border_4(self) -> Self {
        self.border_width(4.0)
    }
    /// 8 px border.
    fn border_8(self) -> Self {
        self.border_width(8.0)
    }

    /// Border colour = theme `primary`.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default().border_2().border_primary();
    /// ```
    fn border_primary(self) -> Self {
        let theme = use_cn_theme().read();
        self.border_color(theme.primary)
    }
    /// Border colour = theme `destructive`.
    fn border_destructive(self) -> Self {
        let theme = use_cn_theme().read();
        self.border_color(theme.destructive)
    }

    // ==========================================================
    // Effects
    // ==========================================================

    /// Opacity 0 % (transparent).
    fn opacity_0(self) -> Self {
        self.opacity(0.0)
    }
    /// Opacity 25 %.
    fn opacity_25(self) -> Self {
        self.opacity(0.25)
    }
    /// Opacity 50 %.
    fn opacity_50(self) -> Self {
        self.opacity(0.5)
    }
    /// Opacity 75 %.
    fn opacity_75(self) -> Self {
        self.opacity(0.75)
    }
    /// Opacity 100 % (opaque).
    fn opacity_100(self) -> Self {
        self.opacity(1.0)
    }

    /// Small shadow: `(0, 1, 2, 0)` with 30 % black.
    fn shadow_sm(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(1.0)
                .blur(2.0)
                .spread(0.0)
                .color((0, 0, 0, 0.30)),
        )
    }
    /// Medium shadow: `(0, 4, 6, -1)` with 15 % black.
    fn shadow_md(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(4.0)
                .blur(6.0)
                .spread(-1.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    /// Large shadow: `(0, 10, 15, -3)` with 15 % black.
    fn shadow_lg(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(10.0)
                .blur(15.0)
                .spread(-3.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    /// Extra-large shadow: `(0, 20, 25, -5)` with 15 % black.
    fn shadow_xl(self) -> Self {
        self.shadow(
            Shadow::new()
                .x(0.0)
                .y(20.0)
                .blur(25.0)
                .spread(-5.0)
                .color((0, 0, 0, 0.15)),
        )
    }
    /// Remove every accumulated shadow.
    fn shadow_none(mut self) -> Self {
        self.get_style().shadows.clear();
        self
    }

    // ==========================================================
    // Flex
    // ==========================================================

    /// Stack children vertically.
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let column = Style::default().flex_col().gap_4();
    /// ```
    fn flex_col(self) -> Self {
        self.direction(Direction::Vertical)
    }
    /// Stack children horizontally.
    fn flex_row(self) -> Self {
        self.direction(Direction::Horizontal)
    }

    /// `justify-content: flex-start`.
    fn justify_start(self) -> Self {
        self.main_align(Alignment::Start)
    }
    /// `justify-content: center`.
    fn justify_center(self) -> Self {
        self.main_align(Alignment::Center)
    }
    /// `justify-content: flex-end`.
    fn justify_end(self) -> Self {
        self.main_align(Alignment::End)
    }
    /// `justify-content: space-between`.
    fn justify_between(self) -> Self {
        self.main_align(Alignment::SpaceBetween)
    }
    /// `justify-content: space-around`.
    fn justify_around(self) -> Self {
        self.main_align(Alignment::SpaceAround)
    }
    /// `justify-content: space-evenly`.
    fn justify_evenly(self) -> Self {
        self.main_align(Alignment::SpaceEvenly)
    }

    /// `align-items: flex-start`.
    fn items_start(self) -> Self {
        self.cross_align(Alignment::Start)
    }
    /// `align-items: center`.
    fn items_center(self) -> Self {
        self.cross_align(Alignment::Center)
    }
    /// `align-items: flex-end`.
    fn items_end(self) -> Self {
        self.cross_align(Alignment::End)
    }

    /// Raw pixel gap.
    fn gap(self, size: f32) -> Self {
        self.spacing(size)
    }

    spacing_scale!(
        gap, gap, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    // ==========================================================
    // Sizing
    // ==========================================================

    /// Width in pixels.
    fn w(self, px: f32) -> Self {
        self.width(Size::px(px))
    }
    /// Height in pixels.
    fn h(self, px: f32) -> Self {
        self.height(Size::px(px))
    }
    /// Minimum width in pixels.
    fn min_w(self, px: f32) -> Self {
        self.min_width(Size::px(px))
    }
    /// Minimum height in pixels.
    fn min_h(self, px: f32) -> Self {
        self.min_height(Size::px(px))
    }
    /// Maximum width in pixels.
    fn max_w(self, px: f32) -> Self {
        self.max_width(Size::px(px))
    }
    /// Maximum height in pixels.
    fn max_h(self, px: f32) -> Self {
        self.max_height(Size::px(px))
    }

    /// Width as a percentage of the parent.
    fn w_percent(self, pct: f32) -> Self {
        self.width(Size::percent(pct))
    }
    /// Height as a percentage of the parent.
    fn h_percent(self, pct: f32) -> Self {
        self.height(Size::percent(pct))
    }

    /// Width = 100 % of parent.
    fn w_full(self) -> Self {
        self.width(Size::percent(100.0))
    }
    /// Alias for `w_full`.
    fn w_screen(self) -> Self {
        self.width(Size::percent(100.0))
    }
    /// Width determined by content.
    fn w_auto(self) -> Self {
        self.width(Size::auto())
    }
    /// Height = 100 % of parent.
    fn h_full(self) -> Self {
        self.height(Size::percent(100.0))
    }
    /// Alias for `h_full`.
    fn h_screen(self) -> Self {
        self.height(Size::percent(100.0))
    }
    /// Height determined by content.
    fn h_auto(self) -> Self {
        self.height(Size::auto())
    }

    spacing_scale!(
        w, w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        h, h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        min_w, min_w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        min_h, min_h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        max_w, max_w, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        max_h, max_h, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    // ==========================================================
    // Spacing
    // ==========================================================

    /// Padding on all sides.
    fn p(self, size: f32) -> Self {
        self.padding(Gaps::new_all(size))
    }
    /// Padding on the x-axis.
    fn px(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, size, 0.0))
    }
    /// Padding on the y-axis.
    fn py(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, size))
    }
    /// Padding on the top.
    fn pt(self, size: f32) -> Self {
        self.padding(Gaps::new(size, 0.0, 0.0, 0.0))
    }
    /// Padding on the right.
    fn pr(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, size, 0.0, 0.0))
    }
    /// Padding on the bottom.
    fn pb(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, size, 0.0))
    }
    /// Padding on the left.
    fn pl(self, size: f32) -> Self {
        self.padding(Gaps::new(0.0, 0.0, 0.0, size))
    }

    /// Margin on all sides.
    fn m(self, size: f32) -> Self {
        self.margin(Gaps::new_all(size))
    }
    /// Margin on the x-axis.
    fn mx(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, size, 0.0))
    }
    /// Margin on the y-axis.
    fn my(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, size))
    }
    /// Margin on the top.
    fn mt(self, size: f32) -> Self {
        self.margin(Gaps::new(size, 0.0, 0.0, 0.0))
    }
    /// Margin on the right.
    fn mr(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, size, 0.0, 0.0))
    }
    /// Margin on the bottom.
    fn mb(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, size, 0.0))
    }
    /// Margin on the left.
    fn ml(self, size: f32) -> Self {
        self.margin(Gaps::new(0.0, 0.0, 0.0, size))
    }

    spacing_scale!(
        p, p, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        px, px, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        py, py, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        pt, pt, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        pr, pr, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        pb, pb, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        pl, pl, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    spacing_scale!(
        m, m, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        mx, mx, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        my, my, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        mt, mt, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        mr, mr, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        mb, mb, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );
    spacing_scale!(
        ml, ml, 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 48, 64, 96
    );

    // ==========================================================
    // style-string API
    // ==========================================================

    /// Apply a Tailwind‑style style string.
    ///
    /// Splits `stylees` on whitespace and dispatches each token to the
    /// matching helper. Unknown tokens are silently ignored — matching
    /// Tailwind's behaviour of treating unrecognised stylees as no‑ops.
    ///
    /// # Supported tokens
    ///
    /// | Prefix | Example | Dispatches to |
    /// | --- | --- | --- |
    /// | `bg-…` | `bg-primary`, `bg-red-500`, `bg-white` | background colour |
    /// | `text-…` | `text-xs`, `text-left`, `text-primary`, `text-slate-100` | size, align, colour |
    /// | `font-…` | `font-bold`, `font-mono` | weight, family |
    /// | `border-…` | `border-2`, `border-primary`, `border-red-500` | width, colour |
    /// | `opacity-…` | `opacity-50`, `opacity-85` | opacity |
    /// | `shadow-…` | `shadow-md` | shadow |
    /// | `flex-row`, `flex-col` | — | direction |
    /// | `justify-…` | `justify-between` | main align |
    /// | `items-…` | `items-center` | cross align |
    /// | `gap-…` | `gap-4`, `gap-7.5` | gap |
    /// | `w-…`, `h-…` | `w-4`, `h-full`, `w-50%` | sizing |
    /// | `min-w-…`, `max-h-…`, etc. | `min-w-8`, `max-h-96` | sizing constraints |
    /// | `p-…`, `px-…`, `pt-…`, … | `p-4`, `px-6` | padding |
    /// | `m-…`, `mx-…`, `mt-…`, … | `m-2`, `my-4` | margin |
    /// | — | `italic`, `not-italic`, `underline`, `overline`, `line-through`, `no-underline` | typography |
    ///
    /// Scale values follow the Tailwind spacing scale (`1` = 4 px). Raw pixel
    /// values are accepted for sizing (`w-120.5`) and percentages via `%`
    /// (`w-50%`). Sizing also accepts `full`, `screen`, and `auto`.
    ///
    /// # Examples
    ///
    /// ## A CTA button
    ///
    /// ```no_run
    /// use freyacn::extensions::{Style, StyleExt};
    ///
    /// let s = Style::default()
    ///     .style("bg-primary text-primary-foreground px-4 py-2 text-sm font-semibold shadow-sm");
    /// ```
    ///
    /// ## A destructive action
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default()
    ///     .style("bg-destructive text-destructive-foreground px-3 py-2 border-2 border-red-700");
    /// ```
    ///
    /// ## A flex column
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default()
    ///     .style("flex-col items-center justify-between gap-4 p-6 bg-slate-900 text-slate-100");
    /// ```
    ///
    /// ## Raw values and percentages
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default()
    ///     .style("w-120.5 h-50% p-10 m-4");
    /// ```
    ///
    /// ## Overrides — last write wins
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default()
    ///     .style("bg-slate-800")
    ///     .style("bg-emerald-400");   // the final background is emerald-400
    /// ```
    ///
    /// ## Mixing style strings with typed helpers
    ///
    /// ```no_run
    /// # use freyacn::extensions::{Style, StyleExt};
    /// let s = Style::default()
    ///     .style("bg-slate-800 text-slate-100 p-6 gap-4")
    ///     .corner_radius(12.0)       // typed, no style equivalent yet
    ///     .shadow_lg();
    /// ```
    fn style(mut self, stylees: impl AsRef<str>) -> Self {
        for token in stylees.as_ref().split_whitespace() {
            self = style_token(self, token);
        }
        self
    }
}
