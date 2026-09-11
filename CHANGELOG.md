# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## TODO

## [Unreleased]

## [0.8.0] - 2026-09-11

### Added

+ `errors` module: `check` turns the CSPICE error state into a Rust `Result`, and `quiet` stops the
  toolkit printing to the screen and aborting the process on failure. Also wraps `failed`, `reset`,
  `getmsg`, `qcktrc`, `erract`, `errdev` and `errprt`.
+ `ffi` module: the `SpiceArg`/`SpiceRet`/`SpiceReturn` traits describing how each Rust type is
  handed to, and read back from, a C routine.
+ 120 more wrapped functions, taking the total from 47 to 167 of the 649 CSPICE exposes. Every
  entry of the previous *TODO* list is now wrapped: `ckcov`, `ckgp`, `ckgpav`, `ckobj`, `dsksrf`,
  `gcpool`, `gipool`, `latsrf`, `pckcov`, `scdecd`, `sce2c`, `sce2s`, `scencd`, `scs2e`, `sct2e`,
  `spkcov`, `spkcpo`, `spkcpt`, `spkcvo`, `spkcvt`, `spkobj`, `srfc2s`, `srfcss`, `srfnrm`,
  `srfs2c`, `srfscc` and `sxform`, along with the CK, DSK and SPK writers and the coordinate,
  vector, matrix, two-body, kernel pool, surface geometry and toolkit constant families.
+ 10 more, taking the total to 177 and leaving nothing marked *TODO* in the index for the first
  time: `dskxsi` and `dskxv` cast rays at whatever shape data is loaded, `limbpt` and `termpt` find
  limb and terminator points, `getelm` and `evsgp4` read and propagate a two-line element set, and
  `gfoclt` searches for occultations, with `wninsd`, `wnfetd` and `wncard` to build the confinement
  window it takes and read the window it returns.
+ 13 more, taking the total to 190: `axisar`, `eul2m`, `eul2xf`, `invort`, `isrot`, `m2eul`, `m2q`,
  `q2m`, `qxq`, `rav2xf`, `raxisa`, `xf2eul` and `xf2rav` cover rotations, quaternions, Euler angles
  and state transformations.
+ 35 more, taking the total to 225: the whole of the window arithmetic (`wncomd`, `wncond`,
  `wndifd`, `wnelmd`, `wnexpd`, `wnextd`, `wnfild`, `wnfltd`, `wnincd`, `wnintd`, `wnreld`,
  `wnsumd`, `wnunid`, `wnvald`) and of the cell and set operations (`union`, `inter`, `diff`,
  `copy`, `card`, `scard`, `size`, `ssize`, `valid`, and the `elem`, `insrt`, `remov` and `appnd`
  families). The set operations are generic over the element type, so one declaration covers the
  integer, double precision and character cells.
+ 33 more, taking the total to 258: the conversions between the curvilinear coordinate systems
  (`latcyl`, `cyllat`, `latsph`, `sphlat`, `cylsph`, `sphcyl`, `azlrec`, `recazl`), all twelve
  Jacobians relating them to rectangular coordinates, `xfmsta` to move a state between systems, and
  `vpack`, `vupack`, `vzero`, `vperp`, `vproj`, `vrotv`, `vlcom`, `vlcom3`, `vtmv`, `ident`, `mequ`
  and `xpose6`.
+ 21 more, taking the total to 279: planes and ellipses, as the `PLANE` and `ELLIPSE` types and the
  routines that build, take apart, intersect and project them (`nvc2pl`, `nvp2pl`, `psv2pl`,
  `pl2nvc`, `pl2nvp`, `pl2psv`, `cgv2el`, `el2cgv`, `saelgv`, `inedpl`, `inelpl`, `inrypl`,
  `edlimb`, `pjelpl`, `vprjp`, `vprjpi`, `npelpt`, `npedln`, `nplnpt`, `surfnm`, `surfpv`).
+ 24 more, taking the total to 303: the vector and matrix routines of arbitrary dimension. They
  take slices and size the result themselves, and check that the lengths agree with the dimensions
  rather than letting CSPICE read past the end. The dimension arguments keep the names CSPICE gives
  them, which differ in meaning between `mxmg`, `mtxmg` and `mxmtg`.
+ `Cell<T>` is generic over its element type, owns its backing storage, and gained `len`,
  `capacity`, `get`, `iter`, `to_vec`, `push`, `clear` and a deep `Clone`.
+ A self contained test suite: the SPK, CK, DSK, PCK and text kernels it needs are generated in a
  temporary directory when it starts, so `cargo test` works on any machine that can build the
  crate. Every wrapped function is covered, and the marshalling of each argument shape is checked
  against a direct call to the C routine.
+ GitHub Actions CI: formatting, clippy, the test suite with and without `lock`, and the
  documentation, on Linux and macOS. `.github/install-cspice.sh` installs the toolkit, and is
  useful outside CI too.

### Fixed

+ The `lock` feature did not compile: `dskp02`, `dskv02`, `kdata` and `timout` each generated two
  methods of the same name on `SpiceLock`. The guard now exposes the neat version where there is
  one, as the documentation always claimed.
+ Wrappers no longer leak. Every string and buffer handed to CSPICE was `malloc`ed or leaked through
  `CString::into_raw` and never freed, once per call.
+ String outputs were read out of uninitialised memory when a routine failed without writing them.
+ `raw::bodc2n` and the other wrappers taking a `lenout` allocated `MAX_LEN_OUT` bytes whatever the
  length they promised CSPICE, so a larger `lenout` overran the buffer.
+ `neat::timout` sized its output buffer from the length of the format picture, truncating any
  output at least as long as its picture.
+ `Cell` leaked its buffer, computed the address of the data area of a character cell in elements
  rather than bytes, and `get_data_character` returned a single character.
+ `Cell::new_time` allocated a buffer of integers for a cell CSPICE reads as doubles.
+ `gdpool` reported the values of a variable absent from the pool rather than an empty vector.
+ `raw::dskz02` documented its two outputs in the wrong order; it returns the vertex count first.
+ `SpiceLock` kept its flag in a `static mut`, which is unsound and rejected from the 2024 edition.
+ Several wrong links in the bindings table.

### Changed

+ The procedural macro no longer builds Rust code by formatting and re-parsing strings, and no
  longer knows about the SPICE types: the conversions live in `ffi`, where the compiler checks them.
  It also reports a proper compile error instead of panicking with `->1`.
+ Strings crossing the FFI boundary are copied into a stack buffer while they fit, so a wrapper no
  longer allocates once per string argument: marshalling one argument went from 22 ns to 3.8 ns, and
  a `spkpos` call from 1597 ns to 1548 ns (Apple M-series, CSPICE N0067).
+ `spkw09`, `ckw03` and `dskw02` panic rather than letting CSPICE read past the end of a slice.
+ `spkw09` takes `&[...]` instead of `&mut [...]`.
+ `raw::dskobj` takes the cell to fill, like CSPICE; `neat::dskobj` keeps returning an owned one.
+ `recpgr` returns the longitude, latitude and altitude as a tuple rather than as a `[f64; 3]`, and
  `getfov` takes the instrument ID as an `i32`. `getfov` also gained a neat version, which is what
  `spice::getfov` now refers to.
+ `cstr!`, `fcstr!`, `malloc!` and `mallocstr!` are kept for use with the unsafe C API. `malloc!`
  and `mallocstr!` now zero what they allocate, and `fcstr!` no longer panics on invalid UTF-8.
+ Edition 2021, `syn` 2 to 3, and the unused `nalgebra`, `serde`, `serde_json`, `serde_repr` and
  `log` dependencies dropped. `approx`, `itertools` and `serial_test` moved to dev-dependencies.

### Removed

+ The declaration of the `rust-spice/hera` submodule, which was never registered and which the test
  suite no longer needs.
+ `Cell<bool>`: no CSPICE C routine operates on a boolean cell. `Cell::new_bool` still hands back an
  integer cell tagged as boolean.


## [0.7.1] - 2021-10-24

### Added

+ func: bodfnd
+ test for above function
+ possibility to return bool directly

### Changed

+ logo squared on docs.rs

## [0.7.0] - 2021-09-14

### Fixed

+ crate badge

### Changed

+ updated Hera test kernels
+ path to test kernels

### Added

+ functions: georec, mxv, radrec, spkezr, vcrss, vdot, xpose
+ contributing guideline and contributor list in readme
+ roadmap in readme

## [0.6.16 - 0.6.17] - 2021-06-08

### Changed

+ strings in input are now `&str`
+ strings in output stay `String`

Quick recall that functions in C that ouput string always ask for allocation
size. In this lib, neat functions default to `spice::MAX_LEN_OUT`, and you are
free to call the raw version of the fonction to get the hand of the signature
with the size argument.

## [0.6.14 - 0.6.15] - 2021-06-07

### Added

+ dskobj
+ Cell (only integer for now)
+ bodc2n
+ bodn2c

## [0.6.11 - 0.6.13] - 2021-06-06

### Added

+ ktotal
+ kdata (default length 256)

### Fixed

+ string allocation for output parameters

## [0.6.7 - 0.6.10] - 2021-06-05

### Added

+ dskz02
+ dskp02
+ dskv02
+ illumf

## [0.6.6] - 2021-06-04

### Added

+ kclear

### Fixed

+ clippy warnings

## [0.6.5] - 2021-05-28

### Added

+ dla & dsk descr type alias

## [0.6.3 - 0.6.4] - 2021-05-24

### Added

+ dascls
+ dasopr
+ dlabfs
+ dskgd
+ dskn02
+ dskx02
+ latrec
+ recrad
+ vsep

## [0.6.2] - 2021-05-17

### Fixed

+ some links

## [0.6.1] - 2021-05-17

### Added

+ documentation

## [0.6.0] - 2021-05-17

### Added

+ crate `rust-spice-derive`

### Changed

+ implement Rust idiomatic interface to CSPICE with procedural macros
+ moved `rust-spice` crate to subforlder of `rust-spice` root repo, to contain
  the main crate and the derive

### Removed

+ mods `spicetools`, `kernel`, `check_geometric_conditions`
+ examples, for now if you want example -> go check tests

## [0.5.4] - 2021-05-13

### Added

+ documentation

## [0.5.1 - 0.5.3] - 2021-05-13

Many trials to make CSPICE wrapper working on Mac and Windows also.
Ended up proposing to overwrite the build of `cspice-sys` inside
`$HOME/.cargo/config.toml`, thanks to the `links` attributes.

## [0.4.8] - 2021-05-12

### Added

+ implement occult_c

### Changed

+ improve readme
+ simplify error handling with crate `thiserror`

### Removed

+ unnecessary examples
+ unnecessary tests

## [0.4.7] - 2021-04-23

### Changed

+ using nalgebra 0.26

## [0.4.6] - 2021-04-16

### Added

+ more examples

### Changed

+ use rustool 0.3.12

## [0.4.5] - 2021-04-09

### Changed

+ improved example

## [0.4.4] - 2021-04-09

### Added

+ tests kernel
+ frames functions
+ example frames

## [0.4.3] - 2021-04-05

### Added

+ error managment
+ examples
+ tests

### Changed

+ System getters not mutable

### Removed

+ module toolbox is now external to this crate
+ static list of loaded kernels, let user deal with asynchronous problem if they
  want to do it, this crate ensures the type Kernel loads and unloads correctly

## [0.4.2] - 2021-04-01

### Added

+ documentation guide online

## [0.4.1] - 2021-04-01

### Added

+ documentation and test coverages in badges

## [0.4.0] - 2021-04-01

### Changed

+ this crate is not anymore creating the binding, it uses cspice-rust, to focus
  more on the Rust layer.

## [0.3.5] - 2021-03-31

### Changed

+ spicetools toolbox is now 100% Rust -> safer and simplier build script

## [0.3.4] - 2021-03-29

### Added

+ example
+ documentation

## [0.3.3] - 2021-03-29

### Added

+ Rust layer for the struct System
+ documentation
+ test on 100% of C spicetools

## [0.3.2] - 2021-03-29

+ moved cspice to OUT_DIR, I don't like it because wget procs on every build of
  every single different target, didn't find a workaround yet

## [0.3.1] - 2021-03-29

### Added

+ README comment on objectives

## [0.3.0] - 2021-03-29

### Removed

+ dependency spice-sys

### Modified

+ spice-sys script to get cscpice
+ README

## [0.2.1] - 2021-03-29

+ tried to incorporate spice-sys to use its build script to get cspice but the
  OUT_DIR environment variable is not configurable for dependency so the cspice
  library was hidden in target/ and it was not easy to link it automatically
+ cspice was being downloaded every time the build is launch... and multiple
  time for the different targets, so it was annoying and had to move to v0.3.0

## [0.2.0] - 2021-03-28

### Added

+ cspice submodule (fork of official code)
+ some lib test

### Changed

+ spicetools submodule (refer to v0.2.0 in spicetools CHANGELOG)
+ cspice lib name
+ build script to correctly link with spice and spicetools functions

## [0.1.0] - 2021-03-27

+ version not working, I was trying to understand concepts of wrapping and
  binding with bindgen.

### Added

+ Initial commit
