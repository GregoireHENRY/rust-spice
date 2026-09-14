# Contributing to **rust-spice**

## Contributing to code

### Local development

You will need the source of **rust-spice** to start contributing on the codebase.
You will need to fork the project, clone your forked repository and place
yourself in its directory.

> If you are new to GitHub collaboration, you can refer to the
> [Forking Projects Guide][fork guide].

Replace `USERNAME` to match your forked repository.

```sh
git clone git@github.com:USERNAME/rust-spice.git
cd rust-spice
```

You will also need the [CSPICE toolkit][cspice install link] and the
`CSPICE_DIR` environment variable pointing at it; on Linux and macOS,
[`.github/install-cspice.sh`][install script link] does that for you.

These are the checks [CI][ci link] runs on every pull request, and running them
before you push saves a round trip:

```sh
cargo fmt --all --check
cargo clippy --all --all-targets -- --deny warnings
cargo clippy --package rust-spice --features lock --all-targets -- --deny warnings
cargo test --all
cargo test --package rust-spice --features lock
RUSTDOCFLAGS='--deny warnings' cargo doc --all --no-deps
```

The test suite writes the kernels it needs into a temporary directory as it
starts, so there is nothing to download and nothing to configure.

### Pull requests

The main branch (`main`) is the version of the code users get when they install
the library from [`cargo`][crate url]. The development branch (`dev`) is where
the code gets updated and validated before being merged into the main branch
(`main`).

Thus, all pull requests, unless otherwise instructed, need to be accepted into
the development branch (`dev`).

Be sure that your pull request contains tests and documentation that covers the
changed or added code.

*Thank you for your time contributing!!*

[fork guide]: https://guides.github.com/activities/forking/
[ci link]: https://github.com/GregoireHENRY/rust-spice/blob/main/.github/workflows/ci.yml
[cspice install link]: https://naif.jpl.nasa.gov/naif/toolkit_C.html
[install script link]: https://github.com/GregoireHENRY/rust-spice/blob/main/.github/install-cspice.sh
[crate url]: https://crates.io/crates/rust-spice
