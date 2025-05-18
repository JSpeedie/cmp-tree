# Test Input 012

This test input has 2 almost-identical directory trees containing only
directories, with 4 levels of subdirectories with one directory that mismatches
in modification time.

```
first                                        second
├── a                                        ├── a
│   └── i                                    │   └── i
├── b                                        ├── b
│   ├── i                                    │   ├── i
│   │   └── 1                                │   │   └── 1
│   └── ii                                   │   └── ii
│       └── 2                                │       └── 2
└── c                                        └── c
    ├── i                                        ├── i
    │   └── 1                                    │   └── 1
    │       └── a  <-- differs in mtime -->      │       └── a
    ├── ii                                       ├── ii
    │   └── 2                                    │   └── 2
    │       └── b                                │       └── b
    └── iii                                      └── iii
        └── 3                                        └── 3
            └── c                                        └── c
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
2. Correctly identifies identical directories as identical
3. Can correctly identify when directories are identical in every way except in
   their modification time.

## Expected Exit Codes

Ran from `tests/012`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `cmp-tree -d first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 0.
