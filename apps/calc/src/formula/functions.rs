//! Built-in spreadsheet functions (SUM, AVERAGE, IF, etc.).

use crate::cell::CellValue;

/// Registry of all built-in functions and their implementations.
pub fn builtin_function(name: &str, args: &[CellValue]) -> CellValue {
    match name.to_uppercase().as_str() {
        "SUM" => {
            let total: f64 = args
                .iter()
                .filter_map(|v| match v {
                    CellValue::Number(n) => Some(*n),
                    _ => None,
                })
                .sum();
            CellValue::Number(total)
        }
        "AVERAGE" => {
            let nums: Vec<f64> = args
                .iter()
                .filter_map(|v| match v {
                    CellValue::Number(n) => Some(*n),
                    _ => None,
                })
                .collect();
            if nums.is_empty() {
                CellValue::Error(crate::cell::CellError::DivisionByZero)
            } else {
                CellValue::Number(nums.iter().sum::<f64>() / nums.len() as f64)
            }
        }
        "COUNT" => {
            let count = args
                .iter()
                .filter(|v| matches!(v, CellValue::Number(_)))
                .count();
            CellValue::Number(count as f64)
        }
        "MIN" => {
            let min = args
                .iter()
                .filter_map(|v| match v {
                    CellValue::Number(n) => Some(*n),
                    _ => None,
                })
                .fold(f64::INFINITY, f64::min);
            CellValue::Number(min)
        }
        "MAX" => {
            let max = args
                .iter()
                .filter_map(|v| match v {
                    CellValue::Number(n) => Some(*n),
                    _ => None,
                })
                .fold(f64::NEG_INFINITY, f64::max);
            CellValue::Number(max)
        }
        _ => {
            log::warn!("Unknown function: {name}");
            CellValue::Error(crate::cell::CellError::ParseError)
        }
    }
}
