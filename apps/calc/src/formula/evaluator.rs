//! Formula evaluator — walks the AST and resolves cell references.

use crate::cell::CellValue;
use crate::formula::parser::FormulaNode;
use crate::grid::Grid;

/// Evaluate a parsed formula against the given grid, returning a `CellValue`.
pub fn evaluate(_node: &FormulaNode, _grid: &Grid) -> CellValue {
    // TODO: Implement recursive evaluation with dependency tracking.
    log::info!("Formula evaluator invoked (stub)");
    CellValue::Number(0.0)
}
