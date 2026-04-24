//! # Gukasha Rustrade
//!
//! **Gukasha Rustrade** 是一个专注于**贸易与物流领域**的 Rust 工具箱。
//!
//! ## 当前能力
//!
//! *   HS Code (协调制度编码) 的校验与转换
//!
//! ## 未来规划 (Roadmap)
//!
//! 本项目不止于此，它旨在成为一个全面的贸易工具箱，未来计划加入：
//!
//! *   **更多编码标准**: 如 BOM (物料清单)、ECN (工程变更通知) 等。
//! *   **贸易单证处理**: 发票、装箱单、提单等关键数据的结构化抽取与验证。
//! *   **物流追踪**: 处理不同运输方式的追踪号验证和解析。
//! *   **国家/地区编码**: ISO 3166 国家的验证、名称转换等。
//!
//! ## 贡献
//!
//! 本项目处于极早期 (0.0.1)，欢迎提出建议或参与贡献！

use crate::HscodeError::{HsChapterError, HsCodeLenError, InputError};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HscodeError {
    #[error("输入格式错误")]
    InputError,
    #[error("位数输入错误，需要8/10位，但你却输入了 {0} 位")]
    HsCodeLenError(usize),
    #[error("章节范围 1-97 但意外得到了 {0} 章")]
    HsChapterError(u8),
}

pub struct HsCode(Vec<u8>);

impl fmt::Display for HsCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0.iter().map(|&d| (d + b'0') as char).collect();
        write!(f, "{}", s)
    }
}

impl HsCode {
    pub fn new_from_str(input: &str) -> Self {
        Self::try_new_from_str(input).unwrap()
    }

    pub fn get_chapter(&self) -> u8 {
        self.0[0] * 10 + self.0[1]
    }

    pub fn try_new_from_str(input: &str) -> Result<Self, HscodeError> {
        verify_and_trans_hs_code(input).map(HsCode)
    }
}
fn verify_and_trans_hs_code(input: &str) -> Result<Vec<u8>, HscodeError> {
    let bytes = input.as_bytes();
    if !(bytes.len() == 8 || bytes.len() == 10) {
        return Err(HsCodeLenError(bytes.len()));
    }
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return Err(InputError);
    }
    let chap: Vec<_> = bytes.iter().map(|b| b - b'0').collect();
    let chapter = chap[0] * 10 + chap[1];
    if chapter > 97 || chapter < 1 {
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
    fn test_invalid_length() {
        let result = HsCode::try_new_from_str("123");
        assert!(matches!(result, Err(HsCodeLenError(3))));
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
        // 第 1 章应该有效
        let result = HsCode::try_new_from_str("01012900");
        assert!(result.is_ok());

        // 第 97 章应该有效
        let result = HsCode::try_new_from_str("97012900");
        assert!(result.is_ok());

        // 第 98 章应该无效
        let result = HsCode::try_new_from_str("98012900");
        assert!(matches!(result, Err(HsChapterError(98))));
    }
}