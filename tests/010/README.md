# Test Input 010

This test input has directory trees with contents that are identical in terms
of existence, and file-type, but not in substance. The two corresponding
soft links point to two different directories that themselves are identical in
terms of existence, file-type, and substance, but the soft link target paths
differ.

> Note: by default, `cmp-tree` should evaluate two soft links to be identical
> if and only if their link paths are identical, regardless of whether or not
> what they point to is identical or even exists. In this way, `cmp-tree`
> should (by default) behave the way `diff -qr --no-dereference` would behave.

```
adirectory      anotherdirectory
├── dir         ├── dir
└── file.txt    └── file.txt
```
```
first                                                     second
└── link -> ../adirectory/  <-- differs in link path -->  └── link -> ../anotherdirectory/
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
1. Correctly recurses into subdirectories
2. Correctly identifies differing soft links as differing

## Expected Exit Codes

Ran from `tests/010`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 0.
* `diff -qr --no-dereference first/ second/` should exit with an exit code of 1.
