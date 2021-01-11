//! Decorate content and apply alignment.
use crate::Renderer;

pub use iced_graphics::block::{Style, StyleSheet};

/// An element decorating some content.
///
/// This is an alias of an `iced_native` container with a default
/// `Renderer`.
pub type Block<'a, Message> = iced_native::Block<'a, Message, Renderer>;
