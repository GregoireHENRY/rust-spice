# Coverage tooling

Scripts used to wrap the rest of the CSPICE API. They exist because wrapping several hundred
routines by hand is slow and, more importantly, error prone in ways the compiler cannot see: a
pointer argument may be one number or six, and getting it wrong compiles cleanly and corrupts the
stack at run time.

All four need `CSPICE_DIR` set, and a `cargo build` to have run at least once so the bindings exist.

## The workflow

```bash
# 1. What can be generated safely, and what cannot.
CSPICE_DIR=~/cspice python3 tools/generate.py tools/spiceypy-coverage.json > /tmp/generated.rs

# 2. Paste a coherent family into rust-spice/src/core/raw.rs, then build.
cargo build -p rust-spice

# 3. Check the Rust outputs against what each routine documents.
CSPICE_DIR=~/cspice python3 tools/check.py /tmp/batch.json

# 4. Read the signatures next to the CSPICE -Brief_I/O sections, by eye.
CSPICE_DIR=~/cspice python3 tools/review.py /tmp/batch.json

# 5. Regenerate the re-export lists, the bindings table and the link definitions.
echo '{"name": "description", ...}' | python3 tools/sync.py

# 6. Write the tests, then:
cargo fmt --all && cargo clippy --all --all-targets -- --deny warnings && cargo test --all
```

`tools/spiceypy-coverage.json` lists the routines SpiceyPy 8.2.0 wraps, used as the target set.

`tools/bindings.py` is separate from that workflow. It copies the bindings `cspice-sys` generates
into `rust-spice/src/unlinked.rs`, which is what the `unlinked` feature puts behind `spice::c` so
that the crate can be documented without the toolkit. Run it after a `cargo build`, and only when
the toolkit version changes; CI fails if the copy has drifted.

## What `generate.py` refuses, and why

Every one of these was a real bug first, found by step 4 rather than by the compiler:

+ **Array dimensions come from the C source, not the bindings.** bindgen decays `SpiceDouble
  azlsta[6]` to a bare pointer, indistinguishable from a pointer to one number. Taking it as a
  scalar had CSPICE write six doubles into one.
+ **CSPICE puts its documentation between the prototype and the body**, so a regex that expects the
  body to follow the signature matches nothing and every dimension silently comes back empty.
+ **Inputs and outputs come from the documented `I`/`O` column, not from argument position.**
  `fovray_c` takes its epoch through a `SpiceDouble *`, which a positional split reads as an output.
+ **A cell's element type is nowhere in the signature.** `bltfrm` returns frame IDs, but nothing in
  the C declaration says the cell holds integers rather than doubles.
+ **A bare pointer output next to a count is an array.** `bodvar_c(body, item, dim, values)` writes
  `dim` doubles through a `SpiceDouble *`.
+ **So is one next to an input that could size it.** `dafgsr_c(handle, recno, begin, end, data,
  found)` writes `end - begin + 1` words.

The refusals are the safety margin: each rule above cut the "safely generatable" set, and the
routines it removed are the ones that had to be written by hand.

## Traps worth knowing about

+ The headers declare 649 `*_c` functions. Four are private internals named `zz*`, and `prefix_c`
  is declared in `SpiceZpr.h` but is not compiled into the library, so it cannot be called at all.
  The toolkit is 644 callable routines, not the 649 the headers advertise.
+ `dskd02` and `dski02` index their items from **zero**, unlike almost everything else.
+ `szpool` reports the pool's own limits, not the size of a variable. `dtpool` does that.
+ The set ordinals (`ordc`, `ordd`, `ordi`) count from zero and return `-1` when absent.
+ `rdtext` keeps a file open until it has been read to the end, and CSPICE then refuses to load it
  as a kernel. The C API has no counterpart to Fortran's `CLTEXT`.
+ `wnsumd` reports the shortest and longest intervals as indices into the flat endpoint array, so
  the second interval comes back as `2`.
+ `mxmg`, `mtxmg` and `mxmtg` name their dimensions after what they mean, and the meaning differs
  between them.
