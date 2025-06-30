# 📚 `runs` – Run Rust Doctests on Standalone Files

[![Crates.io](https://img.shields.io/crates/v/runs.svg)](https://crates.io/crates/runs)
[![Documentation](https://docs.rs/runs/badge.svg)](https://docs.rs/runs)
[![License](https://img.shields.io/crates/l/runs.svg)](https://opensource.org/licenses/MIT)

**`runs`** lets you execute Rust documentation tests (doctests) on a **standalone `.rs` file**, even if it's not part of a full Cargo project.

> Normally, `cargo test --doc` works only inside valid Cargo projects. `runs` bridges this limitation.

---

## ✨ Features

- ✅ Accepts a standalone `.rs` file with doctests
- 🛠️ Automatically creates a temporary Cargo project
- 🧪 Runs `cargo test --doc` on the file
- 🧼 Cleans up the temp project after running
- 💡 Optional usage as a library crate

---

## 🔧 Installation

### Install from crates.io

```sh
cargo install runs
