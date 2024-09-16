# Basic Rust Code Logic Analyzer

## Overview
The Rust Code Logic Analyzer is a tool designed to analyse Rust programmes for logical errors and assist programmers during development.
This analyser will focus specifically on basic data types and control flow, rather than attempting to fully verify large codebases.
The goal is to provide useful insights that help developers catch common issues early in the development process.

## Features

- **Code Parsing**: Parses Rust source code into an abstract syntax tree (AST).
- **Logical Analysis**: Identifies common logical errors, such as uninitialized variables and incorrect conditionals.
- **Automated Verification**: Integrates formal verification techniques to ensure code correctness.
- **Reporting**: Generates reports detailing findings and suggestions.

## Requirements

- Rust 1.XX or higher
- `Cargo` (Rust's package manager and build system)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Catapult2297/3931-Individual-Project.git
   cd rust-code-logic-analyzer
   ```
2. Build the project:
   ```bash
   cargo build
   ```
## Acknowledgments
- [Rust Language](https://www.rust-lang.org/)
- [syn](https://crates.io/crates/syn) for parsing Rust code
- [serde](https://crates.io/crates/serde) for serialization 

## Contact
For questions or feedback, please contact sc21wfc@leeds.ac.uk.
