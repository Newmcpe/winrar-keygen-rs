# winrar-keygen-rs

WinRAR license key generator written in Rust.

Generates valid `rarreg.key` file contents for WinRAR 7.x using ECC/GF(2^(15*17)) cryptography — a pure Rust reimplementation of the keygen algorithm.

## Usage

```bash
cargo run --bin winrar-keygen -- "<Name>" "<License Type>" [output-file]
```

**Examples:**

```bash
# Print to stdout
cargo run --bin winrar-keygen -- "Your Name" "Single PC usage license"

# Save to file
cargo run --bin winrar-keygen -- "Your Name" "Single PC usage license" rarreg.key
```

**Output:**
```
RAR registration data
Your Name
Single PC usage license
UID=a1b2c3d4e5f6a7b8c9d0
6412212250d03d8f43f93cff00cf3a15b0ce94b666c37927...
```

## Installation

Place the generated `rarreg.key` in the WinRAR installation directory (usually `C:\Program Files\WinRAR`).

## Library

```toml
[dependencies]
winrar-keygen = { git = "https://github.com/Newmcpe/winrar-keygen-rs.git" }
```

```rust
use winrar_keygen::keygen::generate_license_text;

let key = generate_license_text("Your Name", "Single PC usage license");
// key contains the full rarreg.key content
```

## Credits

Based on the WinRAR keygen algorithm reverse-engineering by [BitCrackers](https://github.com/BitCrackers).
