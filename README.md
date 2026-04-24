## LGPL-2.1 Compliance Note

If you statically link this crate and distribute the binary, LGPL-2.1 requires
that you also provide the object files for relinking.

**What you need to do:**

- Locate `target/release/deps/libgukasha_rustrade-*.rlib` after building
- Distribute this `.rlib` file alongside your binary (e.g., in a `lib/` folder)
- Or, switch to dynamic linking by setting `crate-type = ["cdylib"]` in your `Cargo.toml`

**Most users choose dynamic linking**, which completely removes this obligation.