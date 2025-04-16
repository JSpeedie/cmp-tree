# cmp-tree

### Project Description

This is the Rust implementation of `cmp-tree`. The program compares two
directory trees telling you if they are identical.

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

```
cargo test
```

Or:

```
cargo test --test integration_tests
```
