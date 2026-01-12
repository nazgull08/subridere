//! Event-driven UI pattern for Bevy using Observers.
//!
//! # Пример
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use bevy_ui_actions::prelude::*;
//!
//! struct MyAction;
//!
//! impl UiAction for MyAction {
//!     fn execute(&self, world: &mut World) {
//!         info!("Clicked!");
//!     }
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Node::default()).with_children(|parent| {
//!         parent.spawn_action_button(MyAction, "Click me");
//!     });
//! }
//! ```

mod action;
mod builder;
mod button;
mod observer;
mod plugin;
mod style;

pub mod prelude;

pub use action::UiAction;
pub use builder::{ButtonConfig, SpawnActionButton};
pub use button::{ActionButton, OnHover, OnPress};
pub use plugin::UiActionsPlugin;
pub use style::ButtonStyle;
