# Assets & Management

Scrust allows you to define assets (Costumes and Sounds) directly in your source files (`.sr`), keeping resources close to the code that uses them.

## Declaring Assets

You can declare costumes and sounds at the top of your file.

### Costumes

```rust
// Syntax: costume "name" "path/to/file"
costume "idle" "assets/player_idle.svg";
costume "run" "assets/player_run.svg";
```

### Sounds

```rust
// Syntax: sound "name" "path/to/file"
sound "jump" "assets/jump.wav";
sound "bgm" "assets/music.mp3";
```

## Usage

Once declared, you can reference them by name in your code.

```rust
switch_costume_to("run");
start_sound("jump");
```

## Asset Location

The file paths are relative to the `scrust.toml` directory (project root). Ensure your `assets` folder is structured correctly.

```
project/
├── scrust.toml
├── assets/
│   ├── player_idle.svg
│   └── jump.wav
└── src/
    └── player.sr
```
