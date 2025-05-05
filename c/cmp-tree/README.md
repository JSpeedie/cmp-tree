# cmp-tree

### Directory Description

This directory contains the C implementation of `cmp-tree`. The program
compares two directory trees telling you if they are identical.

#### Features of the C Implementation

* The C implementation is multithreaded.
* The C implementation is NOT tested.

&nbsp;

### Building and Running

From this directory, run the following commands:

```bash
make release
./release/cmp-tree [path-to-first-directory] [path-to-second-directory]
```

Alternatively, you can also build and run the debug version of `cmp-tree`, used
for development, by running:

```bash
make
./debug/cmp-tree [path-to-first-directory] [path-to-second-directory]
```
