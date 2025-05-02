# cmp-tree

### Directory Description

This directory contains the Python implementation of `cmp-tree`. The program
compares two directory trees telling you if they are identical.

> Note: The directory trees used for testing the Python implementation are not
> contained within this current directory tree. They are 2 levels above in
> `[repo-root]/tests`

#### Features of the Python Implementation

* The Python implementation is NOT multithreaded.
* The Python implementation is tested.

&nbsp;

### Installation and Running

```bash
cd python/cmp-tree
python cmp-tree.py [path-to-first-directory] [path-to-second-directory]
```

&nbsp;

### Testing

```bash
pipenv shell
pytest cmp-tree_test.py -v
```
