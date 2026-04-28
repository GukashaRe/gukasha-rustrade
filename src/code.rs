use crate::*;
use std::fmt;
use std::str::FromStr;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct HsCode(Vec<u8>);

impl fmt::Display for HsCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0.iter().map(|&d| (d + b'0') as char).collect();
        write!(f, "{}", s)
    }
}

/// Enables parsing an `HsCode` from a string using the `parse()` method.
///
/// # Example
/// ```
/// use gukasha_rustrade::HsCode;
/// use std::str::FromStr;
///
/// let code = HsCode::from_str("01012100").unwrap();
/// ```
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
    /// ```
    /// Creates an `HsCode` from a string, panicking on invalid input.
    ///
    /// Use this only when the input is guaranteed to be valid.
    /// For fallible parsing, use `try_new_from_str` or `FromStr::from_str`.
    pub fn new_from_str(input: &str) -> Self {
        Self::try_new_from_str(input).unwrap()
    }

    /// Returns the chapter number (first two digits) as a `u8`.
    ///
    /// # Example
    /// ```
    /// # use gukasha_rustrade::HsCode;
    /// let code = HsCode::new_from_str("01012100");
    /// assert_eq!(code.get_chapter(), 1);
    /// ```
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

    /// Returns the total number of digits in this HS code.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// HS codes are never empty. This method always returns `false`.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns `true` if this HS code has the standard 6-digit international length.
    pub fn is_six_digit(&self) -> bool {
        self.len() == 6
    }

    /// Returns `true` if this HS code has the full 10-digit China-specific length.
    pub fn is_ten_digit(&self) -> bool {
        self.len() == 10
    }

    /// Returns an iterator over the digits as `u8` values.
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }

    /// Returns an iterator over the digits as `char`s (e.g., '0'..'9').
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().map(|x| (x + b'0') as char)
    }
}
