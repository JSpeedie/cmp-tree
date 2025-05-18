# Test Input 011

This test input has directory trees with contents fully identical in terms of
existence, file-type, and substance, but all the regular files differ in their
modification time.

```
first                                                  second
├── cmp_man_pages.txt        <-- differs in mtime -->  ├── cmp_man_pages.txt
├── Lorem.md                 <-- differs in mtime -->  ├── Lorem.md
└── subdir                                             └── subdir
    ├── linear_gradient.png  <-- differs in mtime -->      ├── linear_gradient.png
    └── rose.png             <-- differs in mtime -->      └── rose.png           
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
2. Can correctly identify when files are identical in every way except their
   modification time.

## Expected Exit Codes

Ran from `tests/011`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 0.
