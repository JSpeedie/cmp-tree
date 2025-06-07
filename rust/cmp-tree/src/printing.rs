use std::path::Path;


// Use statements to get rid of the `config::` prefix
use crate::config::Config;


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


pub fn print_neither_exist(config: &Config, first_path: &Path, second_path: &Path) {
    /* {{{ */
    if config.pretty { print!("{BOLD}{RED}"); }
    println!("Neither {:?} nor {:?} exist", first_path, second_path);
    if config.pretty { print!("{NORMAL}"); }
    /* }}} */
}


pub fn print_first_file_does_not_exist(config: &Config, first_path: &Path, second_path: &Path) {
    /* {{{ */
    if config.pretty { print!("{BOLD}{RED}"); }
    println!("{:?} does NOT exist, but {:?} does", first_path, second_path);
    if config.pretty { print!("{NORMAL}"); }
    /* }}} */
}


pub fn print_second_file_does_not_exist(config: &Config, first_path: &Path, second_path: &Path) {
    /* {{{ */
    if config.pretty { print!("{BOLD}{RED}"); }
    println!("{:?} exists, but {:?} does NOT exist", first_path, second_path);
    if config.pretty { print!("{NORMAL}"); }
    /* }}} */
}


pub fn print_file_types_mismatch(config: &Config, first_path: &Path, second_path: &Path) {
    /* {{{ */
    if config.pretty { print!("{BOLD}{RED}"); }
    println!("{:?} has a different file type to {:?}", first_path, second_path);
    if config.pretty { print!("{NORMAL}"); }
    /* }}} */
}


pub fn print_files_differ_in_mtime(config: &Config, first_path: &Path, second_path: &Path) {
    /* {{{ */
    if config.pretty { print!("{BOLD}{RED}"); }
    println!("{:?} has different modification time to {:?}", first_path, second_path);
    if config.pretty { print!("{NORMAL}"); }
    /* }}} */
}


// pub fn print_neither_exist(config: &Config, first_path: &Path, second_path: &Path) {
//     /* {{{ */
//     FileCmp::SubstanceRegFileContentMismatch => {
//         if config.pretty { print!("{BOLD}{RED}"); }
//         println!("{:?} differs from {:?}", fc.first_file.file_path, fc.second_file.file_path);
//         if config.pretty { print!("{NORMAL}"); }
//     },
//     FileCmp::SubstanceSoftLinkLinkMismatch => {
//         if config.pretty { print!("{BOLD}{RED}"); }
//         println!("{:?} has a different link path than {:?}", fc.first_file.file_path,
//             fc.second_file.file_path);
//         if config.pretty { print!("{NORMAL}"); }
//     },
//     FileCmp::Match => {
//         if config.matches {
//             if config.pretty { print!("{BOLD}{GREEN}"); }
//             println!("{:?} == {:?}", fc.first_file.file_path, fc.second_file.file_path);
//             if config.pretty { print!("{NORMAL}"); }
//         }
//     },
//     /* }}} */
// }
