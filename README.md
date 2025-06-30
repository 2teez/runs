# 📚 `runs` – Run Rust Doctests on Standalone Files

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
```
---

### Or from source

git clone https://github.com/2teez/runs.git

cd runs

cargo install --path .

### 🚀 Usage
> runs my_script.rs

This will:

  1.  Create a temp project: my_script_proj/

  2.  Copy my_script.rs into src/lib.rs

  3.  Generate Cargo.toml

  4.  Run cargo test --doc

  5.  Delete the project

  6.  Print results to your terminal

  ---

  📝 Example
  Given math.rs:

  /// Add two numbers.
  ///
  /// ```
  /// use math_proj::add;
  /// assert_eq!(add(2, 3), 5);
  /// ```
  pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
      a + b
  }
