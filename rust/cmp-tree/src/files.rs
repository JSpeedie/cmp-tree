use std::fs::DirEntry;
use std::path::{Path, PathBuf};


/// On success, this function returns a sorted vector list of `SimpleFile`s containing the relative
/// file paths for all files (in the broad sense of the word, including links and directories, as
/// well as hidden files) in a directory tree rooted at the directory pointed to by the path `root`
/// / `extension`. The file paths included in the list will all begin with `extension`, but not
/// `root`.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// This function is recursive, and it is often made use of by calling it with `root` as a path to
/// a directory that roots a directory tree and with `extension` set to an empty ("") path.
///
/// #### Parameters:
/// * `root` the beginning of the file path to the directory that roots a directory tree the
///     contents of which we wish to get a list of `SimpleFile` data on. It will be combined with
///     `extension` to produce the complete path. `root` can be an empty path, but if `root` is an
///     empty path, `extension` should not be empty.
/// * `extension` the end of the file path to the directory that roots a directory tree the
///     contents of which we wish to get a list of `SimpleFile` data on. It will be combined with
///     `root` to produce the complete path. `extension` can be an empty path, but if `extension`
///     is an empty path, `root` should not be empty.
/// #### Return:
/// * On success, a sorted vector list of `SimpleFile`s representing the basic details (like
///     file path and file type) for all the files in the directory located at `root` /
///     `extension`. The file paths included in the list will omit `root` from their path, but
///     include `extension`.
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error reading a given directory entry
///     * 2 -> There was an error reading the given directory
///     * 3 -> There was an error getting the file type of a given file
///     * 4 -> There was an error recursing further down the directory tree
pub fn relative_files_in_tree(root: &Path, extension: &Path)
    -> Result<Vec<PathBuf>, (i32, String)> {
    /* {{{ */

    let full_dir_path = root.join(extension);
    let mut dir_contents: Vec<DirEntry> = Vec::new();

    /* Do a first pass of all the contents of the directory and sort them by name for the second
     * pass */
    match std::fs::read_dir(full_dir_path) {
        Ok(dir_entries) => {
            for e in dir_entries {
                match e {
                    Ok(entry) => dir_contents.push(entry),
                    Err(_) => {
                        let err_str: String = format!("{}{}",
                            "Failed to read a directory entry of directory ",
                            root.join(extension).display());
                        return Err((1, err_str));
                    }
                }
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}",
                "Failed to read directory ",
                root.join(extension).display());
            return Err((2, err_str));
        }
    }

    /* Sort all the files in the directory by their file name */
    dir_contents.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    /* Perform the second pass where we recurse all flesh out the full directory tree */
    let mut ret: Vec<PathBuf> = Vec::with_capacity(dir_contents.len());

    for entry in dir_contents {
        if let Ok(ft) = entry.file_type() {
            ret.push(PathBuf::from(extension.join(entry.file_name())));

            if ft.is_dir() {
                match relative_files_in_tree(root, &extension.join(entry.file_name())) {
                    /* Append all the relative paths from the sub dir to our return list */
                    Ok(subdir_dir_tree) => ret.extend(subdir_dir_tree),
                    Err(_) => {
                        let err_str: String = format!("{}{}",
                            "Failed to get the directory tree of ",
                            root.join(extension).display());
                        return Err((3, err_str));
                    }
                }
            }
        } else {
            let err_str: String = format!("{}{}",
                "Could not get the file type of a directory entry of directory ",
                root.join(extension).display());
            return Err((4, err_str));
        }
    }

    return Ok(ret);
    /* }}} */
}


/// On success, this function returns a sorted vector list of `SimpleFile`s containing the relative
/// file paths for all files (in the broad sense of the word, including links and directories, as
/// well as hidden files) in a directory tree rooted at the directory pointed to by the path
/// `dir_path`. The file paths included in the list will NOT begin with `dir_path`.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `dir_path` a file path to the directory that roots a directory tree the contents of which we
///     wish to get a list of `SimpleFile` data on.
/// #### Return:
/// * On success, a sorted vector list of `SimpleFile`s representing the basic details (like file
///     path and file type) for all the files in the directory tree rooted at `dir_path`. The file
///     paths included in the list will omit `dir_path` from their path.
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error reading a given directory entry
///     * 2 -> There was an error reading the given directory
///     * 3 -> There was an error getting the file type of a given file
///     * 4 -> There was an error recursing further down the directory tree
#[allow(dead_code)]
pub fn files_in_tree(dir_path: &Path) -> Result<Vec<PathBuf>, (i32, String)> {
    /* {{{ */
    let extension = Path::new("");
    return relative_files_in_tree(dir_path, extension);
    /* }}} */
}
