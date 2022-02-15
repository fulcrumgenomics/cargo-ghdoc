# `cargo-ghdoc`

<p align="center">
  <!-- <a href="https://github.com/sstadick/cargo-ghdoc/actions?query=workflow%3ACheck"><img src="https://github.com/sstadick/cargo-ghdoc/workflows/Check/badge.svg" alt="Build Status"></a> -->
  <img src="https://img.shields.io/crates/l/cargo-ghdoc.svg" alt="license">
  <a href="https://crates.io/crates/cargo-ghdoc"><img src="https://img.shields.io/crates/v/cargo-ghdoc.svg?colorB=319e8c" alt="Version info"></a><br>
</p>

Launch cargo docs from a github PR.

## Install

```bash
cargo install cargo-ghdoc
```

## Usage

```bash
cargo ghdoc https://github.com/sstadick/cargo-ghdoc/pull/1
```

## Implementation notes

This tool is just running `git` and `cargo` shell commands under the hood. As such both binaries should be findable in your path.
