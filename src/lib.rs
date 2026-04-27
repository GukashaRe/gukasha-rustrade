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

use crate::HscodeError::{HsChapterError, HsCodeLenError, InputError};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/hs_data.rs"));

#[derive(Error, Debug)]
pub enum HscodeError {
    #[error("Invalid input format: non-digit character")]
    InputError,
    #[error("Invalid HS code length: expected 6-14 even digits, got {0}")]
    HsCodeLenError(usize),
    #[error("Chapter out of range: expected 1-97, got {0}")]
    HsChapterError(u8),
}

/// A validated HS Code stored as a vector of numeric bytes (0-9).
///
/// The inner `Vec<u8>` stores each digit as its numeric value for efficient
/// comparison and slicing. Use the provided constructors to create a valid `HsCode`.
///
/// # Example
/// ```
/// let code = HsCode::new_from_str("010121");
/// assert_eq!(code.get_chapter(), 1);
/// ```
#[derive(PartialOrd, PartialEq, Debug)]
pub struct HsCode(Vec<u8>);

impl fmt::Display for HsCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0.iter().map(|&d| (d + b'0') as char).collect();
        write!(f, "{}", s)
    }
}

impl FromStr for HsCode {
    type Err = HscodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new_from_str(s)
    }
}

/// Looks up a commodity description by its 6-digit HS code.
///
/// # Examples
///
/// ```
/// use gukasha_rustrade::lookup;
///
/// let desc = lookup("010121").unwrap();
/// assert_eq!(desc, "Horses; live");
/// ```
pub fn lookup(code: &str) -> Option<&'static str> {
    let key = if code.len() >= 6 { &code[..6] } else { code };
    HS_MAP.get(key).copied()
}

impl HsCode {
    /// Creates an `HsCode` from a string, panicking on invalid input.
    ///
    /// Use this only when the input is guaranteed to be valid.
    /// For fallible parsing, use `try_new_from_str` or `FromStr::from_str`.
    pub fn new_from_str(input: &str) -> Self {
        Self::try_new_from_str(input).unwrap()
    }

    /// Returns the chapter number (first two digits).
    pub fn get_chapter(&self) -> u8 {
        self.0[0] * 10 + self.0[1]
    }

    /// Attempts to parse an `HsCode` from a string.
    ///
    /// This is the main fallible constructor. It validates length, digit characters,
    /// and chapter range.
    pub fn try_new_from_str(input: &str) -> Result<Self, HscodeError> {
        verify_and_trans_hs_code(input).map(HsCode)
    }

    /// Returns the 0‑based indices where two HS Codes differ.
    ///
    /// Only valid positions in the shorter code are considered.
    ///
    /// # Example
    /// ```
    /// let a = HsCode::new_from_str("010121");
    /// let b = HsCode::new_from_str("010128");
    /// assert_eq!(a.diff(&b), vec![5]);
    /// ```
    pub fn diff(&self, other: &HsCode) -> Vec<usize> {
        self.0
            .iter()
            .zip(other.0.iter())
            .enumerate()
            .filter_map(|(idx, (x, y))| if x != y { Some(idx) } else { None })
            .collect()
    }

    /// Looks up the commodity description for the first 6 digits.
    ///
    /// The description comes from a precompiled static map generated from
    /// `data/harmonized-system.csv` at build time.
    /// Returns `None` if the 6‑digit prefix is not found.
    pub fn description(&self) -> Option<&'static str> {
        let key: String = self.0.iter().take(6).map(|&d| (d + b'0') as char).collect();
        HS_MAP.get(&key).copied()
    }
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
