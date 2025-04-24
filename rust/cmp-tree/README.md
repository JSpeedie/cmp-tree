# cmp-tree

### Project Description

This is the Rust implementation of `cmp-tree`. The program compares two
directory trees telling you if they are identical.

The Rust implementation is multithreaded.

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
