# gukasha-rustrade

**A lightweight HS Code validation and lookup library for Rust, designed for trade and customs applications.**

[![Crates.io](https://img.shields.io/crates/v/gukasha-rustrade.svg)](https://crates.io/crates/gukasha-rustrade)
[![Documentation](https://docs.rs/gukasha-rustrade/badge.svg)](https://docs.rs/gukasha-rustrade)
[![License](https://img.shields.io/crates/l/gukasha-rustrade.svg)](https://github.com/GukashaRe/gukasha-rustrade/blob/main/LICENSE)

This library provides fast, zero‑cost HS Code validation and commodity description lookup, with a focus on trade and logistics automation. It is the only Rust crate dedicated to HS Code handling.

---

## ✨ Features

- ✅ Validate HS Code format (digits only, chapter 01–97)
- ✅ Support **6 to 14 even‑digit** codes (e.g. `010121`, `01012100`, `0101210012`)
- ✅ Extract chapter (`get_chapter()`)
- ✅ Compare two codes and get the first differing digit index (`diff()`)
- ✅ Lookup official commodity descriptions from a precompiled static map (`description()`)
- ✅ `FromStr` trait – parse directly from strings
- ✅ Zero runtime overhead – data is compiled into the binary
- ✅ No‑std ready (core only, with alloc)

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gukasha-rustrade = "0.1.10"
```

---

## 🚀 Quick Start

```rust
use gukasha_rustrade::HsCode;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from a string (fallible)
    let code = HsCode::try_new_from_str("01012100")?;

    // Or use FromStr
    let code = "01012100".parse::<HsCode>()?;

    // Extract chapter
    println!("Chapter: {}", code.get_chapter());   // 1

    // Look up commodity description
    println!("Description: {:?}", code.description());
    // Some("Purebred breeding horses")

    // Compare two codes
    let student = HsCode::new_from_str("01012800");
    let answer  = HsCode::new_from_str("01012100");
    println!("Diff indices: {:?}", student.diff(&answer));   // [5]

    Ok(())
}
```

---

## 📚 API Overview

| Method | Description |
|--------|-------------|
| `HsCode::try_new_from_str(input)` | Fallible constructor, returns `Result` |
| `HsCode::new_from_str(input)` | Panicking constructor (use only with trusted input) |
| `<HsCode as FromStr>::from_str` | `"010121".parse()` support |
| `code.get_chapter()` | Returns the first two digits as `u8` |
| `code.description()` | Returns commodity description for first 6 digits |
| `code.diff(&other)` | Returns indices where two codes differ |

---

## ⚖️ License & Compliance

This crate is licensed under the **LGPL-2.1-only**.

### LGPL-2.1 Compliance Note

If you statically link this crate and distribute the binary, LGPL-2.1 requires that you make the object files available for relinking.

**What you need to do:**

- Build your project, then locate:  
  `target/release/deps/libgukasha_rustrade-*.rlib`
- Distribute this `.rlib` file together with your binary (e.g., inside a `lib/` folder)
- End users can then relink your binary with a modified version of this crate

**Simpler option – use dynamic linking**

In your `Cargo.toml`, set:

```toml
[lib]
crate-type = ["cdylib"]
```

Dynamic linking **removes the obligation** to provide relinking artifacts.

> You are **not** required to open‑source your own application – only modifications to `gukasha-rustrade` itself must be released under LGPL-2.1.

---

## 🧪 Minimum Supported Rust Version (MSRV)

The minimum supported Rust version is **1.85** (2024 edition).

---

## 🤝 Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/GukashaRe/gukasha-rustrade).

- For new features, open an issue first to discuss.
- For bug fixes, include a test case that fails without the fix.

---

## 📄 License

This project is licensed under the **GNU Lesser General Public License v2.1 only** – see the [LICENSE](LICENSE) file for details.

---

## ⚠️ Technical Disclaimer

This library processes HS codes based on publicly available customs data. Region identifiers (`cn-mainland`, `tw`, `hk`, `mo`) are used solely to distinguish:

1. Different customs territories as defined by WTO agreements
2. Varying tariff extension rules and commodity descriptions
3. Technical implementation requirements for trade automation

No political position is expressed or implied. If you have concerns about terminology, please consider alternative solutions.

---
**gukasha** ── where Rust meets trade logistics.