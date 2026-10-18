//! # Unified Style Extension Trait
//!
//! Fixed for **Freya 0.5.0-rc.10**.
//!
//! ## `State::read()` and temporaries
//!
//! In 0.5.0-rc, `State<T>::read()` returns a `ReadableRef<'_, T>` whose
//! lifetime is tied to `&self` (like `RefCell::borrow`). So
//! `use_cn_theme().read()` creates a `State<Theme>` temporary, borrows
//! it, and drops the `State` at the `;` — the guard dangles. Every
//! theme lookup in this file binds the `State` to a local first:
//!
//! ```ignore
//! let state = use_cn_theme();
//! let c = state.read().colors.white;   // Color is Copy → owned
//! ```
//!
//! The same rule applies in `div.rs`, `label.rs`, `grid.rs`, and any
//! other component that calls `use_cn_theme()`.

use crate::theme::use_cn_theme;
use freya::prelude::{
    Alignment, Border, Content, CornerRadius, Direction, Fill, FontSize, FontSlant, FontWeight,
    FontWidth, Gaps, Overflow, Position, Scale, Shadow, Size, TextAlign, TextDecoration,
    TextHeightBehavior, TextShadow, TransformOrigin, VisibleSize,
};
use std::borrow::Cow;

use freya::prelude::Color;
use freya_core::integration::State;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserSelect {
    None,
    Text,
    All,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whitespace {
    Normal,
    Nowrap,
    Pre,
    PreLine,
    PreWrap,
    BreakSpaces,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordBreak {
    Normal,
    Words,
    All,
    Keep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEllipsis {
    Clip,
    Ellipsis,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListStyleType {
    None,
    Disc,
    Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundRepeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundSize {
    Auto,
    Cover,
    Contain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderCollapse {
    Collapse,
    Separate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableLayout {
    Auto,
    Fixed,
}

/// Deferred positioning mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionKind {
    Relative,
    Absolute,
    Global,
}

/// Direction of a scroll container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAxis {
    Vertical,
    Horizontal,
    Both,
}

// ==============================================================
// Animation / transition types
// ==============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationName {
    Spin,
    Ping,
    Pulse,
    Bounce,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationSpec {
    pub name: AnimationName,
    pub duration_ms: f32,
}

impl AnimationSpec {
    fn make(name: AnimationName, duration_ms: f32) -> Self {
        Self { name, duration_ms }
    }
    pub fn spin() -> Self {
        Self::make(AnimationName::Spin, 1000.0)
    }
    pub fn ping() -> Self {
        Self::make(AnimationName::Ping, 1000.0)
    }
    pub fn pulse() -> Self {
        Self::make(AnimationName::Pulse, 2000.0)
    }
    pub fn bounce() -> Self {
        Self::make(AnimationName::Bounce, 1000.0)
    }
    pub fn is_none(&self) -> bool {
        matches!(self.name, AnimationName::None)
    }
}

impl Default for AnimationSpec {
    fn default() -> Self {
        Self::spin()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionSpec {
    pub colors: bool,
    pub opacity: bool,
    pub transform: bool,
    pub duration_ms: f32,
}

impl TransitionSpec {
    pub fn all() -> Self {
        Self {
            colors: true,
            opacity: true,
            transform: true,
            duration_ms: 150.0,
        }
    }
    pub fn colors() -> Self {
        Self {
            colors: true,
            opacity: false,
            transform: false,
            duration_ms: 150.0,
        }
    }
    pub fn opacity() -> Self {
        Self {
            colors: false,
            opacity: true,
            transform: false,
            duration_ms: 150.0,
        }
    }
    pub fn transform() -> Self {
        Self {
            colors: false,
            opacity: false,
            transform: true,
            duration_ms: 150.0,
        }
    }
}

impl Default for TransitionSpec {
    fn default() -> Self {
        Self::colors()
    }
}

// ==============================================================
// Contexts (shared with Div and Group)
// ==============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivPosition {
    pub index: usize,
    pub total: usize,
}

#[derive(Clone, PartialEq)]
pub struct GroupState {
    pub hovered: freya::prelude::State<bool>,
    pub focused: freya::prelude::State<bool>,
}

/// The active `StateFlags` of the nearest ancestor `Div`.
#[derive(Clone)]
pub struct DivActiveState {
    pub flags: State<StateFlags>,
}

impl PartialEq for DivActiveState {
    fn eq(&self, other: &Self) -> bool {
        *self.flags.peek() == *other.flags.peek()
    }
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
                    let state = use_cn_theme();
                    self.$field = Some(state.read().colors.[<$family _ $shade>].into());
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
                    let state = use_cn_theme();
                    self.$field = Some(state.read().$token.into());
                    self
                }
            }
        )*
    };
}

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
// Helper macros — expand every palette family in one shot
// ==============================================================

macro_rules! style_all_palette_families {
    ($prefix:ident, $field:ident) => {
        style_palette_family!($prefix, $field, stone);
        style_palette_family!($prefix, $field, mauve);
        style_palette_family!($prefix, $field, olive);
        style_palette_family!($prefix, $field, mist);
        style_palette_family!($prefix, $field, taupe);
        style_palette_family!($prefix, $field, slate);
        style_palette_family!($prefix, $field, gray);
        style_palette_family!($prefix, $field, zinc);
        style_palette_family!($prefix, $field, neutral);
        style_palette_family!($prefix, $field, red);
        style_palette_family!($prefix, $field, orange);
        style_palette_family!($prefix, $field, amber);
        style_palette_family!($prefix, $field, yellow);
        style_palette_family!($prefix, $field, lime);
        style_palette_family!($prefix, $field, green);
        style_palette_family!($prefix, $field, emerald);
        style_palette_family!($prefix, $field, teal);
        style_palette_family!($prefix, $field, cyan);
        style_palette_family!($prefix, $field, sky);
        style_palette_family!($prefix, $field, blue);
        style_palette_family!($prefix, $field, indigo);
        style_palette_family!($prefix, $field, violet);
        style_palette_family!($prefix, $field, purple);
        style_palette_family!($prefix, $field, fuchsia);
        style_palette_family!($prefix, $field, pink);
        style_palette_family!($prefix, $field, rose);
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

macro_rules! ext_all_palette_families {
    ($prefix:ident) => {
        ext_palette_family!($prefix, stone);
        ext_palette_family!($prefix, mauve);
        ext_palette_family!($prefix, olive);
        ext_palette_family!($prefix, mist);
        ext_palette_family!($prefix, taupe);
        ext_palette_family!($prefix, slate);
        ext_palette_family!($prefix, gray);
        ext_palette_family!($prefix, zinc);
        ext_palette_family!($prefix, neutral);
        ext_palette_family!($prefix, red);
        ext_palette_family!($prefix, orange);
        ext_palette_family!($prefix, amber);
        ext_palette_family!($prefix, yellow);
        ext_palette_family!($prefix, lime);
        ext_palette_family!($prefix, green);
        ext_palette_family!($prefix, emerald);
        ext_palette_family!($prefix, teal);
        ext_palette_family!($prefix, cyan);
        ext_palette_family!($prefix, sky);
        ext_palette_family!($prefix, blue);
        ext_palette_family!($prefix, indigo);
        ext_palette_family!($prefix, violet);
        ext_palette_family!($prefix, purple);
        ext_palette_family!($prefix, fuchsia);
        ext_palette_family!($prefix, pink);
        ext_palette_family!($prefix, rose);
    };
}

/// Interactive, environmental, structural, and inheritance states a
/// style can be conditioned on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct StateFlags(u32);

impl StateFlags {
    pub const NONE: StateFlags = StateFlags(0);

    pub const DARK: StateFlags = StateFlags(1 << 0);
    pub const HOVER: StateFlags = StateFlags(1 << 1);
    pub const FOCUS: StateFlags = StateFlags(1 << 2);
    pub const FOCUS_VISIBLE: StateFlags = StateFlags(1 << 3);
    pub const FOCUS_WITHIN: StateFlags = StateFlags(1 << 4);
    pub const ACTIVE: StateFlags = StateFlags(1 << 5);
    pub const DISABLED: StateFlags = StateFlags(1 << 6);
    pub const VISITED: StateFlags = StateFlags(1 << 7);
    pub const CHECKED: StateFlags = StateFlags(1 << 8);
    pub const TARGET: StateFlags = StateFlags(1 << 9);

    pub const FIRST: StateFlags = StateFlags(1 << 10);
    pub const LAST: StateFlags = StateFlags(1 << 11);
    pub const ODD: StateFlags = StateFlags(1 << 12);
    pub const EVEN: StateFlags = StateFlags(1 << 13);
    pub const ONLY: StateFlags = StateFlags(1 << 14);

    pub const GROUP_HOVER: StateFlags = StateFlags(1 << 15);
    pub const GROUP_FOCUS: StateFlags = StateFlags(1 << 16);
    pub const GROUP_ACTIVE: StateFlags = StateFlags(1 << 17);

    pub const fn or(self, other: StateFlags) -> StateFlags {
        StateFlags(self.0 | other.0)
    }

    pub const fn is_subset_of(self, other: StateFlags) -> bool {
        self.0 & other.0 == self.0
    }

    pub const fn intersects(self, other: StateFlags) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct StateCondition {
    pub positive: StateFlags,
    pub negative: StateFlags,
}

impl StateCondition {
    pub const fn none() -> Self {
        Self {
            positive: StateFlags::NONE,
            negative: StateFlags::NONE,
        }
    }
    pub const fn pos(flag: StateFlags) -> Self {
        Self {
            positive: flag,
            negative: StateFlags::NONE,
        }
    }
    pub const fn neg(flag: StateFlags) -> Self {
        Self {
            positive: StateFlags::NONE,
            negative: flag,
        }
    }
    pub const fn and(self, other: StateCondition) -> Self {
        Self {
            positive: self.positive.or(other.positive),
            negative: self.negative.or(other.negative),
        }
    }
    pub const fn is_empty(&self) -> bool {
        self.positive.is_empty() && self.negative.is_empty()
    }
    pub fn matches(&self, active: StateFlags) -> bool {
        self.positive.is_subset_of(active) && !self.negative.intersects(active)
    }
}

pub type StateStyle = (StateCondition, Box<Style>);

fn state_flag_for(name: &str) -> Option<StateFlags> {
    Some(match name {
        "dark" => StateFlags::DARK,
        "hover" => StateFlags::HOVER,
        "focus" => StateFlags::FOCUS,
        "focus-visible" => StateFlags::FOCUS_VISIBLE,
        "focus-within" => StateFlags::FOCUS_WITHIN,
        "active" => StateFlags::ACTIVE,
        "disabled" => StateFlags::DISABLED,
        "visited" => StateFlags::VISITED,
        "checked" => StateFlags::CHECKED,
        "target" => StateFlags::TARGET,
        "first" => StateFlags::FIRST,
        "last" => StateFlags::LAST,
        "odd" => StateFlags::ODD,
        "even" => StateFlags::EVEN,
        "only" => StateFlags::ONLY,
        "group-hover" => StateFlags::GROUP_HOVER,
        "group-focus" => StateFlags::GROUP_FOCUS,
        "group-active" => StateFlags::GROUP_ACTIVE,
        "has-hover" => StateFlags::GROUP_HOVER,
        "has-focus" => StateFlags::GROUP_FOCUS,
        "has-active" => StateFlags::GROUP_ACTIVE,
        _ => return None,
    })
}

fn split_state_prefix(token: &str) -> (StateCondition, &str) {
    let mut cond = StateCondition::none();
    let mut rest = token;
    while let Some((prefix, tail)) = rest.split_once(':') {
        if let Some(flag) = state_flag_for(prefix) {
            cond = cond.and(StateCondition::pos(flag));
        } else if let Some(neg_flag) = prefix.strip_prefix("not-").and_then(state_flag_for) {
            cond = cond.and(StateCondition::neg(neg_flag));
        } else {
            break;
        }
        rest = tail;
    }
    (cond, rest)
}

// ==============================================================
// Gap merging helper
// ==============================================================

fn merge_gaps(base: Option<Gaps>, over: Option<Gaps>) -> Option<Gaps> {
    match (base, over) {
        (None, x) | (x, None) => x,
        (Some(a), Some(b)) => Some(Gaps::new(
            if b.top() != 0.0 { b.top() } else { a.top() },
            if b.right() != 0.0 {
                b.right()
            } else {
                a.right()
            },
            if b.bottom() != 0.0 {
                b.bottom()
            } else {
                a.bottom()
            },
            if b.left() != 0.0 { b.left() } else { a.left() },
        )),
    }
}

// ==============================================================
// Style: the data container
// ==============================================================

#[derive(Clone, Debug, PartialEq)]
pub struct Style {
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
    pub outline_width: Option<f32>,
    pub outline_color: Option<Color>,
    pub outline_offset: Option<f32>,
    pub divide_x: Option<f32>,
    pub divide_y: Option<f32>,
    pub divide_color: Option<Color>,
    pub border_collapse: Option<BorderCollapse>,

    pub bg_repeat: Option<BackgroundRepeat>,
    pub bg_size: Option<BackgroundSize>,
    pub bg_attachment: Option<BackgroundAttachment>,
    pub bg_clip: Option<BackgroundClip>,

    pub ring_width: Option<f32>,
    pub ring_color: Option<Color>,
    pub ring_offset: Option<f32>,
    pub ring_offset_color: Option<Color>,
    pub inset_ring_width: Option<f32>,
    pub inset_ring_color: Option<Color>,

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
    pub whitespace: Option<Whitespace>,
    pub word_break: Option<WordBreak>,
    pub text_ellipsis: Option<TextEllipsis>,
    pub line_clamp: Option<u32>,
    pub decoration_color: Option<Color>,
    pub decoration_thickness: Option<f32>,
    pub underline_offset: Option<f32>,
    pub list_style_type: Option<ListStyleType>,
    pub list_style_position: Option<ListStylePosition>,
    pub selection_color: Option<Color>,
    pub accent_color: Option<Color>,
    pub caret_color: Option<Color>,
    pub placeholder_color: Option<Color>,

    pub text_shadows: Vec<TextShadow>,
    pub font_width: Option<FontWidth>,
    pub text_height: Option<TextHeightBehavior>,
    pub visible_width: Option<VisibleSize>,
    pub visible_height: Option<VisibleSize>,

    pub rotate: Option<f32>,
    pub scale: Option<Scale>,
    pub transform_origin: Option<TransformOrigin>,
    pub blur: Option<f32>,

    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,

    pub padding: Option<Gaps>,
    pub margin: Option<Gaps>,

    pub width: Option<Size>,
    pub height: Option<Size>,
    pub min_width: Option<Size>,
    pub min_height: Option<Size>,
    pub max_width: Option<Size>,
    pub max_height: Option<Size>,
    pub aspect_ratio: Option<f32>,
    pub table_layout: Option<TableLayout>,
    pub columns: Option<u32>,
    pub container: Option<bool>,

    pub position_kind: Option<PositionKind>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,
    pub order: Option<i32>,

    pub overflow: Option<Overflow>,
    pub overflow_x: Option<Overflow>,
    pub overflow_y: Option<Overflow>,

    pub scroll_axis: Option<ScrollAxis>,
    pub scrollbar: Option<bool>,
    pub grid_cols: Option<u32>,
    pub grid_rows: Option<u32>,
    pub col_span: Option<u32>,
    pub row_span: Option<u32>,

    pub direction: Option<Direction>,
    pub main_align: Option<Alignment>,
    pub cross_align: Option<Alignment>,
    pub align_self: Option<Alignment>,
    pub align_content: Option<Alignment>,
    pub content: Option<Content>,
    pub spacing: Option<f32>,
    pub gap_x: Option<f32>,
    pub gap_y: Option<f32>,
    pub space_x: Option<f32>,
    pub space_y: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_grow: Option<f32>,

    pub opacity: Option<f32>,
    pub visibility: Option<Visibility>,
    pub user_select: Option<UserSelect>,
    pub resize: Option<Resize>,
    pub pointer_events: Option<bool>,
    pub sr_only: Option<bool>,

    pub states: Vec<StateStyle>,

    pub animation: Option<AnimationSpec>,
    pub transition: Option<TransitionSpec>,

    pub object_fit: Option<ObjectFit>,
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
            outline_width: None,
            outline_color: None,
            outline_offset: None,
            divide_x: None,
            divide_y: None,
            divide_color: None,
            border_collapse: None,
            bg_repeat: None,
            bg_size: None,
            bg_attachment: None,
            bg_clip: None,
            ring_width: None,
            ring_color: None,
            ring_offset: None,
            ring_offset_color: None,
            inset_ring_width: None,
            inset_ring_color: None,
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
            whitespace: None,
            word_break: None,
            text_ellipsis: None,
            line_clamp: None,
            decoration_color: None,
            decoration_thickness: None,
            underline_offset: None,
            list_style_type: None,
            list_style_position: None,
            selection_color: None,
            accent_color: None,
            caret_color: None,
            placeholder_color: None,
            text_shadows: Vec::new(),
            font_width: None,
            text_height: None,
            visible_width: None,
            visible_height: None,
            rotate: None,
            scale: None,
            transform_origin: None,
            blur: None,
            offset_x: None,
            offset_y: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            aspect_ratio: None,
            table_layout: None,
            columns: None,
            container: None,
            position_kind: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            z_index: None,
            order: None,
            overflow: None,
            overflow_x: None,
            overflow_y: None,
            scroll_axis: None,
            scrollbar: None,
            grid_cols: None,
            grid_rows: None,
            col_span: None,
            row_span: None,
            direction: None,
            main_align: None,
            cross_align: None,
            align_self: None,
            align_content: None,
            content: None,
            spacing: None,
            gap_x: None,
            gap_y: None,
            space_x: None,
            space_y: None,
            flex_shrink: None,
            flex_grow: None,
            opacity: None,
            visibility: None,
            user_select: None,
            resize: None,
            pointer_events: None,
            sr_only: None,
            states: Vec::new(),
            animation: None,
            transition: None,
            object_fit: None,
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
                self.divide_y,
                self.divide_x,
            ]
            .into_iter()
            .flatten()
            .reduce(f32::max)
        })?;
        let mut b = Border::default().width(width);
        if let Some(colour) = self.border_color.or(self.divide_color) {
            b = b.fill(colour);
        }
        Some(b)
    }

    pub fn to_position(&self) -> Option<Position> {
        let sides = Box::new(PositionSides {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
        });
        match self.position_kind? {
            PositionKind::Relative => Some(Position::Stacked(sides)),
            PositionKind::Absolute => Some(Position::Absolute(sides)),
            PositionKind::Global => Some(Position::Global(sides)),
        }
    }

    pub fn merge(self, other: Style) -> Style {
        let mut shadows = self.shadows;
        shadows.extend(other.shadows);
        let mut font_families = self.font_families;
        font_families.extend(other.font_families);
        let mut text_shadows = self.text_shadows;
        text_shadows.extend(other.text_shadows);

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
            outline_width: other.outline_width.or(self.outline_width),
            outline_color: other.outline_color.or(self.outline_color),
            outline_offset: other.outline_offset.or(self.outline_offset),
            divide_x: other.divide_x.or(self.divide_x),
            divide_y: other.divide_y.or(self.divide_y),
            divide_color: other.divide_color.or(self.divide_color),
            border_collapse: other.border_collapse.or(self.border_collapse),

            bg_repeat: other.bg_repeat.or(self.bg_repeat),
            bg_size: other.bg_size.or(self.bg_size),
            bg_attachment: other.bg_attachment.or(self.bg_attachment),
            bg_clip: other.bg_clip.or(self.bg_clip),

            ring_width: other.ring_width.or(self.ring_width),
            ring_color: other.ring_color.or(self.ring_color),
            ring_offset: other.ring_offset.or(self.ring_offset),
            ring_offset_color: other.ring_offset_color.or(self.ring_offset_color),
            inset_ring_width: other.inset_ring_width.or(self.inset_ring_width),
            inset_ring_color: other.inset_ring_color.or(self.inset_ring_color),

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
            whitespace: other.whitespace.or(self.whitespace),
            word_break: other.word_break.or(self.word_break),
            text_ellipsis: other.text_ellipsis.or(self.text_ellipsis),
            line_clamp: other.line_clamp.or(self.line_clamp),
            decoration_color: other.decoration_color.or(self.decoration_color),
            decoration_thickness: other.decoration_thickness.or(self.decoration_thickness),
            underline_offset: other.underline_offset.or(self.underline_offset),
            list_style_type: other.list_style_type.or(self.list_style_type),
            list_style_position: other.list_style_position.or(self.list_style_position),
            selection_color: other.selection_color.or(self.selection_color),
            accent_color: other.accent_color.or(self.accent_color),
            caret_color: other.caret_color.or(self.caret_color),
            placeholder_color: other.placeholder_color.or(self.placeholder_color),

            text_shadows,
            font_width: other.font_width.or(self.font_width),
            text_height: other.text_height.or(self.text_height),
            visible_width: other.visible_width.or(self.visible_width),
            visible_height: other.visible_height.or(self.visible_height),
            rotate: other.rotate.or(self.rotate),
            scale: other.scale.or(self.scale),
            transform_origin: other.transform_origin.or(self.transform_origin),
            blur: other.blur.or(self.blur),
            offset_x: other.offset_x.or(self.offset_x),
            offset_y: other.offset_y.or(self.offset_y),

            padding: merge_gaps(self.padding, other.padding),
            margin: merge_gaps(self.margin, other.margin),

            width: other.width.or(self.width),
            height: other.height.or(self.height),
            min_width: other.min_width.or(self.min_width),
            min_height: other.min_height.or(self.min_height),
            max_width: other.max_width.or(self.max_width),
            max_height: other.max_height.or(self.max_height),
            aspect_ratio: other.aspect_ratio.or(self.aspect_ratio),
            table_layout: other.table_layout.or(self.table_layout),
            columns: other.columns.or(self.columns),
            container: other.container.or(self.container),

            position_kind: other.position_kind.or(self.position_kind),
            top: other.top.or(self.top),
            right: other.right.or(self.right),
            bottom: other.bottom.or(self.bottom),
            left: other.left.or(self.left),
            z_index: other.z_index.or(self.z_index),
            order: other.order.or(self.order),

            overflow: other.overflow.or(self.overflow),
            overflow_x: other.overflow_x.or(self.overflow_x),
            overflow_y: other.overflow_y.or(self.overflow_y),

            scroll_axis: other.scroll_axis.or(self.scroll_axis),
            scrollbar: other.scrollbar.or(self.scrollbar),
            grid_cols: other.grid_cols.or(self.grid_cols),
            grid_rows: other.grid_rows.or(self.grid_rows),
            col_span: other.col_span.or(self.col_span),
            row_span: other.row_span.or(self.row_span),

            direction: other.direction.or(self.direction),
            main_align: other.main_align.or(self.main_align),
            cross_align: other.cross_align.or(self.cross_align),
            align_self: other.align_self.or(self.align_self),
            align_content: other.align_content.or(self.align_content),
            content: other.content.or(self.content),
            spacing: other.spacing.or(self.spacing),
            gap_x: other.gap_x.or(self.gap_x),
            gap_y: other.gap_y.or(self.gap_y),
            space_x: other.space_x.or(self.space_x),
            space_y: other.space_y.or(self.space_y),
            flex_shrink: other.flex_shrink.or(self.flex_shrink),
            flex_grow: other.flex_grow.or(self.flex_grow),

            opacity: other.opacity.or(self.opacity),
            visibility: other.visibility.or(self.visibility),
            user_select: other.user_select.or(self.user_select),
            resize: other.resize.or(self.resize),
            pointer_events: other.pointer_events.or(self.pointer_events),
            sr_only: other.sr_only.or(self.sr_only),

            states: {
                let mut v = self.states;
                v.extend(other.states);
                v
            },

            animation: other.animation.or(self.animation),
            transition: other.transition.or(self.transition),

            object_fit: other.object_fit.or(self.object_fit),
        }
    }

    pub fn or(self, defaults: Style) -> Style {
        defaults.merge(self)
    }

    pub fn merge_from(&mut self, other: Style) {
        let taken = std::mem::take(self);
        *self = taken.merge(other);
    }

    pub fn or_from(&mut self, defaults: Style) {
        let taken = std::mem::take(self);
        *self = taken.or(defaults);
    }

    pub fn is_empty(&self) -> bool {
        *self == Style::default()
    }

    pub fn with_active_states(mut self, active: StateFlags) -> Style {
        let states = std::mem::take(&mut self.states);
        for (cond, state_style) in states {
            if cond.matches(active) {
                self = self.merge(*state_style);
            }
        }
        self
    }

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
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = Some(z);
        self
    }
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
    pub fn scroll_axis(mut self, axis: ScrollAxis) -> Self {
        self.scroll_axis = Some(axis);
        self
    }
    pub fn overflow_scroll(self) -> Self {
        self.scroll_axis(ScrollAxis::Vertical)
    }
    pub fn overflow_scroll_x(self) -> Self {
        self.scroll_axis(ScrollAxis::Horizontal)
    }
    pub fn overflow_scroll_xy(self) -> Self {
        self.scroll_axis(ScrollAxis::Both)
    }
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.scrollbar = Some(show);
        self
    }
    pub fn grid_cols(mut self, n: u32) -> Self {
        self.grid_cols = Some(n);
        self
    }
    pub fn grid_rows(mut self, n: u32) -> Self {
        self.grid_rows = Some(n);
        self
    }
    pub fn col_span(mut self, n: u32) -> Self {
        self.col_span = Some(n);
        self
    }
    pub fn row_span(mut self, n: u32) -> Self {
        self.row_span = Some(n);
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

    pub fn text_shadow(mut self, shadow: impl Into<TextShadow>) -> Self {
        self.text_shadows.push(shadow.into());
        self
    }
    pub fn font_width(mut self, width: impl Into<FontWidth>) -> Self {
        self.font_width = Some(width.into());
        self
    }
    pub fn text_height(mut self, height: impl Into<TextHeightBehavior>) -> Self {
        self.text_height = Some(height.into());
        self
    }
    pub fn visible_width(mut self, size: impl Into<VisibleSize>) -> Self {
        self.visible_width = Some(size.into());
        self
    }
    pub fn visible_height(mut self, size: impl Into<VisibleSize>) -> Self {
        self.visible_height = Some(size.into());
        self
    }

    pub fn rotate(mut self, degrees: f32) -> Self {
        self.rotate = Some(degrees);
        self
    }
    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn transform_origin(mut self, origin: impl Into<TransformOrigin>) -> Self {
        self.transform_origin = Some(origin.into());
        self
    }
    pub fn blur(mut self, radius: f32) -> Self {
        self.blur = Some(radius);
        self
    }

    pub fn offset_x(mut self, px: f32) -> Self {
        self.offset_x = Some(px);
        self
    }
    pub fn offset_y(mut self, px: f32) -> Self {
        self.offset_y = Some(px);
        self
    }

    // ---- Literal colours (bind state first) ----
    pub fn text_white(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.white;
        self.color(c)
    }
    pub fn text_black(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.black;
        self.color(c)
    }
    pub fn bg_white(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.white;
        self.background(c)
    }
    pub fn bg_black(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.black;
        self.background(c)
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
        let state = use_cn_theme();
        let c = state.read().primary;
        self.border_color(c)
    }
    pub fn border_destructive(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().destructive;
        self.border_color(c)
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

    pub fn ring(self) -> Self {
        self.ring_width(1.0)
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
    pub fn ring_3(self) -> Self {
        self.ring_width(3.0)
    }
    pub fn ring_4(self) -> Self {
        self.ring_width(4.0)
    }
    pub fn ring_6(self) -> Self {
        self.ring_width(6.0)
    }
    pub fn ring_8(self) -> Self {
        self.ring_width(8.0)
    }

    pub fn ring_offset_0(self) -> Self {
        self.ring_offset(0.0)
    }
    pub fn ring_offset_1(self) -> Self {
        self.ring_offset(1.0)
    }
    pub fn ring_offset_2(self) -> Self {
        self.ring_offset(2.0)
    }
    pub fn ring_offset_4(self) -> Self {
        self.ring_offset(4.0)
    }
    pub fn ring_offset_8(self) -> Self {
        self.ring_offset(8.0)
    }

    pub fn ring_offset_color(mut self, c: Color) -> Self {
        self.ring_offset_color = Some(c);
        self
    }

    pub fn inset_ring(self) -> Self {
        self.inset_ring_width(1.0)
    }
    pub fn inset_ring_0(self) -> Self {
        self.inset_ring_width(0.0)
    }
    pub fn inset_ring_1(self) -> Self {
        self.inset_ring_width(1.0)
    }
    pub fn inset_ring_2(self) -> Self {
        self.inset_ring_width(2.0)
    }
    pub fn inset_ring_4(self) -> Self {
        self.inset_ring_width(4.0)
    }
    pub fn inset_ring_8(self) -> Self {
        self.inset_ring_width(8.0)
    }
    pub fn inset_ring_width(mut self, width: f32) -> Self {
        self.inset_ring_width = Some(width);
        self
    }
    pub fn inset_ring_color(mut self, c: Color) -> Self {
        self.inset_ring_color = Some(c);
        self
    }

    pub fn ring_white(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.white;
        self.ring_color(c)
    }
    pub fn ring_black(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.black;
        self.ring_color(c)
    }
    pub fn ring_transparent(self) -> Self {
        self.ring_color(Color::TRANSPARENT)
    }
    pub fn ring_offset_white(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.white;
        self.ring_offset_color(c)
    }
    pub fn ring_offset_black(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.black;
        self.ring_offset_color(c)
    }
    pub fn inset_ring_white(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.white;
        self.inset_ring_color(c)
    }
    pub fn inset_ring_black(self) -> Self {
        let state = use_cn_theme();
        let c = state.read().colors.black;
        self.inset_ring_color(c)
    }

    // ---- Animations ----
    pub fn animate_spin(mut self) -> Self {
        self.animation = Some(AnimationSpec::spin());
        self
    }
    pub fn animate_ping(mut self) -> Self {
        self.animation = Some(AnimationSpec::ping());
        self
    }
    pub fn animate_pulse(mut self) -> Self {
        self.animation = Some(AnimationSpec::pulse());
        self
    }
    pub fn animate_bounce(mut self) -> Self {
        self.animation = Some(AnimationSpec::bounce());
        self
    }
    pub fn animate_none(mut self) -> Self {
        self.animation = None;
        self
    }

    pub fn transition_all(mut self) -> Self {
        self.transition = Some(TransitionSpec::all());
        self
    }
    pub fn transition_colors(mut self) -> Self {
        self.transition = Some(TransitionSpec::colors());
        self
    }
    pub fn transition_opacity(mut self) -> Self {
        self.transition = Some(TransitionSpec::opacity());
        self
    }
    pub fn transition_transform(mut self) -> Self {
        self.transition = Some(TransitionSpec::transform());
        self
    }
    pub fn transition_none(mut self) -> Self {
        self.transition = None;
        self
    }

    style_all_palette_families!(ring, ring_color);
    style_all_palette_families!(ring_offset, ring_offset_color);
    style_all_palette_families!(inset_ring, inset_ring_color);

    style_semantic_color!(
        ring,
        ring_color,
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
        ring_offset,
        ring_offset_color,
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
        inset_ring,
        inset_ring_color,
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

    // ---- Flex ----
    pub fn flex_col(self) -> Self {
        self.direction(Direction::Vertical)
    }
    pub fn flex_row(self) -> Self {
        self.direction(Direction::Horizontal)
    }
    pub fn flex_wrap_content(self) -> Self {
        self.content(Content::Wrap { wrap_spacing: None })
    }
    pub fn flex_wrap_spacing(self, spacing: f32) -> Self {
        self.content(Content::Wrap {
            wrap_spacing: Some(spacing),
        })
    }
    pub fn content_flex(self) -> Self {
        self.content(Content::Flex)
    }
    pub fn content_fit(self) -> Self {
        self.content(Content::Fit)
    }
    pub fn content_normal(self) -> Self {
        self.content(Content::Normal)
    }
    pub fn flex_1(self) -> Self {
        self.width(Size::flex(1.0)).height(Size::flex(1.0))
    }
    pub fn grow(self) -> Self {
        self.width(Size::flex(1.0)).height(Size::flex(1.0))
    }
    pub fn grow_0(self) -> Self {
        self.width(Size::auto()).height(Size::auto())
    }
    pub fn shrink(mut self) -> Self {
        self.flex_shrink = Some(1.0);
        self
    }
    pub fn shrink_0(mut self) -> Self {
        self.flex_shrink = Some(0.0);
        self.width(Size::auto())
    }
    pub fn shrink_1(mut self) -> Self {
        self.flex_shrink = Some(1.0);
        self
    }
    pub fn flex_shrink(mut self, value: f32) -> Self {
        self.flex_shrink = Some(value);
        self
    }

    pub fn grow_factor(mut self, value: f32) -> Self {
        self.flex_grow = Some(value);
        self
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

    pub fn aspect(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }
    pub fn aspect_square(self) -> Self {
        self.aspect(1.0)
    }
    pub fn aspect_video(self) -> Self {
        self.aspect(16.0 / 9.0)
    }
    pub fn aspect_auto(mut self) -> Self {
        self.aspect_ratio = None;
        self
    }

    pub fn hidden(self) -> Self {
        self.visibility(Visibility::Collapse)
    }
    pub fn visible(self) -> Self {
        self.visibility(Visibility::Visible)
    }
    pub fn invisible(self) -> Self {
        self.visibility(Visibility::Hidden)
    }
    pub fn visibility(mut self, v: Visibility) -> Self {
        self.visibility = Some(v);
        self
    }
    pub fn sr_only(mut self) -> Self {
        self.sr_only = Some(true);
        self.w(1.0).h(1.0).overflow_clip()
    }
    pub fn not_sr_only(mut self) -> Self {
        self.sr_only = Some(false);
        self.w_auto().h_auto().overflow_none()
    }

    pub fn order(mut self, n: i32) -> Self {
        self.order = Some(n);
        self
    }
    pub fn order_first(self) -> Self {
        self.order(-9_999)
    }
    pub fn order_last(self) -> Self {
        self.order(9_999)
    }
    pub fn order_none(mut self) -> Self {
        self.order = None;
        self
    }

    pub fn pointer_events_none(mut self) -> Self {
        self.pointer_events = Some(false);
        self
    }
    pub fn pointer_events_auto(mut self) -> Self {
        self.pointer_events = Some(true);
        self
    }
    pub fn select_none(mut self) -> Self {
        self.user_select = Some(UserSelect::None);
        self
    }
    pub fn select_text(mut self) -> Self {
        self.user_select = Some(UserSelect::Text);
        self
    }
    pub fn select_all(mut self) -> Self {
        self.user_select = Some(UserSelect::All);
        self
    }
    pub fn select_auto(mut self) -> Self {
        self.user_select = Some(UserSelect::Auto);
        self
    }
    pub fn resize(mut self) -> Self {
        self.resize = Some(Resize::Both);
        self
    }
    pub fn resize_none(mut self) -> Self {
        self.resize = Some(Resize::None);
        self
    }
    pub fn resize_x(mut self) -> Self {
        self.resize = Some(Resize::Horizontal);
        self
    }
    pub fn resize_y(mut self) -> Self {
        self.resize = Some(Resize::Vertical);
        self
    }

    pub fn whitespace(mut self, w: Whitespace) -> Self {
        self.whitespace = Some(w);
        self
    }
    pub fn whitespace_normal(self) -> Self {
        self.whitespace(Whitespace::Normal)
    }
    pub fn whitespace_nowrap(self) -> Self {
        self.whitespace(Whitespace::Nowrap)
    }
    pub fn whitespace_pre(self) -> Self {
        self.whitespace(Whitespace::Pre)
    }
    pub fn whitespace_pre_line(self) -> Self {
        self.whitespace(Whitespace::PreLine)
    }
    pub fn whitespace_pre_wrap(self) -> Self {
        self.whitespace(Whitespace::PreWrap)
    }
    pub fn whitespace_break_spaces(self) -> Self {
        self.whitespace(Whitespace::BreakSpaces)
    }

    pub fn word_break(mut self, w: WordBreak) -> Self {
        self.word_break = Some(w);
        self
    }
    pub fn break_normal(self) -> Self {
        self.word_break(WordBreak::Normal)
    }
    pub fn break_words(self) -> Self {
        self.word_break(WordBreak::Words)
    }
    pub fn break_all(self) -> Self {
        self.word_break(WordBreak::All)
    }
    pub fn break_keep(self) -> Self {
        self.word_break(WordBreak::Keep)
    }

    pub fn text_ellipsis(mut self, e: TextEllipsis) -> Self {
        self.text_ellipsis = Some(e);
        self
    }
    pub fn text_clip(self) -> Self {
        self.text_ellipsis(TextEllipsis::Clip)
    }
    pub fn text_ellipsis_set(self) -> Self {
        self.text_ellipsis(TextEllipsis::Ellipsis)
    }

    pub fn line_clamp(mut self, n: u32) -> Self {
        self.line_clamp = Some(n);
        self
    }

    pub fn decoration_color(mut self, c: Color) -> Self {
        self.decoration_color = Some(c);
        self
    }
    pub fn decoration_thickness(mut self, px: f32) -> Self {
        self.decoration_thickness = Some(px);
        self
    }
    pub fn decoration_from_font(self) -> Self {
        self
    }
    pub fn underline_offset(mut self, px: f32) -> Self {
        self.underline_offset = Some(px);
        self
    }

    pub fn list_style_type(mut self, t: ListStyleType) -> Self {
        self.list_style_type = Some(t);
        self
    }
    pub fn list_none(self) -> Self {
        self.list_style_type(ListStyleType::None)
    }
    pub fn list_disc(self) -> Self {
        self.list_style_type(ListStyleType::Disc)
    }
    pub fn list_decimal(self) -> Self {
        self.list_style_type(ListStyleType::Decimal)
    }
    pub fn list_inside(mut self) -> Self {
        self.list_style_position = Some(ListStylePosition::Inside);
        self
    }
    pub fn list_outside(mut self) -> Self {
        self.list_style_position = Some(ListStylePosition::Outside);
        self
    }

    pub fn accent_color(mut self, c: Color) -> Self {
        self.accent_color = Some(c);
        self
    }
    pub fn caret_color(mut self, c: Color) -> Self {
        self.caret_color = Some(c);
        self
    }
    pub fn selection_color(mut self, c: Color) -> Self {
        self.selection_color = Some(c);
        self
    }
    pub fn placeholder_color(mut self, c: Color) -> Self {
        self.placeholder_color = Some(c);
        self
    }

    pub fn bg_repeat(mut self, r: BackgroundRepeat) -> Self {
        self.bg_repeat = Some(r);
        self
    }
    pub fn bg_repeat_1(self) -> Self {
        self.bg_repeat(BackgroundRepeat::Repeat)
    }
    pub fn bg_no_repeat(self) -> Self {
        self.bg_repeat(BackgroundRepeat::NoRepeat)
    }
    pub fn bg_repeat_x(self) -> Self {
        self.bg_repeat(BackgroundRepeat::RepeatX)
    }
    pub fn bg_repeat_y(self) -> Self {
        self.bg_repeat(BackgroundRepeat::RepeatY)
    }
    pub fn bg_size(mut self, s: BackgroundSize) -> Self {
        self.bg_size = Some(s);
        self
    }
    pub fn bg_auto(self) -> Self {
        self.bg_size(BackgroundSize::Auto)
    }
    pub fn bg_cover(self) -> Self {
        self.bg_size(BackgroundSize::Cover)
    }
    pub fn bg_contain(self) -> Self {
        self.bg_size(BackgroundSize::Contain)
    }
    pub fn bg_fixed(mut self) -> Self {
        self.bg_attachment = Some(BackgroundAttachment::Fixed);
        self
    }
    pub fn bg_local(mut self) -> Self {
        self.bg_attachment = Some(BackgroundAttachment::Local);
        self
    }
    pub fn bg_scroll(mut self) -> Self {
        self.bg_attachment = Some(BackgroundAttachment::Scroll);
        self
    }
    pub fn bg_clip(mut self, c: BackgroundClip) -> Self {
        self.bg_clip = Some(c);
        self
    }
    pub fn bg_clip_border(self) -> Self {
        self.bg_clip(BackgroundClip::Border)
    }
    pub fn bg_clip_padding(self) -> Self {
        self.bg_clip(BackgroundClip::Padding)
    }
    pub fn bg_clip_content(self) -> Self {
        self.bg_clip(BackgroundClip::Content)
    }
    pub fn bg_clip_text(self) -> Self {
        self.bg_clip(BackgroundClip::Text)
    }

    pub fn divide_x(mut self, px: f32) -> Self {
        self.divide_x = Some(px);
        self
    }
    pub fn divide_y(mut self, px: f32) -> Self {
        self.divide_y = Some(px);
        self
    }
    pub fn divide_color(mut self, c: Color) -> Self {
        self.divide_color = Some(c);
        self
    }

    pub fn outline_width(mut self, px: f32) -> Self {
        self.outline_width = Some(px);
        self
    }
    pub fn outline_color(mut self, c: Color) -> Self {
        self.outline_color = Some(c);
        self
    }
    pub fn outline_offset(mut self, px: f32) -> Self {
        self.outline_offset = Some(px);
        self
    }
    pub fn outline_none(mut self) -> Self {
        self.outline_width = Some(0.0);
        self
    }

    pub fn space_x(mut self, px: f32) -> Self {
        self.space_x = Some(px);
        self
    }
    pub fn space_y(mut self, px: f32) -> Self {
        self.space_y = Some(px);
        self
    }

    pub fn overflow_x(mut self, o: Overflow) -> Self {
        self.overflow_x = Some(o);
        self
    }
    pub fn overflow_y(mut self, o: Overflow) -> Self {
        self.overflow_y = Some(o);
        self
    }
    pub fn overflow_x_clip(self) -> Self {
        self.overflow_x(Overflow::Clip)
    }
    pub fn overflow_y_clip(self) -> Self {
        self.overflow_y(Overflow::Clip)
    }
    pub fn overflow_x_none(self) -> Self {
        self.overflow_x(Overflow::None)
    }
    pub fn overflow_y_none(self) -> Self {
        self.overflow_y(Overflow::None)
    }

    pub fn table_layout(mut self, t: TableLayout) -> Self {
        self.table_layout = Some(t);
        self
    }
    pub fn table_auto(self) -> Self {
        self.table_layout(TableLayout::Auto)
    }
    pub fn table_fixed(self) -> Self {
        self.table_layout(TableLayout::Fixed)
    }
    pub fn border_collapse(mut self) -> Self {
        self.border_collapse = Some(BorderCollapse::Collapse);
        self
    }
    pub fn border_separate(mut self) -> Self {
        self.border_collapse = Some(BorderCollapse::Separate);
        self
    }

    pub fn columns(mut self, n: u32) -> Self {
        self.columns = Some(n);
        self
    }
    pub fn container(mut self) -> Self {
        self.container = Some(true);
        self
    }

    pub fn relative(mut self) -> Self {
        self.position_kind = Some(PositionKind::Relative);
        self
    }
    pub fn absolute(mut self) -> Self {
        self.position_kind = Some(PositionKind::Absolute);
        self
    }
    pub fn global(mut self) -> Self {
        self.position_kind = Some(PositionKind::Global);
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

    pub fn overflow_none(self) -> Self {
        self.overflow(Overflow::None)
    }
    pub fn overflow_clip(self) -> Self {
        self.overflow(Overflow::Clip)
    }

    pub fn p(mut self, size: f32) -> Self {
        self.padding = Some(Gaps::new_all(size));
        self
    }
    pub fn px(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(g.top(), size, g.bottom(), size));
        self
    }
    pub fn py(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(size, g.right(), size, g.left()));
        self
    }
    pub fn pt(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(size, g.right(), g.bottom(), g.left()));
        self
    }
    pub fn pr(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(g.top(), size, g.bottom(), g.left()));
        self
    }
    pub fn pb(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(g.top(), g.right(), size, g.left()));
        self
    }
    pub fn pl(mut self, size: f32) -> Self {
        let g = self.padding.unwrap_or_default();
        self.padding = Some(Gaps::new(g.top(), g.right(), g.bottom(), size));
        self
    }
    pub fn m(mut self, size: f32) -> Self {
        self.margin = Some(Gaps::new_all(size));
        self
    }
    pub fn mx(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(g.top(), size, g.bottom(), size));
        self
    }
    pub fn my(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(size, g.right(), size, g.left()));
        self
    }
    pub fn mt(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(size, g.right(), g.bottom(), g.left()));
        self
    }
    pub fn mr(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(g.top(), size, g.bottom(), g.left()));
        self
    }
    pub fn mb(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(g.top(), g.right(), size, g.left()));
        self
    }
    pub fn ml(mut self, size: f32) -> Self {
        let g = self.margin.unwrap_or_default();
        self.margin = Some(Gaps::new(g.top(), g.right(), g.bottom(), size));
        self
    }

    pub fn object_fit(mut self, fit: ObjectFit) -> Self {
        self.object_fit = Some(fit);
        self
    }
    pub fn object_contain(self) -> Self {
        self.object_fit(ObjectFit::Contain)
    }
    pub fn object_cover(self) -> Self {
        self.object_fit(ObjectFit::Cover)
    }
    pub fn object_fill(self) -> Self {
        self.object_fit(ObjectFit::Fill)
    }
    pub fn object_none(self) -> Self {
        self.object_fit(ObjectFit::None)
    }
    pub fn object_scale_down(self) -> Self {
        self.object_fit(ObjectFit::ScaleDown)
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
    style_spacing_scale!(
        size,
        size,
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

    fn text_shadow(self, shadow: impl Into<TextShadow>) -> Self {
        self.map_style(|s| s.text_shadow(shadow))
    }
    fn font_width(self, width: impl Into<FontWidth>) -> Self {
        self.map_style(|s| s.font_width(width))
    }
    fn text_height(self, height: impl Into<TextHeightBehavior>) -> Self {
        self.map_style(|s| s.text_height(height))
    }
    fn visible_width(self, size: impl Into<VisibleSize>) -> Self {
        self.map_style(|s| s.visible_width(size))
    }
    fn visible_height(self, size: impl Into<VisibleSize>) -> Self {
        self.map_style(|s| s.visible_height(size))
    }

    fn rotate(self, degrees: f32) -> Self {
        self.map_style(|s| s.rotate(degrees))
    }
    fn scale(self, scale: impl Into<Scale>) -> Self {
        self.map_style(|s| s.scale(scale))
    }
    fn transform_origin(self, origin: impl Into<TransformOrigin>) -> Self {
        self.map_style(|s| s.transform_origin(origin))
    }
    fn blur(self, radius: f32) -> Self {
        self.map_style(|s| s.blur(radius))
    }

    fn offset_x(self, px: f32) -> Self {
        self.map_style(|s| s.offset_x(px))
    }
    fn offset_y(self, px: f32) -> Self {
        self.map_style(|s| s.offset_y(px))
    }

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
    fn ring_3(self) -> Self {
        self.map_style(|s| s.ring_3())
    }
    fn ring_4(self) -> Self {
        self.map_style(|s| s.ring_4())
    }
    fn ring_6(self) -> Self {
        self.map_style(|s| s.ring_6())
    }
    fn ring_8(self) -> Self {
        self.map_style(|s| s.ring_8())
    }

    fn ring_offset_0(self) -> Self {
        self.map_style(|s| s.ring_offset_0())
    }
    fn ring_offset_1(self) -> Self {
        self.map_style(|s| s.ring_offset_1())
    }
    fn ring_offset_2(self) -> Self {
        self.map_style(|s| s.ring_offset_2())
    }
    fn ring_offset_4(self) -> Self {
        self.map_style(|s| s.ring_offset_4())
    }
    fn ring_offset_8(self) -> Self {
        self.map_style(|s| s.ring_offset_8())
    }
    fn ring_offset_color(self, c: Color) -> Self {
        self.map_style(|s| s.ring_offset_color(c))
    }

    fn inset_ring(self) -> Self {
        self.map_style(|s| s.inset_ring())
    }
    fn inset_ring_0(self) -> Self {
        self.map_style(|s| s.inset_ring_0())
    }
    fn inset_ring_1(self) -> Self {
        self.map_style(|s| s.inset_ring_1())
    }
    fn inset_ring_2(self) -> Self {
        self.map_style(|s| s.inset_ring_2())
    }
    fn inset_ring_4(self) -> Self {
        self.map_style(|s| s.inset_ring_4())
    }
    fn inset_ring_8(self) -> Self {
        self.map_style(|s| s.inset_ring_8())
    }
    fn inset_ring_width(self, w: f32) -> Self {
        self.map_style(|s| s.inset_ring_width(w))
    }
    fn inset_ring_color(self, c: Color) -> Self {
        self.map_style(|s| s.inset_ring_color(c))
    }

    fn ring_white(self) -> Self {
        self.map_style(|s| s.ring_white())
    }
    fn ring_black(self) -> Self {
        self.map_style(|s| s.ring_black())
    }
    fn ring_transparent(self) -> Self {
        self.map_style(|s| s.ring_transparent())
    }
    fn ring_offset_white(self) -> Self {
        self.map_style(|s| s.ring_offset_white())
    }
    fn ring_offset_black(self) -> Self {
        self.map_style(|s| s.ring_offset_black())
    }
    fn inset_ring_white(self) -> Self {
        self.map_style(|s| s.inset_ring_white())
    }
    fn inset_ring_black(self) -> Self {
        self.map_style(|s| s.inset_ring_black())
    }

    ext_all_palette_families!(ring);
    ext_all_palette_families!(ring_offset);
    ext_all_palette_families!(inset_ring);

    ext_semantic_color!(
        ring,
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
        ring_offset,
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
        inset_ring,
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
    fn shrink(self) -> Self {
        self.map_style(|s| s.shrink())
    }
    fn shrink_0(self) -> Self {
        self.map_style(|s| s.shrink_0())
    }
    fn shrink_1(self) -> Self {
        self.map_style(|s| s.shrink_1())
    }
    fn flex_shrink(self, value: f32) -> Self {
        self.map_style(|s| s.flex_shrink(value))
    }
    fn grow_factor(self, value: f32) -> Self {
        self.map_style(|s| s.grow_factor(value))
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

    fn aspect(self, ratio: f32) -> Self {
        self.map_style(|s| s.aspect(ratio))
    }
    fn aspect_square(self) -> Self {
        self.map_style(|s| s.aspect_square())
    }
    fn aspect_video(self) -> Self {
        self.map_style(|s| s.aspect_video())
    }
    fn aspect_auto(self) -> Self {
        self.map_style(|s| s.aspect_auto())
    }

    fn hidden(self) -> Self {
        self.map_style(|s| s.hidden())
    }
    fn visible(self) -> Self {
        self.map_style(|s| s.visible())
    }
    fn invisible(self) -> Self {
        self.map_style(|s| s.invisible())
    }
    fn visibility(self, v: Visibility) -> Self {
        self.map_style(|s| s.visibility(v))
    }
    fn sr_only(self) -> Self {
        self.map_style(|s| s.sr_only())
    }
    fn not_sr_only(self) -> Self {
        self.map_style(|s| s.not_sr_only())
    }

    fn order(self, n: i32) -> Self {
        self.map_style(|s| s.order(n))
    }
    fn order_first(self) -> Self {
        self.map_style(|s| s.order_first())
    }
    fn order_last(self) -> Self {
        self.map_style(|s| s.order_last())
    }
    fn order_none(self) -> Self {
        self.map_style(|s| s.order_none())
    }

    fn pointer_events_none(self) -> Self {
        self.map_style(|s| s.pointer_events_none())
    }
    fn pointer_events_auto(self) -> Self {
        self.map_style(|s| s.pointer_events_auto())
    }
    fn select_none(self) -> Self {
        self.map_style(|s| s.select_none())
    }
    fn select_text(self) -> Self {
        self.map_style(|s| s.select_text())
    }
    fn select_all(self) -> Self {
        self.map_style(|s| s.select_all())
    }
    fn select_auto(self) -> Self {
        self.map_style(|s| s.select_auto())
    }
    fn resize(self) -> Self {
        self.map_style(|s| s.resize())
    }
    fn resize_none(self) -> Self {
        self.map_style(|s| s.resize_none())
    }
    fn resize_x(self) -> Self {
        self.map_style(|s| s.resize_x())
    }
    fn resize_y(self) -> Self {
        self.map_style(|s| s.resize_y())
    }

    fn whitespace(self, w: Whitespace) -> Self {
        self.map_style(|s| s.whitespace(w))
    }
    fn whitespace_normal(self) -> Self {
        self.map_style(|s| s.whitespace_normal())
    }
    fn whitespace_nowrap(self) -> Self {
        self.map_style(|s| s.whitespace_nowrap())
    }
    fn whitespace_pre(self) -> Self {
        self.map_style(|s| s.whitespace_pre())
    }
    fn whitespace_pre_line(self) -> Self {
        self.map_style(|s| s.whitespace_pre_line())
    }
    fn whitespace_pre_wrap(self) -> Self {
        self.map_style(|s| s.whitespace_pre_wrap())
    }
    fn whitespace_break_spaces(self) -> Self {
        self.map_style(|s| s.whitespace_break_spaces())
    }
    fn word_break(self, w: WordBreak) -> Self {
        self.map_style(|s| s.word_break(w))
    }
    fn break_normal(self) -> Self {
        self.map_style(|s| s.break_normal())
    }
    fn break_words(self) -> Self {
        self.map_style(|s| s.break_words())
    }
    fn break_all(self) -> Self {
        self.map_style(|s| s.break_all())
    }
    fn break_keep(self) -> Self {
        self.map_style(|s| s.break_keep())
    }
    fn text_ellipsis(self, e: TextEllipsis) -> Self {
        self.map_style(|s| s.text_ellipsis(e))
    }
    fn text_clip(self) -> Self {
        self.map_style(|s| s.text_clip())
    }
    fn text_ellipsis_set(self) -> Self {
        self.map_style(|s| s.text_ellipsis_set())
    }
    fn line_clamp(self, n: u32) -> Self {
        self.map_style(|s| s.line_clamp(n))
    }

    fn decoration_color(self, c: Color) -> Self {
        self.map_style(|s| s.decoration_color(c))
    }
    fn decoration_thickness(self, px: f32) -> Self {
        self.map_style(|s| s.decoration_thickness(px))
    }
    fn decoration_from_font(self) -> Self {
        self.map_style(|s| s.decoration_from_font())
    }
    fn underline_offset(self, px: f32) -> Self {
        self.map_style(|s| s.underline_offset(px))
    }

    fn list_style_type(self, t: ListStyleType) -> Self {
        self.map_style(|s| s.list_style_type(t))
    }
    fn list_none(self) -> Self {
        self.map_style(|s| s.list_none())
    }
    fn list_disc(self) -> Self {
        self.map_style(|s| s.list_disc())
    }
    fn list_decimal(self) -> Self {
        self.map_style(|s| s.list_decimal())
    }
    fn list_inside(self) -> Self {
        self.map_style(|s| s.list_inside())
    }
    fn list_outside(self) -> Self {
        self.map_style(|s| s.list_outside())
    }

    fn accent_color(self, c: Color) -> Self {
        self.map_style(|s| s.accent_color(c))
    }
    fn caret_color(self, c: Color) -> Self {
        self.map_style(|s| s.caret_color(c))
    }
    fn selection_color(self, c: Color) -> Self {
        self.map_style(|s| s.selection_color(c))
    }
    fn placeholder_color(self, c: Color) -> Self {
        self.map_style(|s| s.placeholder_color(c))
    }

    fn bg_repeat(self, r: BackgroundRepeat) -> Self {
        self.map_style(|s| s.bg_repeat(r))
    }
    fn bg_repeat_1(self) -> Self {
        self.map_style(|s| s.bg_repeat_1())
    }
    fn bg_no_repeat(self) -> Self {
        self.map_style(|s| s.bg_no_repeat())
    }
    fn bg_repeat_x(self) -> Self {
        self.map_style(|s| s.bg_repeat_x())
    }
    fn bg_repeat_y(self) -> Self {
        self.map_style(|s| s.bg_repeat_y())
    }
    fn bg_size(self, s: BackgroundSize) -> Self {
        self.map_style(|st| st.bg_size(s))
    }
    fn bg_auto(self) -> Self {
        self.map_style(|s| s.bg_auto())
    }
    fn bg_cover(self) -> Self {
        self.map_style(|s| s.bg_cover())
    }
    fn bg_contain(self) -> Self {
        self.map_style(|s| s.bg_contain())
    }
    fn bg_fixed(self) -> Self {
        self.map_style(|s| s.bg_fixed())
    }
    fn bg_local(self) -> Self {
        self.map_style(|s| s.bg_local())
    }
    fn bg_scroll(self) -> Self {
        self.map_style(|s| s.bg_scroll())
    }
    fn bg_clip(self, c: BackgroundClip) -> Self {
        self.map_style(|s| s.bg_clip(c))
    }
    fn bg_clip_border(self) -> Self {
        self.map_style(|s| s.bg_clip_border())
    }
    fn bg_clip_padding(self) -> Self {
        self.map_style(|s| s.bg_clip_padding())
    }
    fn bg_clip_content(self) -> Self {
        self.map_style(|s| s.bg_clip_content())
    }
    fn bg_clip_text(self) -> Self {
        self.map_style(|s| s.bg_clip_text())
    }

    fn divide_x(self, px: f32) -> Self {
        self.map_style(|s| s.divide_x(px))
    }
    fn divide_y(self, px: f32) -> Self {
        self.map_style(|s| s.divide_y(px))
    }
    fn divide_color(self, c: Color) -> Self {
        self.map_style(|s| s.divide_color(c))
    }

    fn outline_width(self, px: f32) -> Self {
        self.map_style(|s| s.outline_width(px))
    }
    fn outline_color(self, c: Color) -> Self {
        self.map_style(|s| s.outline_color(c))
    }
    fn outline_offset(self, px: f32) -> Self {
        self.map_style(|s| s.outline_offset(px))
    }
    fn outline_none(self) -> Self {
        self.map_style(|s| s.outline_none())
    }

    fn space_x(self, px: f32) -> Self {
        self.map_style(|s| s.space_x(px))
    }
    fn space_y(self, px: f32) -> Self {
        self.map_style(|s| s.space_y(px))
    }

    fn overflow_x(self, o: Overflow) -> Self {
        self.map_style(|s| s.overflow_x(o))
    }
    fn overflow_y(self, o: Overflow) -> Self {
        self.map_style(|s| s.overflow_y(o))
    }
    fn overflow_x_clip(self) -> Self {
        self.map_style(|s| s.overflow_x_clip())
    }
    fn overflow_y_clip(self) -> Self {
        self.map_style(|s| s.overflow_y_clip())
    }
    fn overflow_x_none(self) -> Self {
        self.map_style(|s| s.overflow_x_none())
    }
    fn overflow_y_none(self) -> Self {
        self.map_style(|s| s.overflow_y_none())
    }

    fn scroll_axis(self, axis: ScrollAxis) -> Self {
        self.map_style(|s| s.scroll_axis(axis))
    }
    fn overflow_scroll(self) -> Self {
        self.map_style(|s| s.overflow_scroll())
    }
    fn overflow_scroll_x(self) -> Self {
        self.map_style(|s| s.overflow_scroll_x())
    }
    fn overflow_scroll_xy(self) -> Self {
        self.map_style(|s| s.overflow_scroll_xy())
    }
    fn scrollbar(self, show: bool) -> Self {
        self.map_style(|s| s.scrollbar(show))
    }
    fn grid_cols(self, n: u32) -> Self {
        self.map_style(|s| s.grid_cols(n))
    }
    fn grid_rows(self, n: u32) -> Self {
        self.map_style(|s| s.grid_rows(n))
    }
    fn col_span(self, n: u32) -> Self {
        self.map_style(|s| s.col_span(n))
    }
    fn row_span(self, n: u32) -> Self {
        self.map_style(|s| s.row_span(n))
    }

    fn table_layout(self, t: TableLayout) -> Self {
        self.map_style(|s| s.table_layout(t))
    }
    fn table_auto(self) -> Self {
        self.map_style(|s| s.table_auto())
    }
    fn table_fixed(self) -> Self {
        self.map_style(|s| s.table_fixed())
    }
    fn border_collapse(self) -> Self {
        self.map_style(|s| s.border_collapse())
    }
    fn border_separate(self) -> Self {
        self.map_style(|s| s.border_separate())
    }

    fn columns(self, n: u32) -> Self {
        self.map_style(|s| s.columns(n))
    }
    fn container(self) -> Self {
        self.map_style(|s| s.container())
    }

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

    fn overflow_none(self) -> Self {
        self.map_style(|s| s.overflow_none())
    }
    fn overflow_clip(self) -> Self {
        self.map_style(|s| s.overflow_clip())
    }

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

    fn merge_style(self, other: Style) -> Self {
        self.map_style(|s| s.merge(other))
    }
    fn or_style(self, defaults: Style) -> Self {
        self.map_style(|s| s.or(defaults))
    }

    fn state_style(self, cond: StateCondition, style: Style) -> Self {
        self.map_style(|mut s| {
            s.states.push((cond, Box::new(style)));
            s
        })
    }

    fn with_active_states(self, active: StateFlags) -> Self {
        self.map_style(|s| s.with_active_states(active))
    }

    fn animate_spin(self) -> Self {
        self.map_style(|s| s.animate_spin())
    }
    fn animate_ping(self) -> Self {
        self.map_style(|s| s.animate_ping())
    }
    fn animate_pulse(self) -> Self {
        self.map_style(|s| s.animate_pulse())
    }
    fn animate_bounce(self) -> Self {
        self.map_style(|s| s.animate_bounce())
    }
    fn animate_none(self) -> Self {
        self.map_style(|s| s.animate_none())
    }
    fn transition_all(self) -> Self {
        self.map_style(|s| s.transition_all())
    }
    fn transition_colors(self) -> Self {
        self.map_style(|s| s.transition_colors())
    }
    fn transition_opacity(self) -> Self {
        self.map_style(|s| s.transition_opacity())
    }
    fn transition_transform(self) -> Self {
        self.map_style(|s| s.transition_transform())
    }
    fn transition_none(self) -> Self {
        self.map_style(|s| s.transition_none())
    }

    fn object_fit(self, fit: ObjectFit) -> Self {
        self.map_style(|s| s.object_fit(fit))
    }
    fn object_contain(self) -> Self {
        self.map_style(|s| s.object_contain())
    }
    fn object_cover(self) -> Self {
        self.map_style(|s| s.object_cover())
    }
    fn object_fill(self) -> Self {
        self.map_style(|s| s.object_fill())
    }
    fn object_none(self) -> Self {
        self.map_style(|s| s.object_none())
    }
    fn object_scale_down(self) -> Self {
        self.map_style(|s| s.object_scale_down())
    }

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
    ext_spacing_scale!(
        size,
        size,
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
    let state = use_cn_theme();
    let c = state.read().colors;
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
// Opacity + colour helpers
// ==============================================================

fn split_opacity(rest: &str) -> (&str, Option<f32>) {
    if let Some((base, mod_str)) = rest.rsplit_once('/') {
        if let Some(inner) = mod_str.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
            if let Ok(v) = inner.parse::<f32>() {
                return (base, Some(v.clamp(0.0, 1.0)));
            }
        }
        if let Ok(pct) = mod_str.parse::<f32>() {
            return (base, Some((pct / 100.0).clamp(0.0, 1.0)));
        }
    }
    (rest, None)
}

fn with_alpha(c: Color, alpha: Option<f32>) -> Color {
    match alpha {
        Some(a) => {
            let new_a = (c.a() as f32 * a) as u8;
            Color::from_argb(new_a, c.r(), c.g(), c.b())
        }
        None => c,
    }
}

fn semantic_colour(token: &str) -> Option<Color> {
    let state = use_cn_theme();
    let t = *state.read();
    match token {
        "background" => Some(t.background),
        "foreground" => Some(t.foreground),
        "primary" => Some(t.primary),
        "primary-foreground" => Some(t.primary_foreground),
        "secondary" => Some(t.secondary),
        "secondary-foreground" => Some(t.secondary_foreground),
        "muted" => Some(t.muted),
        "muted-foreground" => Some(t.muted_foreground),
        "accent" => Some(t.accent),
        "accent-foreground" => Some(t.accent_foreground),
        "destructive" => Some(t.destructive),
        "destructive-foreground" => Some(t.destructive_foreground),
        "card" => Some(t.card),
        "card-foreground" => Some(t.card_foreground),
        "popover" => Some(t.popover),
        "popover-foreground" => Some(t.popover_foreground),
        "border" => Some(t.border),
        "input" => Some(t.input),
        "ring" => Some(t.ring),
        "sidebar" => Some(t.sidebar),
        "sidebar-foreground" => Some(t.sidebar_foreground),
        "sidebar-primary" => Some(t.sidebar_primary),
        "sidebar-primary-foreground" => Some(t.sidebar_primary_foreground),
        "sidebar-accent" => Some(t.sidebar_accent),
        "sidebar-accent-foreground" => Some(t.sidebar_accent_foreground),
        "sidebar-border" => Some(t.sidebar_border),
        "sidebar-ring" => Some(t.sidebar_ring),
        "disabled" => Some(t.disabled),
        "disabled-foreground" => Some(t.disabled_foreground),
        "chart-1" => Some(t.chart_1),
        "chart-2" => Some(t.chart_2),
        "chart-3" => Some(t.chart_3),
        "chart-4" => Some(t.chart_4),
        "chart-5" => Some(t.chart_5),
        "white" => Some(t.colors.white),
        "black" => Some(t.colors.black),
        _ => None,
    }
}

fn parse_hex_color(s: &str) -> Option<Color> {
    let s = s
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(s);
    let s = s.strip_prefix('#')?;
    let (r, g, b, a) = match s.len() {
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
            (r, g, b, 255)
        }
        6 => (
            u8::from_str_radix(&s[0..2], 16).ok()?,
            u8::from_str_radix(&s[2..4], 16).ok()?,
            u8::from_str_radix(&s[4..6], 16).ok()?,
            255,
        ),
        8 => (
            u8::from_str_radix(&s[0..2], 16).ok()?,
            u8::from_str_radix(&s[2..4], 16).ok()?,
            u8::from_str_radix(&s[4..6], 16).ok()?,
            u8::from_str_radix(&s[6..8], 16).ok()?,
        ),
        _ => return None,
    };
    Some(Color::from_argb(a, r, g, b))
}

// ==============================================================
// color-mix() support
// ==============================================================

#[derive(Clone, Copy, Debug)]
struct Oklch {
    l: f32,
    c: f32,
    h: f32,
}

impl Oklch {
    fn from_srgb(r: u8, g: u8, b: u8) -> Self {
        let to_linear = |c: f32| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        let r = to_linear(r as f32 / 255.0);
        let g = to_linear(g as f32 / 255.0);
        let b = to_linear(b as f32 / 255.0);

        let x = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
        let y = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
        let z = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

        let l_ = (0.8189330101 * x + 0.3618667424 * y - 0.1288597137 * z).cbrt();
        let m_ = (0.0329845436 * x + 0.9293118715 * y + 0.0361456387 * z).cbrt();
        let s_ = (0.0482003018 * x + 0.2643662691 * y + 0.6338517070 * z).cbrt();

        let ok_l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let ok_a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let ok_b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

        let c = (ok_a * ok_a + ok_b * ok_b).sqrt();
        let h = ok_b.atan2(ok_a).to_degrees().rem_euclid(360.0);

        Self { l: ok_l, c, h }
    }

    fn to_srgb(self) -> (u8, u8, u8) {
        let h_rad = self.h.to_radians();
        let ok_a = self.c * h_rad.cos();
        let ok_b = self.c * h_rad.sin();
        let ok_l = self.l;

        let l_ = ok_l + 0.3963377774 * ok_a + 0.2158037573 * ok_b;
        let m_ = ok_l - 0.1055613458 * ok_a - 0.0638541728 * ok_b;
        let s_ = ok_l - 0.0894841775 * ok_a - 1.2914855480 * ok_b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let x = 1.2270138511 * l - 0.5577999807 * m - 0.2812561490 * s;
        let y = -0.0405801784 * l + 1.1122568696 * m - 0.0716766787 * s;
        let z = -0.0763812845 * l - 0.4214819784 * m + 1.5861632204 * s;

        let r_lin = 4.0767416621 * x - 3.3077115913 * y + 0.2309699292 * z;
        let g_lin = -1.2684380046 * x + 2.6097574011 * y - 0.3413193965 * z;
        let b_lin = -0.0041960863 * x - 0.7034186147 * y + 1.7076147010 * z;

        let to_srgb = |c: f32| {
            let c = c.clamp(0.0, 1.0);
            if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        };

        (
            (to_srgb(r_lin) * 255.0).round() as u8,
            (to_srgb(g_lin) * 255.0).round() as u8,
            (to_srgb(b_lin) * 255.0).round() as u8,
        )
    }
}

fn mix_oklch(a: Oklch, b: Oklch, weight_b: f32) -> Oklch {
    let weight_a = 1.0 - weight_b;

    let mut h_diff = b.h - a.h;
    if h_diff > 180.0 {
        h_diff -= 360.0;
    } else if h_diff < -180.0 {
        h_diff += 360.0;
    }
    let h = (a.h + h_diff * weight_b).rem_euclid(360.0);

    Oklch {
        l: a.l * weight_a + b.l * weight_b,
        c: a.c * weight_a + b.c * weight_b,
        h,
    }
}

fn decode_bracket_value(s: &str) -> String {
    s.replace('_', " ")
}

fn parse_color_mix(expr: &str) -> Option<Color> {
    let inner = expr
        .strip_prefix("color-mix(")
        .and_then(|s| s.strip_suffix(')'))?;

    let mut parts = inner.splitn(3, ',');
    let space_part = parts.next()?.trim();
    let c1_part = parts.next()?.trim();
    let c2_part = parts.next()?.trim();

    let space = space_part.strip_prefix("in ")?.trim();

    let (c1, pct1) = parse_color_and_pct(c1_part)?;
    let (c2, pct2) = parse_color_and_pct(c2_part)?;

    let (w1, w2) = match (pct1, pct2) {
        (Some(p1), Some(p2)) => (p1, p2),
        (Some(p1), None) => (p1, 100.0 - p1),
        (None, Some(p2)) => (100.0 - p2, p2),
        (None, None) => (50.0, 50.0),
    };
    let total = w1 + w2;
    if total <= 0.0 {
        return None;
    }
    let weight_b = (w2 / total).clamp(0.0, 1.0);

    match space {
        "oklch" => {
            let a = Oklch::from_srgb(c1.r(), c1.g(), c1.b());
            let b = Oklch::from_srgb(c2.r(), c2.g(), c2.b());
            let (r, g, b) = mix_oklch(a, b, weight_b).to_srgb();
            Some(Color::from_argb(255, r, g, b))
        }
        "srgb" => {
            let lerp = |a: u8, b: u8| -> u8 {
                (a as f32 * (1.0 - weight_b) + b as f32 * weight_b).round() as u8
            };
            Some(Color::from_argb(
                255,
                lerp(c1.r(), c2.r()),
                lerp(c1.g(), c2.g()),
                lerp(c1.b(), c2.b()),
            ))
        }
        _ => None,
    }
}

fn parse_color_and_pct(s: &str) -> Option<(Color, Option<f32>)> {
    let s = s.trim();

    if let Some((head, tail)) = s.rsplit_once(' ') {
        if let Some(pct) = tail
            .strip_suffix('%')
            .and_then(|x| x.trim().parse::<f32>().ok())
        {
            if let Some(c) = parse_color_value(head.trim()) {
                return Some((c, Some(pct)));
            }
        }
    }

    parse_color_value(s).map(|c| (c, None))
}

fn parse_color_value(s: &str) -> Option<Color> {
    let s = s.trim();

    if let Some(token) = s.strip_prefix("var(--").and_then(|x| x.strip_suffix(')')) {
        return semantic_colour(token);
    }

    if s.starts_with('#') {
        return parse_hex_color(s);
    }

    match s {
        "white" => return Some(Color::WHITE),
        "black" => return Some(Color::BLACK),
        "transparent" => return Some(Color::TRANSPARENT),
        _ => {}
    }

    None
}

fn parse_color_mix_bracket(s: &str) -> Option<Color> {
    let inner = s.strip_prefix('[').and_then(|x| x.strip_suffix(']'))?;
    let decoded = decode_bracket_value(inner);
    parse_color_mix(&decoded)
}

// ==============================================================
// Known-token lookup
// ==============================================================

fn is_known_token(token: &str) -> bool {
    // Negative tokens like `-ml-4`, `-top-2`, `-translate-y-1` are
    // valid iff their positive counterpart is valid. Recursing once is
    // enough — users never write `--ml-4`.
    if let Some(rest) = token.strip_prefix('-') {
        return is_known_token(rest);
    }

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
        "shrink",
        "shrink-0",
        "shrink-1",
        "flex-wrap",
        "content-flex",
        "content-fit",
        "content-normal",
        "relative",
        "absolute",
        "global",
        "overflow-clip",
        "overflow-none",
        "overflow-scroll",
        "overflow-scroll-x",
        "overflow-scroll-xy",
        "grid",
        "bg-transparent",
        "text-transparent",
        "border",
        "rounded",
        "ring",
        "inset-ring",
        "block",
        "inline-block",
        "inline-flex",
        "flex",
        "hidden",
        "visible",
        "invisible",
        "sr-only",
        "not-sr-only",
        "pointer-events-none",
        "pointer-events-auto",
        "select-none",
        "select-text",
        "select-all",
        "select-auto",
        "resize",
        "resize-none",
        "resize-x",
        "resize-y",
        "whitespace-normal",
        "whitespace-nowrap",
        "whitespace-pre",
        "whitespace-pre-line",
        "whitespace-pre-wrap",
        "whitespace-break-spaces",
        "break-normal",
        "break-words",
        "break-all",
        "break-keep",
        "text-ellipsis",
        "text-clip",
        "truncate",
        "list-none",
        "list-disc",
        "list-decimal",
        "list-inside",
        "list-outside",
        "table-auto",
        "table-fixed",
        "border-collapse",
        "border-separate",
        "bg-repeat",
        "bg-no-repeat",
        "bg-repeat-x",
        "bg-repeat-y",
        "bg-auto",
        "bg-cover",
        "bg-contain",
        "bg-fixed",
        "bg-local",
        "bg-scroll",
        "bg-clip-border",
        "bg-clip-padding",
        "bg-clip-content",
        "bg-clip-text",
        "outline-none",
        "aspect-square",
        "aspect-video",
        "aspect-auto",
        "order-first",
        "order-last",
        "order-none",
        "container",
        "animate-spin",
        "animate-ping",
        "animate-pulse",
        "animate-bounce",
        "animate-none",
        "transition",
        "transition-all",
        "transition-colors",
        "transition-opacity",
        "transition-transform",
        "transition-none",
    ];

    const PREFIXES: &[&str] = &[
        "size-",
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
        "order-",
        "aspect-",
        "columns-",
        "space-x-",
        "space-y-",
        "divide-x-",
        "divide-y-",
        "divide-",
        "outline-offset-",
        "outline-",
        "accent-",
        "caret-",
        "decoration-",
        "underline-offset-",
        "line-clamp-",
        "overflow-x-",
        "overflow-y-",
        "grid-cols-",
        "grid-rows-",
        "col-span-",
        "row-span-",
        "shrink-",
        "rotate-",
        "scale-",
        "blur-",
        "origin-",
        "text-shadow-",
        "font-stretch-",
        "ring-offset-",
        "ring-",
        "inset-ring-",
    ];

    let (_, inner) = split_state_prefix(token);

    KEYWORDS.contains(&inner) || PREFIXES.iter().any(|p| inner.starts_with(p))
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

    if let Some(v) = s.strip_suffix("px") {
        return v.trim().parse::<f32>().ok();
    }
    if let Some(v) = s.strip_suffix("rem") {
        return v.trim().parse::<f32>().ok().map(|n| n * 16.0);
    }
    if let Some(v) = s.strip_suffix("em") {
        return v.trim().parse::<f32>().ok().map(|n| n * 16.0);
    }
    if let Some(v) = s.strip_suffix("pt") {
        return v.trim().parse::<f32>().ok().map(|n| n * 4.0 / 3.0);
    }
    if let Some(v) = s.strip_suffix('%') {
        return v.trim().parse::<f32>().ok();
    }
    s.parse::<f32>().ok()
}

fn parse_size(s: &str) -> Option<Size> {
    let s = s.trim();
    let s = s
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(s);

    match s {
        "auto" => return Some(Size::auto()),
        "full" => return Some(Size::percent(100.0)),
        "screen" => return Some(Size::window_percent(100.0)),
        "min" | "max" | "fit" => return Some(Size::auto()),
        _ => {}
    }

    if let Some(v) = s.strip_suffix('%') {
        return v.trim().parse::<f32>().ok().map(Size::percent);
    }
    if let Some(v) = s.strip_suffix("vw").or_else(|| s.strip_suffix("vh")) {
        return v.trim().parse::<f32>().ok().map(Size::window_percent);
    }
    if let Some(v) = s.strip_suffix("rem").or_else(|| s.strip_suffix("em")) {
        return v.trim().parse::<f32>().ok().map(|n| Size::px(n * 16.0));
    }
    if let Some(v) = s.strip_suffix("pt") {
        return v
            .trim()
            .parse::<f32>()
            .ok()
            .map(|n| Size::px(n * 4.0 / 3.0));
    }
    if let Some(v) = s.strip_suffix("px") {
        return v.trim().parse::<f32>().ok().map(Size::px);
    }
    s.parse::<f32>().ok().map(Size::px)
}

macro_rules! style_scale_dispatch {
    ($this:expr, $rest:expr, $prefix:ident; $($name:literal),* $(,)?) => {
        paste! {
            $(
                if $rest == stringify!($name)
                    || $rest == stringify!($name).replace('_', ".")
                {
                    return $this.[<$prefix _ $name>]();
                }
            )*
        }
    };
}

fn style_token<T: StyleExt>(this: T, token: &str) -> T {
    let (cond, inner) = split_state_prefix(token);
    if cond.is_empty() {
        apply_unprefixed_token(this, token)
    } else {
        let state_style = apply_unprefixed_token(Style::default(), inner);
        this.state_style(cond, state_style)
    }
}

/// Parse a Tailwind spacing-scale suffix (`"4"`, `"0.5"`, `"96"`, …)
/// into pixels (multiplied by `SPACING_UNIT`). Returns `None` for
/// anything that isn't a known scale name — the caller falls through
/// to `parse_len`, which handles `[10px]`, `2rem`, `50%`, etc.
fn parse_spacing_scale(s: &str) -> Option<f32> {
    let v: f32 = match s {
        "0" => 0.0,
        "0.5" | "0_5" => 0.5,
        "1" => 1.0,
        "1.5" | "1_5" => 1.5,
        "2" => 2.0,
        "2.5" | "2_5" => 2.5,
        "3" => 3.0,
        "3.5" | "3_5" => 3.5,
        "4" => 4.0,
        "5" => 5.0,
        "6" => 6.0,
        "7" => 7.0,
        "8" => 8.0,
        "9" => 9.0,
        "10" => 10.0,
        "11" => 11.0,
        "12" => 12.0,
        "14" => 14.0,
        "16" => 16.0,
        "20" => 20.0,
        "24" => 24.0,
        "28" => 28.0,
        "32" => 32.0,
        "36" => 36.0,
        "40" => 40.0,
        "44" => 44.0,
        "48" => 48.0,
        "52" => 52.0,
        "56" => 56.0,
        "60" => 60.0,
        "64" => 64.0,
        "72" => 72.0,
        "80" => 80.0,
        "96" => 96.0,
        _ => return None,
    };
    Some(SPACING_UNIT * v)
}

/// Generates a handler that parses a scale suffix and applies it with
/// the given sign to a `StyleExt` method.
macro_rules! style_signed_length {
    ($fn_name:ident, $base:ident, $sign:expr) => {
        fn $fn_name<T: StyleExt>(this: T, rest: &str) -> T {
            let sign: f32 = $sign;
            if let Some(v) = parse_spacing_scale(rest) {
                return this.$base(sign * v);
            }
            if let Some(px) = parse_len(rest) {
                return this.$base(sign * px);
            }
            this
        }
    };
}

/// Generates a handler for negative margin tokens (`-m-4`, `-mx-2`, …).
macro_rules! style_negative_spacing {
    ($fn_name:ident, $base:ident) => {
        fn $fn_name<T: StyleExt>(this: T, rest: &str) -> T {
            if let Some(v) = parse_spacing_scale(rest) {
                return this.$base(-v);
            }
            if let Some(px) = parse_len(rest) {
                return this.$base(-px);
            }
            this
        }
    };
}

// ---- Negative margins ----------------------------------------
style_negative_spacing!(style_m_negative, m);
style_negative_spacing!(style_mx_negative, mx);
style_negative_spacing!(style_my_negative, my);
style_negative_spacing!(style_mt_negative, mt);
style_negative_spacing!(style_mr_negative, mr);
style_negative_spacing!(style_mb_negative, mb);
style_negative_spacing!(style_ml_negative, ml);

// ---- Positional (top/right/bottom/left/inset), positive and negative
style_signed_length!(style_top, top, 1.0);
style_signed_length!(style_top_negative, top, -1.0);
style_signed_length!(style_right, right, 1.0);
style_signed_length!(style_right_negative, right, -1.0);
style_signed_length!(style_bottom, bottom, 1.0);
style_signed_length!(style_bottom_negative, bottom, -1.0);
style_signed_length!(style_left, left, 1.0);
style_signed_length!(style_left_negative, left, -1.0);
style_signed_length!(style_inset, inset, 1.0);
style_signed_length!(style_inset_negative, inset, -1.0);
style_signed_length!(style_inset_x, inset_x, 1.0);
style_signed_length!(style_inset_x_negative, inset_x, -1.0);
style_signed_length!(style_inset_y, inset_y, 1.0);
style_signed_length!(style_inset_y_negative, inset_y, -1.0);

// ---- Translate (maps to rendering offset) --------------------
style_signed_length!(style_translate_x, offset_x, 1.0);
style_signed_length!(style_translate_x_negative, offset_x, -1.0);
style_signed_length!(style_translate_y, offset_y, 1.0);
style_signed_length!(style_translate_y_negative, offset_y, -1.0);

// ---- Negative space-x / space-y ------------------------------
//
// Matches the positive `style_space_x` / `style_space_y` behaviour:
// `parse_len * SPACING_UNIT`. (Yes, the positive version has the same
// quirk with `[10px]` — negative matches it byte-for-byte so the two
// stay symmetric.)
fn style_space_x_negative<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(units) = parse_len(rest) {
        return this.space_x(-(units * SPACING_UNIT));
    }
    this
}

fn style_space_y_negative<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(units) = parse_len(rest) {
        return this.space_y(-(units * SPACING_UNIT));
    }
    this
}

fn apply_unprefixed_token<T: StyleExt>(this: T, token: &str) -> T {
    if let Some(rest) = token.strip_prefix('-') {
        if let Some(r) = rest.strip_prefix("rotate-") {
            return style_rotate_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("scale-") {
            return style_scale_negative(this, r);
        }
    }

    if let Some(rest) = token.strip_prefix("animate-") {
        return match rest {
            "spin" => this.animate_spin(),
            "ping" => this.animate_ping(),
            "pulse" => this.animate_pulse(),
            "bounce" => this.animate_bounce(),
            "none" => this.animate_none(),
            _ => this,
        };
    }
    if let Some(rest) = token.strip_prefix("transition-") {
        return match rest {
            "" | "all" => this.transition_all(),
            "colors" => this.transition_colors(),
            "opacity" => this.transition_opacity(),
            "transform" => this.transition_transform(),
            "none" => this.transition_none(),
            _ => this,
        };
    }
    if let Some(rest) = token.strip_prefix('-') {
        // Rotate / scale — pre-existing handlers
        if let Some(r) = rest.strip_prefix("rotate-") {
            return style_rotate_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("scale-") {
            return style_scale_negative(this, r);
        }

        // Translate (must be checked before shorter prefixes)
        if let Some(r) = rest.strip_prefix("translate-x-") {
            return style_translate_x_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("translate-y-") {
            return style_translate_y_negative(this, r);
        }

        // Space (child spacing, negative)
        if let Some(r) = rest.strip_prefix("space-x-") {
            return style_space_x_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("space-y-") {
            return style_space_y_negative(this, r);
        }

        // Inset — `inset-x-` / `inset-y-` MUST come before `inset-`
        if let Some(r) = rest.strip_prefix("inset-x-") {
            return style_inset_x_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("inset-y-") {
            return style_inset_y_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("inset-") {
            return style_inset_negative(this, r);
        }

        // Positional
        if let Some(r) = rest.strip_prefix("top-") {
            return style_top_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("right-") {
            return style_right_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("bottom-") {
            return style_bottom_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("left-") {
            return style_left_negative(this, r);
        }

        // Margins — `mx-`/`my-`/`mt-`/… before plain `m-`
        if let Some(r) = rest.strip_prefix("mx-") {
            return style_mx_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("my-") {
            return style_my_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("mt-") {
            return style_mt_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("mr-") {
            return style_mr_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("mb-") {
            return style_mb_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("ml-") {
            return style_ml_negative(this, r);
        }
        if let Some(r) = rest.strip_prefix("m-") {
            return style_m_negative(this, r);
        }
    }

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
        "shrink" => return this.shrink(),
        "shrink-0" => return this.shrink_0(),
        "shrink-1" => return this.shrink_1(),
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
        "inset-ring" => return this.inset_ring(),
        "block" => return this.content_normal(),
        "inline-block" => return this.content_fit(),
        "flex" => return this.flex_row().content_flex(),
        "inline-flex" => return this.flex_row().content_flex(),
        "hidden" => return this.hidden(),
        "visible" => return this.visible(),
        "invisible" => return this.invisible(),
        "sr-only" => return this.sr_only(),
        "not-sr-only" => return this.not_sr_only(),
        "pointer-events-none" => return this.pointer_events_none(),
        "pointer-events-auto" => return this.pointer_events_auto(),
        "select-none" => return this.select_none(),
        "select-text" => return this.select_text(),
        "select-all" => return this.select_all(),
        "select-auto" => return this.select_auto(),
        "resize" => return this.resize(),
        "resize-none" => return this.resize_none(),
        "resize-x" => return this.resize_x(),
        "resize-y" => return this.resize_y(),
        "whitespace-normal" => return this.whitespace_normal(),
        "whitespace-nowrap" => return this.whitespace_nowrap(),
        "whitespace-pre" => return this.whitespace_pre(),
        "whitespace-pre-line" => return this.whitespace_pre_line(),
        "whitespace-pre-wrap" => return this.whitespace_pre_wrap(),
        "whitespace-break-spaces" => return this.whitespace_break_spaces(),
        "break-normal" => return this.break_normal(),
        "break-words" => return this.break_words(),
        "break-all" => return this.break_all(),
        "break-keep" => return this.break_keep(),
        "text-ellipsis" => return this.text_ellipsis_set(),
        "text-clip" => return this.text_clip(),
        "truncate" => return this.text_ellipsis_set().whitespace_nowrap().overflow_clip(),
        "list-none" => return this.list_none(),
        "list-disc" => return this.list_disc(),
        "list-decimal" => return this.list_decimal(),
        "list-inside" => return this.list_inside(),
        "list-outside" => return this.list_outside(),
        "table-auto" => return this.table_auto(),
        "table-fixed" => return this.table_fixed(),
        "border-collapse" => return this.border_collapse(),
        "border-separate" => return this.border_separate(),
        "bg-repeat" => return this.bg_repeat_1(),
        "bg-no-repeat" => return this.bg_no_repeat(),
        "bg-repeat-x" => return this.bg_repeat_x(),
        "bg-repeat-y" => return this.bg_repeat_y(),
        "bg-auto" => return this.bg_auto(),
        "bg-cover" => return this.bg_cover(),
        "bg-contain" => return this.bg_contain(),
        "bg-fixed" => return this.bg_fixed(),
        "bg-local" => return this.bg_local(),
        "bg-scroll" => return this.bg_scroll(),
        "bg-clip-border" => return this.bg_clip_border(),
        "bg-clip-padding" => return this.bg_clip_padding(),
        "bg-clip-content" => return this.bg_clip_content(),
        "bg-clip-text" => return this.bg_clip_text(),
        "outline-none" => return this.outline_none(),
        "aspect-square" => return this.aspect_square(),
        "aspect-video" => return this.aspect_video(),
        "aspect-auto" => return this.aspect_auto(),
        "order-first" => return this.order_first(),
        "order-last" => return this.order_last(),
        "order-none" => return this.order_none(),
        "container" => return this.container(),
        "overflow-scroll" => return this.overflow_scroll(),
        "overflow-scroll-x" => return this.overflow_scroll_x(),
        "overflow-scroll-xy" => return this.overflow_scroll_xy(),
        "grid" => return this.content_flex().flex_wrap_content(),
        "grid-cols-1" => return this.grid_cols(1),
        "grid-cols-2" => return this.grid_cols(2),
        "grid-cols-3" => return this.grid_cols(3),
        "grid-cols-4" => return this.grid_cols(4),
        "grid-cols-5" => return this.grid_cols(5),
        "grid-cols-6" => return this.grid_cols(6),
        "grid-cols-7" => return this.grid_cols(7),
        "grid-cols-8" => return this.grid_cols(8),
        "grid-cols-9" => return this.grid_cols(9),
        "grid-cols-10" => return this.grid_cols(10),
        "grid-cols-11" => return this.grid_cols(11),
        "grid-cols-12" => return this.grid_cols(12),
        _ => {}
    }

    if let Some(r) = token.strip_prefix("size-") {
        return style_size(this, r);
    }
    if let Some(r) = token.strip_prefix("order-") {
        return style_order(this, r);
    }
    if let Some(r) = token.strip_prefix("shrink-") {
        return style_shrink(this, r);
    }
    if let Some(r) = token.strip_prefix("rotate-") {
        return style_rotate(this, r);
    }
    if let Some(r) = token.strip_prefix("scale-") {
        return style_scale(this, r);
    }
    if let Some(r) = token.strip_prefix("blur-") {
        return style_blur(this, r);
    }
    if let Some(r) = token.strip_prefix("aspect-") {
        return style_aspect(this, r);
    }
    if let Some(r) = token.strip_prefix("columns-") {
        return style_columns(this, r);
    }
    if let Some(r) = token.strip_prefix("inset-x-") {
        return style_inset_x(this, r);
    }
    if let Some(r) = token.strip_prefix("inset-y-") {
        return style_inset_y(this, r);
    }
    if let Some(r) = token.strip_prefix("inset-") {
        return style_inset(this, r);
    }
    if let Some(r) = token.strip_prefix("top-") {
        return style_top(this, r);
    }
    if let Some(r) = token.strip_prefix("right-") {
        return style_right(this, r);
    }
    if let Some(r) = token.strip_prefix("bottom-") {
        return style_bottom(this, r);
    }
    if let Some(r) = token.strip_prefix("left-") {
        return style_left(this, r);
    }
    if let Some(r) = token.strip_prefix("translate-x-") {
        return style_translate_x(this, r);
    }
    if let Some(r) = token.strip_prefix("translate-y-") {
        return style_translate_y(this, r);
    }
    if let Some(r) = token.strip_prefix("divide-x-") {
        return style_divide_x(this, r);
    }
    if let Some(r) = token.strip_prefix("divide-y-") {
        return style_divide_y(this, r);
    }
    if let Some(r) = token.strip_prefix("divide-") {
        return style_divide_colour(this, r);
    }
    if let Some(r) = token.strip_prefix("outline-offset-") {
        return style_outline_offset(this, r);
    }
    if let Some(r) = token.strip_prefix("outline-") {
        return style_outline(this, r);
    }
    if let Some(r) = token.strip_prefix("accent-") {
        return style_accent(this, r);
    }
    if let Some(r) = token.strip_prefix("caret-") {
        return style_caret(this, r);
    }
    if let Some(r) = token.strip_prefix("decoration-") {
        return style_decoration(this, r);
    }
    if let Some(r) = token.strip_prefix("underline-offset-") {
        return style_underline_offset(this, r);
    }
    if let Some(r) = token.strip_prefix("line-clamp-") {
        return style_line_clamp(this, r);
    }
    if let Some(r) = token.strip_prefix("overflow-x-") {
        return style_overflow_x(this, r);
    }
    if let Some(r) = token.strip_prefix("overflow-y-") {
        return style_overflow_y(this, r);
    }
    if let Some(r) = token.strip_prefix("grid-cols-") {
        return style_grid_cols(this, r);
    }
    if let Some(r) = token.strip_prefix("grid-rows-") {
        return style_grid_rows(this, r);
    }
    if let Some(r) = token.strip_prefix("col-span-") {
        return style_col_span(this, r);
    }
    if let Some(r) = token.strip_prefix("row-span-") {
        return style_row_span(this, r);
    }

    if let Some(r) = token.strip_prefix("ring-offset-") {
        return style_ring_offset(this, r);
    }
    if let Some(r) = token.strip_prefix("ring-") {
        return style_ring(this, r);
    }
    if let Some(r) = token.strip_prefix("inset-ring-") {
        return style_inset_ring(this, r);
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
    if let Some(rest) = token.strip_prefix("object-") {
        return match rest {
            "contain" => this.object_contain(),
            "cover" => this.object_cover(),
            "fill" => this.object_fill(),
            "none" => this.object_none(),
            "scale-down" => this.object_scale_down(),
            _ => this,
        };
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

// ---------- Extended layout dispatchers ----------

fn style_size<T: StyleExt>(this: T, rest: &str) -> T {
    if rest == "full" {
        return this.size_full();
    }

    paste! {
        style_scale_dispatch!(this, rest, size;
            0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    }

    if rest.starts_with('[') {
        if let Some(size) = parse_size(rest) {
            return this.width(size.clone()).height(size);
        }
    }

    if let Some(px) = parse_len(rest) {
        return this.size(px);
    }
    this
}

fn style_order<T: StyleExt>(this: T, rest: &str) -> T {
    let raw = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    if let Ok(n) = raw.parse::<i32>() {
        this.order(n)
    } else {
        this
    }
}

fn style_shrink<T: StyleExt>(this: T, rest: &str) -> T {
    let raw = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    if let Ok(v) = raw.parse::<f32>() {
        if v == 0.0 {
            return this.shrink_0();
        }
        return this.flex_shrink(v);
    }
    this
}

fn style_rotate<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(deg) = parse_len(rest) {
        return this.rotate(deg);
    }
    match rest {
        "0" => this.rotate(0.0),
        "1" => this.rotate(1.0),
        "2" => this.rotate(2.0),
        "3" => this.rotate(3.0),
        "6" => this.rotate(6.0),
        "12" => this.rotate(12.0),
        "45" => this.rotate(45.0),
        "90" => this.rotate(90.0),
        "180" => this.rotate(180.0),
        _ => this,
    }
}

fn style_rotate_negative<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(deg) = parse_len(rest) {
        return this.rotate(-deg);
    }
    match rest {
        "0" => this.rotate(0.0),
        "1" => this.rotate(-1.0),
        "2" => this.rotate(-2.0),
        "3" => this.rotate(-3.0),
        "6" => this.rotate(-6.0),
        "12" => this.rotate(-12.0),
        "45" => this.rotate(-45.0),
        "90" => this.rotate(-90.0),
        "180" => this.rotate(-180.0),
        _ => this,
    }
}

fn style_scale<T: StyleExt>(this: T, rest: &str) -> T {
    if rest.starts_with('[') {
        if let Some(v) = parse_len(rest) {
            return this.scale(v);
        }
        return this;
    }
    if let Ok(n) = rest.parse::<f32>() {
        return this.scale(n / 100.0);
    }
    this
}

fn style_scale_negative<T: StyleExt>(this: T, rest: &str) -> T {
    if rest.starts_with('[') {
        if let Some(v) = parse_len(rest) {
            return this.scale(-v);
        }
        return this;
    }
    if let Ok(n) = rest.parse::<f32>() {
        return this.scale(-(n / 100.0));
    }
    this
}

fn style_blur<T: StyleExt>(this: T, rest: &str) -> T {
    let radius = match rest {
        "none" => 0.0,
        "sm" => 4.0,
        "md" => 12.0,
        "lg" => 16.0,
        "xl" => 24.0,
        "2xl" => 40.0,
        "3xl" => 64.0,
        "" => 8.0,
        _ => match parse_len(rest) {
            Some(v) => v,
            None => return this,
        },
    };
    this.blur(radius)
}

fn style_aspect<T: StyleExt>(this: T, rest: &str) -> T {
    let stripped = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    if let Some((a, b)) = stripped.split_once('/') {
        if let (Ok(n), Ok(d)) = (a.parse::<f32>(), b.parse::<f32>()) {
            if d != 0.0 {
                return this.aspect(n / d);
            }
        }
    }
    if let Some(px) = parse_len(rest) {
        return this.aspect(px);
    }
    this
}

fn style_columns<T: StyleExt>(this: T, rest: &str) -> T {
    let raw = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    if let Ok(n) = raw.parse::<u32>() {
        this.columns(n)
    } else {
        this
    }
}

fn style_space_x<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(units) = parse_len(rest) {
        return this.space_x(units * SPACING_UNIT);
    }
    this
}

fn style_space_y<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(units) = parse_len(rest) {
        return this.space_y(units * SPACING_UNIT);
    }
    this
}

fn style_divide_x<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(px) = parse_len(rest) {
        return this.divide_x(px);
    }
    this
}

fn style_divide_y<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(px) = parse_len(rest) {
        return this.divide_y(px);
    }
    this
}

fn style_divide_colour<T: StyleExt>(this: T, rest: &str) -> T {
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.divide_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.divide_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.divide_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.divide_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_decoration<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => return this.decoration_thickness(0.0),
        "1" => return this.decoration_thickness(1.0),
        "2" => return this.decoration_thickness(2.0),
        "4" => return this.decoration_thickness(4.0),
        "8" => return this.decoration_thickness(8.0),
        "auto" => return this.decoration_from_font(),
        "from-font" => return this.decoration_from_font(),
        "solid" | "double" | "dotted" | "dashed" | "wavy" => return this,
        _ => {}
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.decoration_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.decoration_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.decoration_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.decoration_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_outline_offset<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(px) = parse_len(rest) {
        this.outline_offset(px)
    } else {
        this
    }
}

fn style_ring<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => return this.ring_0(),
        "1" => return this.ring_1(),
        "2" => return this.ring_2(),
        "3" => return this.ring_3(),
        "4" => return this.ring_4(),
        "6" => return this.ring_6(),
        "8" => return this.ring_8(),
        _ => {}
    }
    if let Some(px) = parse_len(rest) {
        return this.ring_width(px);
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.ring_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.ring_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.ring_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.ring_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_ring_offset<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => return this.ring_offset_0(),
        "1" => return this.ring_offset_1(),
        "2" => return this.ring_offset_2(),
        "4" => return this.ring_offset_4(),
        "8" => return this.ring_offset_8(),
        _ => {}
    }
    if let Some(px) = parse_len(rest) {
        return this.ring_offset(px);
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.ring_offset_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.ring_offset_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.ring_offset_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.ring_offset_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_inset_ring<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "0" => return this.inset_ring_0(),
        "1" => return this.inset_ring_1(),
        "2" => return this.inset_ring_2(),
        "4" => return this.inset_ring_4(),
        "8" => return this.inset_ring_8(),
        _ => {}
    }
    if let Some(px) = parse_len(rest) {
        return this.inset_ring_width(px);
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.inset_ring_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.inset_ring_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.inset_ring_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.inset_ring_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_outline<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some(px) = parse_len(rest) {
        return this.outline_width(px);
    }
    match rest {
        "0" => return this.outline_width(0.0),
        "1" => return this.outline_width(1.0),
        "2" => return this.outline_width(2.0),
        "4" => return this.outline_width(4.0),
        "8" => return this.outline_width(8.0),
        "none" => return this.outline_none(),
        _ => {}
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.outline_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.outline_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.outline_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.outline_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_underline_offset<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "auto" => this,
        "0" => this.underline_offset(0.0),
        "1" => this.underline_offset(1.0),
        "2" => this.underline_offset(2.0),
        "4" => this.underline_offset(4.0),
        "8" => this.underline_offset(8.0),
        _ => {
            if let Some(px) = parse_len(rest) {
                this.underline_offset(px)
            } else {
                this
            }
        }
    }
}

fn style_line_clamp<T: StyleExt>(this: T, rest: &str) -> T {
    if rest == "none" {
        return this;
    }
    if let Ok(n) = rest.parse::<u32>() {
        return this.line_clamp(n);
    }
    this
}

fn style_overflow_x<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "auto" | "scroll" => this.overflow_scroll_x(),
        "hidden" | "clip" => this.overflow_x(Overflow::Clip),
        "visible" => this.overflow_x(Overflow::None),
        _ => this,
    }
}

fn style_overflow_y<T: StyleExt>(this: T, rest: &str) -> T {
    match rest {
        "auto" | "scroll" => this.overflow_scroll(),
        "hidden" | "clip" => this.overflow_y(Overflow::Clip),
        "visible" => this.overflow_y(Overflow::None),
        _ => this,
    }
}

fn style_accent<T: StyleExt>(this: T, rest: &str) -> T {
    if rest == "auto" {
        return this;
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.accent_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.accent_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.accent_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.accent_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_grid_cols<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(n) = rest.parse::<u32>() {
        return this.grid_cols(n);
    }
    this
}

fn style_grid_rows<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(n) = rest.parse::<u32>() {
        return this.grid_rows(n);
    }
    this
}

fn style_col_span<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(n) = rest.parse::<u32>() {
        return this.col_span(n);
    }
    this
}

fn style_row_span<T: StyleExt>(this: T, rest: &str) -> T {
    if let Ok(n) = rest.parse::<u32>() {
        return this.row_span(n);
    }
    this
}

fn style_caret<T: StyleExt>(this: T, rest: &str) -> T {
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.caret_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.caret_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.caret_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.caret_color(with_alpha(c, alpha));
        }
    }
    this
}

fn style_bg<T: StyleExt>(this: T, rest: &str) -> T {
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.background(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.background(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.background(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.background(with_alpha(c, alpha));
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
    if rest.starts_with('[') && !rest.starts_with("[#") && !rest.starts_with("[color-mix(") {
        if let Some(px) = parse_len(rest) {
            return this.font_size(px);
        }
    }
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.color(with_alpha(c, alpha));
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
        "solid" => return this.border_solid(),
        "dashed" => return this.border_dashed(),
        "dotted" => return this.border_dotted(),
        "double" => return this.border_double(),
        "none" => return this.border_none(),
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
    let (rest, alpha) = split_opacity(rest);
    if rest.starts_with('[') {
        if let Some(c) = parse_hex_color(rest) {
            return this.border_color(with_alpha(c, alpha));
        }
        if let Some(c) = parse_color_mix_bracket(rest) {
            return this.border_color(with_alpha(c, alpha));
        }
    }
    if let Some(c) = semantic_colour(rest) {
        return this.border_color(with_alpha(c, alpha));
    }
    if let Some((family, shade)) = rest.rsplit_once('-') {
        if let Some(c) = palette_colour(family, shade) {
            return this.border_color(with_alpha(c, alpha));
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
    let raw = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    let raw = raw.strip_suffix('%').unwrap_or(raw);
    if let Ok(v) = raw.parse::<f32>() {
        let v = if rest.contains('%') || v > 1.0 {
            v / 100.0
        } else {
            v
        };
        return this.opacity(v.clamp(0.0, 1.0));
    }
    this
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
    let raw = rest
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or(rest);
    if let Ok(v) = raw.parse::<i32>() {
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
                    if rest == stringify!($name)
                        || rest == stringify!($name).replace('_', ".")
                    {
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

fn style_min_w<T: StyleExt>(this: T, rest: &str) -> T {
    style_scale_dispatch!(this, rest, min_w;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.min_width(size);
    }
    this
}

fn style_min_h<T: StyleExt>(this: T, rest: &str) -> T {
    style_scale_dispatch!(this, rest, min_h;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.min_height(size);
    }
    this
}

fn style_max_w<T: StyleExt>(this: T, rest: &str) -> T {
    style_scale_dispatch!(this, rest, max_w;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.max_width(size);
    }
    this
}

fn style_max_h<T: StyleExt>(this: T, rest: &str) -> T {
    style_scale_dispatch!(this, rest, max_h;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.max_height(size);
    }
    this
}

fn style_w<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some((num, den)) = rest.split_once('/') {
        if let (Ok(n), Ok(d)) = (num.parse::<f32>(), den.parse::<f32>()) {
            if d != 0.0 {
                return this.w_percent((n / d) * 100.0);
            }
        }
    }
    style_scale_dispatch!(this, rest, w;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.width(size);
    }
    this
}

fn style_h<T: StyleExt>(this: T, rest: &str) -> T {
    if let Some((num, den)) = rest.split_once('/') {
        if let (Ok(n), Ok(d)) = (num.parse::<f32>(), den.parse::<f32>()) {
            if d != 0.0 {
                return this.h_percent((n / d) * 100.0);
            }
        }
    }
    style_scale_dispatch!(this, rest, h;
        0,0_5,1,1_5,2,2_5,3,3_5,4,5,6,7,8,9,10,11,12,14,16,20,24,28,32,36,40,44,48,52,56,60,64,72,80,96);
    if let Some(size) = parse_size(rest) {
        return this.height(size);
    }
    this
}
