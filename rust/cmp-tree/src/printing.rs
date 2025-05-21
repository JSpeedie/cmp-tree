// Use statements to get rid of the `config::` prefix
use crate::config::Config;

// Use statements to get rid of the `data_structures::` prefix
use crate::data_structures::FileCmp;
use crate::data_structures::FullFileComparison;
use crate::totals::Totals;


/* For printing coloured output */
#[allow(dead_code)]
const NOTHING: &str = "";
const BOLD: &str = "\x1B[1m";
const NORMAL: &str = "\x1B[0m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
#[allow(dead_code)]
const YELLOW: &str = "\x1B[33m";
#[allow(dead_code)]
const BLUE: &str = "\x1B[34m";
#[allow(dead_code)]
const MAGENTA: &str = "\x1B[35m";
#[allow(dead_code)]
const CYAN: &str = "\x1B[36m";
#[allow(dead_code)]
const WHITE: &str = "\x1B[37m";


/// Takes a `FullFileComparison` and prints out the necessary information about it. What
/// information is printed will depend on the values of `config`.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `full_comp` a `FullFileComparison` containing all the information about the 2 files that
///     were compared.
pub fn print_one_comparison(config: &Config, full_comp: &FullFileComparison) {
    /* {{{ */
    match full_comp.partial_cmp.file_cmp {
        FileCmp::ExistenceNeitherFile => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("Neither {:?} nor {:?} exist", full_comp.first_path, full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::ExistenceOnlyFirstFile => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} exists, but {:?} does NOT exist", full_comp.first_path,
                full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::ExistenceOnlySecondFile => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} does NOT exist, but {:?} does exist", full_comp.first_path,
                full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::FileTypeTypeMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} is not of the same type as {:?}", full_comp.first_path,
                full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::SubstanceRegFileContentMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} differs from {:?}", full_comp.first_path, full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::SubstanceSoftLinkLinkMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} has a different link path than {:?}", full_comp.first_path,
                full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::MetadataModificationTimeMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} has different modification time to {:?}", full_comp.first_path,
                full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::Match => {
            if config.matches {
                if config.pretty { print!("{BOLD}{GREEN}"); }
                println!("{:?} == {:?}", full_comp.first_path, full_comp.second_path);
                if config.pretty { print!("{NORMAL}"); }
            }
        },
    }
    /* }}} */
}


/// Takes a `Result` possibly containing a `Vec` of `FullFileComparison`s and prints out the
/// necessary information about the list of file comparisons if the `directory_tree_comparison`
/// is not an `Err`. What information is printed will depend on the values of `config`.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `directory_tree_comparison` a `Result` possibly containing a `Vec` of `FullFileComparison`s.
///     Typically, this parameter is the result of a call to `compare_directory_trees()`.
pub fn print_output(config: &Config, directory_tree_comparison: &Vec<FullFileComparison>) {
    /* {{{ */

    for e in directory_tree_comparison {
        /* Print what needs to be printed for the current comparison. This function call may very
         * well print nothing */
        print_one_comparison(&config, &e);
    }
    /* }}} */
}


/// Takes a `Totals` representing information about the number of matches for various file types
/// out of the maximum number of matches possible for each file type and prints it.
///
/// #### Parameters:
/// * `totals_count` a `Totals` representing various counts (such as the actual number of regular
///     file matches out of the maximum possible number of regular file matches) from an execution
///     of `cmp-tree`.
pub fn print_totals(totals_count: &Totals) {
    /* {{{ */
    println!("All done!");
    println!("File byte-for-byte matches: {0}/{1}",
        totals_count.file_matches,
        totals_count.max_file_matches);
    println!("Directory matches: {0}/{1}",
        totals_count.dir_matches,
        totals_count.max_dir_matches);
    println!("Soft link matches: {0}/{1}",
        totals_count.softlink_matches,
        totals_count.max_softlink_matches);
    /* }}} */
}
