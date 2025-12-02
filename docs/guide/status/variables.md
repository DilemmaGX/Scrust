# Variables & Lists Blocks

## Variables

| Scrust Syntax | Scratch Block | Notes |
| :--- | :--- | :--- |
| `var = val` | <pre class="blocks">set [var v] to (val)</pre> | |
| `var += val` | <pre class="blocks">change [var v] by (val)</pre> | Also support `-=` |
| `show_variable("var")` | <pre class="blocks">show variable [var v]</pre> | |
| `hide_variable("var")` | <pre class="blocks">hide variable [var v]</pre> | |

## Lists

| Scrust Syntax | Scratch Block | Notes |
| :--- | :--- | :--- |
| `add_to_list(list, item)` | <pre class="blocks">add (item) to [list v]</pre> | |
| `delete_of_list(list, index)` | <pre class="blocks">delete (index) of [list v]</pre> | |
| `delete_all_of_list(list)` | <pre class="blocks">delete all of [list v]</pre> | |
| `insert_at_list(list, index, item)` | <pre class="blocks">insert (item) at (index) of [list v]</pre> | |
| `replace_item_of_list(list, index, item)` | <pre class="blocks">replace item (index) of [list v] with (item)</pre> | |
| `item_of_list(list, index)` | <pre class="blocks">item (index) of [list v]</pre> | |
| `item_num_of_list(list, item)` | <pre class="blocks">item # of (item) in [list v]</pre> | |
| `length_of_list(list)` | <pre class="blocks">length of [list v]</pre> | |
| `list_contains(list, item)` | <pre class="blocks">&lt;[list v] contains (item)?&gt;</pre> | |
| `show_list("list")` | <pre class="blocks">show list [list v]</pre> | |
| `hide_list("list")` | <pre class="blocks">hide list [list v]</pre> | |
