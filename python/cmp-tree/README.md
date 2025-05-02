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

### Installation

```bash
cd python/cmp-tree
pipenv install  # This will install all dependencies and setup up the virtual environment
```

&nbsp;

### Running

```bash
cd python/cmp-tree
# We use the following command to enter the virtual environment (which has the
correct # version of python and all the dependencies)
pipenv shell
python cmp-tree.py [path-to-first-directory] [path-to-second-directory]
```

&nbsp;

### Testing

```bash
pipenv shell
pytest cmp-tree_test.py -v
```
