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
pub mod territory;
mod tests;

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

