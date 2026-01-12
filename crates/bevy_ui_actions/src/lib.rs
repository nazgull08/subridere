//! Event-driven UI pattern for Bevy using Observers.
//!
//! # Problem
//!
//! Bevy UI code often suffers from "query hell" - adding buttons requires
//! adding Query parameters, leading to unmaintainable systems with dozens
//! of parameters.
//!
//! # Solution
//!
//! This library provides a trait-based approach using Bevy's Observer pattern
//! to create scalable, composable UI interactions.
//!
//! # Status
//!
//! **Currently under development.** API may change.
//!
//! # Example (planned)
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use bevy_ui_actions::prelude::*;
//!
//! struct IncrementAction;
//!
//! impl UiAction for IncrementAction {
//!     fn execute(&self, world: &mut World) {
//!         // Your game logic here
//!     }
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn_action_button(IncrementAction, "Increment");
//! }
//! ```

// Core abstractions
mod action;
mod button;
mod observer;
mod builder;
mod style;
mod plugin;

// Convenience re-exports
pub mod prelude;

// Top-level exports for direct usage
pub use action::UiAction;
pub use button::ActionButton;
pub use plugin::UiActionsPlugin;
pub use style::ButtonStyle;
