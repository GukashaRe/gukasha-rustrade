// Copyright (C) 2026 WFGukasha
//
// SPDX-License-Identifier: LGPL-2.1-only

//! # Gukasha Rustrade
//!
//! A Rust toolbox for trade and logistics, focusing on HS Code validation,
//! description lookup, and digit-level comparison.
//!
//! ## Current features
//! - HS Code validation (length: 6–14 even digits)
//! - Chapter extraction (first 2 digits)
//! - Diff: find differing digit indices between two codes
//! - Commodity description lookup (from precompiled HS table)
//! - `FromStr` trait for `"010121".parse()`

pub mod code;
pub mod lookup;

use crate::HscodeError::{HsChapterError, HsCodeLenError, InputError};
pub use code::HsCode;
use thiserror::Error;
include!(concat!(env!("OUT_DIR"), "/hs_data.rs"));

/// Errors that can occur during HS Code parsing and validation.
#[derive(Error, Debug)]
pub enum HscodeError {
    /// Input contains non-digit characters.
    #[error("Invalid input format: non-digit character")]
    InputError,
    /// HS Code length is invalid (expected 6-14 even digits).
    #[error("Invalid HS code length: expected 6-14 even digits, got {0}")]
    HsCodeLenError(usize),
    /// Chapter number (first two digits) is outside the valid range 1..=97.
    #[error("Chapter out of range: expected 1-97, got {0}")]
    HsChapterError(u8),
}

/// Converts an HS code string into a vector of numeric bytes.
///
/// # Rules
/// - Length must be between 6–14 digits (inclusive) and even.
/// - All characters must be ASCII digits.
/// - The first two digits (chapter) must be in 1..=97.
///
/// # Returns
/// - `Ok(Vec<u8>)` on success, with each element as the numeric value (0-9).
/// - `Err(HscodeError)` on failure.
pub(crate) fn verify_and_trans_hs_code(input: &str) -> Result<Vec<u8>, HscodeError> {
    let bytes = input.as_bytes();

    if bytes.len() < 6 || !bytes.len().is_multiple_of(2) || bytes.len() >= 16 {
        return Err(HsCodeLenError(bytes.len()));
    }
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return Err(InputError);
    }

    let chap: Vec<_> = bytes.iter().map(|b| b - b'0').collect();
    let chapter = chap[0] * 10 + chap[1];
    if !(1..=97).contains(&chapter) {
        return Err(HsChapterError(chapter));
    }

    Ok(chap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use code::*;

    #[test]
    fn test_valid_8_digit_hscode() {
        let code = HsCode::new_from_str("01012900");
        assert_eq!(code.get_chapter(), 1);
        assert_eq!(code.to_string(), "01012900");
    }

    #[test]
    fn test_valid_10_digit_hscode() {
        let code = HsCode::new_from_str("0101290010");
        assert_eq!(code.get_chapter(), 1);
        assert_eq!(code.to_string(), "0101290010");
    }

    #[test]
    fn test_valid_12_digit_hscode() {
        let code = HsCode::new_from_str("010121001012");
        assert_eq!(code.get_chapter(), 1);
    }

    #[test]
    fn test_invalid_length() {
        let result = HsCode::try_new_from_str("123");
        assert!(matches!(result, Err(HsCodeLenError(3))));
    }

    #[test]
    fn test_invalid_odd_length() {
        let result = HsCode::try_new_from_str("0101211");
        assert!(matches!(result, Err(HsCodeLenError(7))));
    }

    #[test]
    fn test_invalid_too_long() {
        let result = HsCode::try_new_from_str("0101210010121416");
        assert!(matches!(result, Err(HsCodeLenError(16))));
    }

    #[test]
    fn test_non_digit_input() {
        let result = HsCode::try_new_from_str("ABCDEFGH");
        assert!(matches!(result, Err(InputError)));
    }

    #[test]
    fn test_invalid_chapter() {
        let result = HsCode::try_new_from_str("9912345678");
        assert!(matches!(result, Err(HsChapterError(99))));
    }

    #[test]
    fn test_chapter_boundary() {
        let result = HsCode::try_new_from_str("01012900");
        assert!(result.is_ok());

        let result = HsCode::try_new_from_str("97012900");
        assert!(result.is_ok());

        let result = HsCode::try_new_from_str("98012900");
        assert!(matches!(result, Err(HsChapterError(98))));
    }

    #[test]
    fn test_diff() {
        let code1 = HsCode::try_new_from_str("01012900").unwrap();
        let code2 = HsCode::try_new_from_str("01012800").unwrap();
        assert_eq!(code1.diff(&code2), vec![5]);
    }

    #[test]
    fn test_fromstr() {
        let code = "10011001".parse::<HsCode>().unwrap();
        assert_eq!(code, HsCode::try_new_from_str("10011001").unwrap())
    }

    #[test]
    fn test_descrip() {
        let code = HsCode::new_from_str("01012100");
        assert!(code.description().is_some());
        assert_eq!(code.description().unwrap(), "Horses; live");
    }
}
