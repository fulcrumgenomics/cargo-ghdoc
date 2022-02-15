# `cargo-ghdoc`

<p align="center">
  <!-- <a href="https://github.com/sstadick/cargo-ghdoc/actions?query=workflow%3ACheck"><img src="https://github.com/sstadick/cargo-ghdoc/workflows/Check/badge.svg" alt="Build Status"></a> -->
  <img src="https://img.shields.io/crates/l/cargo-ghdoc.svg" alt="license">
  <a href="https://crates.io/crates/cargo-ghdoc"><img src="https://img.shields.io/crates/v/cargo-ghdoc.svg?colorB=319e8c" alt="Version info"></a><br>
</p>

Launch cargo docs from a github PR.

This toll is intended as an aid when reviewing PRs in Rust projects.
It's often hard to tell what the docs will look like before rendering.
It's also often hard to gain context for a PR without seeing the docs.
This tool solves both of those problems with a single command.

## Install

```bash
cargo install cargo-ghdoc
```

## Usage

From an open PR page in github, copy the URL at the top of the page. Then run the following, replacing the URL here with your open PR.

```bash
cargo ghdoc https://github.com/sstadick/cargo-ghdoc/pull/1
```

This will open the Rust docs for the Rust project from a checkout of that PR.

Proceed to nit pick.

## Implementation notes

This tool is just running `git` and `cargo` shell commands under the hood.
As such both binaries should be findable in your path.
Additionally this tool requires that you have set up SSH credentials and that they are valid for the repo in question.
