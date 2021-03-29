# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## TODO

## [Unreleased]

## [0.3.0] - 2021-03-28

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
