use std::fs::DirEntry;
use std::path::{Path, PathBuf};


// TODO: remove:    DOC = updated 2025-06-04
/// On success, this function returns an unsorted vector list of `DirEntry`s representing every
/// file (in the broad sense of the word, including links and directories, as well as hidden files)
/// in a directory located at `dir_path`.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `dir_path`: the file path to the directory the contents of which we wish to get a list of
///     `DirEntry` data on.
/// #### Return:
/// * On success, an unsorted vector list of `DirEntry`s representing the basic details (like file
///     name and file type) for all the files in the directory located at `dir_path`.
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error reading a given directory entry
///     * 2 -> There was an error reading the given directory
pub fn dir_entries_of_dir(dir_path: &Path) -> Result<Vec<DirEntry>, (i32, String)> {
    /* {{{ */
    let mut ret: Vec<DirEntry> = Vec::new();

    match std::fs::read_dir(dir_path) {
        Ok(dir_entries) => {
            for e in dir_entries {
                if let Ok(entry) = e {
                    ret.push(entry);
                } else {
                    let err_str: String = format!("{}{}",
                        "Failed to read a directory entry of directory ",
                        dir_path.display());
                    return Err((1, err_str));
                }
            }
        },
        /* If we were completely unable to read the directory */
        Err(_) => {
            let err_str: String = format!("{}{}",
                "Failed to read directory ",
                dir_path.display());
            return Err((2, err_str));
        }
    }

    return Ok(ret);
    /* }}} */
}


// TODO: remove:    DOC = updated 2025-06-04
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
/// * `root`: the beginning of the file path to the directory that roots a directory tree the
///     contents of which we wish to get a list of `SimpleFile` data on. It will be combined with
///     `extension` to produce the complete path. `root` can be an empty path, but if `root` is an
///     empty path, `extension` should not be empty.
/// * `extension`: the end of the file path to the directory that roots a directory tree the
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
///     * 3 -> There was an error recursing further down the directory tree
///     * 4 -> There was an error getting the file type of a given file
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


// TODO: remove:    DOC = updated 2025-06-04
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
/// * `dir_path`: a file path to the directory that roots a directory tree the contents of which we
///     wish to get a list of `SimpleFile` data on.
/// #### Return:
/// * On success, a sorted vector list of `SimpleFile`s representing the basic details (like file
///     path and file type) for all the files in the directory tree rooted at `dir_path`. The file
///     paths included in the list will omit `dir_path` from their path.
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error reading a given directory entry
///     * 2 -> There was an error reading the given directory
///     * 3 -> There was an error recursing further down the directory tree
///     * 4 -> There was an error getting the file type of a given file
#[allow(dead_code)]
pub fn files_in_tree(dir_path: &Path) -> Result<Vec<PathBuf>, (i32, String)> {
    /* {{{ */
    let extension = Path::new("");
    return relative_files_in_tree(dir_path, extension);
    /* }}} */
}


/* Unit tests */
#[test]
fn ut_files_in_tree_001() {
    /* {{{ */
    let root_dir = Path::new("../../tests/001/first");
    /* `expected_ret` would be `Ok(expected_ret_content)` */
    let expected_ret_content = Vec::from([
        Path::new("Lorem.txt"),
        Path::new("cmp_man_pages.txt"),
    ]);

    match files_in_tree(&root_dir) {
        Ok(ret_content) => {
            println!("ret_content = {:#?}", ret_content);
            println!("expected_ret_content = {:#?}", expected_ret_content);
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}

#[test]
fn ut_files_in_tree_002() {
    /* {{{ */
    let root_dir = Path::new("../../tests/003/second");
    /* `expected_ret` would be `Ok(expected_ret_content)` */
    let expected_ret_content = Vec::from([
        Path::new("Lorem.txt"),
        Path::new("cmp_man_pages.txt"),
        Path::new("subdir"),
        Path::new("subdir/linear_gradient.png"),
        Path::new("subdir/rose.png"),
    ]);

    match files_in_tree(&root_dir) {
        Ok(ret_content) => {
            println!("ret_content = {:#?}", ret_content);
            println!("expected_ret_content = {:#?}", expected_ret_content);
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}

#[test]
fn ut_files_in_tree_003() {
    /* {{{ */
    let root_dir = Path::new("../../tests/006/second");
    /* `expected_ret` would be `Ok(expected_ret_content)` */
    let expected_ret_content = Vec::from([
        Path::new("a"),
        Path::new("a/i"),
        Path::new("b"),
        Path::new("b/i"),
        Path::new("b/i/1"),
        Path::new("b/ii"),
        Path::new("b/ii/2"),
        Path::new("c"),
        Path::new("c/i"),
        Path::new("c/i/1"),
        Path::new("c/i/1/a"),
        Path::new("c/ii"),
        Path::new("c/ii/2"),
        Path::new("c/ii/2/b"),
        Path::new("c/iii"),
        Path::new("c/iii/3"),
        Path::new("c/iii/3/c"),
    ]);

    match files_in_tree(&root_dir) {
        Ok(ret_content) => {
            println!("ret_content = {:#?}", ret_content);
            println!("expected_ret_content = {:#?}", expected_ret_content);
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}

#[test]
fn ut_files_in_tree_004() {
    /* {{{ */
    let root_dir = Path::new("../../tests/007/second");
    /* `expected_ret` would be `Ok(expected_ret_content)` */
    let expected_ret_content = Vec::from([
        Path::new("a"),
        Path::new("a/i"),
        Path::new("b"),
        Path::new("b/i"),
        Path::new("b/i/1"),
        Path::new("b/ii"),
        Path::new("b/ii/2"),
        Path::new("d"),
        Path::new("d/i"),
        Path::new("d/i/1"),
        Path::new("d/i/1/a"),
        Path::new("d/ii"),
        Path::new("d/ii/2"),
        Path::new("d/ii/2/b"),
        Path::new("d/iii"),
        Path::new("d/iii/3"),
        Path::new("d/iii/3/c"),
        Path::new("d/iv"),
        Path::new("d/iv/4"),
        Path::new("d/iv/4/d"),
    ]);

    match files_in_tree(&root_dir) {
        Ok(ret_content) => {
            println!("ret_content = {:#?}", ret_content);
            println!("expected_ret_content = {:#?}", expected_ret_content);
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}
