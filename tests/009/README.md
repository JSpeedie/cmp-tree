# Test Input 009

This test input has directory trees with contents that are identical in terms
of existence, and file-type, but not in substance. The two corresponding soft
links point to two different directories that differ in their contents.

> Note: by default, `cmp-tree` should evaluate two soft links to be identical
> if and only if their link paths are identical, regardless of whether or not
> what they point to is identical or even exists. In this way, `cmp-tree`
> should (by default) behave the way `diff -qr --no-dereference` would behave.

```
adirectory                                  anotherdirectory
├── dir       <----- only in first ------>  │
└── file.txt  <-- differs in substance -->  └── file.txt
```
```
first                                                     second
└── link -> ../adirectory/  <-- differs in link path -->  └── link -> ../anotherdirectory/
```

## Generating the Test Input

The directory trees associated with this test input are built through a series
of commands that are executed by running `generate-test-input.sh`. We can run
that script by running:

```bash
./generate-test-input.sh
```

## The Aim of This Test Input

The aim of this test input is to facilitate the verification that `cmp-tree`
does the following:
1. Correctly recurses into subdirectories
2. Correctly identifies differing soft links as differing

## Expected Exit Codes

Ran from `tests/009`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
* `diff -qr --no-dereference first/ second/` should exit with an exit code of 1.
