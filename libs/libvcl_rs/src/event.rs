//! High-level UI events translated from raw winit input.

/// Application-level UI events consumed by widgets and documents.
#[derive(Debug, Clone)]
pub enum UiEvent {
    /// A keyboard key was pressed.
    KeyPress { key: String, modifiers: Modifiers },
    /// A keyboard key was released.
    KeyRelease { key: String },
    /// The mouse cursor moved.
    MouseMove { x: f32, y: f32 },
    /// A mouse button was pressed.
    MouseDown { x: f32, y: f32, button: MouseButton },
    /// A mouse button was released.
    MouseUp { x: f32, y: f32, button: MouseButton },
    /// The window was resized.
    Resize { width: u32, height: u32 },
    /// A request to close the window.
    CloseRequested,
}

/// Modifier key state.
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
