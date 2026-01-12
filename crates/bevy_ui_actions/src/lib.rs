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
mod button;
mod observer;
mod builder;
mod style;
mod plugin;

pub mod prelude;

pub use action::UiAction;
pub use button::{ActionButton, OnHover, OnPress};
pub use builder::{ButtonConfig, SpawnActionButton, ActionButtonExt};
pub use plugin::UiActionsPlugin;
pub use style::ButtonStyle;
