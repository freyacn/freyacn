/*
//! # Card Component
//!
//! A flexible card component inspired by shadcn/ui with support for headers,
//! titles, descriptions, actions, content, and footers.
//!
//! ## Examples
//!
//! ```no_run
//! use freyacn::components::card::{Card, CardHeader, CardTitle, CardContent, CardFooter};
//! use freyacn::components::label::Label;
//! use freyacn::extensions::*;
//!
//! let card = Card()
//!     .size(CardSize::Sm)
//!     .child(
//!         CardHeader()
//!             .child(CardTitle::new("Card Title"))
//!             .child(CardDescription::new("This is a description"))
//!     )
//!     .child(
//!         CardContent()
//!             .child(Label("Some content here"))
//!     )
//!     .child(
//!         CardFooter()
//!             .child(Label("Footer text"))
//!     );
//! ```

use crate::extensions::*;
use crate::theme::use_cn_theme;
use freya::prelude::*;

/// Size variants for the card.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CardSize {
    /// Default size with larger padding and gap.
    #[default]
    Default,
    /// Small size with reduced padding and gap.
    Sm,
}

/// The main card container component.
#[derive(Clone, PartialEq)]
pub struct Card {
    size: CardSize,
    elements: Vec<Element>,
    key: DiffKey,

    // ---- Extension state ----
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
    opacity: Option<f32>,
    shadow: Option<Shadow>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            elements: Vec::new(),
            key: DiffKey::None,
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
            opacity: None,
            shadow: None,
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Extension trait implementations ----

impl BackgroundExt for Card {
    fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
}

impl ForegroundExt for Card {
    fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl SpacingExt for Card {
    fn padding(mut self, gaps: impl Into<Gaps>) -> Self {
        self.padding = Some(gaps.into());
        self
    }

    fn margin(mut self, gaps: impl Into<Gaps>) -> Self {
        self.margin = Some(gaps.into());
        self
    }
}

impl SizingExt for Card {
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

impl BorderExt for Card {
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

impl EffectsExt for Card {
    fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }
}

impl ChildrenExt for Card {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Card {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl CornerRadiusExt for Card {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

// ---- Component implementation ----

impl Component for Card {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        // Determine padding and gap based on size.
        let (padding, gap) = match self.size {
            CardSize::Default => (Gaps::new_all(16.0), 16.0),
            CardSize::Sm => (Gaps::new_all(12.0), 12.0),
        };

        // Start building the container.
        let mut container = rect()
            .vertical()
            .spacing(gap)
            .padding(self.padding.unwrap_or(padding))
            .background(self.background.unwrap_or(theme.card))
            .color(self.color.unwrap_or(theme.card_foreground))
            .corner_radius(self.corner_radius.unwrap_or_else(|| 12.0.into()));

        // Apply border – use theme.border for the default.
        let border_color = self.border_color.unwrap_or(theme.border);
        let border_width = self.border_width.unwrap_or(1.0);
        container = container.border(Border::new().fill(border_color).width(border_width));

        // Apply margin.
        if let Some(margin) = self.margin {
            container = container.margin(margin);
        }

        // Apply sizing.
        if let Some(width) = self.width.clone() {
            container = container.width(width);
        }
        if let Some(height) = self.height.clone() {
            container = container.height(height);
        }
        if let Some(min_width) = self.min_width.clone() {
            container = container.min_width(min_width);
        }
        if let Some(min_height) = self.min_height.clone() {
            container = container.min_height(min_height);
        }
        if let Some(max_width) = self.max_width.clone() {
            container = container.max_width(max_width);
        }
        if let Some(max_height) = self.max_height.clone() {
            container = container.max_height(max_height);
        }

        // Apply opacity.
        if let Some(opacity) = self.opacity {
            container = container.opacity(opacity);
        }

        // Apply shadow (user-provided only – no default shadow to keep it clean).
        if let Some(shadow) = self.shadow.clone() {
            container = container.shadow(shadow);
        }

        // Add children.
        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

/// Constructor for the Card component.
#[allow(non_snake_case)]
pub fn Card() -> Card {
    Card::new()
}

// ============================================================
// Card Sub-components
// ============================================================

/// Card header container.
#[derive(Clone, PartialEq)]
pub struct CardHeader {
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CardHeader {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardHeader {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardHeader {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        let mut container = rect()
            .vertical()
            .spacing(4.0)
            .padding(Gaps::new(0.0, 16.0, 0.0, 16.0)) // px-4
            .color(theme.card_foreground);

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardHeader() -> CardHeader {
    CardHeader::new()
}

/// Card title (typically a heading).
#[derive(Clone, PartialEq)]
pub struct CardTitle {
    text: String,
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl ChildrenExt for CardTitle {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardTitle {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardTitle {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        let mut container = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .padding(Gaps::new(0.0, 16.0, 0.0, 16.0))
            .color(theme.card_foreground);

        let label = label()
            .text(self.text.clone())
            .font_size(16.0)
            .font_weight(FontWeight::MEDIUM);

        container = container.child(label);

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardTitle(text: impl Into<String>) -> CardTitle {
    CardTitle::new(text)
}

/// Card description (muted text).
#[derive(Clone, PartialEq)]
pub struct CardDescription {
    text: String,
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl ChildrenExt for CardDescription {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardDescription {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardDescription {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        let mut container = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .padding(Gaps::new(0.0, 16.0, 0.0, 16.0))
            .color(theme.muted_foreground);

        let label = label()
            .text(self.text.clone())
            .font_size(14.0)
            .color(theme.muted_foreground);

        container = container.child(label);

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardDescription(text: impl Into<String>) -> CardDescription {
    CardDescription::new(text)
}

/// Card action container (typically for buttons/actions).
#[derive(Clone, PartialEq)]
pub struct CardAction {
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardAction {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl Default for CardAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CardAction {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardAction {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardAction {
    fn render(&self) -> impl IntoElement {
        let mut container = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .padding(Gaps::new(0.0, 16.0, 0.0, 16.0));

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardAction() -> CardAction {
    CardAction::new()
}

/// Card content container.
#[derive(Clone, PartialEq)]
pub struct CardContent {
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardContent {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl Default for CardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CardContent {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardContent {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardContent {
    fn render(&self) -> impl IntoElement {
        let mut container = rect()
            .vertical()
            .spacing(4.0)
            .padding(Gaps::new(0.0, 16.0, 0.0, 16.0));

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardContent() -> CardContent {
    CardContent::new()
}

/// Card footer container with border-top and muted background.
#[derive(Clone, PartialEq)]
pub struct CardFooter {
    elements: Vec<Element>,
    key: DiffKey,
}

impl CardFooter {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            key: DiffKey::None,
        }
    }
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CardFooter {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for CardFooter {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CardFooter {
    fn render(&self) -> impl IntoElement {
        let theme = use_cn_theme().read();

        let mut container = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .padding(Gaps::new_all(16.0))
            .background(theme.muted)
            .border(Border::new().fill(theme.border).width(1.0))
            .corner_radius(CornerRadius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: 12.0,
                bottom_right: 12.0,
                smoothing: 0.0,
            });

        for child in &self.elements {
            container = container.child(child.clone());
        }

        container.key(self.key.clone())
    }
}

#[allow(non_snake_case)]
pub fn CardFooter() -> CardFooter {
    CardFooter::new()
}

 */
