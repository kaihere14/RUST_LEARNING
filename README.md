
# 🦀 Rust Learning Projects

A curated collection of beginner-friendly Rust examples demonstrating core programming concepts through practical, self-contained Cargo projects.

![Rust Logo](https://icons.veryicon.com/png/o/business/vscode-program-item-icon/rust-1.png)  
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

**Core Concepts Demonstrated**:
- Basic syntax (`hello_world`, `hello_cargo`)
- Variables and mutability (`variables`)
- Control flow (`branches`, `loops`)
- Input/output and randomness (`guessing_game`)
- String manipulation and ownership (`first_word`)
- Structs and methods (`structs`, `struct_implementation`)
- Enums and pattern matching (`enums`)
- Collections and key-value maps (`hashmap`)
- Error handling and file reading (`error_handling`)
- Generics and trait bounds (`Generics`)
- Traits and shared behavior (`traits`)
- Command-line task manager with task management features (`cli-task-manager`)
- Web server implementation with Actix Web (`web_server`)
- General practice (`practice`)

---
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
- `structs`: Prints user details using a custom struct
- `struct_implementation`: Calculates area of shapes using struct methods (`impl` block)
- `cli-task-manager`: Command-line task manager using subcommands to add, list, complete, and delete tasks.
  bash
  cargo run -- add "Buy groceries"
  cargo run -- list
  cargo run -- done 1
  cargo run -- delete 1
  

---
## 📦 Dependencies

- `rand` crate (v0.8.5) for random number generation in the guessing game
- `clap` crate for command-line argument parsing with subcommands in `cli-task-manager`
- `serde` and `serde_json` crates for task serialization/deserialization in `cli-task-manager`
- `thiserror` crate for custom error handling in `cli-task-manager`

---
## 📝 Contributing

This repository welcomes contributions that:
- Add new learning examples
- Improve existing implementations
- Follow Rust 2024 Edition guidelines

Examples should remain minimal and focused on single concepts with clear documentation.

---

## 📎 License

MIT License - see [LICENSE](LICENSE) file for details.