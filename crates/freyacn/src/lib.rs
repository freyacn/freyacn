#![doc = include_str!("../README.md")]

extern crate self as freyacn;

pub mod cn;
pub mod components;
mod extensions;
mod icon_context;
mod theme;
mod utils;
mod variants;
pub use variants::Variant;

pub use crate::extensions::style::{Style, StyleExt};
pub use crate::theme::{
    Colors, Theme, get_cn_theme_or_default, use_cn_theme, use_init_cn_theme, use_provide_cn_theme,
};
pub use crate::utils::*;
pub use freya::prelude::{Button as ButtonPrimitive, StyleExt as _, Theme as _, *};
pub use freya_core::element::EventHandlerType;
pub use freya_core::events::name::EventName;
pub use freyacn_macros::*;
pub use icon_context::*;
pub use rustc_hash::FxHashMap;
