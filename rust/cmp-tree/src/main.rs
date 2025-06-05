use clap::{command,Arg}; // For parsing commandline args.
use std::path::Path;
use std::process::exit; // For exiting with an exit code on failure. Not idiomatic.


// This file depends heavily on the contents of lib.rs, which is imported implicitly
// ??

// Use statements to get rid of the `cmp_tree::` prefix (keeping the `config::` prefix!)
use cmp_tree::config;


fn main() {
    let match_result = command!()
        .arg(
            Arg::new("first_root_dir").required(true).index(1)
        )
        .arg(
            Arg::new("second_root_dir").required(true).index(2)
        )
        .arg(
            Arg::new("date").short('d').long("date").num_args(0)
        )
        .arg(
            Arg::new("matches").short('m').long("matches").num_args(0)
        )
        .arg(
            Arg::new("pretty").short('p').long("pretty").num_args(0)
        )
        .arg(
            Arg::new("silent").short('s').long("silent").num_args(0)
        )
        .arg(
            Arg::new("multithread").short('M').long("multithread").num_args(0)
        )
        .arg(
            Arg::new("shallow_pass").long("shallow-pass").num_args(0)
        ).get_matches();

    let first_dir_arg = match_result.get_one::<String>("first_root_dir");
    let second_dir_arg = match_result.get_one::<String>("second_root_dir");

    if first_dir_arg.is_none() {
        eprintln!("Expected 2 paths to 2 directories, received 0\n");
    }
    if second_dir_arg.is_none() {
        eprintln!("Expected 2 paths to 2 directories, received 1\n");
    }

    /* These unwraps are guaranteed not to fail because we checked if the result was none
     * already */
    let first_dir = Path::new(first_dir_arg.unwrap());
    let second_dir = Path::new(second_dir_arg.unwrap());

    // If either of the given directories don't exist, or errors occur when the program tries
    // to access them, exit the program early
    match first_dir.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            eprintln!("ERROR: the first directory tree does not exist or could not be accessed.");
            exit(2)
        },
        Err(_) => {
            eprintln!("ERROR: the first directory tree does not exist or could not be accessed.");
            exit(2)
        },
    }
    match second_dir.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            eprintln!("ERROR: the second directory tree does not exist or could not be accessed.");
            exit(2)
        },
        Err(_) => {
            eprintln!("ERROR: the second directory tree does not exist or could not be accessed.");
            exit(2)
        },
    }

    /* Instantiate a default config */
    let mut conf = config::default_config();

    /* Modify the config as the commandline flags/argument require */
    if match_result.get_flag("matches") { conf.matches = true; }
    if match_result.get_flag("date") { conf.compare_modification_times = true; }
    if match_result.get_flag("pretty") { conf.pretty = true; }
    if match_result.get_flag("silent") { conf.silent = true; }
    if match_result.get_flag("multithread") { conf.multithread = true; }
    if match_result.get_flag("shallow_pass") { conf.shallow_pass = true; }

    /* Call the god function */
    let exit_code: i32 = cmp_tree::cmp_tree(&conf, first_dir, second_dir);
    exit(exit_code);
}
