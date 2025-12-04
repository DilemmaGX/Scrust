# Standard Blocks

Scrust includes bindings for most standard Scratch blocks.

## Block Stacks

Statements placed consecutively form a single connected stack of blocks. To create separate, independent stacks (which will appear as floating scripts in Scratch), simply leave a blank line between them.

```rust
// Stack 1
move_steps(10);
turn_right(15);

// Stack 2 (Floating separately)
go_to(0, 0);
```

<pre class="blocks">
move (10) steps
turn right (15) degrees

go to x: (0) y: (0)
</pre>

## Motion

```rust
move_steps(10);
turn_right(15);
turn_left(15);

go_to(0, 0);
go_to("mouse-pointer"); // or "random-position"

glide(1.0, 0, 0);
glide_to(1.0, "mouse-pointer");

point_in_direction(90);
point_towards("mouse-pointer");

change_x_by(10);
set_x_to(0);
change_y_by(10);
set_y_to(0);

if_on_edge_bounce();
set_rotation_style("left-right");
```

<pre class="blocks">
move (10) steps
turn right (15) degrees
turn left (15) degrees

go to x: (0) y: (0)
go to [mouse-pointer v]

glide (1.0) secs to x: (0) y: (0)
glide (1.0) secs to [mouse-pointer v]

point in direction (90)
point towards [mouse-pointer v]

change x by (10)
set x to (0)
change y by (10)
set y to (0)

if on edge, bounce
set rotation style [left-right v]
</pre>

## Looks

```rust
say("Hello!");
say_for("Hello!", 2.0);
think("Hmm...");
think_for("Hmm...", 2.0);

switch_costume_to("costume1");
next_costume();
switch_backdrop_to("backdrop1");
next_backdrop();

change_size_by(10);
set_size_to(100);

show();
hide();

go_to_front_layer();
go_forward_layers(1);
```

<pre class="blocks">
say [Hello!]
say [Hello!] for (2.0) seconds
think [Hmm...]
think [Hmm...] for (2.0) seconds

switch costume to [costume1 v]
next costume
switch backdrop to [backdrop1 v]
next backdrop

change size by (10)
set size to (100) %

show
hide

go to [front v] layer
go [forward v] (1) layers
</pre>

## Sound

```rust
start_sound("Meow");
play_sound_until_done("Meow");
stop_all_sounds();

change_volume_by(-10);
set_volume_to(100);
```

<pre class="blocks">
start sound [Meow v]
play sound [Meow v] until done
stop all sounds

change volume by (-10)
set volume to (100) %
</pre>

## Sensing

```rust
touching("mouse-pointer");
touching_color("#ff0000");

distance_to("mouse-pointer");

ask_and_wait("What's your name?");
// answer() retrieves the answer

key_pressed("space");
mouse_down();
mouse_x();
mouse_y();

timer();
reset_timer();

current_year(); // and month, date, day_of_week, hour, minute, second
days_since_2000();
username();
```

<pre class="blocks">
&lt;touching [mouse-pointer v] ?&gt;
&lt;touching color [#ff0000] ?&gt;

(distance to [mouse-pointer v])

ask [What's your name?] and wait
(answer)

&lt;key [space v] pressed?&gt;
&lt;mouse down?&gt;
(mouse x)
(mouse y)

(timer)
reset timer

(current [year v])
(days since 2000)
(username)
</pre>

## Top-Level Blocks & Clustering

You can write blocks directly at the top level of a file (outside of any function or event). These blocks will be placed in the workspace but not attached to any Hat block (floating blocks).

- **Consecutive statements** are grouped into a single connected cluster.
- **Comments (`//!`)** can be used to separate clusters.

```rust
// Cluster 1
go_to(0, 0);
move_steps(10);

//! Break cluster

// Cluster 2
go_to(100, 100);
```

<pre class="blocks">
go to x: (0) y: (0)
move (10) steps

go to x: (100) y: (100)
</pre>
