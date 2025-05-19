# Test Input 016

This test input has directory trees with contents that are identical in terms
of existence, file-type, and substance, except for the corresponding files
`first/link` and `second/link` which differ in their file-type. `first/link` is
a soft link, and `second/link` is a regular file.

```
first                                                  second
├── adirectory                                         ├── adirectory
│   ├── dir                                            │   ├── dir
│   └── file.txt                                       │   └── file.txt
└── link -> adirectory/  <-- differs in file-type -->  └── link
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
1. Correctly recurses into subdirectories
2. Correctly handles file-type mismatches

## Expected Exit Codes

Ran from `tests/016`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 0.
* `diff -qr --no-dereference first/ second/` should exit with an exit code of 0.
