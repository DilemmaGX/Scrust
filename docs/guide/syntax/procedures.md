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

You can define a return type for a procedure and return values from it. Scrust compiles this into standard Scratch blocks (using lists/variables), so it is fully compatible with vanilla Scratch 3.0.

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

## Comprehensive Example

Here is a complete example demonstrating various procedure features, including warp modes and parameter handling.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
#[warp]
proc add(a: number, b: number) -> number {
    return a + b;
}

#[nowarp]
proc greet(name: string, msg: string) {
    say(join(msg, join(", ", name)));
}

proc simple_proc(p1: number, p2: number) {
    say(join("p1: ", join(p1, join(", p2: ", p2))));
}

#[on_flag_clicked]
fn main() {
    say(add(1, 2));
    greet("World", "Hello");
    simple_proc(10, 20);
}
```

</div>
<div>
<h4>Scratch</h4>

<pre class="blocks">
define add (a) (b)
return ((a) + (b))

define greet (name) (msg)
say (join (msg) (join [, ] (name)))

define simple_proc (p1) (p2)
say (join [p1: ] (join (p1) (join [, p2: ] (p2))))

when flag clicked
say (add (1) (2) :: custom)
greet [World] [Hello] :: custom
simple_proc (10) (20) :: custom
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
