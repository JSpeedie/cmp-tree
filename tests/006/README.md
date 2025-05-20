# Test Input 006

This test input has directory trees with contents that are identical in terms
of existence, file-type, substance, and modification time.

```
first                  second
├── a                  ├── a
│   └── i              │   └── i
├── b                  ├── b
│   ├── i              │   ├── i
│   │   └── 1          │   │   └── 1
│   └── ii             │   └── ii
│       └── 2          │       └── 2
└── c                  └── c
    ├── i                  ├── i
    │   └── 1              │   └── 1
    │       └── a          │       └── a
    ├── ii                 ├── ii
    │   └── 2              │   └── 2
    │       └── b          │       └── b
    └── iii                └── iii
        └── 3                  └── 3
            └── c                  └── c
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
2. Correctly identifies identical directories as identical

## Expected Exit Codes

Ran from `tests/006`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 0.
