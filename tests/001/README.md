# Test Input 001

This test input has directory trees with contents that are identical in terms
of existence, file-type, substance, and modification time.

```
first                      second
├── cmp_man_pages.txt      ├── cmp_man_pages.txt
└── Lorem.txt              └── Lorem.txt
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
1. Correctly identifies identical regular files as identical
1. Can correctly identify when files are identical in every way including in
   their modification time.

## Expected Exit Codes

Ran from `tests/001`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `cmp-tree -d first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 0.
