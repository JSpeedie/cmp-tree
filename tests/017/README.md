# Test Input 017

This test input has directory trees with contents fully identical in terms of
existence, file-type, substance, and modification time. Importantly,
`[...]/empty_file.txt` has 0 bytes of data.


```
first                   second
└── empty_file.txt      └── empty_file.txt
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
1. Correctly handles regular files that contain no data.

## Expected Exit Codes

Ran from `tests/017`:

* `cmp-tree first/ second/` should exit with an exit code of 0.
* `diff -qr first/ second/` should exit with an exit code of 0.
