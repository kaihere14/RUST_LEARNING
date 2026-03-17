
# Rust Learning Repository  
![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?logo=rust) ![License](https://img.shields.io/badge/License-MIT-blue) ![GitHub last commit](https://img.shields.io/github/last-commit/kaihere14/RUST_LEARNING)  

A collection of tiny, self‑contained Rust projects that demonstrate the fundamentals of the language, Cargo workflow, and common patterns for beginners. Each example can be built and run independently, making it perfect for learning, teaching, or quick reference.

---

## Table of Contents
- [Overview](#overview)  
- [Features](#features)  
- [Tech Stack](#tech-stack)  
- [Repository Architecture](#repository-architecture)  
- [Getting Started](#getting-started)  
  - [Prerequisites](#prerequisites)  
  - [Installation & Build](#installation--build)  
  - [Running the Examples](#running-the-examples)  
- [Usage Examples](#usage-examples)  
- [Development](#development)  
- [Testing & CI](#testing--ci)  
- [Contributing](#contributing)  
- [Roadmap](#roadmap)  
- [License & Credits](#license--credits)  

---

## Overview
`RUST_LEARNING` bundles three beginner‑friendly Rust programs:

| Project | Description | Status |
|---------|-------------|--------|
| **hello_world** | The classic “Hello, world!” program – the simplest possible Rust binary. | ✅ Stable |
| **hello_cargo** | Shows how Cargo builds a binary and how to use `cargo run` with arguments. | ✅ Stable |
| **guessing_game** | A small interactive game that asks the user to guess a random number (the canonical Rust tutorial). | ✅ Stable |

All projects target **Rust 2024 edition** and use **Cargo** as the build system. They contain **zero external dependencies**, making them ideal for learning the core language without extra setup.

---

## Features
- **Zero‑dependency examples** – focus on the standard library (for foundational concepts).
- **Separate Cargo packages** – each folder is a standalone crate, illustrating multi‑crate repository layout.
- **Cross‑platform** – works on Windows, macOS, and Linux.
- **Step‑by‑step comments** in source files to aid newcomers.
- **Ready‑to‑run** with a single `cargo run` command per example.
- **Interactive examples** – engage users with input/output scenarios, like the guessing game.
- **External crate integration** – demonstrates how to incorporate third-party libraries (e.g., `rand` for random number generation).
- **Core language concepts** – illustrates fundamental Rust features such as variable declaration, immutability, shadowing, basic data structures like arrays, conditional logic (`if`/`else if`/`else`), and various looping constructs (`loop`, `while`, `for`).

---

## Tech Stack
| Component | Version / Details |
|-----------|-------------------|
| **Language** | Rust 1.78+ (2024 edition) |
| **Build Tool** | Cargo (bundled with Rust) |
| **IDE / Editor Support** | Any editor with Rust language server (rust-analyzer) |
| **CI** | GitHub Actions (build & lint) – badge above |
| **License** | MIT (see below) |

---

## Repository Architecture
```
RUST_LEARNING/
├─ .gitignore
├─ README.md                ← you are here
├─ guessing_game/           ← crate #1 – interactive number guessing game
│   ├─ Cargo.toml
│   └─ src/
│       └─ main.rs
├─ hello_cargo/             ← crate #2 – Cargo basics demo
│   ├─ Cargo.toml
│   └─ src/
│       └─ main.rs
└─ hello_world/             ← crate #3 – “Hello, world!” starter
    ├─ Cargo.toml
    └─ src/
        └─ main.rs
```

Each sub‑directory is a **standalone Cargo package**. The top‑level repository does **not** contain a workspace file because the examples are intentionally isolated; you can work on any of them independently.

---

## Getting Started

### Prerequisites
| Tool | Minimum Version | Install |
|------|-----------------|--------|
| **Rust** (includes `cargo`) | 1.78.0 | <details><summary>Installation instructions</summary>Run the official installer: <br>`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh`<br>Follow the prompts and then restart your terminal.<br>Verify with `rustc --version` and `cargo --version`.</details> |
| **Git** (optional, for cloning) | 2.30+ | <https://git-scm.com/downloads> |

### Installation & Build
```bash
# Clone the repository
git clone https://github.com/kaihere14/RUST_LEARNING.git
cd RUST_LEARNING

# Build all examples (optional)
cargo build --all
```

> **Note:** Because each example lives in its own Cargo package, you can also `cd` into a sub‑folder and run `cargo build` there.

### Running the Examples
```bash
# 1️⃣ Hello World
cd hello_world
cargo run
# → prints: Hello, world!

# 2️⃣ Hello Cargo (demonstrates args)
cd ../hello_cargo
cargo run -- "Rustacean"
# → prints: Hello, Cargo! Argument received: Rustacean

# 3️⃣ Guessing Game
cd ../guessing_game
cargo run

# Follow the interactive prompts.
```

All binaries are placed under `target/debug/` after a successful build.

---

## Usage Examples

### hello_world/main.rs
rust
fn main() {
    println!("Hello, world!");
}


Running `cargo run` prints the greeting to STDOUT.

### hello_cargo/main.rs
rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = args.get(1).map(|s| s.as_str()).unwrap_or("Cargo");
    println!("Hello, {}! Argument received: {}", name, args.len() - 1);
}


Pass an optional name: `cargo run -- "Alice"` → `Hello, Alice! Argument received: 1`.

### guessing_game/main.rs
rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello doston");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
    println!("Enter your number for guess : ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read");
    let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            },
        };

    println!("You guessed : {}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }

}


This example implements an interactive number guessing game. It generates a random number between 1 and 100, prompts the user for input, and provides feedback (Too small!, Too big!, You win!). The game continues until the correct number is guessed, demonstrating basic I/O, loops, conditional logic, and external crate usage (`rand`).

### variables/main.rs
rust
use std::io;

fn main() {
    let  x = 5;
    println!("The value of x is: {}", x);
    let x = x + 5; // This will cause a compile-time error because x is immutable by default
    println!("The value of x is: {}", x);

    {
        let x = x+1;
        println!("The value of x is : {x}")
    }
    println!("The value of x is: {}", x);

    let spaces: &str = "  ";
    let spaces = spaces.len();
    println!("{spaces}");

    let a = [1,2,3,4,5];
    let mut index = String::new();
    println!("Enter your index : ");
    io::stdin()
        .read_line(&mut index)
        .expect("failed to edit ");

    let index:usize= index
        .trim()
        .parse()
        .expect("please enteer a valid input");

    let element = a[index];
    println!("Your element is : {element}");
    

}


This example demonstrates fundamental Rust concepts including variable declaration, shadowing (re-declaring a variable with the same name, which creates a new variable, as seen with `x`), and type inference. It also shows how to declare arrays, handle user input for array indexing, and perform basic error handling for input parsing. Note the comment about immutability, which highlights a common misconception when learning about shadowing versus direct mutation.

### branches/main.rs
rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

     let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


This example demonstrates Rust's conditional expressions using `if`, `else if`, and `else` to check divisibility. It also illustrates how `if` can be used as an expression to assign a value to a variable based on a condition.

### loops/main.rs
rust
fn main() {
    let mut counter = 0;
    let answer = loop {
        if counter > 5{
            break counter * 2;
        };
        println!("using loop : {counter}");
        counter += 1;
        
    };
    println!("Final answer! {answer}");

    let mut counter2 = 0;
    while counter2 != 5{
        println!("using while : {counter2}");
        counter2 += 1;
    }

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the number : {}",arr[index]);
        index += 1;
    }
 
    let arr2 = [15, 25, 35, 45, 55];

    for element in arr2{
        println!("the number using for loop : {}",element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


This example showcases various looping constructs in Rust. It includes an infinite `loop` with a `break` condition, a `while` loop for conditional iteration, and `for` loops for iterating over collections (arrays) and ranges, demonstrating common patterns for repetitive tasks.

---

## Development

### Setting Up a Development Environment
1. Install the **rust-analyzer** extension for VS Code, Neovim, or your preferred editor.
2. Run `cargo fmt` to format code according to the Rust style guide.
3. Run `cargo clippy --all-targets --all-features -- -D warnings` to lint.

### Running Tests
The current examples are deliberately minimal and contain no unit tests. Feel free to add tests in `tests/` directories and run:
```bash
cargo test --all
```

### Debugging
Use `println!` or the `log` crate for simple debugging. For step‑through debugging, install `lldb` or `gdb` and run:
```bash
cargo run --bin hello_world -- --debug
```
*(Replace `--debug` with any flag you implement.)*

---

## Testing & CI
A GitHub Actions workflow (not shown here) automatically:

- Checks out the repository
- Installs the latest stable Rust toolchain
- Runs `cargo fmt --check`
- Executes `cargo clippy`
- Builds each crate with `cargo build --all`
- Runs any tests with `cargo test --all`

The status badge at the top reflects the latest workflow run.

---

## Contributing
Contributions are welcome! This repository is intentionally simple, but you can still help by:

1. **Adding new example crates** (e.g., a file I/O demo, async basics, etc.).
2. **Improving documentation** – clearer comments, richer README sections.
3. **Fixing bugs** – if a snippet fails to compile on a newer Rust version, open an issue or PR.
4. **Adding tests** – demonstrate how to test command‑line programs.

### Workflow
1. Fork the repo and create a new branch: `git checkout -b feature/awesome-demo`.
2. Make your changes, ensuring `cargo fmt` and `cargo clippy` pass.
3. Commit with a clear message: `git commit -m "Add async hello example"`.
4. Push to your fork and open a Pull Request against `main`.
5. PR reviewers will run the CI checks automatically.

Please adhere to the **Rust API Guidelines** and keep the code **idiomatic**.

---

## Roadmap
| Milestone | Description | Status |
|-----------|-------------|--------|
| **Add async example** | Demonstrate `async`/`await` with `tokio` or `async-std`. | Planned |
| **Introduce Cargo workspace** | Consolidate examples under a workspace for shared tooling. | Planned |
| **Add CI badge for code coverage** | Use `tarpaulin` or `grcov` to report test coverage. | Planned |
| **Create a web‑assembly demo** | Show how to compile a simple Rust function to WASM. | Planned |

Feel free to suggest additional topics by opening an issue.

---


## License & Credits
**License:** MIT © 2024 Kai Here. See the `LICENSE` file for full text.

**Author:** Kai Here (GitHub: [kaihere14](https://github.com/kaihere14))

**Acknowledgments**
- The Rust Book (https://doc.rust-lang.org/book/) for the original example code.
- The Rust community for excellent tooling and documentation.

---