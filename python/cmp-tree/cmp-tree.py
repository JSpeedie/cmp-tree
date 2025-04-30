import filecmp # Possibly unnecessary
import os
from pathlib import Path


class PartialFileComparison():
    def __init__(self, file_cmp: FileCmp, first_ft: SimpleFileType,
        second_ft: SimpleFileType):

        self.file_cmp = file_cmp
        self.first_ft = first_ft
        self.second_ft = second_ft


class FullFileComparison():
    def __init__(self, partial_cmp: PartialFileComparison, first_path: Path,
        second_path: Path):

        self.partial_cmp = partial_cmp
        self.first_path = first_path
        self.second_path = second_path


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
    '''
    # {{{
    extension = Path("")
    return relative_files_in_tree(root, extension)
    # }}}


def compare_directory_trees(first_root: Path, second_root: Path) -> [FullFileComparison]
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
    combined_ftree.append(first_ftree)
    combined_ftree.append(second_ftree)
    # Sort the combined file tree and remove duplicate items
    combined_ftree.sort()
    combined_ftree = list(dict.fromkeys(combined_ftree))

    # Go through all the files in the combined  file list, create two full
    # paths to the file, one rooted at `first_root`, one rooted at
    # `second_root`, and compare them
    for e in combined_ftree:
        FullFileComparison res
        res.first_path = first_root / e
        res.second_path = second_root / e
        res.partial_cmp = compare_path(res.first_path, res.second_path)
        ret.append(res)

    return ret
    # }}}


if __name__ == "__main__":
    # print(files_in_tree(Path("~/kingston1")))
    print(files_in_tree(Path("/home/me/kingston1")))
