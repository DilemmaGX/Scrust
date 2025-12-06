# Control Blocks

| Scrust Syntax | Scratch Block | Notes |
| :--- | :--- | :--- |
| `wait(seconds)` | <pre class="blocks">wait (seconds) seconds</pre> | |
| `repeat(times) { ... }` | <pre class="blocks">repeat (times)</pre> | |
| `forever { ... }` | <pre class="blocks">forever</pre> | |
| `if condition { ... }` | <pre class="blocks">if &lt;condition&gt; then</pre> | |
| `if condition { ... } else { ... }` | <pre class="blocks">if &lt;condition&gt; then ... else ... end</pre> | |
| `match expr { val => { ... } }` | <pre class="blocks">if &lt;(expr) = (val)&gt; then</pre> | Compiles to nested if-else |
| `wait_until(condition)` | <pre class="blocks">wait until &lt;condition&gt;</pre> | |
| `until condition { ... }` | <pre class="blocks">repeat until &lt;condition&gt;</pre> | |
| `stop("all")` | <pre class="blocks">stop [all v]</pre> | Options: "all", "this script", "other scripts in sprite" |
| `#[on_clone_start]` | <pre class="blocks">when I start as a clone</pre> | Attribute for function/procedure |
| `create_clone_of("sprite")` | <pre class="blocks">create clone of [sprite v]</pre> | |
| `delete_this_clone()` | <pre class="blocks">delete this clone</pre> | |
