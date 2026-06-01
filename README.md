
# RUST_LEARNING

A collection of beginner-friendly Rust examples demonstrating core programming concepts through practical, self-contained projects.

![Rust Logo](https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/logos/rust-logo-blk.svg)  
[GitHub license](LICENSE) | [GitHub commits](https://github.com/kaihere14/RUST_LEARNING/commits/main) | [Rust 2024 Edition](https://blog.rust-lang.org/2024/02/08/Rust-2024-Edition.html)

---

## 🧠 Key Concepts

This repository provides hands-on demonstrations of:
- Basic program structure and execution
- Rust's ownership model (immutability, mutability)
- Control flow (conditionals and loops)
- Input/output operations
- External crate integration (random number generation)
- Cargo project management

---

## 📁 Project Structure

Each directory contains a standalone Cargo project:


RUST_LEARNING/
├── hello_world/       # Simplest Rust program
├── hello_cargo/       # Cargo-based project setup
├── variables/         # Variable declaration and mutability
├── branches/          # Conditional logic (if/else)
├── loops/             # Loop constructs (loop, while, for)
├── guessing_game/     # Interactive game with I/O and rand
├── first_word/        # Extract the first word from a string
└── practice/          # General coding exercises


All projects include:
- `Cargo.toml` manifest file
- `src/main.rs` source code
- Runnable examples with clear outputs

---
## 🛠️ Requirements

- Rust 2024 Edition (requires Rust 1.75+)
- Cargo (Rust's build system and package manager)

Install via:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

---

## ▶️ Usage

1. Clone the repository:
bash
git clone https://github.com/kaihere14/RUST_LEARNING.git
cd RUST_LEARNING


2. Run a project:
bash
cd <project_name>
cargo run


**Example Outputs**:
- `hello_world`: `Hello, world!`
- `variables`: Demonstrates variable binding, mutability, and shadowing
- `guessing_game`: Interactive number guessing game with input validation
- `practice`: Demonstrates string length checking, conditional branching, and looping through indices
## 📦 Dependencies

- `rand` crate (v0.8.5) used in the guessing game for random number generation

---

## 📝 Contributing

This repository is actively maintained. Contributions of new examples or improvements to existing ones are welcome. Please ensure:
- Examples remain minimal and focused
- Concepts are clearly documented
- Code follows Rust 2024 Edition guidelines

---

## 📎 License

MIT License - see [LICENSE](LICENSE) file for details.