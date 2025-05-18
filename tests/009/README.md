# Test Input 009

This test input has directory trees with contents that are identical in terms
of existence, and file-type, but not in substance. The two corresponding soft
links point to two different directories that differ in their contents.

> Note: `cmp-tree` should evaluate the two soft links as being different
> because they do not point to the same path, *not* because what they point
> differs. `diff -qr` evaluates soft links as identical if what they point to
> is identical, so it will report a difference between the two directory trees
> like `cmp-tree` would, but for different reasons.

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

* `cmp-tree` should exit with an exit code of 1.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr` should exit with an exit code of 1.
