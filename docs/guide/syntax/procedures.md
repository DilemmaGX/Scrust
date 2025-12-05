# Procedures (My Blocks)

Procedures (also known as "My Blocks" in Scratch) allow you to create custom blocks to reuse code.

## Defining a Procedure

Use the `proc` keyword to define a new procedure. You can define input parameters with types.

```rust
proc jump(height: number) {
    change_y_by(height);
    wait(0.1);
    change_y_by(0 - height);
}
```

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
proc jump(height: number) {
    change_y_by(height);
    wait(0.1);
    change_y_by(0 - height);
}
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
define jump (height)
change y by (height)
wait (0.1) seconds
change y by ((0) - (height))
</pre>
</div>
</div>

## Calling a Procedure

You can call a procedure just like any other function.

```rust
jump(10);
```

<pre class="blocks">
jump::custom (10)
</pre>

## Parameter Types

Supported types for parameters:
- `number`: Number input
- `string`: String input
- `boolean`: Boolean input

```rust
proc log_message(msg: string, is_error: boolean) {
    if is_error {
        say(msg);
    }
}
```

<pre class="blocks">
define log_message (msg) &lt;is_error&gt;
if &lt;is_error&gt; then
    say (msg)
end
</pre>

## Screen Refresh (Warp)

By default, procedures run **with screen refresh** (normal speed). This allows you to see animations and movements within the procedure.

You can control this behavior using attributes:

- `#[warp]`: Runs the procedure **without screen refresh** (Turbo Mode/Atomic). Useful for fast calculations or drawing.
- `#[nowarp]`: Runs the procedure **with screen refresh** (Default). Useful if you want to be explicit.

```rust
// Runs instantly (without screen refresh)
#[warp]
proc calculate_pi() {
    // ... complex math ...
}

// Runs normally (with screen refresh) - Default behavior
#[nowarp]
proc animate_movement() {
    repeat(10) {
        move_steps(10);
        wait(0.1);
    }
}
```

<div class="comparison">
<div>
<h4>#[warp]</h4>
<pre class="blocks">
define calculate_pi
...
</pre>
<p><i>(Run without screen refresh checked)</i></p>
</div>
<div>
<h4>Default / #[nowarp]</h4>
<pre class="blocks">
define animate_movement
...
</pre>
<p><i>(Run without screen refresh unchecked)</i></p>
</div>
</div>

## Returning Values

You can define a return type for a procedure and return values from it.

> [!WARNING]
> **Extension Feature**
> Returning values from procedures is **not supported in vanilla Scratch 3.0**. This feature uses custom opcodes (`procedures_return`) that work in **TurboWarp** and other modified Scratch environments. If you load the project in vanilla Scratch, these blocks may appear as undefined.

```rust
proc add(a: number, b: number) -> number {
    return a + b;
}

// Usage
say(add(10, 20));
```

<pre class="blocks">
define add (a) (b)
return ((a) + (b))

say (add (10) (20) :: custom)
</pre>

## Custom Block Formatting

You can customize the text and layout of your procedure block using the `#[format(...)]` attribute. This allows you to create blocks that read more like natural sentences, just like built-in Scratch blocks.

### Basic Formatting

Use the `#[format]` attribute with a string containing `{}` placeholders. The arguments following the string match the procedure parameters.

```rust
#[format("say {} to {}", message, target)]
proc greet(target: string, message: string) {
    say(join(message, join(", ", target)));
}
```

<pre class="blocks">
define say (message) to (target)
say (join (message) (join [, ] (target)))
</pre>

### Unused Parameters

Any parameters defined in the procedure but not included in the `#[format]` string will be automatically appended to the end of the block.

```rust
#[format("update score: {}", score)]
proc update(score: number, hidden_param: boolean) {
    // ...
}
```

<pre class="blocks">
define update score: (score) &lt;hidden_param&gt;
</pre>

### Combining with Warp

You can use `#[format]` together with `#[warp]` or `#[nowarp]` to control both appearance and execution speed.

```rust
#[format("{} + {}", a, b)]
#[warp]
proc fast_add(a: number, b: number) -> number {
    return a + b;
}
```

## Comprehensive Example

Here is a complete example demonstrating various procedure features, including custom formatting, warp modes, and parameter handling.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
#[format("{} + {}", a, b)]
#[warp]
proc add(a: number, b: number) -> number {
    return a + b;
}

#[format("say {} to {}", msg, name)]
#[nowarp]
proc greet(name: string, msg: string) {
    say(join(msg, join(", ", name)));
}

#[format("param2: {}", p2)]
proc partial_format(p1: number, p2: number) {
    say(join("p1: ", join(p1, join(", p2: ", p2))));
}

proc no_format(val: string) {
    say(join("Default: ", val));
}

#[on_flag_clicked]
fn main() {
    say(add(1, 2));
    greet("World", "Hello");
    partial_format(10, 20);
    no_format("test");
}
```

</div>
<div>
<h4>Scratch</h4>

<pre class="blocks">
define (a) + (b)
return ((a) + (b))

define say (msg) to (name)
say (join (msg) (join [, ] (name)))

define param2: (p2) (p1)
say (join [p1: ] (join (p1) (join [, p2: ] (p2))))

define no_format (val)
say (join [Default: ] (val))

when flag clicked
say ((1) + (2) :: custom)
say [Hello] to [World] :: custom
param2: (20) (10) :: custom
no_format [test] :: custom
</pre>

</div>
</div>

<style>
.comparison {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin-bottom: 20px;
    align-items: start;
}
.comparison h4 {
    margin-top: 0;
}
</style>
