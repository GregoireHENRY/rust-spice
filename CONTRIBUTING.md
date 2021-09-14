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

**rust-spice** uses [`pre-commit`][pre-commit url] to make sure that you don't
accidentally commit code that does not follow the coding style. The codebase
will be check against the run of the tests, the build of the documentation, the
preparation of publication. But also the `fmt`, `check` and `clippy` tools from
`cargo`, and few other git related and filesystem checks. These can be seen in
the [pre-commit config file][pre-commit file]. You can install the hook script
that will check that everything is in order:

```sh
pre-commit install
```

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
[pre-commit url]: https://pre-commit.com
[pre-commit file]: https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/.pre-commit-config.yaml
[crate url]: https://crates.io/crates/rust-spice
