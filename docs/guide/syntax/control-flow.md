# Control Flow

Scrust supports standard control flow structures that map to Scratch's Control blocks.

## Loops

### Forever Loop

Runs the code inside indefinitely.

```rust
forever {
    move_steps(10);
    if_on_edge_bounce();
}
```

<pre class="blocks">
forever
    move (10) steps
    if on edge, bounce
end
</pre>

### Repeat Loop

Runs the code a specific number of times.

```rust
repeat(10) {
    change_size_by(10);
}
```

<pre class="blocks">
repeat (10)
    change size by (10)
end
</pre>

### Wait Until

Pauses the script until a condition is met.

```rust
wait_until(touching("edge"));
```

<pre class="blocks">
wait until &lt;touching [edge v]?&gt;
</pre>

### Repeat Until

Repeats the loop until a condition is true.

```rust
until touching("edge") {
    move_steps(10);
}
```

<pre class="blocks">
repeat until &lt;touching [edge v]?&gt;
    move (10) steps
end
</pre>

## Conditionals

### If Statement

```rust
if score > 10 {
    say("You win!");
}
```

<pre class="blocks">
if &lt;(score) > (10)&gt; then
    say [You win!]
end
</pre>

### If-Else Statement

```rust
if touching("mouse-pointer") {
    set_effect_to("brightness", 50);
} else {
    set_effect_to("brightness", 0);
}
```

<pre class="blocks">
if &lt;touching [mouse-pointer v]?&gt; then
    set [brightness v] effect to (50)
else
    set [brightness v] effect to (0)
end
</pre>

### Match Statement

The `match` statement allows you to compare a value against multiple patterns. It compiles to nested `if-else` blocks in Scratch.

```rust
match my_var {
    "A" => {
        say("Option A");
    }
    "B" => {
        say("Option B");
    }
    _ => {
        say("Default");
    }
}
```

<pre class="blocks">
if &lt;(my_var) = [A]&gt; then
    say [Option A]
else
    if &lt;(my_var) = [B]&gt; then
        say [Option B]
    else
        say [Default]
    end
end
</pre>

## Cloning

Creating and managing clones.

```rust
// Create clone of self
create_clone_of("myself");

// Delete this clone
delete_this_clone();
```

<pre class="blocks">
create clone of [myself v]
delete this clone
</pre>

## Waiting

Pause execution for a set amount of time.

```rust
wait(1.0); // Wait 1 second
```

<pre class="blocks">
wait (1.0) secs
</pre>
