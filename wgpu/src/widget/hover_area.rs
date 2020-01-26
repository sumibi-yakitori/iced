//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
//!
//! [`Button`]: type.Button.html
//! [`State`]: struct.State.html
use crate::Renderer;

pub use iced_native::hover_area::State;
pub use iced_style::hover_area::{Style, StyleSheet};

/// A widget that produces a message when clicked.
///
/// This is an alias of an `iced_native` hover_area with an
/// `iced_wgpu::Renderer`.
pub type HoverArea<'a, Message> = iced_native::HoverArea<'a, Message, Renderer>;
