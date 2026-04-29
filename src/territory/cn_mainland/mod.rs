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
    /// let code = HsCode::new(b"1234567890");
    /// assert_eq!(code.last_two_digits(), Some(&b"90"[..]));
    ///
    /// let short_code = HsCode::new(b"123456");
    /// assert_eq!(short_code.last_two_digits(), None);
    /// ```
    pub fn last_two_digits(&self) -> Option<&[u8]> {
        if validate_is_10_digits(self) {
            return Some(&self.iter().as_slice()[self.len() - 2..]);
        }
        None
    }
}
