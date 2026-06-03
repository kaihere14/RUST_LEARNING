
# Rust Learning Projects

A curated collection of beginner-friendly Rust examples demonstrating core programming concepts through practical, self-contained Cargo projects.

![Rust Logo](https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/logos/rust-logo-blk.svg)  
[License](LICENSE) | [Commits](https://github.com/kaihere14/RUST_LEARNING/commits/main) | [Rust 2024 Edition](https://blog.rust-lang.org/2024/02/08/Rust-2024-Edition.html)

---

## 📌 Overview

This repository provides hands-on Rust learning material through 8 standalone projects, each focusing on specific programming concepts. All examples follow Rust 2024 Edition guidelines and include complete Cargo project structures with clear documentation.

---

## 📁 Project Structure

Each directory contains a complete Cargo project with:
- `Cargo.toml` manifest
- `src/main.rs` implementation
- Example output documentation


RUST_LEARNING/
├── hello_world/             # Basic Rust program structure
├── hello_cargo/             # Cargo project setup demonstration
├── variables/               # Variable binding and mutability
├── branches/                # Conditional logic (if/else)
├── loops/                   # Loop constructs (loop, while, for)
├── guessing_game/           # I/O and random number generation
├── first_word/              # String manipulation and ownership
├── practice/                # General coding exercises
├── structs/                 # Struct definition and instantiation
└── struct_implementation/   # Implementing methods on structs (impl block)

## 🛠️ Requirements

- Rust 2024 Edition (Rust 1.75+)
- Cargo (Rust's build system and package manager)

Install Rust with:
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
- `variables`: Demonstrates variable binding and shadowing
- `guessing_game`: Interactive number guessing game with input validation
- `practice`: Implements string length checking and index iteration
- `structs`: Prints user details (name, age, email) using a custom struct
- `struct_implementation`: Calculates and prints the area of shapes using struct methods (`impl` block)
## 📦 Dependencies

- `rand` crate (v0.8.5) for random number generation in the guessing game

---

## 📝 Contributing

This repository welcomes contributions that:
- Add new learning examples
- Improve existing implementations
- Follow Rust 2024 Edition guidelines

Please ensure examples remain minimal and focused on single concepts with clear documentation.

---

## 📎 License

MIT License - see [LICENSE](LICENSE) file for details.