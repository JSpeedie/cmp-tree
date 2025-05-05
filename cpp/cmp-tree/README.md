# cmp-tree

### Directory Description

This directory contains the C++ implementation of `cmp-tree`. The program
compares two directory trees telling you if they are identical.

#### Features of the C++ Implementation

* The C++ implementation is NOT multithreaded.
* The C++ implementation is NOT tested.

&nbsp;

### Building and Running

From this directory, run the following commands:

```bash
cmake -S . -B release/ -D CMAKE_BUILD_TYPE=Release
cmake --build release/
./release/cmp-tree [path-to-first-directory] [path-to-second-directory]
```

Alternatively, you can also build and run the debug version of `cmp-tree`, used
for development, by running:

```bash
cmake -S . -B debug/ -D CMAKE_BUILD_TYPE=Debug
cmake --build debug/
./debug/cmp-tree [path-to-first-directory] [path-to-second-directory]
```
