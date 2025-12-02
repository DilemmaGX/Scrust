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
