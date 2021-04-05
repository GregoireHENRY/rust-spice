# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## TODO

+ continuous improvement of the Rust layer
+ send mail to Marc Costa about this crate

## [Unreleased]

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
