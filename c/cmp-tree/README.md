# cmp-tree

### Directory Description

This directory contains the C implementation of `cmp-tree`. The program
compares two directory trees telling you if they are identical.

#### Features of the C Implementation

* The C implementation is multithreaded.
* The C implementation supports the `-m`, `-p`, and `-t` flags.
* The C implementation is NOT tested.
* The C implementation does NOT support soft links.
* The C implementation MAY have memory leaks, it has not been checked.

&nbsp;

### Building and Running

From this directory, run the following commands:

```bash
cmake -S src/ -B release/ -D CMAKE_BUILD_TYPE=Release
cmake --build release/
./release/cmp-tree [path-to-first-directory] [path-to-second-directory]
```

Alternatively, you can also build and run the debug version of `cmp-tree`, used
for development, by running:

```bash
cmake -S src/ -B debug/ -D CMAKE_BUILD_TYPE=Debug
cmake --build debug/
./debug/cmp-tree [path-to-first-directory] [path-to-second-directory]
```
