#[cfg(test)]
mod tests {
    use crate::HsCode;
    use crate::HscodeError::*;

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
