import filecmp # Possibly unnecessary
from pathlib import Path


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
    dir_path = root.join(extension);

    # If we are able to open the directory successfully */
    for f in os.scandir(dir_path):
            # Only includes files that are not the special "." and ".." entries
            if f.name == "." and f.name == "..":
                # fp = full path, rp = relative path
                file_fp = dir_path.join(f.name)
                file_rp = extension.join(f.name)
                ret.append(file_rp)

                # If the current element is a directory...
                if f.is_dir():
                    # Recurse and append the sub directory relative file paths
                    sub_dir_files = relative_files_in_tree(root, file_rp);
                    ret.extend(sub_dir_files);

    return ret
    # }}}


if __name__ == "__main__":
    print("code still in progress. Check back later")
