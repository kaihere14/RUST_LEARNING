
# 🦀 RUST_LEARNING 📚

![Rust Logo](https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/logos/rust-logo-blk.svg)

A curated collection of beginner-friendly Rust programming examples, designed to help new learners grasp fundamental concepts through practical, runnable code.

[![GitHub license](https://img.shields.io/github/license/kaihere14/RUST_LEARNING)](https://github.com/kaihere14/RUST_LEARNING/blob/main/LICENSE)
[![GitHub last commit](https://img.shields.io/github/last-commit/kaihere14/RUST_LEARNING)](https://github.com/kaihere14/RUST_LEARNING/commits/main)
[![Rust Edition](https://img.shields.io/badge/Rust%20Edition-2024-orange)](https://blog.rust-lang.org/2024/02/08/Rust-2024-Edition.html)

---
## 📖 Overview

This repository serves as a hands-on guide for learning the Rust programming language. It contains a series of small, self-contained projects, each focusing on a specific core concept of Rust. From the classic "Hello, World!" to more complex topics like control flow and external dependencies, these examples are structured to be easy to understand and experiment with.

**Why use this repository?**
*   **Practical Examples**: Each concept is demonstrated with runnable code.
*   **Step-by-Step Learning**: Projects are organized to build knowledge incrementally.
*   **Beginner-Friendly**: Designed for those new to Rust or programming in general.
*   **Modern Rust**: All examples use the latest Rust 2024 Edition.

**Target Audience**:
*   Beginners learning Rust.
*   Developers transitioning from other languages to Rust.
*   Anyone looking for quick, clear examples of Rust syntax and concepts.

**Current Status**: This repository is actively maintained with fundamental examples. New topics and refinements will be added over time.

---
## ✨ Features

Each directory in this repository represents a distinct Rust project, demonstrating a specific concept.

*   **`hello_world`**:
    *   **Description**: The quintessential "Hello, World!" program, showcasing the absolute basics of writing and running a Rust application.
    *   **Concepts**: `main` function, `println!` macro.
    *   **Status**: Stable

*   **`hello_cargo`**:
    *   **Description**: Introduces Cargo, Rust's build system and package manager, by creating and running a simple "Hello, World!" project using Cargo.
    *   **Concepts**: Cargo project creation, `cargo run`, `Cargo.toml`.
    *   **Status**: Stable

*   **`variables`**:
    *   **Description**: Explores Rust's approach to variables, including immutability by default, mutability (`mut`), constants, and shadowing.
    *   **Concepts**: `let`, `mut`, `const`, data types, shadowing.
    *   **Status**: Stable

*   **`branches`**:
    *   **Description**: Demonstrates conditional logic using `if`, `else if`, and `else` expressions.
    *   **Concepts**: `if/else` statements, boolean expressions.
    *   **Status**: Stable

*   **`loops`**:
    *   **Description**: Covers various looping constructs in Rust: `loop` (infinite loop), `while` loops, and `for` loops with iterators.
    *   **Concepts**: `loop`, `break`, `continue`, `while`, `for` with ranges and collections.
    *   **Status**: Stable

*   **`guessing_game`**:
    *   **Description**: A classic number guessing game that integrates user input, random number generation, conditional logic, and loops.
    *   **Concepts**: Input/output (`std::io`), external crates (`rand`), `match` expressions, error handling basics.
    *   **Status**: Stable

*   **`practice`**:
    *   **Description**: A dedicated space for general Rust exercises and experimentation.
    *   **Concepts**: Syntax practice, logic implementation.
    *   **Status**: Active

---
## 🛠️ Tech Stack

*   **Language**: [Rust](https://www.rust-lang.org/)
*   **Build System/Package Manager**: [Cargo](https://doc.rust-lang.org/cargo/)
*   **Rust Edition**: 2024

**Key Dependencies**:
*   **`rand`**: Used in the `guessing_game` project for generating random numbers.
    *   Version: `0.8.5`

---
## 🏗️ Architecture

This repository is structured as a collection of independent Rust projects within a single monorepo-like setup. Each top-level directory is its own Cargo package, complete with its own `Cargo.toml` file and `src/main.rs`.


RUST_LEARNING/
├── branches/             # Project demonstrating conditional logic
│   └── src/main.rs
├── guessing_game/        # Interactive guessing game
│   └── src/main.rs
├── hello_cargo/          # Basic Cargo project
│   └── src/main.rs
├── hello_world/          # First Rust program
│   └── src/main.rs
├── loops/                # Project demonstrating different loop types
│   └── src/main.rs
├── practice/             # General practice exercises
│   └── src/main.rs
└── variables/            # Project demonstrating variable concepts
    └── src/main.rs


This structure allows each example to be self-contained and runnable independently, making it easy to navigate and focus on specific concepts without interference from other projects.

---
## 🚀 Getting Started

To run any of the examples in this repository, you'll need to have the Rust toolchain installed.

### Prerequisites

*   **Rust Toolchain**:
    *   Install `rustup` (Rust installer) by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
    *   Ensure you have a recent version of Rust. You can check your version with:
        bash
        rustc --version
        cargo --version
        
    *   This repository uses the 2024 Edition, which requires a relatively recent Rust compiler. `rustup update` will ensure you have the latest stable toolchain.

### Installation

1.  **Clone the repository**:
    bash
    git clone https://github.com/kaihere14/RUST_LEARNING.git
    cd RUST_LEARNING
    

2.  **Navigate to a specific project**:
    Each example is a separate Cargo project. To work with one, `cd` into its directory. For example:
    bash
    cd guessing_game
    

### Configuration

These examples are simple and generally do not require any specific configuration beyond the default Cargo settings. If an example needed environment variables or configuration files, they would be documented within that project's specific section.

---
## 💡 Usage

To run any of the examples, navigate into its directory and use `cargo run`.

### Running `hello_world`

bash
cd hello_world
cargo run

**Expected Output**:

Hello, world!


### Running `variables`

bash
cd variables
cargo run

**Expected Output**: (Demonstrates variable values, mutability, and shadowing)

The value of x is: 5
The value of x is: 6
...


### Running `guessing_game`

bash
cd guessing_game
cargo run


### Running `practice`

bash
cd practice
cargo run
