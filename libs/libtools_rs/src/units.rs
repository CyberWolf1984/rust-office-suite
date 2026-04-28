//! Measurement unit conversions for document layout.
//!
//! Internally we work in **twips** (1/1440 inch) which is the same unit
//! LibreOffice uses, allowing lossless round-tripping of ODF/OOXML files.

/// One inch expressed in twips.
pub const TWIPS_PER_INCH: f64 = 1440.0;

/// One centimetre expressed in twips.
pub const TWIPS_PER_CM: f64 = TWIPS_PER_INCH / 2.54;

/// One point (1/72 inch) expressed in twips.
pub const TWIPS_PER_PT: f64 = TWIPS_PER_INCH / 72.0;

/// Convert twips to logical pixels at the given DPI.
pub fn twips_to_px(twips: f64, dpi: f64) -> f64 {
    twips * dpi / TWIPS_PER_INCH
}

/// Convert logical pixels to twips at the given DPI.
pub fn px_to_twips(px: f64, dpi: f64) -> f64 {
    px * TWIPS_PER_INCH / dpi
}

/// Convert points to twips.
pub fn pt_to_twips(pt: f64) -> f64 {
    pt * TWIPS_PER_PT
}

/// Convert centimetres to twips.
pub fn cm_to_twips(cm: f64) -> f64 {
    cm * TWIPS_PER_CM
}
