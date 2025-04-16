# Two Identical, Simple Directory Trees

The inputs of this test are two simple, identical directory trees. Each contain
the same 2 files (that are byte-for-byte identical) and nothing else.

The aim of this test is to serve as one of many tests that check if `cmp-tree`:
1. Correctly identifies identical regular files as identical

`cmp-tree` should exit with an exit code of 0.

`diff -qr` should exit with an exit code of 0.
