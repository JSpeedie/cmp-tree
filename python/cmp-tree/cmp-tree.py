from enum import Enum # For enums
from pathlib import Path # For Paths
import argparse # For commandline argument parsing
import filecmp # Currently unused. Should test to see if it's faster than my handwritten stuff
import os # For os.scandir()


# For printing coloured output to the terminal
NOTHING = ""
BOLD = "\x1B[1m"
NORMAL = "\x1B[0m"
RED = "\x1B[31m"
GREEN = "\x1B[32m"
YELLOW = "\x1B[33m"
BLUE = "\x1B[34m"
MAGENTA = "\x1B[35m"
CYAN = "\x1B[36m"
WHITE = "\x1B[37m"


class FileCmp(Enum):
    # For when the two files (understood in the broad sense) match. For regular
    # files, this indicates that the two files are byte-for-byte identical. For
    # directories, this mean they both exist.
    MATCH = 1
    # For when the two files (understood in the broad sense) mismatch in their
    # type (e.g. one is a directory, one is a regular file).
    MISMATCH_TYPE = 2
    # For when the two files (understood in the broad sense) match in their
    # type, but mismatch in their content (e.g. both are regular files, but
    # they are not byte-for-byte identical).
    MISMATCH_CONTENT = 3
    # For when neither of the two files (understood in the broad sense) exist.
    MISMATCH_NEITHER_EXISTS = 4
    # For when only the first of the two files (understood in the broad sense)
    # exists.
    MISMATCH_ONLY_FIRST_EXISTS = 5
    # For when only the second of the two files (understood in the broad sense)
    # exists.
    MISMATCH_ONLY_SECOND_EXISTS = 6

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}"


class SimpleFileType(Enum):
    RegFile = 1
    Directory = 2
    SoftLink = 3

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}"


class Config():
    matches: bool
    pretty: bool
    totals: bool


class PartialFileComparison():
    file_cmp: FileCmp
    first_ft: SimpleFileType
    second_ft: SimpleFileType

    def __repr__(self):
        return f"<{self.__class__.__name__}: " \
            + f"file_cmp: {self.file_cmp}, " \
            + f"first_ft: {self.first_ft}, " \
            + f"second_ft: {self.second_ft}>"


class FullFileComparison():
    partial_cmp: PartialFileComparison
    first_path: Path
    second_path: Path

    def __repr__(self):
        return f"<{self.__class__.__name__}: " \
            + f"partial_cmp: {self.partial_cmp}, " \
            + f"first_path: {self.first_path}, " \
            + f"second_path: {self.second_path}>"


def dedup(lst):
    '''
    Deduplicates elements in a list. To say a little more, this function takes
    a sorted list containing elements of some type that can be assessed for
    inequality and returns a list with all the same elements but with all
    duplicate elements absent.

    Args:
        `lst`: a sorted list of some type of elements which we want to
            deduplicate.

    Returns:
        A list of the same type of elements, but without any duplicate
        elements.

    Raises:
        Nothing.
    '''
    # {{{
    # TODO: what does this check for exactly?
    if not lst:
        return []

    # Put the first element in our return list
    ret = [lst[0]]
    # Iterate through the rest of the input list
    for e in lst[1:]:
        # If the current element is not equal to the last element in our return
        # list ...
        if e != ret[-1]:
            # ... then `e` is a unique item. Add it to the return list
            ret.append(e)

    return ret
    # }}}


def default_config() -> Config:
    '''
    Returns the default config for `cmp-tree`.

    Args:
        Nothing.

    Returns:
        A `Config` with all its values set to the default values for
        `cmp-tree`.

    Raises:
        Nothing.
    '''
    # {{{
    ret = Config()
    ret.matches = False
    ret.pretty = False
    ret.totals = False

    return ret
    # }}}


def get_simplefiletype(filepath: Path) -> SimpleFileType:
    '''
    Returns a `SimpleFileType` representing the file type of the file pointed
    to by `filepath`.

    Args:
        `filepath`: the file path for the file (understood in the broad sense) which we
            wish to get the file type of.

    Returns:
        A `SimpleFileType` representing the file type of the file.

    Raises:
        `Exception`: if the function is unable to classify the file type as one
            of the supported file types in `SimpleFileType`.
    '''
    # {{{
    if filepath.is_file(follow_symlinks=False):
        return SimpleFileType.RegFile
    elif filepath.is_dir(follow_symlinks=False):
        return SimpleFileType.Directory
    elif filepath.is_symlink():
        return SimpleFileType.SoftLink
    else:
        raise Exception("Sorry, files of this type are not supported")
    # }}}


def relative_files_in_tree(root: Path, extension: Path) -> [Path]:
    '''
    Returns an unsorted list of relative file paths for all files (in the broad
    sense of the word, including links and directories, as well as hidden
    files) in a directory tree rooted at the directory pointed to by the path
    of `root` joined with `extension`. The file paths included in the list will
    omit `root` and instead will begin with `extension'.

    Args:
        `root`: the beginning of the file path to the directory for which we
            wish to get a list of all the files in the directory tree. It will
            be combined with `extension` to produce the complete path.
        `extension`: the end of the file path to the directory for which we
            wish to get a list of all the files in the directory tree. It will
            be combined with `root` to produce the complete path.

    Returns:
        An unsorted list of `Path` objects representing the relative file paths
        to all the files in the file tree.

    Raises:
        Nothing.
    '''
    # {{{
    ret = []
    dir_path = root / extension

    # If we are able to open the directory successfully */
    for f in os.scandir(dir_path):
        # Only includes files that are not the special "." and ".." entries
        if f.name != "." and f.name != "..":
            # fp = full path, rp = relative path
            file_fp = dir_path / f.name
            file_rp = extension / f.name
            ret.append(file_rp)

            # If the current element is a directory...
            if f.is_dir():
                # Recurse and append the sub directory relative file paths
                sub_dir_files = relative_files_in_tree(root, file_rp)
                ret.extend(sub_dir_files)

    return ret
    # }}}


def files_in_tree(root: Path) -> [Path]:
    '''
    Returns an unsorted vector list of file paths for all the files (in the
    broad sense of the word, including links and directories, as well as hidden
    files) in a directory tree rooted at the directory pointed to by `root`

    Args:
        `root`: a file path to the directory that roots a directory tree for
            which we wish to get a list of all the files (in the broad sense)
            contained within.

    Returns:
        An unsorted list of the relative file paths for all the files (in the
        broad sense) in the directory tree rooted at `root`.

    Raises:
        Nothing.
    '''
    # {{{
    extension = Path("")
    return relative_files_in_tree(root, extension)
    # }}}


def compare_files(first_path: Path, second_path: Path) -> bool:
    '''
    Takes two paths pointing to two regular files and returns `True` if the
    files are byte-for-byte identical, and `False` if they are not. Both file
    paths must point to regular files and both regular files must exist.

    Args:
        `first_path`: a file path that points to the first regular file we wish
            to compare.
        `second_path`: a file path that points to the second regular file we
            wish to compare.

    Returns:
        `True` if the files are byte-for-byte identical, `False` otherwise.

    Raises:
        Nothing.
    '''
    # {{{
    # Check if the files differ in size. If they do, they cannot be
    # byte-for-byte identical
    first_file_info = first_path.stat(follow_symlinks=False);
    second_file_info = second_path.stat(follow_symlinks=False);

    if first_file_info.st_size != second_file_info.st_size:
        return False

    # Read through both files simultaneously, comparing their bytes. If at any
    # point two bytes at the same location in the files differ, return `False`
    with open(first_path, 'rb') as first_file, \
        open(second_path, 'rb') as second_file:

        while True:
            first_chunk = first_file.read(8192)
            second_chunk = second_file.read(8192)

            # If the read calls failed? TODO: what is this checking exactly?
            if not first_chunk and not second_chunk:
                break;

            # One file ended before the other
            if len(first_chunk) != len(second_chunk):
                return False

            # If the read chunks differ in content.
            # This line check is very important and in Python, the `!=` operator
            # performs a low-level memory comparison (optimized in C) and short
            # circuits, so it is the fastest way to do this very important
            # comparison.
            if first_chunk != second_chunk:
                return False

    # If we haven't returned `False` by this point, these two files must have
    # passed all our checks and must be equal.
    return True;
    # }}}


def compare_path(first_path: Path, second_path: Path) -> PartialFileComparison:
    '''
    Takes two paths and returns a `PartialFileComparison` that represents
    whether the two files pointed to by the two paths are the same or
    different.

    Args:
        `first_path`: a file path that points to the first file we wish to
            compare.
        `second_path`: a file path that points to the second file we wish to
            compare.

    Returns:
        A `PartialFileComparison` that will represents whether the two files
        are equivalent, if they differ and how they differ, as well as the two
        file types of the files.

    Raises:
        Nothing.
    '''
    # {{{

    ret = PartialFileComparison()

    # Check file existences first. If neither path points to files that exist,
    # return that neither exists. If one file exists, but the other does not,
    # get the file mode/type of the existing file and return, setting the
    # comparison member so that the caller knows which file does not exist
    if not first_path.exists(follow_symlinks=False) \
        and not second_path.exists(follow_symlinks=False):

        ret.file_cmp = FileCmp.MISMATCH_NEITHER_EXISTS
        return ret
    elif first_path.exists(follow_symlinks=False) \
        and not second_path.exists(follow_symlinks=False):

        try:
            ret.first_ft = get_simplefiletype(first_path)
        except:
            raise Exception("Sorry, files of this type are not supported")
        ret.file_cmp = FileCmp.MISMATCH_ONLY_FIRST_EXISTS
        return ret
    elif not first_path.exists(follow_symlinks=False) \
        and second_path.exists(follow_symlinks=False):

        try:
            ret.second_ft = get_simplefiletype(second_path)
        except:
            raise Exception("Sorry, files of this type are not supported")
        ret.file_cmp = FileCmp.MISMATCH_ONLY_SECOND_EXISTS
        return ret

    # Check file modes/types. At this point we know both files exist, but if
    # they are of different types (e.g. a fifo vs a regular file) then return
    # with the two file modes/types and setting the comparison member so the
    # caller knows the types of the two files
    try:
        ret.first_ft = get_simplefiletype(first_path)
    except:
        raise Exception("Sorry, files of this type are not supported")
    try:
        ret.second_ft = get_simplefiletype(second_path)
    except:
        raise Exception("Sorry, files of this type are not supported")

    # If the two paths point to files that are of different types (e.g. a
    # directory vs. a symlink, a fifo vs a regular file) then return early,
    # with the match member set to false
    if ret.first_ft != ret.second_ft:
        ret.file_cmp = FileCmp.MISMATCH_TYPE
        return ret

    # Check that the two files are equivalent. At this point we know both files
    # exist and that they are of the same type. The various types the files
    # could both be need individual methods for checking equivalence. Regular
    # files will use check for byte-for-byte equivalence, directories will
    # simply return a match if both exist, soft links will compare link paths
    if ret.first_ft == SimpleFileType.Directory:
        ret.file_cmp = FileCmp.MATCH
        return ret
    elif ret.first_ft == SimpleFileType.RegFile:
        # If the file comparison succeeded we know that this means the two
        # files are byte-for-byte identical. Return with the comparison member
        # set to match
        if compare_files(first_path, second_path) == True:
            ret.file_cmp = FileCmp.MATCH
            return ret
        else:
            ret.file_cmp = FileCmp.MISMATCH_CONTENT
            return ret
    # TODO: Other file types do not yet have support. At the moment, they are
    # treated the same way directories are: if they both exist, and are of the
    # same type, return that they match.
    else:
        ret.file_cmp = FileCmp.MATCH
        return ret
    # }}}


def compare_directory_trees(first_root: Path, second_root: Path) -> [FullFileComparison]:
    '''
    Returns a sorted list of `FullFileComparisons` representing comparisons
    between every file contained in one of the root directories and the file of
    the same relative path in the other root directory. This includes
    comparisons between a file and its non-existent equivalent if there is no
    equivalent in the other root directory. The list is sorted by the relative
    path of each `FullFileComparison`.

    Args:
        `first_root`: the file path to the root of the first directory tree.
        `second_root`: the file path to the root of the second directory tree.

    Returns:
        A list of `FullFileComparisons` representing the comparisons between
        every file contained in both root directories.

    Raises:
        Nothing.
    '''
    # {{{

    ret = []
    # Get the first directory file list and the second directory file list: the
    # list of files in each directory
    first_ftree = files_in_tree(first_root)
    second_ftree = files_in_tree(second_root)

    # Create a vector that contains both the files from the first directory
    # tree and the files from the second directory tree
    combined_ftree = []
    combined_ftree.extend(first_ftree)
    combined_ftree.extend(second_ftree)
    # Sort the combined file tree and remove duplicate items
    combined_ftree.sort()
    combined_ftree = dedup(combined_ftree)

    # Go through all the files in the combined  file list, create two full
    # paths to the file, one rooted at `first_root`, one rooted at
    # `second_root`, and compare them
    for e in combined_ftree:
        res = FullFileComparison()
        res.first_path = first_root / e
        res.second_path = second_root / e
        res.partial_cmp = compare_path(res.first_path, res.second_path)
        ret.append(res)

    return ret
    # }}}


def cmp_tree(conf: Config, first_root: Path, second_root: Path) -> int:
    '''
    Takes a two `Path`s pointing to two directory trees and compares the two
    directory trees, returning an `int` representing the appropriate exit code
    for this program given how the execution went.

    Args:
        `conf`: a `Config` representing a configuration for executing
            `cmp-tree`, usually modified through command line arguments to the
            program.
        `first_root`: a file path that points to the root directory of the first
            directory tree we wish to compare. This function assumes that this
            path points to a directory and that the directory exists.
        `second_root`: a file path that points to the root directory of the
            second directory tree we wish to compare. This function assumes
            that this path points to a directory and that the directory exists.

    Returns:
        An `int` that represents how execution of the directory tree comparison
        went. If there was an error during execution, 2 is returned. If the
        comparison proceeded without error, but mismatches between files were
        found, 1 is returned. If the comparison proceeeded without error and no
        mismatches were found, 0 is returned.

    Raises:
        Nothing.
    '''
    # {{{
    # Compare the directory trees!
    comparisons = compare_directory_trees(first_root, second_root);

    mismatch_occurred = False

    for c in comparisons:
        if c.partial_cmp.file_cmp == FileCmp.MATCH:
            if conf.matches:
                if conf.pretty:
                    print("%s%s" % (BOLD, GREEN), end="")
                    print("\"%s\" == \"%s\"\n" \
                        % (c.first_path, c.second_path), end="")
                if conf.pretty:
                    print("%s" % NORMAL, end="")
        elif c.partial_cmp.file_cmp == FileCmp.MISMATCH_TYPE:
            if conf.pretty:
                print("%s%s" % (BOLD, RED), end="")
            print("\"%s\" is not of the same type as \"%s\"\n" \
                % (c.first_path, c.second_path), end="")
            if conf.pretty:
                print("%s" % NORMAL, end="")
            mismatch_occurred = True
        elif c.partial_cmp.file_cmp == FileCmp.MISMATCH_CONTENT:
            if conf.pretty:
                print("%s%s" % (BOLD, RED), end="")
            print("\"%s\" differs from \"%s\"\n" \
                % (c.first_path, c.second_path), end="")
            if conf.pretty:
                print("%s" % NORMAL, end="")
            mismatch_occurred = True
        elif c.partial_cmp.file_cmp == FileCmp.MISMATCH_NEITHER_EXISTS:
            if conf.pretty:
                print("%s%s" % (BOLD, RED), end="")
            print("Neither \"%s\" nor \"%s\" exist\n" \
                % (c.first_path, c.second_path), end="")
            if conf.pretty:
                print("%s" % NORMAL, end="")
            mismatch_occurred = True
        elif c.partial_cmp.file_cmp == FileCmp.MISMATCH_ONLY_FIRST_EXISTS:
            if conf.pretty:
                print("%s%s" % (BOLD, RED), end="")
            print("\"%s\" exists, but \"%s\" does NOT exist\n" \
                % (c.first_path, c.second_path), end="")
            if conf.pretty:
                print("%s" % NORMAL, end="")
            mismatch_occurred = True
        elif c.partial_cmp.file_cmp == FileCmp.MISMATCH_ONLY_SECOND_EXISTS:
            if conf.pretty:
                print("%s%s" % (BOLD, RED), end="")
            print("\"%s\" does NOT exist, but \"%s\" does exist\n" \
                % (c.first_path, c.second_path), end="")
            if conf.pretty:
                print("%s" % NORMAL, end="")
            mismatch_occurred = True

    if mismatch_occurred:
        return 1
    else:
        return 0
    # }}}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    # Add the two positional arguments
    parser.add_argument('first_root', type=Path)
    parser.add_argument('second_root', type=Path)
    # Add the flag-based arguments
    parser.add_argument('-m','--matches', action='store_true')
    parser.add_argument('-p','--pretty', action='store_true')
    # TODO: functionality unimplemented
    #parser.add_argument('-t','--totals', action='store_true')
    # Process the arguments given to this program
    args = parser.parse_args()

    # Get the default config
    conf = default_config()
    # And modify it as the commandline args demand
    if args.matches == True:
        conf.matches = True
    if args.pretty == True:
        conf.pretty = True
    # TODO: functionality unimplemented
    #if args.totals == True:
    #    conf.totals = True

    exit_code = cmp_tree(conf, args.first_root, args.second_root)
    exit(exit_code)
