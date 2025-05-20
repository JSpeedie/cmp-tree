# Test Input 004

This test input has directory trees with contents that are largely identical
(in terms of existence, file-type, and substance, but not modification time)
with the exception of `[...]/Lorem.txt` which differs in substance.

```
first                                                      second
├── cmp_man_pages.txt                                      ├── cmp_man_pages.txt
├── Lorem.txt                                              ├── Lorem.txt
└── subdir                                                 └── subdir
    ├── linear_gradient.png                                    ├── linear_gradient.png
    └── rose.png             <-- differs in substance -->      └── rose.png
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
1. Correctly identifies substance mismatches in regular files

## Expected Exit Codes

Ran from `tests/004`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
