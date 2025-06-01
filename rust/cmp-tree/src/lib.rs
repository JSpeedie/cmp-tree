use std::fs::DirEntry;
use std::path::{Path, PathBuf};


mod compare;
pub mod config;
// Re-export `Config` and `default_config()`
// Use statements to get rid of the `config::` prefix
pub use config::{Config,default_config};
mod data_structures;
use data_structures::{FileCmp,SimpleFileType};
mod files;


struct ComparisonContext<'a> {
    pub first_root: &'a Path,
    pub second_root: &'a Path,
    pub extension: &'a Path,
}


// TODO: write doc
// TODO: move to files.rs
fn dir_entries_of_dir(dir_path: &Path) -> Result<Vec<DirEntry>, (i32, String)> {
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


struct FileTypeCmp {
    pub cmp: FileCmp,
    pub first_ft: SimpleFileType,
    #[allow(dead_code)]
    pub second_ft: SimpleFileType,
}


// TODO: write doc
fn compare_file_types(cc: &ComparisonContext, first_de: &DirEntry, second_de: &DirEntry)
    -> Result<FileTypeCmp, (i32, String)> {
    /* {{{ */

    let first_ft: SimpleFileType;
    let second_ft: SimpleFileType;

    match first_de.file_type() {
        Ok(ft) => {
            match SimpleFileType::try_from(&ft) {
                Ok(sft) => first_ft = sft,
                Err(_) => {
                    let err_str: String = format!("{}{}",
                        "Failed to get the file type of file ",
                        cc.first_root.join(first_de.file_name()).display());
                    return Err((1, err_str));
                }
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}",
                "Failed to get the file type of file ",
                cc.first_root.join(first_de.file_name()).display());
            return Err((1, err_str));
        }
    }
    match second_de.file_type() {
        Ok(ft) => {
            match SimpleFileType::try_from(&ft) {
                Ok(sft) => second_ft = sft,
                Err(_) => {
                    let err_str: String = format!("{}{}",
                        "Failed to get the file type of file ",
                        cc.second_root.join(second_de.file_name()).display());
                    return Err((1, err_str));
                }
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}",
                "Failed to get the file type of file ",
                cc.second_root.join(second_de.file_name()).display());
            return Err((1, err_str));
        }
    }

    let cmp: FileCmp;
    if first_ft == second_ft {
        cmp = FileCmp::Match;
    } else {
        cmp = FileCmp::Mismatch;
    }

    return Ok(FileTypeCmp {
        cmp: cmp,
        first_ft: first_ft,
        second_ft: second_ft,
    });
    /* }}} */
}


// TODO: write doc
// TODO: break up and simplify - the function is currently 280 lines long!
fn compare_directory(config: &Config, cc: &ComparisonContext) -> i32 {
    /* {{{ */
    /* Get the directory contents of both directories */
    let mut first_dir: Vec<DirEntry>;
    let mut second_dir: Vec<DirEntry>;
    let mut mismatch_occurred: bool = false;

    match dir_entries_of_dir(&cc.first_root.join(cc.extension)) {
        Ok(dir_entries) => first_dir = dir_entries,
        Err((_, err_str)) => {
            eprintln!("{}{}", "Error: ", err_str);
            return 2;
        },
    }

    match dir_entries_of_dir(&cc.second_root.join(cc.extension)) {
        Ok(dir_entries) => second_dir = dir_entries,
        Err((_, err_str)) => {
            eprintln!("{}{}", "Error: ", err_str);
            return 2;
        },
    }

    /* Sort the directory entries of each directory by their file name */
    first_dir.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    second_dir.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < first_dir.len() && j < second_dir.len() {
        if first_dir[i].file_name() == second_dir[j].file_name() {
            /* Compare the file types of the two files */
            match compare_file_types(&cc, &first_dir[i], &second_dir[j]) {
                Ok(ft_cmp) => {
                    if ft_cmp.cmp == FileCmp::Mismatch {
                        eprintln!("\"{}\" has a different file type to \"{}\"",
                            cc.first_root.join(first_dir[i].file_name()).display(),
                            cc.second_root.join(second_dir[j].file_name()).display());
                        mismatch_occurred = true;
                    } else {
                        let first_fpath: PathBuf =
                            cc.first_root.join(cc.extension.join(first_dir[i].file_name()));
                        let second_fpath: PathBuf =
                            cc.second_root.join(cc.extension.join(second_dir[i].file_name()));
                        /* If `ft_cmp.cmp == FileCmp::Match` then we are guaranteed that
                         * `ft_cmp.first_ft == ft_cmp.second_ft` */
                        match compare::compare_substance(&ft_cmp.first_ft, &first_fpath, &second_fpath) {
                            Ok(FileCmp::Match) => {
                                if config.compare_modification_times {
                                    match compare::compare_modification_time(
                                        config.follow_softlinks, &ft_cmp.first_ft, &first_fpath,
                                        &second_fpath) {

                                        Ok(FileCmp::Match) => {
                                            if config.matches {
                                                println!("[..]/{} matches in existence, file type, substance, and modification time",
                                                    cc.extension.join(first_dir[i].file_name()).display());
                                            }
                                        },
                                        Ok(FileCmp::Mismatch) => {
                                            eprintln!("\"{}\" differs in modification time from \"{}\"",
                                                cc.first_root.join(first_dir[i].file_name()).display(),
                                                cc.second_root.join(second_dir[j].file_name()).display());
                                            mismatch_occurred = true;
                                        },
                                        Err(_) => {
                                            eprintln!("Error: comparing the modification time of the two files \"{}\", and \"{}\"",
                                                cc.first_root.join(first_dir[i].file_name()).display(),
                                                cc.second_root.join(second_dir[j].file_name()).display());
                                            return 2;
                                        },
                                    }
                                } else {
                                    if config.matches {
                                        println!("[..]/{} matches in existence, file type and substance",
                                            cc.extension.join(first_dir[i].file_name()).display());
                                    }
                                }
                            },
                            Ok(FileCmp::Mismatch) => {
                                eprintln!("\"{}\" differs in content from \"{}\"",
                                    cc.first_root.join(first_dir[i].file_name()).display(),
                                    cc.second_root.join(second_dir[j].file_name()).display());
                                mismatch_occurred = true;
                            },
                            Err(_) => {
                                eprintln!("Error: comparing the content of the two files \"{}\", and \"{}\"",
                                    cc.first_root.join(first_dir[i].file_name()).display(),
                                    cc.second_root.join(second_dir[j].file_name()).display());
                                return 2;
                            },
                        }

                        /* Recurse if both dir entries are directories */
                        if ft_cmp.first_ft == SimpleFileType::Directory {
                            /* Recurse on first_dir[i].file_name() */
                            let new_cc = ComparisonContext {
                                first_root: cc.first_root,
                                second_root: cc.second_root,
                                extension: &cc.extension.join(first_dir[i].file_name()),
                            };
                            let cmp_exit_code = compare_directory(config, &new_cc);
                            if 1 == cmp_exit_code { mismatch_occurred = true; }
                        }
                    }
                },
                Err((_, err_str)) => {
                    eprintln!("{}{}", "Error: ", err_str);
                    return 2;
                },
            }

            i += 1;
            j += 1;
        /* If the file represented by `first_dir[i]` does not exist in the second dir tree */
        } else if first_dir[i].file_name() < second_dir[j].file_name() {
            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                cc.first_root.join(first_dir[i].file_name()).display(),
                cc.second_root.join(first_dir[i].file_name()).display());
            mismatch_occurred = true;
            /* If first_dir[i] is a directory... */
            let is_dir: bool;
            match first_dir[i].file_type() {
                Ok(ft) => is_dir = ft.is_dir(),
                Err(_) => {
                    eprintln!("{}{}",
                        "Error: Failed to get the file type of file ",
                        cc.first_root.join(first_dir[i].file_name()).display());
                    return 2;
                }
            }
            if is_dir {
                /* Recurse on the lonely directory so we can print all the files that are missing
                 * */
                match files::relative_files_in_tree(&cc.first_root.join(cc.extension),
                    Path::new(&first_dir[i].file_name())) {

                    Ok(file_list) => {
                        for f in file_list {
                            let first_fpath = cc.first_root.join(&f);
                            let second_fpath = cc.second_root.join(&f);

                            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                                first_fpath.display(), second_fpath.display());
                        }
                    },
                    Err((_, err_str)) => {
                        eprintln!("Error: {}", err_str);
                    }
                }
            }
            i += 1;
        /* If the file represented by `second_dir[j]` does not exist in the first dir tree */
        /* first_dir[i].file_name() > second_dir[j].file_name() */
        } else {
            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                cc.second_root.join(second_dir[j].file_name()).display(),
                cc.first_root.join(second_dir[j].file_name()).display());
            mismatch_occurred = true;
            /* If first_dir[i] is a directory... */
            let is_dir: bool;
            match first_dir[i].file_type() {
                Ok(ft) => is_dir = ft.is_dir(),
                Err(_) => {
                    eprintln!("{}{}",
                        "Error: Failed to get the file type of file ",
                        cc.first_root.join(first_dir[i].file_name()).display());
                    return 2;
                }
            }
            if is_dir {
                /* Recurse on the lonely directory so we can print all the files that are missing
                 * */
                match files::relative_files_in_tree(&cc.second_root.join(cc.extension),
                    Path::new(&second_dir[j].file_name())) {

                    Ok(file_list) => {
                        for f in file_list {
                            let first_fpath = cc.first_root.join(&f);
                            let second_fpath = cc.second_root.join(&f);

                            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                                second_fpath.display(), first_fpath.display());
                        }
                    },
                    Err((_, err_str)) => {
                        eprintln!("Error: {}", err_str);
                    }
                }
            }
            j += 1;
        }
    }

    while i < first_dir.len() {
        eprintln!("\"{}\" exists, but \"{}\" does NOT",
            cc.first_root.join(first_dir[i].file_name()).display(),
            cc.second_root.join(first_dir[i].file_name()).display());
        mismatch_occurred = true;
        /* If first_dir[i] is a directory... */
        let is_dir: bool;
        match first_dir[i].file_type() {
            Ok(ft) => is_dir = ft.is_dir(),
            Err(_) => {
                eprintln!("{}{}",
                    "Error: Failed to get the file type of file ",
                    cc.first_root.join(first_dir[i].file_name()).display());
                return 2;
            }
        }
        if is_dir {
            /* Recurse on the lonely directory so we can print all the files that are missing */
            match files::relative_files_in_tree(&cc.first_root.join(cc.extension),
                Path::new(&first_dir[i].file_name())) {

                Ok(file_list) => {
                    for f in file_list {
                        let first_fpath = cc.first_root.join(&f);
                        let second_fpath = cc.second_root.join(&f);

                        eprintln!("\"{}\" exists, but \"{}\" does NOT",
                            first_fpath.display(), second_fpath.display());
                    }
                },
                Err((_, err_str)) => {
                    eprintln!("Error: {}", err_str);
                }
            }
        }
        i += 1;
    }

    while j < second_dir.len() {
        eprintln!("\"{}\" exists, but \"{}\" does NOT",
            cc.second_root.join(second_dir[j].file_name()).display(),
            cc.first_root.join(second_dir[j].file_name()).display());
        mismatch_occurred = true;
        /* If second_dir[j] is a directory... */
        let is_dir: bool;
        match second_dir[j].file_type() {
            Ok(ft) => is_dir = ft.is_dir(),
            Err(_) => {
                eprintln!("{}{}",
                    "Error: Failed to get the file type of file ",
                    cc.first_root.join(first_dir[i].file_name()).display());
                return 2;
            }
        }
        if is_dir {
            /* Recurse on the lonely directory so we can print all the files that are missing */
            match files::relative_files_in_tree(&cc.second_root.join(cc.extension),
                Path::new(&second_dir[j].file_name())) {

                Ok(file_list) => {
                    for f in file_list {
                        let first_fpath = cc.first_root.join(&f);
                        let second_fpath = cc.second_root.join(&f);

                        eprintln!("\"{}\" exists, but \"{}\" does NOT",
                            second_fpath.display(), first_fpath.display());
                    }
                },
                Err((_, err_str)) => {
                    eprintln!("Error: {}", err_str);
                }
            }
        }
        j += 1;
    }

    if mismatch_occurred {
        return 1;
    } else {
        return 0;
    }
    /* }}} */
}


// TODO: write doc
pub fn cmp_tree(config: &Config, first_root: &Path, second_root: &Path) -> i32 {
    /* {{{ */
    let cc = ComparisonContext {
        first_root: first_root,
        second_root: second_root,
        extension: Path::new(""),
    };

    return compare_directory(config, &cc);
    /* }}} */
}
