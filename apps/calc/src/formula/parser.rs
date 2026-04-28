//! Formula parser — converts a string like `=SUM(A1:A10)` into an AST.

use crate::cell::CellAddress;

/// A node in the formula abstract syntax tree.
#[derive(Debug, Clone)]
pub enum FormulaNode {
    /// A literal number.
    Number(f64),
    /// A literal string.
    Text(String),
    /// A reference to a single cell.
    CellRef(CellAddress),
    /// A range reference (top-left to bottom-right).
    Range(CellAddress, CellAddress),
    /// A function call with a name and arguments.
    Function {
        name: String,
        args: Vec<FormulaNode>,
    },
    /// A binary operation.
    BinaryOp {
        op: BinaryOp,
        left: Box<FormulaNode>,
        right: Box<FormulaNode>,
    },
    /// A unary negation.
    Negate(Box<FormulaNode>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

/// Parse a formula string (without the leading `=`) into an AST.
pub fn parse_formula(_input: &str) -> Result<FormulaNode, String> {
    // TODO: Implement recursive-descent parser.
    log::info!("Formula parser invoked (stub)");
    Ok(FormulaNode::Number(0.0))
}
