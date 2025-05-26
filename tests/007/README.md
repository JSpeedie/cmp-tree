# Test Input 007

This test input has directory trees with contents that are partially identical
(in terms of existence, file-type, substance, and modification time), but
mostly differing in terms of existence.

```
first                                      second
├── a                                      ├── a
│   └── i                                  │   └── i
├── b                                      ├── b
│   ├── i                                  │   ├── i
│   │   └── 1                              │   │   └── 0
│   └── ii                                 │   └── ii
│       └── 2                              │       └── 1
└── c              <-- only in first --->  │
    ├── i          <-- only in first --->  │
    │   └── 1      <-- only in first --->  │
    │       └── a  <-- only in first --->  │
    ├── ii         <-- only in first --->  │
    │   └── 2      <-- only in first --->  │
    │       └── b  <-- only in first --->  │
    └── iii        <-- only in first --->  │
        └── 3      <-- only in first --->  │
            └── c  <-- only in first --->  │
                   <-- only in second -->  └── d
                   <-- only in second -->      ├── i
                   <-- only in second -->      │   └── 1
                   <-- only in second -->      │       └── a
                   <-- only in second -->      ├── ii
                   <-- only in second -->      │   └── 2
                   <-- only in second -->      │       └── b
                   <-- only in second -->      ├── iii
                   <-- only in second -->      │   └── 3
                   <-- only in second -->      │       └── c
                   <-- only in second -->      └── iv
                   <-- only in second -->          └── 4
                   <-- only in second -->              └── d
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
2. Correctly identifies missing directories

## Expected Exit Codes

Ran from `tests/007`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
