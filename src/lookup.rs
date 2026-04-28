use crate::HS_MAP;

pub fn lookup(code: &str) -> Option<&'static str> {
    let key = if code.len() >= 6 { &code[..6] } else { code };
    HS_MAP.get(key).copied()
}
