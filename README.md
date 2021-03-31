# rust-spice

[![logo image]][crate link]

[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]

> WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust

---

[Rational](#rational) |
[Disclaimer](#disclaimer) |
[Installation](#installation) |
[License](#license)

---

## Rational

Other crates already exist for wrapping C spice: [spice-sys][spice-sys link]
and [cspice-sys][cspice-sys link].
The motivation behind the creation of [rust-spice][crate link] was 1) the need
to access the complete spice API from Rust and 2) the need to have a nice Rust
layer to code naturally.

### Complete API

[spice-sys][spice-sys link] does not use [bindgen][bindgen link] to wrap
which is error-prone and does not provide a complete API. The description of
spice-sys says:

> *Currently, it provides only the most common API calls as outlined
[here][cspice most common].*

This boosted me to build a complete C spice wrapper.

[cspice-sys][cspice-sys link] is build with bindgen but the cspice code is
collected in a strange way, the original architecture of cspice is not
respected, plus the repository is missing somehow. I cannot ensure the full-
API is available with this crate.

### Rust interface

These crates are bare wrappers and do not provide a Rust user-friendly interface,
which constrains the user to use unsafe and FFI types conversion which is simply
a pain to use.

I wanted to use spice from Rust without feeling it being a C wrapper, for
quicker and simplier development and usage in my crates.

I could have started by forking one these two crates or using them in dependency,
but both of them did not feel sane to use. I decided to also provide my own
wrapper, which can be used during the process of the creation of the Rust layer.

### S/O

Many thanks to the [author][author spice-sys] of the crate
[spice-sys][spice-sys link] for the idea of the wget script to avoid crate
upload, I included your script copyrighted under your MIT license and added some
modifications that I listed carefully.

## Disclaimer

This crate is my first exercice to write a wrapper, all kind advices are
warmly welcomed.

The Rust layer is being built in real time as I face the need of it. Raise an
issue with the need of a function/struct and we will work on it immediately.
Any time a function is not wrapped inside safe code, you can still use the
unsafe code to call it.

Apparently the documentation is not building online, though I spent a lot of
time writting it. On my pc it is building just nice. By reading docs.rs output,
the header `cspice/include/SpiceUser.h` is not in the include PATH obviously.
Locally it works because in my `build.rs` I use `.clang_arg()` on the the
`bindgen:Builder` to tell Rust where to find the damn header. And it should work
online also, I guess. Any help is appreciated to solve this issue!! I can send
you the offline documentation in the meantime if you want.

## Installation

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
...
[dependencies]
rust-spice = "0.3.5"
```

## License

Licensed under the [Apache License, Version 2.0][license link].

[repository link]: https://github.com/GregoireHENRY/rust-spice
[logo image]: https://github.com/GregoireHENRY/rust-spice/blob/main/rsc/img/rust-spice-logo.png
[crate link]: https://crates.io/crates/rust-spice
[crate badge]: https://meritbadge.herokuapp.com/rust-spice?style=flat-square
[doc link]: https://docs.rs/rust-spice
[doc badge]: https://docs.rs/rust-spice/badge.svg
[license link]: https://github.com/GregoireHENRY/rust-spice/blob/main/LICENSE
[license badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[pre-commit link]: https://pre-commit.com
[pre-commit badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[spice-sys link]: https://crates.io/crates/spice-sys
[author spice-sys]: https://github.com/rjpower4
[cspice-sys link]: https://crates.io/crates/cspice-sys
[cspice most common]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/info/mostused.html
[bindgen link]: https://crates.io/crates/bindgen
