# cmp-tree

### Project Description

This is the Python implementation of `cmp-tree`. The program compares two
directory trees telling you if they are identical.

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
