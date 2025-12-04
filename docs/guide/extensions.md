# Extensions

::: warning EXPERIMENTAL FEATURE
Extension support in Scrust is currently **highly unstable** and intended for **experimental use only**.

Only the **Pen** and **Music** extensions have been tested. Use other extensions at your own risk.
:::

Scrust supports Scratch extensions (both official and custom/TurboWarp) via a flexible plugin system. This allows you to use blocks from extensions like Pen, Music, or any community-created extension by defining them in TOML configuration files.

## Using Extensions

To use an extension in your project, you need to:

1.  Ensure the extension definition file (`.toml`) exists.
2.  Add the extension to your `scrust.toml` configuration file.

### Configuration in scrust.toml

In your `scrust.toml`, you can configure extensions in the `extensions` field under the `[project]` section. There are two ways to do this:

#### 1. Simple List (Built-in Extensions)

For extensions that are already known to Scrust (like `pen` or `music`) or located in the default `extensions/` directory, you can simply list their IDs:

```toml
[project]
name = "my-project"
output = "dist/project.sb3"
extensions = ["pen", "music"]
```

#### 2. Detailed Configuration (Custom/Local Extensions)

For custom extensions, you can provide a detailed configuration to specify the definition file and the source URL. This is done by adding an inline table to the `extensions` list.

```toml
[project]
name = "my-project"
output = "dist/project.sb3"
extensions = [
    "pen",
    "music",
    { name = "Comment Blocks", id = "lmscomments", source = "https://extensions.turbowarp.org/Lily/CommentBlocks.js", definition = "extensions/lmscomments.toml" },
    { name = "RegExp", id = "truefantomregexp", source = "extensions/regexp.js", definition = "extensions/truefantomregexp.toml" }
]
```

-   **name**: The display name of the extension (optional).
-   **id**: The extension ID (optional, can be inferred from definition).
-   **source**: The URL where the extension source code is hosted.
    -   If this is a **web URL** (http/https), it will be added to the `extensionURLs` list in the project.
    -   If this is a **local file path**, the file content will be Base64 encoded and embedded as a **Data URI** in `extensionURLs`. This allows you to distribute projects with self-contained custom extensions.
-   **definition**: Path to the TOML definition file that maps Scrust functions to opcodes.

## Extension Definitions

Extensions are defined using TOML files. Each file maps Scrust function names to Scratch opcodes.

### File Structure

A typical extension definition looks like this:

```toml
name = "Extension Name"
id = "extensionId"

[blocks.function_name]
opcode = "extensionId_blockOpcode"
block_type = "command"
inputs = { "ARG_NAME" = { arg = 0 } }
```

-   **name**: The display name of the extension.
-   **id**: The unique ID of the extension.
-   **blocks**: A table mapping Scrust function names to block definitions.

### Block Definition

Each block definition requires an `opcode` and optional `inputs`, `fields`, or `block_type`.

-   **opcode**: The internal opcode used by Scratch/TurboWarp for this block.
-   **block_type**: The type of the block. Supported values are:
    -   `"command"` (default): A stack block that performs an action.
    -   `"reporter"`: A block that returns a value.
    -   `"boolean"`: A block that returns a boolean value (true/false).
    -   `"hat"`: A block that starts a script.
    -   `"c_block"`: A C Block (like `if` or `repeat`) that wraps other blocks.
-   **inputs**: Maps Scratch argument names (e.g., `MESSAGE`, `STEPS`) to function arguments.
    -   `{ arg = 0 }` maps to the first argument of the Scrust function.
    -   `{ arg = 1 }` maps to the second argument, and so on.
-   **fields**: Maps dropdown menu fields to specific values.

### Using Hat Blocks

If an extension defines a `hat` block, you can use it as an attribute on a function. For example, if you have a Hat block named `when_received` in your extension:

```toml
[blocks.when_received]
opcode = "extension_whenReceived"
block_type = "hat"
inputs = { "MESSAGE" = { arg = 0 } }
```

You can use it in Scrust like this:

```rust
#[when_received("message1")]
fn on_message() {
    // ...
}
```

## Built-in vs Custom Extensions

-   **Built-in Extensions**: Extensions like **Pen** and **Music** are included by default.
-   **Custom Extensions**: You can add support for any TurboWarp extension by creating a new `.toml` file and configuring it in `scrust.toml`.

## Compatibility Warning

When building a project that uses extensions other than the official Scratch extensions, Scrust will display a warning:

```plaintext
Warning: Project contains extensions not supported by vanilla Scratch. It may only run on TurboWarp or compatible mods.
```

This is purely informational and does not prevent the project from building. It serves as a reminder that the generated `.sb3` file might not load or function correctly in the standard Scratch editor.

**Official Scratch Extensions (No Warning):**
- `pen`
- `music`

Any extension not in this list, or any extension loaded from a custom URL, will trigger the compatibility warning.

## Credits

Special thanks to the authors of the extensions used in the examples:

-   **Comment Blocks** by [@LilyMakesThings](https://scratch.mit.edu/users/LilyMakesThings/)
-   **RegExp** by [@TrueFantom](https://scratch.mit.edu/users/TrueFantom/)
