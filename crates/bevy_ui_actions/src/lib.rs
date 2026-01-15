pub mod core;
pub mod interactions;
pub mod widgets;

mod plugin;
pub mod prelude;

// Re-export plugin
pub use plugin::UiActionsPlugin;

// Re-export core
pub use core::{ButtonConfig, ButtonStyle, SpawnActionButton, SpawnUiExt, UiAction};

// Re-export interactions
pub use interactions::{
    DragGhost, DragGhostStyle, DragPhase, DragState, Draggable, DropTarget, OnClick, OnDragCancel,
    OnDragStart, OnDrop, OnHover, OnHoverExit, OnPress, OnRightClick, PreviousInteraction,
};

// Re-export widgets
pub use widgets::{
    Active, Disabled, InteractiveVisual, ProgressBar, ProgressBarConfig, ProgressBarFill,
    SpawnProgressBarExt, Tab, TabContent, TabGroup, Tooltip, TooltipState, TooltipStyle, TooltipUI,
    VisualStyle,
};
