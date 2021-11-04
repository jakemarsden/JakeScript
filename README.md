# JakeScript

[![CI][workflow-ci-badge]][workflow-ci]
[![License][license-badge]][license-file]

A work-in-progress JavaScript lexer, parser, and interpreter. Written in Rust
&#x1F980; for fun and learning.

## Usage

```shell
cargo build --release --workspace
./target/release/jakescript-cli [--eval | --parse | --lex] <source-path>
```

## Packages

- &#x1F56E; [`enumerate`][file-enumerate]
  Utility traits and macros for working with enums.
- &#x1F56E; [`enumerate-derive`][file-enumerate-derive]
  Implementation of the procedural macros provided by `enumerate`.
- &#x1F56E; [`jakescript`][file-jakescript]
  The core of the project. Contains code for the lexer, parser, interpreter, and
  most of the tests.
- &#x25B7; [`jakescript-cli`][file-jakescript-cli]
  Simple command line utility to run the lexer, parser, and/or interpreter on a
  file. Will eventually include a REPL.

## Tests

Run all tests:

```shell
cargo test --workspace
```

Run the main integration test on its own, which evaluates each JavaScript file
in the [`tests-js`][file-tests-js] directory:

```shell
cargo test --package jakescript --test js_tests -- --nocapture
```

[file-enumerate]: enumerate
[file-enumerate-derive]: enumerate-derive
[file-jakescript]: jakescript
[file-jakescript-cli]: jakescript-cli
[file-tests-js]: jakescript/tests-js
[license-badge]: https://img.shields.io/github/license/jakemarsden/JakeScript
[license-file]: LICENSE
[workflow-ci]: https://github.com/jakemarsden/JakeScript/actions/workflows/ci.yml
[workflow-ci-badge]: https://github.com/jakemarsden/JakeScript/actions/workflows/ci.yml/badge.svg?branch=master
