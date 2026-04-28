//! Undo / Redo history stack.

use crate::command::Command;

/// Manages a linear undo/redo history of commands.
pub struct UndoManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    /// Maximum number of undo steps to retain (memory cap).
    max_depth: usize,
}

impl UndoManager {
    pub fn new(max_depth: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_depth,
        }
    }

    /// Execute a command and push it onto the undo stack.
    pub fn execute(&mut self, mut cmd: Box<dyn Command>) {
        cmd.execute();
        self.undo_stack.push(cmd);
        // Any new action invalidates the redo history.
        self.redo_stack.clear();
        // Enforce memory cap.
        while self.undo_stack.len() > self.max_depth {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the most recent command.
    pub fn undo(&mut self) {
        if let Some(mut cmd) = self.undo_stack.pop() {
            cmd.undo();
            self.redo_stack.push(cmd);
        }
    }

    /// Redo the most recently undone command.
    pub fn redo(&mut self) {
        if let Some(mut cmd) = self.redo_stack.pop() {
            cmd.execute();
            self.undo_stack.push(cmd);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
