use std::path::Path;


// Declare `src/compare.rs` as a module
mod compare;
use compare::ComparisonContext;

// Declare `src/config.rs` as a module
pub mod config;
// Re-export `Config` and `default_config()`
// Use statements to get rid of the `config::` prefix
pub use config::{Config,default_config};

// Declare `src/data_structures.rs` as a module
mod data_structures;
// Declare `src/files.rs` as a module
mod files;


/// Takes a `Config` and two `Path`s pointing to two directories and compares the two directory
/// trees rooted at those directories, returning an `i32` representing the appropriate exit code
/// for this program given how the execution went.
///
/// #### Parameters:
/// * `config`: a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `first_root`: a file path that points to the root directory of the first directory tree we
///     wish to compare. This function assumes that this path points to a directory and that the
///     directory exists.
/// * `second_root`: a file path that points to the root directory of the second directory tree we
///     wish to compare. This function assumes that this path points to a directory and that the
///     directory exists.
/// #### Return:
/// * an `i32` that represents how execution of the directory tree comparison went. If there was an
///     error during execution, 2 is returned. If the comparison proceeded without error, but
///     mismatches between files were found, 1 is returned. If the comparison proceeeded without
///     error and no mismatches were found, 0 is returned.
pub fn cmp_tree(config: &Config, first_root: &Path, second_root: &Path) -> i32 {
    /* {{{ */
    let cc = ComparisonContext {
        first_root: first_root,
        second_root: second_root,
        extension: Path::new(""),
    };

    return compare::compare_directory(config, &cc);
    /* }}} */
}
