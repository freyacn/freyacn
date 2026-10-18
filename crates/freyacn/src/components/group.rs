//! # Group
//!
//! Wraps a subtree and exposes its hover / focus state to descendants
//! via a `GroupState` context. `Div` reads this context and sets
//! `GROUP_HOVER` / `GROUP_FOCUS`, so any descendant with a
//! `group-hover:` / `has-hover:` utility reacts when the **group**
//! is hovered — including when the hover comes from a different
//! descendant.
//!
//! Uses `on_pointer_enter` / `on_pointer_leave` (not `over` / `out`)
//! because the latter bubble. With `over` / `out`, moving the pointer
//! from the group's own area onto one of its children fires `out` on
//! the group (bubbled from the child boundary), clearing the hover
//! state — and then `over` immediately re-fires, causing a flicker.
//! `enter` / `leave` fire exactly once per boundary crossing, which
//! is what we want.

use crate::extensions::GroupState;
use crate::prelude::*;

#[component(props(), ext(style, children, accessibility))]
#[derive(PartialEq, Clone)]
pub fn Group() -> impl IntoElement {
    let hovered = use_state(|| false);
    let focused = use_state(|| false);

    // `State<T>` is `Copy`, so this is free.
    let h_for_ctx = hovered;
    let f_for_ctx = focused;

    use_provide_context(move || GroupState {
        hovered: h_for_ctx,
        focused: f_for_ctx,
    });

    let mut h_enter = hovered;
    let mut h_leave = hovered;

    Div()
        .merge_style(style.clone())
        .accessibility(accessibility.clone())
        .on_pointer_enter(move |_| h_enter.set(true))
        .on_pointer_leave(move |_| h_leave.set(false))
        .children(children.clone())
}
