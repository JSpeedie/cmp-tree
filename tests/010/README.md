# Test Input 010

This test input has directory trees with contents that are identical in terms
of existence, and file-type, but not in substance. The two corresponding
soft links point to two different directories that themselves are identical in
terms of existence, file-type, and substance, but the soft link target paths
differ.

> Note: `cmp-tree` should evaluate the two soft links as being different
> because they do not point to the same path, *despite* the fact that what they
> point to is identical. `diff -qr` evaluates soft links as identical if what
> they point to is identical, so it will succeed on this test case even though
> `cmp-tree` would not.

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

* `cmp-tree` should exit with an exit code of 1.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 0.
