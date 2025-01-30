# `cargo-ghdoc`

<p align="center">
  <img src="https://img.shields.io/crates/l/cargo-ghdoc.svg" alt="license">
  <a href="https://crates.io/crates/cargo-ghdoc"><img src="https://img.shields.io/crates/v/cargo-ghdoc.svg?colorB=319e8c" alt="Version info"></a><br>
</p>

Launch cargo docs from a github PR.

<p>
<a href float="left"="https://fulcrumgenomics.com"><img src=".github/logos/fulcrumgenomics.svg" alt="Fulcrum Genomics" height="100"/></a>
</p>

[Visit us at Fulcrum Genomics](www.fulcrumgenomics.com) to learn more about how we can power your Bioinformatics with cargo-ghdoc and beyond.

<a href="mailto:contact@fulcrumgenomics.com?subject=[GitHub inquiry]"><img src="https://img.shields.io/badge/Email_us-brightgreen.svg?&style=for-the-badge&logo=gmail&logoColor=white"/></a>
<a href="https://www.fulcrumgenomics.com"><img src="https://img.shields.io/badge/Visit_Us-blue.svg?&style=for-the-badge&logo=wordpress&logoColor=white"/></a>

This tool is intended as an aid when reviewing PRs in Rust projects.
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
cargo ghdoc https://github.com/fulcrumgenomics/cargo-ghdoc/pull/1
```

This will open the Rust docs for the Rust project from a checkout of that PR.

Proceed to nit pick.

## Implementation notes

This tool is just running `git` and `cargo` shell commands under the hood.
As such both binaries should be findable in your path.
Additionally this tool requires that you have set up SSH credentials and that they are valid for the repo in question.

Equivalent shell commands:

```bash
mkdir <tempdir>
cd <tempdir>
git clone <repo>
cd <repo>
git fetch origin pull/<PR number>/head:GHDOC
git checkout GHDOC
cargo doc --open
```

Since this tool is piggybacking off of `git`, you must have sufficient permissions to run those command.
