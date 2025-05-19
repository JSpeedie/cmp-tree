# Test Input 003

This test input has directory trees with contents that are largely identical
(in terms of existence, file-type, and substance, but not modification time)
with the exception of `[...]/Lorem.txt` which differs in substance.

```
first                                                  second
├── cmp_man_pages.txt                                  ├── cmp_man_pages.txt
├── Lorem.txt                                          ├── Lorem.txt
└── subdir                                             └── subdir
    ├── linear_gradient.png                                ├── linear_gradient.png
    └── rose.png                                           └── rose.png
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
1. Correctly identifies identical regular files as identical

## Expected Exit Codes

Ran from `tests/003`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 0.
