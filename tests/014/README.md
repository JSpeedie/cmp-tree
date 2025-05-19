# Test Input 014

This test input has directory trees with contents that are identical in terms
of existence, file-type, substance, and modification time. Importantly, the two
corresponding soft links in the two directory trees both point to a
*non-existent* destination.

> Note: by default, `cmp-tree` should evaluate two soft links to be identical
> if and only if their link paths are identical, regardless of whether or not
> what they point to is identical or even exists. In this way, `cmp-tree`
> should (by default) behave the way `diff -qr --no-dereference` would behave.

```
first                       second
└── link -> ../adirectory/  └── link -> adirectory/
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
1. Correctly handles links that don't currently point to a valid destination

## Expected Exit Codes

Ran from `tests/014`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 1.
* `diff -qr --no-dereference first/ second/` should exit with an exit code of 0.
