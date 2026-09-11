# Where the coverage work stands

Branch `coverage`, on top of `main` at 0.8.0 (PR #21, merged).

## Position

| | routines |
| --- | --- |
| CSPICE routines that can actually be called | 648 |
| SpiceyPy 8.2.0 wraps | 626 |
| **rust-spice wraps** | **562** |
| left to match SpiceyPy | 64 |

Measured by `tools/`, not counted by hand. 216 tests, `cargo fmt`, clippy with warnings denied,
and the documentation build are all clean, with and without the `lock` feature.

## What is left

| family | count | shape of the work |
| --- | --- | --- |
| geometry finder `gf*` | 25 | 7 of them take C function pointers |
| events kernel `ek*` | 37 | plain wrappers, just volume |
| `repmct` | 1 | already written; listed here only because it was the gap the SpiceyPy comparison found |

### The callbacks

Seven `gf*` routines (`gfevnt`, `gffove`, `gfocce`, `gfudb`, `gfuds`, `uddc`, `uddf`) take C
function pointers. SpiceyPy solves this with a 205 line `callbacks.py` defining nine `CFUNCTYPE`
signatures plus adapters that wrap a Python callable.

Rust's equivalent is simpler and checked at compile time: take `extern "C" fn(..)` directly. The
one real constraint is that a Rust closure can only become a C function pointer if it captures
nothing, so the honest signature is a plain `extern "C" fn` rather than a closure. A capturing
version would need a thread-local trampoline, which is worth avoiding unless someone asks for it.

### The events kernel

Not a design problem, contrary to my first reading of it. SpiceyPy wraps all 37 as ordinary
functions, with the caller holding the cursor by row and column index. It is volume, not shape.

## How to pick it up

`tools/README.md` has the workflow and, more usefully, the list of traps that the generator now
refuses to guess at. Each of those was a silent memory-corruption bug before it became a rule.

The one habit worth keeping: after generating a batch, read `tools/review.py` output against the
CSPICE `-Brief_I/O` sections before believing it. The compiler checks the arity of a call. It does
not check whether a pointer means one number or six, and that is where every real bug has been.
