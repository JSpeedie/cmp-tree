# Test Input 020

This test input has directory trees with contents that are identical in terms
of existence, file-type, but not substance or modification time.
`[...]/random_file` is almost entirely byte-for-byte identical, but the very
final byte of the two files differs.

```
first                                          second
└── random_file  <-- differs in substance -->  └── random_file
```

## Generating the Test Input

The directory trees associated with the test are built through a series of
commands that are executed by running `generate-test-input.sh`. We can run that
script by running:

```bash
./generate-test-input.sh
```

## The Aim of This Test

The aim of this test is to serve as one of many tests that make sure
`cmp-tree`:
1. Correctly identifies files that differ in substance but are the same size.
2. Correctly identifies files that differ in substance even when the changes
   are small.

## Expected Exit Codes

Ran from `tests/020`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
