# Packages

Packages allow you to organize and reuse code across multiple sprites or projects. They function as libraries that can define procedures and extensions.

## Defining a Package

A package is defined in a `.sr` file (typically placed in a `packages/` directory). It must start with a `package` declaration block.

```rust
package math_utils {
    dependencies = [] // Optional: List of other packages this package depends on
}

// Define procedures as usual
proc add(a: number, b: number) -> number {
    return a + b;
}
```

### Package Structure

The `package` block supports the following fields:

- **dependencies**: A list of other package names that this package depends on. When you use this package, all of its dependencies will be automatically loaded.

## Using a Package

To use a package in your sprite or stage, you must first register it in your `scrust.toml` and then import it in your code.

### 1. Register in `scrust.toml`

Add the path to your package file in the `packages` list under the `[project]` section:

```toml
[project]
name = "my_game"
output = "dist/game.sb3"
packages = [
    "packages/math_utils.sr",
    "packages/logic.sr"
]
```

### 2. Import in Code

Use the `use` keyword to import the package in your `.sr` file.

```rust
use math_utils;
use logic;

#[on_flag_clicked]
fn start() {
    // Call package procedures using the package name as a prefix
    let sum = math_utils::add(10, 20);
    
    if logic::is_even(sum) {
        say("The sum is even!");
    }
}
```

## Dependencies

Packages can depend on other packages. For example, if `advanced_math` depends on `basic_math`, you only need to declare the dependency in `advanced_math`.

**packages/basic_math.sr**
```rust
package basic_math {
    // extensions = [] 
}

proc add(a: number, b: number) -> number {
    return a + b;
}
```

**packages/advanced_math.sr**
```rust
package advanced_math {
    extensions = ["return"],
    dependencies = ["basic_math"]
}

proc average(a: number, b: number) -> number {
    // We can use basic_math::add here because it's a dependency
    let sum = basic_math::add(a, b);
    return sum / 2;
}
```


