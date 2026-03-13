# Rust Learning Repository  

![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange) ![License](https://img.shields.io/badge/License-MIT-blue) ![Version](https://img.shields.io/badge/Version-0.1.0-lightgrey)  

A curated collection of tiny Rust projects that demonstrate the fundamentals of the language, Cargo workflow, and common patterns for beginners.  

---  

## Overview  

**RUST_LEARNING** is a sandbox of three self‑contained Rust examples:

| Example | Description |
|---------|-------------|
| **hello_world** | The classic “Hello, world!” program, showing the minimal Cargo project layout and binary configuration. |
| **hello_cargo** | A second “Hello, world!” project that demonstrates Cargo’s default binary discovery (no explicit `[[bin]]` entry needed). |
| **guessing_game** | A starter template for the popular “Guess the Number” tutorial – currently prints “Hello, world!” and is ready for you to extend. |

These projects are ideal for anyone who wants to get a Rust development environment up and running in under five minutes.  

---  

## Features  

- **Zero‑dependency examples** – each crate uses only the Rust standard library.  
- **Multiple Cargo configurations** – shows both implicit (`hello_cargo`) and explicit (`hello_world`) binary declarations.  
- **Ready‑to‑extend template** – `guessing_game` provides a clean entry point for adding user input, random number generation, and control flow.  
- **Cross‑platform** – works on Windows, macOS, and Linux as long as the Rust toolchain is installed.  

---  

## Tech Stack  

| Component | Version / Specification |
|-----------|--------------------------|
| **Language** | Rust 2024 edition (edition = `"2024"` in `Cargo.toml`) |
| **Package Manager / Build Tool** | Cargo (bundled with the Rust toolchain) |
| **IDE / Editor (optional)** | VS Code + Rust Analyzer, IntelliJ Rust, or any editor with Rust support |
| **Dependencies** | None (standard library only) |

---  

## Architecture  

The repository follows a flat, workspace‑free layout where each example lives in its own directory:

```
RUST_LEARNING/
├─ guessing_game/
│   ├─ Cargo.toml
│   └─ src/main.rs
├─ hello_cargo/
│   ├─ Cargo.toml
│   └─ src/main.rs
└─ hello_world/
    ├─ Cargo.toml
    └─ src/main.rs
```

Each crate is a **binary crate** (`[[bin]]` optional) that compiles to a single executable. No shared library code or workspace is required, keeping the learning curve minimal.  

---  

## Getting Started  

### Prerequisites  

1. **Rust toolchain** – install via [rustup](https://rustup.rs/):  

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # After installation, ensure the path is updated:
   source $HOME/.cargo/env
   ```  

2. **Cargo** – automatically installed with rustup. Verify the installation:

   ```bash
   rustc --version   # e.g., rustc 1.78.0 (2024‑xx‑xx)
   cargo --version   # e.g., cargo 1.78.0 (2024‑xx‑xx)
   ```

### Installation  

No global installation is required. Clone the repository and run the examples directly:

```bash
git clone https://github.com/kaihere14/RUST_LEARNING.git
cd RUST_LEARNING
```

### Running an Example  

Replace `<example>` with `hello_world`, `hello_cargo`, or `guessing_game`.

```bash
cd <example>
cargo run
```

**Example output (hello_world):**

```
   Compiling hello_world v0.1.0 (/path/to/RUST_LEARNING/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/hello_world`
Hello, world!
```

---  

## Usage  

### Basic Commands  

| Command | Description |
|---------|-------------|
| `cargo build` | Compile the crate without running it. |
| `cargo run`   | Build **and** execute the binary (default). |
| `cargo check` | Quickly verify the code compiles (no binary produced). |
| `cargo clean` | Remove the `target/` directory to free space. |

### Extending `guessing_game`  

The current `guessing_game/src/main.rs` contains a placeholder:

```rust
fn main() {
    println!("Hello, world!");
}
```

You can replace it with the classic guessing‑game logic:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

> **Note:** Add `rand = "0.8"` (or latest) to `guessing_game/Cargo.toml` under `[dependencies]` before running.

---  

## Development  

1. **Open the project** in your favorite editor.  
2. **Run tests** (none currently, but you can add `#[cfg(test)]` modules).  

   ```bash
   cargo test
   ```

3. **Formatting** – keep code tidy with `rustfmt`:

   ```bash
   cargo fmt
   ```

4. **Linting** – optional, use `clippy` for extra warnings:

   ```bash
   cargo clippy
   ```

---  

## Deployment  

These examples are meant for local learning and do not require deployment. If you wish to distribute the binaries:

```bash
cargo build --release
# The binary will be located at target/release/<crate_name>
```

You can then copy the binary to any machine that matches the target architecture.  

---  

## Contributing  

Contributions are welcome! Follow these steps:

1. **Fork** the repository.  
2. **Create a feature branch**:  

   ```bash
   git checkout -b feature/add-guessing-game-logic
   ```

3. **Make your changes** – e.g., add new examples, improve documentation, or flesh out the guessing game.  
4. **Run the test suite** (if you added tests).  
5. **Commit** with a clear message and **push** to your fork.  
6. Open a **Pull Request** against the `main` branch.  

Please adhere to the following guidelines:

- Use `rustfmt` formatting.  
- Keep the repository layout consistent (one crate per directory).  
- Update the README if you add new examples or change existing ones.  

---  

## License & Credits  

This repository is licensed under the **MIT License** – see the `LICENSE` file for details.  

**Author:** kaihere14  

**Acknowledgments**  

- The Rust community for the excellent documentation and tooling.  
- The official *“The Rust Programming Language”* book, which inspired the `guessing_game` example.  

---  

*Happy coding!*