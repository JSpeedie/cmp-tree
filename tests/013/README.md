# Test Input 013

This test input has 2 almost-identical directory trees with multiple nested
directories, each with one file contained within. The files in the directory
trees are fully identical in terms of existence, file-type, substance, and
modification time, except for `[...]/02/03/04/05/06/gradients.mov` which
differs in substance and modification time.

```
first                                                                second
├── 02                                                               ├── 02
│   ├── 03                                                           │   ├── 03
│   │   ├── 04                                                       │   │   ├── 04
│   │   │   ├── 05                                                   │   │   │   ├── 05
│   │   │   │   ├── 06                                               │   │   │   │   ├── 06
│   │   │   │   │   └── gradients.mov  <-- differs in substance -->  │   │   │   │   │   └── gradients.mov
│   │   │   │   └── test.mp4                                         │   │   │   │   └── test.mp4
│   │   │   └── linear_gradient.png                                  │   │   │   └── linear_gradient.png
│   │   └── rose.png                                                 │   │   └── rose.png
│   └── Lorem.txt                                                    │   └── Lorem.txt
└── ls_man_pages.txt                                                 └── ls_man_pages.txt
```

## Generating the Test Input

The directory trees associated with the test are built through a series of
commands that are executed by running `generate-test-input.sh`. We can run that
script by running:

```bash
./generate-test-input.sh
```

## Credits

I found the command used to generate `test.mp4` [here](https://vse-docs.readthedocs.io/video_editing/setup/creating-test-files.html).

## The Aim of This Test

The aim of this test is to serve as one of many tests that make sure
`cmp-tree`:
1. Correctly recurses into subdirectories
2. Correctly identifies files that differ in substance but are the same size.
3. Can correctly identify when directories are identical in every way except in
   their modification time.
4. Can work with files larger than 10 MB.

## Expected Exit Codes

Ran from `tests/013`:

* `cmp-tree first/ second/` should exit with an exit code of 1.
* `diff -qr first/ second/` should exit with an exit code of 1.
