# Test Input 005

This test input has directory trees with contents that are identical in terms
of existence and file-type, but where every regular file differs in substance.

```
first                                                  second
├── linear_gradient.png  <-- differs in substance -->  ├── linear_gradient.png
├── Lorem.txt            <-- differs in substance -->  ├── Lorem.txt
└── rose.png             <-- differs in substance -->  └── rose.png
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
1. Correctly identifies substance mismatches in regular files

## Expected Exit Codes

Ran from `tests/005`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
