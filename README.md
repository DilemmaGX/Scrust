![Scrust](assets/logo-banner.svg)

**Version: 0.1.5 (Alpha)**

[![Docs](https://img.shields.io/badge/docs-vitepress-blue)](https://dilemmagx.github.io/Scrust/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Scrust** is a compiled language for Scratch 3.0 projects. It brings modern programming features like functions, variables, and structured control flow to Scratch, compiling down to `.sb3` files.

> [!WARNING]
> **ALPHA STATUS**
> 
> This project is currently in **early alpha**. 
> - The syntax, API, and features are subject to breaking changes at any time.
> - It is **not stable** and should not be used for critical production projects.
> - Bugs and incomplete features are expected.

## Features

- **Modern Syntax**: Rust-inspired syntax with functions, variables, and blocks.
- **Compilation**: Compiles directly to standard `.sb3` Scratch project files.
- **Extension Support**: Built-in support for Pen and Music extensions.
- **Custom Extensions**: Load any TurboWarp extension from a URL or embed local JS files directly.
- **Project Management**: CLI tools to create, build, and manage projects.
- **Asset Management**: Easily handle sprites, costumes, and sounds.

## Getting Started

### Prerequisites

- **Rust**: You need to have Rust installed (latest stable version recommended).
- **Git**: To clone the repository.

### Installation

Clone the repository:

```sh
git clone https://github.com/DilemmaGX/Scrust.git
cd Scrust
```

### Usage

#### 1. Create a New Project

Use the `create` command to generate a new project structure:

```sh
cargo run -- create my_project
```

This will create a directory `my_project` with a default `scrust.toml` and source files.

#### 2. Build the Project

Navigate to your project directory and build it:

```sh
cd my_project
cargo run --manifest-path ../Cargo.toml -- build
```
> [!TIP]
> If you installed Scrust globally or added it to your PATH, you can simply run `scrust build`.

The compiled project will be in `dist/my_project.sb3` (named after your project in `scrust.toml`). You can load this file into the Scratch editor.

## Documentation

Full documentation is available at [https://dilemmagx.github.io/Scrust/](https://dilemmagx.github.io/Scrust/).

It includes:
- Getting Started Guide
- Language Syntax (Variables, Control Flow, Functions, etc.)
- Block Reference
- Extension Guide

## Contributing

Contributions are welcome! Please feel free to open issues or pull requests. Since this is an alpha project, please discuss major changes in an issue first.

## License

This project is licensed under the [MIT License](LICENSE).
