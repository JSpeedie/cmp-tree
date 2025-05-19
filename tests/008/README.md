# Test Input 008

This test input has directory trees with contents that are identical in terms
of existence, file-type, and substance, except for the corresponding soft links
that differ in their modification time. The two corresponding soft links have
the same link path, but the link path is relative and ultimately they point to
two different directories that happen to be identical in their contents.

> Note: by default, `cmp-tree` should evaluate two soft links to be identical
> if and only if their link paths are identical, regardless of whether or not
> what they point to is identical or even exists. In this way, `cmp-tree`
> should (by default) behave the way `diff -qr --no-dereference` would behave.

```
first                                              second
├── adirectory                                     ├── adirectory
│   ├── dir                                        │   ├── dir
│   └── file.txt                                   │   └── file.txt
└── link -> adirectory/  <-- differs in mtime -->  └── link -> adirectory/
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
2. Correctly identifies identical soft links as identical

## Expected Exit Codes

Ran from `tests/008`:

* `cmp-tree` should exit with an exit code of 0.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr` should exit with an exit code of 0.
* `diff -qr --no-dereference` should exit with an exit code of 0.
