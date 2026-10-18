//! # Avatar
//!
//! User/entity avatar primitives — `Avatar`, `AvatarFallback`,
//! `AvatarGroup`, `AvatarGroupCount`, `AvatarBadge`.
//!
//! ## In-group separator
//!
//! `AvatarGroup` wraps each child in a `Div` with `.p(2.0).bg(card).rounded-full`.
//! That 2px card-coloured padding is the ring — the same visual the
//! reference `ring-2 ring-background` produces. Overlap comes from
//! `.ml(spacing)` on every non-first child; the wrapper padding then
//! cuts into the previous avatar's image.
//!
//! No border, no shadow, no z-index. Freya 0.5.0-rc.10 renders
//! `padding + background` reliably on every element shape; borders and
//! shadows hit enough edge cases that this is the only design that
//! works consistently.

use crate::prelude::*;

// ==============================================================
// Variants
// ==============================================================

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarSize {
    #[variant("size-6 text-[10px]")]
    Xs,

    #[variant("size-8 text-xs")]
    #[default]
    Md,

    #[variant("size-10 text-sm")]
    Lg,

    #[variant("size-12 text-base")]
    Xl,

    #[variant("size-16 text-lg")]
    Xxl,
}

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[variant("rounded-full")]
    #[default]
    Circle,

    #[variant("rounded-lg")]
    Rounded,

    #[variant("rounded-none")]
    Square,
}

#[variant]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarBadgeSize {
    #[variant("size-2")]
    Sm,

    #[variant("size-2.5")]
    #[default]
    Md,

    #[variant("size-3")]
    Lg,

    #[variant("size-4")]
    Xl,
}

// ==============================================================
// AvatarFallback
// ==============================================================

#[component(ext(style, children, key, event_handlers, accessibility))]
#[derive(Clone, PartialEq)]
pub fn AvatarFallback() {
    Div()
        .style(
            "flex size-full items-center justify-center \
             bg-muted text-muted-foreground font-medium",
        )
        .merge_style(style.clone())
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone())
        .children(children.clone())
}

// ==============================================================
// AvatarGroup
// ==============================================================

/// A horizontal row of overlapping avatars.
///
/// Each child is wrapped in a `Div` with `.p(2.0).bg(card).rounded-full`.
/// The 2px card padding *is* the separator ring — it visually cuts into
/// the previous avatar when overlapped via `.ml(spacing)`.
#[component(
    props(
        spacing: f32 = -8.0,
    ),
    ext(style, children, key, event_handlers, accessibility),
)]
#[derive(Clone, PartialEq)]
pub fn AvatarGroup() {
    let spacing_val = *spacing;

    let theme_state = use_cn_theme();
    let card_bg = theme_state.read().card;

    let mut container = Div()
        .style("flex flex-row items-center")
        .merge_style(style.clone())
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone());

    for (i, child) in children.iter().enumerate() {
        // Padding + card background = the ring. Circular so it hugs a
        // circular avatar. z-index keeps later avatars on top of
        // earlier ones (paint order, not needed for the ring visual
        // because padding always renders).
        let mut wrapper = Div()
            .z_index(i as i32)
            .corner_radius(9999.0)
            .padding(Gaps::new_all(2.0))
            .background(card_bg);

        if i > 0 {
            wrapper = wrapper.ml(spacing_val);
        }

        container = container.child(wrapper.child(child.clone()));
    }

    container
}

// ==============================================================
// AvatarGroupCount
// ==============================================================

#[component(
    ext(style, children, key, event_handlers, accessibility),
    variants(
        size: AvatarSize = AvatarSize::Md,
        shape: AvatarShape = AvatarShape::Circle,
    ),
)]
#[derive(Clone, PartialEq)]
pub fn AvatarGroupCount() {
    let _ = (size, shape);

    // The group wrapper adds the card padding (ring) automatically, so
    // this only needs the avatar-shaped box.
    Div()
        .style(
            "relative flex shrink-0 overflow-clip items-center justify-center \
             bg-muted text-muted-foreground font-medium",
        )
        .merge_style(style.clone())
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone())
        .children(children.clone())
}

impl AvatarGroupCountComponent {
    pub fn count(self, n: u32) -> Self {
        self.child(Label().text(format!("+{n}")))
    }
}

// ==============================================================
// AvatarBadge
// ==============================================================

#[component(
    ext(style, children, key, event_handlers, accessibility),
    variants(
        size: AvatarBadgeSize = AvatarBadgeSize::Md,
    ),
)]
#[derive(Clone, PartialEq)]
pub fn AvatarBadge() {
    let _ = size;

    Div()
        .absolute()
        .bottom(0.0)
        .right(0.0)
        .z_index(10)
        .style(
            "rounded-full bg-primary text-primary-foreground \
             border-2 border-background \
             flex items-center justify-center",
        )
        .merge_style(style.clone())
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone())
        .children(children.clone())
}

// ==============================================================
// Avatar
// ==============================================================

/// The avatar wrapper. Renders an image (from `src`/`bytes`) when
/// available; otherwise renders any `AvatarFallback` child.
///
/// No in-group awareness — the separator ring is entirely the
/// responsibility of `AvatarGroup`'s wrapper. A standalone avatar is
/// just a clipped rounded box.
#[component(
    props(
        src: String = String::new(),
        bytes: Option<Bytes> = None,
        alt: String = String::new(),
    ),
    ext(style, children, key, event_handlers, accessibility),
    variants(
        size: AvatarSize = AvatarSize::Md,
        shape: AvatarShape = AvatarShape::Circle,
    ),
)]
#[struct_fields(
    badge: Element,
)]
#[derive(Clone, PartialEq)]
pub fn Avatar() {
    let _ = (size, shape);

    // ── Hooks (fixed order) ────────────────────────────────────
    let mut cached_src = use_state::<String>(|| String::new());
    let mut cached_exists = use_state::<bool>(|| false);

    // ── Image existence check (cached on src) ──────────────────
    let has_image = if bytes.is_some() {
        true
    } else if !src.is_empty() {
        if *cached_src.peek() != *src {
            let exists = std::fs::metadata(src.as_str())
                .map(|m| m.is_file() && m.len() > 0)
                .unwrap_or(false);
            *cached_src.write() = src.to_string();
            *cached_exists.write() = exists;
            exists
        } else {
            *cached_exists.read()
        }
    } else {
        false
    };

    // ── Inner content ──────────────────────────────────────────
    let inner_content: Element = if has_image {
        let mut img = Image().style("size-full object-cover");
        if let Some(b) = bytes.clone() {
            img = img.bytes(b);
        } else {
            img = img.src(src.as_str());
        }
        if !alt.is_empty() {
            img = img.alt(alt.as_str());
        }
        img.into_element()
    } else if !children.is_empty() {
        let mut container = Div().style("size-full");
        for child in children.iter() {
            container = container.child(child.clone());
        }
        container.into_element()
    } else {
        Div()
            .style("flex size-full items-center justify-center bg-muted")
            .into_element()
    };

    // ── Body ───────────────────────────────────────────────────
    //
    // Single wrapper: sizing + rounding + overflow-clip from `style`,
    // content inside. No ring, no shadow, no group special-casing.
    Div()
        .style(
            "relative flex shrink-0 overflow-clip \
             items-center justify-center",
        )
        .merge_style(style.clone())
        .child(inner_content)
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone())
        .into_element()
}

// ==============================================================
// Avatar helpers
// ==============================================================

impl AvatarComponent {
    pub fn fallback(self, text: impl Into<String>) -> Self {
        self.child(AvatarFallback().child(Label().text(text.into())))
    }

    pub fn badge(mut self, badge: impl IntoElement) -> Self {
        self.badge = Some(badge.into_element());
        self
    }

    /// Standalone ring. Adds 2px `theme.card` padding to the avatar's
    /// own body — visually matches the in-group ring, though it shrinks
    /// the visible image area by 2px on each side (the group version
    /// wraps the avatar in an outer padded `Div` and doesn't shrink it).
    /// Prefer `AvatarGroup` when you can.
    pub fn bordered(self) -> Self {
        let theme_state = use_cn_theme();
        let card = theme_state.read().card;
        self.padding(Gaps::new_all(2.0)).background(card)
    }
}
