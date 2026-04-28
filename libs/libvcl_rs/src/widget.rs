//! Abstract widget trait for the retained-mode UI.
//!
//! Every visible element (buttons, text areas, toolbars) implements `Widget`.

use libtools_rs::Rect;
use crate::event::UiEvent;

/// The result of handling an event inside a widget.
pub enum EventResponse {
    /// The widget consumed the event.
    Handled,
    /// The widget ignored the event; propagate to the next handler.
    Ignored,
}

/// Trait implemented by every visible UI element.
pub trait Widget {
    /// Return the bounding rectangle in logical pixels.
    fn bounds(&self) -> Rect;

    /// Draw the widget into the current render pass.
    fn draw(&self);

    /// Handle an incoming UI event.
    fn handle_event(&mut self, event: &UiEvent) -> EventResponse;
}
