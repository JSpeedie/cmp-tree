# Test Input 015

This test input has directory trees with contents that are identical in terms
of existence, but not file-type. `first/lorem` is a directory while
`second/lorem` is a regular file.

```
first                                    second
└── lorem  <-- differs in file-type -->  └── lorem
```

## Generating the Test Input

The directory trees associated with this test input are built through a series
of commands that are executed by running `generate-test-input.sh`. We can run
that script by running:

```bash
./generate-test-input.sh
```

## The Aim of This Test Input

The aim of this test is to serve as one of many tests that make sure
`cmp-tree`:
1. Correctly handles file-type mismatches

## Expected Exit Codes

Ran from `tests/014`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
