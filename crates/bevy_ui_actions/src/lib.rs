//! Event-driven UI actions for Bevy.

mod action;
mod click;
mod drag;
mod helpers;
mod hover;
mod plugin;
mod right_click;
mod style;
mod systems;
mod tooltip; 
mod visual;

pub mod prelude;

pub use action::UiAction;
pub use click::OnClick;
pub use drag::{Draggable, DropTarget, OnDragStart, OnDrop, OnDragCancel, DragState};
pub use helpers::{ButtonConfig, SpawnActionButton, SpawnUiExt};
pub use hover::{OnHover, OnHoverExit, OnPress};
pub use plugin::UiActionsPlugin;
pub use right_click::OnRightClick;
pub use style::ButtonStyle;
pub use tooltip::{Tooltip, TooltipState, TooltipStyle, TooltipUI};
pub use visual::{InteractiveVisual, Disabled};
