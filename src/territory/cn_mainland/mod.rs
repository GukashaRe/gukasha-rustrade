use crate::HsCode;

pub(crate) fn validate_is_10_digits(code: &HsCode) -> bool {
    code.is_ten_digit()
}

impl HsCode {
    /// 提取HS编码的最后两位数字（地区特供版本）
    ///
    /// 根据地区特供业务规则，仅当编码为完整的10位格式时，
    /// 才提取最后两位作为本国子目代码。
    ///
    /// # 返回值
    /// * `Some(&[u8])` - 编码为10位时，返回最后两个字节的切片
    /// * `None` - 编码不足10位时返回None
    ///
    /// # 示例
    /// ```
    /// let code = HsCode::new_from_str("1234567890");
    /// assert_eq!(code.last_two_digits(), Some(&[9, 0][..]));
    ///
    /// let short_code = HsCode::new_from_str("123456");
    /// assert_eq!(short_code.last_two_digits(), None);
    /// ```
    pub fn last_two_digits(&self) -> Option<&[u8]> {
        if validate_is_10_digits(self) {
            return Some(&self.iter().as_slice()[self.len() - 2..]);
        }
        None
    }
    /// 检查HS编码是否为纯数字格式
    ///
    /// 遍历编码的每个字节，确认所有字节均在数字0-9的范围内。
    ///
    /// # 返回值
    /// * `true` - 所有字符均为数字
    /// * `false` - 存在非数字字符
    ///
    /// # 示例
    /// ```
    /// let code = HsCode::new_from_str("1234567890");
    /// assert!(code.is_all_digits());
    ///
    /// let invalid = HsCode::new_from_str("12345ABCDE");
    /// assert!(!invalid.is_all_digits());
    /// ```
    pub fn is_all_digits(&self) -> bool {
        if validate_is_10_digits(self) {
            return self.iter().all(|x| (0..=9).contains(x));
        }
        false
    }
}
