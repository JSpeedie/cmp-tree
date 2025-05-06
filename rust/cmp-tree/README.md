# cmp-tree

### Directory Description

This directory contains the Rust implementation of `cmp-tree`. The program
compares two directory trees telling you if they are identical.

> Note: The directory trees used for testing the Rust implementation are not
> contained within this current directory tree. They are 2 levels above in
> `[repo-root]/tests`

#### Features of the Rust Implementation

* The Rust implementation does support soft links.
* The Rust implementation is multithreaded.
* The Rust implementation is tested.
* The Rust implementation supports the `-d`, `-m`, `-p`, `-s` and `-t` flags.
* The Rust implementation MAY have memory leaks, it has not been checked.

&nbsp;

### Installation and Running

```bash
cd rust/cmp-tree
cargo build --release
./install.sh
cmp-tree [path-to-first-directory] [path-to-second-directory]
man cmp-tree
```

&nbsp;

### Testing

```bash
cargo test
```

Or if you only want to run one test suite at a time:

```bash
cargo test --test integration_tests
cargo test unit_tests
# This project has no doc tests at the moment, but here's the command for
# running only the doc tests anyway
cargo test --doc
```
