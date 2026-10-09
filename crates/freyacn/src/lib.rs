#![doc = include_str!("../README.md")]

extern crate self as freyacn;

mod components;
mod extensions;
mod icon_context;
mod theme;
mod utils;

pub use crate::extensions::style::*;
pub use crate::theme::*;
pub use crate::utils::*;
pub use freya::prelude::{
    ChildrenExt, Color, Component, CornerRadius, CornerRadiusExt, DiffKey, Element,
    EventHandlersExt, Gaps, IntoElement, KeyExt, Size, StyleState, TextStyleData, provide_context,
};
pub use freya_core::element::EventHandlerType;
pub use freya_core::events::name::EventName;
pub use freyacn_macros::*;
pub use icon_context::*;
pub use rustc_hash::FxHashMap;
