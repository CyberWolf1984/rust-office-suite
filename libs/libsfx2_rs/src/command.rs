//! Command dispatch system.
//!
//! Every user action (typing, formatting, inserting) is modelled as a
//! `Command` that can be executed and then pushed onto the undo stack.

/// A reversible operation on a document.
pub trait Command: std::fmt::Debug {
    /// Human-readable name for the undo menu.
    fn description(&self) -> &str;

    /// Apply the operation to the document.
    fn execute(&mut self);

    /// Reverse the operation (undo).
    fn undo(&mut self);
}
