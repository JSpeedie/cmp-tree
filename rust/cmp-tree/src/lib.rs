use std::path::{Path, PathBuf};
use std::thread::{available_parallelism,scope,ScopedJoinHandle};

// Declare `src/compare.rs` as a module
pub mod compare;

// Declare `src/config.rs` as a module
pub mod config;
// Re-export `Config` and `default_config()`
// Use statements to get rid of the `config::` prefix
pub use config::{Config,default_config};

// Declare `src/data_structures.rs` as a module
pub mod data_structures;
// Use statements to get rid of the `data_structures::` prefix
use data_structures::{FileCmp,FullFileComparison};

// Declare `src/printing.rs` as a module
pub mod printing;

// Declare `src/totals.rs` as a module
pub mod totals;
// Use statements to get rid of the `totals::` prefix
use totals::Totals;


/// Intended as a helper function for `files_in_tree()`. Returns an unsorted vector list of
/// relative file paths for all files (in the broad sense of the word, including links and
/// directories, as well as hidden files) in a directory tree rooted at the directory pointed to by
/// the path `root` / `extension`. The file paths included in the list will all begin with
/// `extension`, but not `root`. This function is recursive, and it is often made use of by calling
/// it with `root` as a path to a directory that roots a directory tree and with `extension`
/// set to an empty ("") path.
///
/// #### Parameters:
/// * `root` the beginning of the file path to the directory for which we wish to get a list of
///      all the files in the directory tree. It will be combined with `extension` to produce the
///      complete path.
/// * `extension` the end of the file path to the directory for which we wish to get a list of all
///     the files in the directory tree. It will be combined with `root` to produce the complete
///     path. `extension` can be an empty path.
/// #### Return:
/// * an unsorted vector list of the relative file paths for all the files in the directory tree
///     rooted at `root` / `extension`. The file paths included in the list will omit `root` from
///     their path, but include `extension`.
fn relative_files_in_tree(root: &Path, extension: &Path) -> Vec<PathBuf> {
    /* {{{ */
    let full_dir_path = root.join(extension);
    let mut ret: Vec<PathBuf> = Vec::new();

    /* Get all the files in the dir relative to the 'root' directory */
    match std::fs::read_dir(full_dir_path) {
        Ok(dir_entries) => {
            for e in dir_entries {
                match e {
                    Ok(entry) => {
                        if let Ok(file_type) = entry.file_type() {
                            let rel_path: PathBuf = extension.join(entry.file_name());
                            ret.push(rel_path);

                            if file_type.is_dir() {
                                let subdir_rel_paths = relative_files_in_tree(root,
                                    &extension.join(entry.file_name()));
                                /* Append all the relative paths from the sub dir to our return
                                 * list */
                                ret.extend(subdir_rel_paths);
                            }
                        } else {
                            println!("Error getting the file type of the directory
                                entry");
                        }
                    },
                    Err(_) => {
                        println!("Error reading one of the directory entries");
                    }
                }
            }
        },
        Err(_) => {
            println!("Error reading contents of the directory");
        }
    }

    return ret;
    /* }}} */
}


/// Returns an unsorted vector list of relative file paths for all the files (in the broad sense of
/// the word, including links and directories, as well as hidden files) in a directory tree rooted
/// at the directory pointed to by `root`.
///
/// #### Parameters:
/// * `root` the file path to the directory for which we wish to get a list of all the files in the
///     directory tree.
/// #### Return:
/// * an unsorted vector list of the relative file paths for all the files in the directory tree
///     rooted at `root`.
fn files_in_tree(root: &Path) -> Vec<PathBuf> {
    /* {{{ */
    let extension = Path::new("");
    return relative_files_in_tree(root, extension);
    /* }}} */
}


/// Takes two paths, each pointing to directories that root directory trees and returns a `Result`
/// that either contains a `Vec` of `FullFileComparison`s, representing a list of comparisons
/// between all files in the directory trees, or an empty `Err`, indicating that an error occurred
/// in the process of comparing the two directory trees.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `first_root` a file path that points to the root directory of the first directory tree we
///     wish to compare.
/// * `second_root` a file path that points to the root directory of the second directory tree we
///     wish to compare.
/// #### Return:
/// * a `Result<Vec<FullFileComparison>, ()>` that represents whether this directory tree
///     comparison was able to be caried out successfully or not. If the `Result` turns out to be
///     the `Vec<FullFileComparison>`, then the caller is given a sorted list of all the file
///     comparisons that were performed during the comparison of the two directory trees. If the
///     `Result` turns out to be an `Err`, then this function experienced some sort of error.
fn compare_directory_trees(config: &Config, first_root: &Path, second_root: &Path) -> 
    Result<Vec<FullFileComparison>, ()> {
    /* {{{ */

    let mut ret: Vec<FullFileComparison> = Vec::new();
    /* Get the first directory file list and the second directory file list: the list of files in
     * each directory */
    let first_ft: Vec<PathBuf> = files_in_tree(first_root);
    let second_ft: Vec<PathBuf> = files_in_tree(second_root);

    /* Combine all the relative paths from the first and second directory roots into one combined
     * list of relative paths */
    let mut combined_ft: Vec<PathBuf> = Vec::new();
    combined_ft.extend(first_ft);
    combined_ft.extend(second_ft);
    /* Sort the combined file tree and remove duplicate items */
    combined_ft.sort();
    combined_ft.dedup();
    /* We know our return array we be at most `combined_ft` length. It will only be shorter if
     * errors are encountered when comparing files */
    ret.reserve(combined_ft.len());

    /* If the configuration limits the program to a single thread, perform the directory tree
     * comparison using a single thread */
    if config.single_threaded {
        /* Go through all the file paths in the combined  file list, creating two full paths to the
         * file, one rooted at `first_root`, one rooted at `second_root`, and compare them */
        for e in &combined_ft {
            let first_path = first_root.join(e);
            let second_path = second_root.join(e);

            let cmp_res = compare::compare_files(config, &first_path, &second_path);

            if cmp_res.is_ok() {
                ret.push(
                    FullFileComparison {
                        first_path: first_path,
                        second_path: second_path,
                        partial_cmp: cmp_res.unwrap(),
                    }
                );
            }
        }

        return Ok(ret);
    }

    /* If we make it here that means the program has not been limited to a single thread */

    /* Find out how many cores the computer has. If we fail to get that info, default to 1 thread
     * */
    let num_threads: usize = match available_parallelism() {
        Ok(cores) => cores.get(),
        _ => 1,
    };
    /* Calculate how many file pairs each thread needs to compare. Perform a ceiled division
     * through manual math to make sure every element is a member of some chunk */
    let chunk_size: usize =
        (combined_ft.len() + num_threads - 1) / num_threads;

    scope(|s| {
        let mut thread_handles: Vec<ScopedJoinHandle<'_, Vec<FullFileComparison>>> = Vec::new();
        thread_handles.reserve(num_threads);

        for chunk in combined_ft.chunks(chunk_size) {
            thread_handles.push(s.spawn(move || -> Vec<FullFileComparison> {
                let mut ret_vec: Vec<FullFileComparison> = Vec::new();
                ret_vec.reserve(chunk.len());

                for file_pair in chunk {
                    /* Go through all the file pairs assigned to this thread, creating two full
                     * paths to the file, one rooted at `first_root`, one rooted at `second_root`,
                     * and compare them */
                    let first_file = first_root.join(file_pair);
                    let second_file = second_root.join(file_pair);

                    let cmp_res = compare::compare_files(config, &first_file, &second_file);

                    if cmp_res.is_ok() {
                        ret_vec.push(
                            FullFileComparison {
                                first_path: first_file,
                                second_path: second_file,
                                partial_cmp: cmp_res.unwrap(),
                            }
                        );
                    }
                }

                return ret_vec;
            }));
        }
        /* Join all threads in order of creation */
        for handle in thread_handles.into_iter() {
            match handle.join() {
                /* If the thread succeeded, go through its `ret_vec` and copy all its contents
                 * `ret` */
                Ok(ret_list) => {
                    ret.extend(ret_list);
                },
                _ => (),
            }
        }
    });

    return Ok(ret);
    /* }}} */
}




/// Takes a `Vec` of `FullFileComparison`s representing a directory tree comparison and returns a a
/// boolean representing whether or not the file comparison list received as input contains any
/// mismatches or not.
///
/// #### Parameters:
/// * `directory_tree_comparison` a `Vec` of `FullFileComparison`s. Typically, this parameter is
///     the unwrapped result of a call to `compare_directory_trees()`.
/// #### Return:
/// * a `bool` that will be `true` if there WERE any mismatches in the directory tree comparison
///     and `false` if the directory tree comparison found the two directory trees to be identical.
fn directory_tree_comparison_contains_mismatch(
    directory_tree_comparison: &Vec<FullFileComparison>) -> Result<bool, ()> {
    /* {{{ */

    /* For every comparison in the list... */
    for e in directory_tree_comparison {
        /* If the comparison found a mismatch of any kind between the two files, return early */
        match e.partial_cmp.file_cmp {
            FileCmp::Match => (),
            _ => return Ok(true),
        }
    }
    /* If we make it here, that means no mismatches of any kind were found in the file comparison
     * list. */
    return Ok(false);
    /* }}} */
}


/// Takes a `Config` and two `Path`s pointing to two directory trees and compares the two directory
/// trees, returning an `i32` representing the appropriate exit code for this program given how the
/// execution went.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `first_dir` a file path that points to the root directory of the first directory tree we
///     wish to compare. This function assumes that this path points to a directory and that the
///     directory exists.
/// * `second_dir` a file path that points to the root directory of the second directory tree we
///     wish to compare. This function assumes that this path points to a directory and that the
///     directory exists.
/// #### Return:
/// * an `i32` that represents how execution of the directory tree comparison went. If there was an
///     error during execution, 2 is returned. If the comparison proceeded without error, but
///     mismatches between files were found, 1 is returned. If the comparison proceeeded without
///     error and no mismatches were found, 0 is returned.
pub fn cmp_tree(config: &Config, first_dir: &Path, second_dir: &Path) -> i32 {
    /* {{{ */
    /* Perform the comparison between the two directory trees */
    let directory_tree_comparison_res = compare_directory_trees(&config, first_dir, second_dir);
    if let Err(_) = directory_tree_comparison_res {
        println!("ERROR: Failed to compare the directory trees");
        return 2;
    }

    /* If we make it to this point, this means our directory tree comparison succeeded. Unwrap
     * safely */
    let directory_tree_comparison = directory_tree_comparison_res.unwrap();

    /* Check if any mismatches occurred (this is needed to determine the exit code of this program
    * */
    let mismatch_occurred =
        directory_tree_comparison_contains_mismatch(&directory_tree_comparison);
    /* Print the appropriate output, provided silent mode is off */
    if !config.silent {
        printing::print_output(&config, &directory_tree_comparison);
    }
    if config.totals {
        let totals_count = Totals::calculate_from(&directory_tree_comparison);
        printing::print_totals(&totals_count);
    }

    /* If a mismatch occurred during the comparison, exit with exit code 1. If there were no
     * mismatches, and the directory trees are identical, exit with exit code 0. If there was an
     * error in assessing whether there was any mismatch in the directory tree comparison, exit
     * with exit code 2. */
    match mismatch_occurred {
        Ok(true) => return 1,
        Ok(false) => return 0,
        Err(_) => return 2,
    }
    /* }}} */
}


/* Unit tests */
#[cfg(test)]
mod unit_tests;
