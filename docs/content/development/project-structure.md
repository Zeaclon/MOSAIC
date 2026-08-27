+++
title = "MOSAIC Project Structure"
weight = 2
+++


## Overview

MOSAIC is implemented as a Rust package containing a library crate and an executable binary.

## Source Structure

- `src/lib.rs` - public library entry point
- `src/main.rs` - executable entry point
- `src/mosaic/` - MOSAIC implementation
- `src/mosaic/core/` - core functionality
- `src/mosaic/configuration/` - configuration subsystem
- `src/mosaic/components/` - desktop components
- `src/mosaic/providers/` - provider implementations
- `src/mosaic/runtime/` - runtime functionality

## Tests

- Unit tests are colocated with the modules they test.
- Integration tests are located under `tests/`.

## Development Commands

```bash
cargo fmt
cargo build
cargo test
cargo run
```

## Current Status

The project currently provides an executable initialization skeleton.
Subsystem implementations will be developed in subsequent issues.

## Crate Structure

MOSAIC is split into a library crate and an executable binary.

- The library crate contains MOSAIC application logic and provides the public API.
- The binary provides the executable entry point and delegates to the library.
