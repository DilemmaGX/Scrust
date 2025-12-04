# Project Structure

A typical Scrust project has a simple and intuitive structure.

## File Layout

When you run `scrust create my_project`, it generates the following structure:

```
my_project/
├── assets/             # Store your images and sounds here
│   ├── sprite.svg
│   └── stage.svg
├── src/                # Your source code lives here
│   ├── stage.sr        # Code for the Stage
│   └── sprite.sr       # Code for your Sprite
├── dist/               # Compiled output (created after build)
│   └── project.sb3
└── scrust.toml         # Project configuration file
```

## scrust.toml

The configuration file `scrust.toml` defines your project's metadata and maps source files to Scratch targets (Stage and Sprites).

```toml
[project]
name = "my_project"
output = "dist/project.sb3"

[stage]
path = "src/stage.sr"

[[sprite]]
name = "Player"
path = "src/sprite.sr"

[[sprite]]
name = "Enemy"
path = "src/enemy.sr"
```

- **[project]**: General settings like project name and output path.
  - `name`: The name of the project. This determines the output filename (e.g., `my_project.sb3`).
  - `output`: The output directory or path. If a file extension is provided, the parent directory is used.
  - `extensions`: Extensions to enable. This can be a list of IDs (e.g., `["pen", "music"]`) or a detailed list of tables for custom extensions. See [Extensions](./extensions.md) for details.
- **[stage]**: Defines the source file for the Stage (Backdrop).
- **[[sprite]]**: Defines a sprite. You can have multiple `[[sprite]]` sections.

## Source Files (`.sr`)

Source files contain your Scrust code.

- **stage.sr**: Usually contains global variables, backdrop definitions, and global broadcast handling.
- **sprite.sr**: Contains code specific to a sprite (movement, looks, local variables).

## Assets

The `assets/` directory is the recommended place for SVGs, PNGs, and WAV/MP3 files. You reference these in your code using the `costume` and `sound` declarations.

```rust
costume "idle" "assets/idle.svg";
sound "pop" "assets/pop.wav";
```
