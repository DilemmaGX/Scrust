# Operators

Scrust provides a wide range of mathematical and logical operators.

## Math Operators

Standard arithmetic operators work as expected.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
1 + 2
1 - 2
1 * 2
1 / 2
random(1, 10)
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
(1) + (2)
(1) - (2)
(1) * (2)
(1) / (2)
pick random (1) to (10)
</pre>
</div>
</div>

## Comparison Operators

Used in `if` statements and loops.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
a > b
a < b
a == b
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
&lt;(a) &gt; (b)&gt;
&lt;(a) &lt; (b)&gt;
&lt;(a) = (b)&gt;
</pre>
</div>
</div>

## Logic Operators

Combine boolean conditions.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
a && b
a || b
!a
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
&lt;&lt;a&gt; and &lt;b&gt;&gt;
&lt;&lt;a&gt; or &lt;b&gt;&gt;
&lt;not &lt;a&gt;&gt;
</pre>
</div>
</div>

## String Operators

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
join("Hello ", "World")
join("A", "B", "C")
letter_of("Apple", 1)
length_of("Apple")
contains("Apple", "a")
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
join [Hello ] [World]
join [A] (join [B] [C])
letter (1) of [Apple]
length of [Apple]
&lt;[Apple] contains [a]?&gt;
</pre>
</div>
</div>

## Math Functions

Common math functions are available.

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
mod(10, 3); // Modulo
round(3.5); // Round
abs(-10);   // Absolute value
floor(3.9); // Floor
ceil(3.1);  // Ceiling
sqrt(9);    // Square root
sin(90);    // Sine
cos(0);     // Cosine
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
(10) mod (3)
(round (3.5))
([abs v] of (-10))
([floor v] of (3.9))
([ceiling v] of (3.1))
([sqrt v] of (9))
([sin v] of (90))
([cos v] of (0))
</pre>
</div>
</div>
