//! # FreyaCN Icon Component
//!
//! A flexible SVG icon component that integrates with the FreyaCN
//! theming system and all extension traits.
//!
//! ## Inheriting from a parent
//!
//! `Icon` is non-interactive by default. Hover and press are inherited
//! from the parent `Div` via `DivActiveState`, so an icon inside a
//! `Button` or `AccordionTrigger` doesn't steal hover from its parent.
//! Callers that want an interactive icon can opt in with
//! `.pointer_events_auto()` on their style.
//! ## Interactive by default? No.
//!
//! `Icon` is non-interactive unless the caller opts in with
//! `.pointer-events-auto()`. It inherits hover / focus / active from
//! the parent `Div`, so an icon inside a `Button` or
//! `AccordionTrigger` reacts to the parent's hover without stealing
//! it. See `Div`'s module docs for the reason.
//!
//! Standalone usage:
//!
//! ```ignore
//! Icon()
//!     .svg(bytes)
//!     .size_6()
//!     .style("pointer-events-auto hover:scale-110")
//! ```

use crate::prelude::*;

const DEFAULT_ICON_SIZE: f32 = 16.0;

#[component(
    props(
        svg: Bytes = Bytes::default(),
    ),
    ext(style, accessibility, key),
)]
#[derive(PartialEq, Clone)]
pub fn Icon() {
    let (width, height) = match (&style.width, &style.height) {
        (Some(w), Some(h)) => (w.clone(), h.clone()),
        (Some(w), None) => (w.clone(), w.clone()),
        (None, Some(h)) => (h.clone(), h.clone()),
        (None, None) => {
            let s = Size::px(DEFAULT_ICON_SIZE);
            (s.clone(), s)
        }
    };

    let mut wrapper_style = style.clone();
    wrapper_style.width = None;
    wrapper_style.height = None;

    // Non-interactive by default. Hover is inherited from the parent
    // `Div` via `DivActiveState`. An interactive icon would become the
    // topmost hit-test target when the pointer moves onto it, firing
    // `pointer_leave` on the parent and dropping its hover state.
    if wrapper_style.pointer_events.is_none() {
        wrapper_style.pointer_events = Some(false);
    }

    let svg_viewer = SvgViewer::new(svg.clone()).width(width).height(height);

    Div()
        .merge_style(wrapper_style)
        .accessibility(accessibility.clone())
        .child(svg_viewer)
}
