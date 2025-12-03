# Getting Started

Scrust is a programming language that compiles to Scratch 3.0 projects (.sb3). It offers a Rust-like syntax to write structured and maintainable code for Scratch.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/DilemmaGX/Scrust.git
   cd Scrust
   ```

## Creating a Project

The easiest way to start is using the `create` command:

```bash
cargo run -- create my_first_project
```

This generates a ready-to-use project structure.

### Project Structure

```
my_first_project/
├── scrust.toml         # Configuration
├── assets/             # SVG, PNG, WAV files
│   ├── sprite.svg
│   └── stage.svg
├── src/                # Source Code
│   ├── stage.sr
│   └── sprite.sr
└── dist/               # Compiled output
```

### Configuration (scrust.toml)

The `create` command generates this for you, but here is what it looks like:

```toml
[project]
name = "my_first_project"
output = "dist/project.sb3"

[stage]
path = "src/stage.sr"

[[sprite]]
name = "Sprite1"
path = "src/sprite.sr"
```

## Building

To compile your project, navigate to the project directory and run the build command:

```bash
cd my_first_project
cargo run --manifest-path ../Cargo.toml -- build
```

*(Note: If you have compiled Scrust into a binary and added it to your PATH, you can simply run `scrust build`)*

This will generate `dist/project.sb3`, which you can load into Scratch.
