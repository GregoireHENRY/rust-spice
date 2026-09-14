# Where the coverage work stands

Finished, and on `main` as of 1.0.0 (PR #22, merged).

## Position

| | routines |
| --- | --- |
| CSPICE N0067 routines that can actually be called | 644 |
| SpiceyPy 8.2.0 wraps | 626 |
| **rust-spice wraps** | **644** |
| left | 0 |

The headers declare 649 `*_c` functions. Four are private internals named `zz*`, and `prefix_c` is
declared but never compiled into the library, so it cannot be called at all.

Measured by `tools/`, not counted by hand. Every wrapped routine is called by a test. 241 tests,
`cargo fmt`, clippy with warnings denied, and the documentation build are all clean, with and
without the `lock` feature.

## What was left, and how it went

+ **The geometry finder.** Thirty routines. The five that take callbacks take them as plain
  `extern "C" fn` pointers: a Rust closure can only become a C function pointer if it captures
  nothing, so that is the honest signature. SpiceyPy needs a 205 line `callbacks.py` of `CFUNCTYPE`
  adapters for the same thing; here the compiler checks it. A capturing version would need a
  thread-local trampoline, which is worth avoiding until somebody asks.
+ **The events kernel.** Thirty seven routines, and volume rather than shape, as SpiceyPy's
  treatment suggested. Both of the shapes CSPICE offers are wrapped: a segment written a column at
  a time, and one written a record at a time.
+ **The last fifteen.** Array builders, the variadic extremes, `nthwd`, `prompt`, the command line
  pair and the two DAS character routines. `maxd` and friends are variadic, which Rust cannot hand
  a runtime-length list, so the wrappers fold the two argument form over the slice.

## Bugs the review step caught

+ `gfocce` was passing the observer where CSPICE wants the aberration correction: its `-Brief_I/O`
  section lists them in the opposite order from its own prototype.
+ `dasadc` was passing the number of lines where CSPICE wants the number of characters. Nothing had
  ever read the character data back; writing the `dasrdc` test is what found it.

## How to pick it up

`tools/README.md` has the workflow and, more usefully, the list of traps that the generator now
refuses to guess at. Each of those was a silent memory-corruption bug before it became a rule.

The one habit worth keeping: after generating a batch, read `tools/review.py` output against the
CSPICE `-Brief_I/O` sections before believing it. The compiler checks the arity of a call. It does
not check whether a pointer means one number or six, and that is where every real bug has been.
`tools/sync.py` now refuses to run when it meets a generating macro it has not been told about,
for the same reason: silence was the failure mode.
