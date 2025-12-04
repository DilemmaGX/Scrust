# Scrust Syntax Guide

Scrust provides a modern syntax that compiles directly to Scratch blocks. This guide covers every aspect of the language, from basic variables to complex control flow.

## Design Philosophy

Scrust is designed to feel familiar to Rust developers while mapping 1:1 to Scratch's logic.

- **Explicit Visibility**: Variables are `private` (For this sprite only) or `public` (For all sprites) by default.
- **Structured Flow**: No more dangling blocks. `if`, `loop`, and functions provide structure.
- **Block Stacks**: Separate groups of blocks with blank lines to create multiple scripts.
- **Asset Integration**: Define your costumes and sounds right where you use them.

## Table of Contents

- [Variables & Lists](./variables) - Managing state with `var` and `list`.
- [Procedures](./procedures) - Creating custom blocks ("My Blocks").
- [Events](./events) - Handling flag clicks, key presses, and more.
- [Control Flow](./control-flow) - Loops (`forever`, `repeat`), Conditionals (`if/else`), and Matching (`match`).
- [Operators](./operators) - Math, logic, and string manipulation.
- [Standard Blocks](./blocks) - Motion, Looks, Sound, and Sensing commands.
