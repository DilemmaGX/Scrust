# My Blocks

| Scrust Syntax | Scratch Block | Notes |
| :--- | :--- | :--- |
| `proc name(arg: type, ...) { ... }` | <pre class="blocks">define name (arg) ...</pre> | |
| `name(arg, ...)` | <pre class="blocks">name (arg) ... :: custom</pre> | |
| `#[warp]` | <pre class="blocks">define name (arg) ...</pre> | Run without screen refresh (Turbo mode) |
| `#[nowarp]` | <pre class="blocks">define name (arg) ...</pre> | Run with screen refresh (Default) |
| `#[format("...", arg, ...)]` | <pre class="blocks">define ... (arg) ...</pre> | Custom block text formatting |
| `proc ... -> type { return val; }` | <pre class="blocks">return (val)</pre> | **TurboWarp Extension**: Return values |
