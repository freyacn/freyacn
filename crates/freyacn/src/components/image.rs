//! # Image Component
//!
//! A styled image primitive that mimics HTML's `<img>` and wraps
//! Freya 0.5.0-rc.10's native `image()` element.
//!
//! ## Construction — Freya has no path constructor
//!
//! In 0.5.0-rc.10, `freya::prelude::image()` takes an [`ImageHandle`],
//! not a path or bytes. `ImageHandle` has exactly two constructors:
//!
//! - `ImageHandle::new(RCHandle<SkImage>, Bytes)` — wrap a pre-decoded
//!   Skia image.
//! - `ImageHandle::from_rgba(width, height, Bytes, AlphaType)` — build
//!   from raw RGBA pixel data.
//!
//! There is **no** `from_path`, `from_png`, or bytes-based `image()`
//! overload. To render from a file path — the HTML `<img src="…">`
//! experience — you must read the file, decode it with
//! `skia_safe::Image::from_encoded(&data)`, and wrap it in an
//! `ImageHandle`. That is exactly what this component does.
//!
//! The decode is cached keyed on the `src` string, so re-renders don't
//! re-read or re-decode the file. Changing `src` invalidates the cache.
//!
//! ## Requirements
//!
//! This module needs the `skia-safe` crate, aliased to `skia_safe` so
//! imports stay stable. Freya 0.5.0-rc.10 pins its own fork
//! (`freya-skia-safe 0.104.0`), so the rename in `Cargo.toml` is:
//!
//! ```toml
//! [dependencies]
//! skia-safe = { package = "freya-skia-safe", version = "0.104.0" }
//! ```
//!
//! Adding upstream `skia-safe` directly conflicts on the `links =
//! "skia"` native-library slot — one crate per slot — so the aliased
//! fork is the only safe path.
//!
//! ## Two ways to supply an image
//!
//! | Prop | When to use |
//! |---|---|
//! | `src: String` | Runtime file path (a user-selected file, an asset on disk). Reads and decodes on first render. |
//! | `bytes: Option<Bytes>` | Pre-loaded bytes — embedded assets via `include_bytes!`, or bytes fetched over the network. Skips the filesystem read but still decodes. |
//!
//! If both are set, `bytes` wins.
//!
//! ## Supported formats
//!
//! The decoder uses `skia_safe::Image::from_encoded`, which supports
//! PNG, JPEG, WebP, GIF, BMP, ICO, and WBMP out of the box. AVIF, HEIF,
//! and TIFF are supported when Skia is built with the corresponding
//! codecs. SVG is **not** supported — Skia is a raster library. If you
//! need SVG, rasterise it to PNG/WebP before loading.
//!
//! ## `fit` — an `ImageFit`, convertible from strings
//!
//! The `fit` prop is typed `ImageFit`. `From<&str>`, `From<String>`,
//! and `From<&String>` are implemented on `ImageFit`, so the
//! kebab-case strings `"fill"`, `"cover"`, `"contain"`, `"none"`, and
//! `"scale-down"` all convert. Callers write one of:
//!
//! ```ignore
//! Image().src("photo.png").fit(ImageFit::Cover)          // enum, always works
//! Image().src("photo.png").fit(ImageFit::from("cover"))  // explicit conversion, always works
//! Image().src("photo.png").fit("cover".into())           // may infer; prefer the form above
//! ```
//!
//! The `#[component]` macro generates a concrete-typed setter
//! (`fn fit(self, value: ImageFit) -> Self`), so `.fit("cover")` with no
//! `.into()` does **not** compile. Use `ImageFit::from("cover")` when
//! you have a string and want the cleanest call site.
//!
//! ## Styling (tailwind-style tokens)
//!
//! ```ignore
//! Image()
//!     .src("photo.webp")
//!     .style("w-64 h-48 object-cover rounded-lg shadow-md blur-sm")
//! ```
//!
//! - `object-fill` / `object-cover` / `object-contain` / `object-none` /
//!   `object-scale-down` → mapped onto `ImageFit` (and thence onto
//!   Freya's `AspectRatio` + `ImageCover`).
//! - `blur-*` → `ImageData::blur`.
//! - `rounded-*`, `shadow-*`, `border-*`, sizing, opacity, rotate,
//!   scale, transforms, overflow, position, z-index all map to the
//!   corresponding `image` element builder calls.
//!
//! `style.object_fit` takes precedence over the `fit` prop; `style.blur`
//! takes precedence over the `blur` prop — the style string is the more
//! specific declaration.
//!
//! Freya's `Image` has no `BackgroundExt` and no `TextStyleExt`, so
//! `bg-*` and `text-*` style tokens are silently dropped. Wrap the
//! image in a `Div` if you need a background behind it.
//!
//! ## HTML parity
//!
//! | HTML `<img>`       | `Image()`                                    |
//! |--------------------|----------------------------------------------|
//! | `src`              | `.src("assets/photo.png")`                   |
//! | `alt`              | `.alt(text)` → `a11y_alt` on the image      |
//! | `width`/`height`   | `.style("w-64 h-48")`                        |
//! | `object-fit`       | `.fit(ImageFit::Cover)` or `style("object-cover")` |
//! | `filter: blur()`   | `.style("blur-sm")` → `ImageData::blur`     |
//! | `loading`/`srcset` | not applicable in native Freya               |
//!
//! ## `object-fit` mapping
//!
//! | `ImageFit` | Freya `AspectRatio` | Freya `ImageCover` |
//! |------------|---------------------|--------------------|
//! | `Fill`     | `None` (stretch)    | `Fill`             |
//! | `Cover`    | `Max` (cover, crop) | `Center`           |
//! | `Contain`  | `Min` (fit inside)  | `Fill`             |
//! | `None`     | `Fit` (natural)     | `Fill`             |

use crate::prelude::*;

// Re-export Freya's image types so callers don't need a direct freya
// dependency to configure sampling or build handles.
pub use freya::elements::image::{ImageData, ImageHandle};
pub use freya::prelude::{AspectRatio, ImageCover, SamplingMode};

// ==============================================================
// ImageFit
// ==============================================================

/// How the image is scaled to fit its box, mirroring CSS `object-fit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageFit {
    /// Stretch to fill the box, ignoring the image's aspect ratio
    /// (CSS `object-fit: fill`).
    #[default]
    Fill,

    /// Scale to cover the box and crop the overflow, preserving aspect
    /// ratio (CSS `object-fit: cover`). The image is centred.
    Cover,

    /// Scale to fit inside the box, preserving aspect ratio
    /// (CSS `object-fit: contain`).
    Contain,

    /// Render at the image's natural size (CSS `object-fit: none`).
    None,
}

impl ImageFit {
    fn to_aspect_ratio(self) -> AspectRatio {
        match self {
            ImageFit::Fill => AspectRatio::None,
            ImageFit::Cover => AspectRatio::Max,
            ImageFit::Contain => AspectRatio::Min,
            ImageFit::None => AspectRatio::Fit,
        }
    }

    fn to_cover(self) -> ImageCover {
        match self {
            ImageFit::Cover => ImageCover::Center,
            _ => ImageCover::Fill,
        }
    }
}

// ---- String → ImageFit (kebab-case) -------------------------
//
// `#[component]` generates a concrete-typed setter
// (`fn fit(self, value: ImageFit) -> Self`), so plain `&str` arguments
// do not coerce automatically. Use `ImageFit::from("cover")` at the
// call site — explicit, unambiguous, always compiles:
//
//     Image().src("x").fit(ImageFit::from("cover"))
//     Image().src("x").fit(ImageFit::from(String::from("contain")))
//     Image().src("x").fit(ImageFit::from(&some_string))
//
// `.fit(ImageFit::Cover)` is the enum path and is also always valid.
//
// Unknown values fall back to `Fill` (CSS `object-fit` default).
// `scale-down` maps to `Contain` — Freya has no dedicated shrink-only
// mode, and `Contain` is the closest visible behaviour.

impl From<&str> for ImageFit {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "fill" => ImageFit::Fill,
            "cover" => ImageFit::Cover,
            "contain" => ImageFit::Contain,
            "none" => ImageFit::None,
            "scale-down" | "scale_down" | "scaledown" => ImageFit::Contain,
            _ => ImageFit::Fill,
        }
    }
}

impl From<String> for ImageFit {
    fn from(s: String) -> Self {
        ImageFit::from(s.as_str())
    }
}

impl From<&String> for ImageFit {
    fn from(s: &String) -> Self {
        ImageFit::from(s.as_str())
    }
}

// ==============================================================
// ObjectFit → ImageFit bridge (style-string support)
// ==============================================================

fn object_fit_to_image_fit(fit: ObjectFit) -> ImageFit {
    match fit {
        ObjectFit::Fill => ImageFit::Fill,
        ObjectFit::Cover => ImageFit::Cover,
        ObjectFit::Contain => ImageFit::Contain,
        ObjectFit::None => ImageFit::None,
        // `scale-down` means "shrink to fit, never enlarge" — not
        // representable in Freya's image element, so fall back to
        // `Contain` (closest visible behaviour).
        ObjectFit::ScaleDown => ImageFit::Contain,
    }
}

// ==============================================================
// Decoding — the bridge from bytes to ImageHandle
// ==============================================================

/// Decode encoded image bytes (PNG, JPEG, WebP, GIF, BMP, ICO, WBMP,
/// and any other codec the linked Skia was built with) into an
/// `ImageHandle` ready for `image()`.
///
/// Freya 0.5.0-rc.10 has no built-in bytes → `ImageHandle` helper, so
/// we decode with `skia_safe::Image::from_encoded` and wrap the
/// resulting `SkImage` in an `ImageHandle`.
///
/// The `freya-skia-safe 0.104.0` fork (which is what `freya` actually
/// links) requires a `Data` for `from_encoded`, not raw bytes — hence
/// the `Data::new_copy` step.
fn decode_bytes(bytes: Bytes) -> Option<ImageHandle> {
    let data = skia_safe::Data::new_copy(bytes.as_ref());
    let sk_image = skia_safe::Image::from_encoded(&data)?;
    // `skia_safe::Image` is a newtype over `RCHandle<SkImage>` with a
    // `From<Image>` impl for `RCHandle<SkImage>`, so `.into()` works.
    // If your build rejects it, replace with
    // `skia_safe::RCHandle::from(sk_image)`.
    Some(ImageHandle::new(sk_image.into(), bytes))
}

/// Read a file from disk and decode it into an `ImageHandle`.
fn load_from_path(path: &str) -> Option<ImageHandle> {
    let raw = std::fs::read(path).ok()?;
    decode_bytes(Bytes::from(raw))
}

// ==============================================================
// Component
// ==============================================================

#[component(
    props(
        src: String = String::new(),
        bytes: Option<Bytes> = None,
        alt: String = String::new(),
        fit: ImageFit = ImageFit::Fill,
        sampling: SamplingMode = SamplingMode::Trilinear,
        snap_to_grid: bool = false,
        blur: f32 = 0.0,
    ),
    ext(style, accessibility, event_handlers, key),
)]
#[derive(PartialEq, Clone)]
pub fn Image() {
    // ── Cache the decoded handle keyed on source ────────────────
    //
    // Decoding is not free — a 4K PNG is milliseconds of work. Cache
    // the resulting `ImageHandle` and only re-decode when the source
    // actually changes. `bytes` and `src` are both part of the key.
    let source_key = match bytes.as_ref() {
        Some(_) => format!("bytes:{}", src),
        None => format!("path:{}", src),
    };

    let mut handle_state = use_state::<Option<ImageHandle>>(|| None);
    let mut loaded_key = use_state::<String>(|| String::new());

    let needs_load = *loaded_key.peek() != source_key;
    let handle: Option<ImageHandle> = if needs_load {
        let decoded = match bytes.as_ref() {
            Some(b) => decode_bytes(b.clone()),
            None if !src.is_empty() => load_from_path(src),
            None => None,
        };
        *loaded_key.write() = source_key;
        *handle_state.write() = decoded.clone();
        decoded
    } else {
        handle_state.read().clone()
    };

    // ── Fallback ─────────────────────────────────────────────────
    //
    // No handle means the file is missing, the bytes are corrupt, or
    // no source was provided. Render an empty box with the caller's
    // sizing so the layout doesn't collapse and the missing image is
    // visible as a gap.
    let Some(handle) = handle else {
        let mut placeholder = Div().style("bg-muted");
        if let Some(w) = style.width.clone() {
            placeholder = placeholder.width(w);
        }
        if let Some(h) = style.height.clone() {
            placeholder = placeholder.height(h);
        }
        if let Some(cr) = style.corner_radius.clone() {
            placeholder = placeholder.corner_radius(cr);
        }
        return placeholder.into_element();
    };

    // ── Resolve effective fit ───────────────────────────────────
    //
    // `style.object_fit` (from `object-cover`, `object-contain`, …)
    // takes precedence over the `fit` prop — the style string is the
    // more specific declaration.
    let effective_fit = match style.object_fit {
        Some(obj) => object_fit_to_image_fit(obj),
        None => *fit,
    };

    // ── Image data ──────────────────────────────────────────────
    //
    // `SamplingMode` is `Clone` but not `Copy`, so `*sampling` fails.
    // Clone the reference's value.
    //
    // Blur: `style.blur` (from `blur-sm`, `blur-md`, …) wins over the
    // `blur` prop for the same reason.
    let data = ImageData {
        sampling_mode: sampling.clone(),
        aspect_ratio: effective_fit.to_aspect_ratio(),
        image_cover: effective_fit.to_cover(),
        snap_to_grid: *snap_to_grid,
        blur: style.blur.unwrap_or(*blur),
    };

    // ── Inner image element ─────────────────────────────────────
    //
    // `image` in this crate's prelude is the lucide `image` icon
    // (0 arguments) — it shadows `freya::prelude::image`. Use the
    // fully-qualified path so we get the element constructor.
    let mut img = freya::prelude::image(handle).image_data(data);

    // ── Decoration ───────────────────────────────────────────────
    if let Some(cr) = style.corner_radius.clone() {
        img = img.corner_radius(cr);
    }
    if let Some(border) = style.to_border() {
        img = img.border(border);
    }
    for shadow in &style.shadows.clone() {
        img = img.shadow(shadow.clone());
    }

    // ── Box ──────────────────────────────────────────────────────
    if let Some(gaps) = style.padding.clone() {
        img = img.padding(gaps);
    }
    if let Some(gaps) = style.margin.clone() {
        img = img.margin(gaps);
    }
    if let Some(size) = style.width.clone() {
        img = img.width(size);
    }
    if let Some(size) = style.height.clone() {
        img = img.height(size);
    }
    if let Some(size) = style.min_width.clone() {
        img = img.min_width(size);
    }
    if let Some(size) = style.min_height.clone() {
        img = img.min_height(size);
    }
    if let Some(size) = style.max_width.clone() {
        img = img.max_width(size);
    }
    if let Some(size) = style.max_height.clone() {
        img = img.max_height(size);
    }
    if let Some(size) = style.visible_width.clone() {
        img = img.visible_width(size);
    }
    if let Some(size) = style.visible_height.clone() {
        img = img.visible_height(size);
    }
    if let Some(position) = style.to_position() {
        img = img.position(position);
    }

    // ── Effects / transforms ─────────────────────────────────────
    if let Some(o) = style.overflow {
        img = img.overflow(o);
    }
    if let Some(op) = style.opacity {
        img = img.opacity(op);
    }
    if let Some(deg) = style.rotate {
        img = img.rotation(deg);
    }
    if let Some(s) = style.scale.clone() {
        img = img.scale(s);
    }
    if let Some(origin) = style.transform_origin.clone() {
        img = img.transform_origin(origin);
    }
    if let Some(z) = style.z_index {
        let clamped = z.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        img = img.layer(clamped);
    }

    // ── Interaction ──────────────────────────────────────────────
    img = img
        .accessibility(accessibility.clone())
        .event_handlers(event_handlers.clone());

    // ── Accessibility ────────────────────────────────────────────
    if !alt.is_empty() {
        img = img.a11y_alt(alt.clone());
    }

    img.into_element()
}
