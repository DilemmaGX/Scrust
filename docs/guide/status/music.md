# Music Blocks

To use these blocks, enable the "music" extension in your `scrust.toml`: `extensions = ["music"]`.

| Scrust Syntax | Scratch Block | Notes |
| :--- | :--- | :--- |
| `play_drum(drum, beats)` | <pre class="blocks">play drum (drum) for (beats) beats</pre> | drum: 1=Snare, 2=Bass, etc. |
| `rest_for(beats)` | <pre class="blocks">rest for (beats) beats</pre> | |
| `play_note(note, beats)` | <pre class="blocks">play note (note) for (beats) beats</pre> | note: MIDI number (60=Middle C) |
| `set_instrument(inst)` | <pre class="blocks">set instrument to (inst)</pre> | inst: 1=Piano, etc. |
| `change_tempo_by(delta)` | <pre class="blocks">change tempo by (delta)</pre> | |
| `set_tempo_to(bpm)` | <pre class="blocks">set tempo to (bpm)</pre> | |
| `get_tempo()` | <pre class="blocks">(tempo)</pre> | Returns current tempo |
