use std::fs::{DirEntry,File,Metadata,read_link};
use std::io::Read;
use std::path::{Path,PathBuf};


use crate::config::Config;
use crate::data_structures::{FileCmp,SimpleFileType};
use crate::files;


/* Used to represent an ongoing `cmp-tree` comparison */
pub struct ComparisonContext<'a> {
    pub first_root: &'a Path,
    pub second_root: &'a Path,
    pub extension: &'a Path,
}


/* Used as a return type for `compare_file_types()` */
struct FileTypeCmp {
    pub cmp: FileCmp,
    pub first_ft: SimpleFileType,
    #[allow(dead_code)]
    pub second_ft: SimpleFileType,
}


// TODO: remove:    DOC = updated 2025-06-04
/// Takes two paths and returns a result representing how the files compare. Both file paths must
/// point to regular files and both regular files must exist.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `config`: a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `first_path`: a file path that points to the first file we wish to compare.
/// * `second_path`: a file path that points to the second file we wish to compare.
/// #### Return:
/// * On success, a `FileCmp` that represents whether the two regular files are equivalent in terms
///     of their substance (`Match` or `Mismatch`).
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error getting the metadata of one of the files
///     * 2 -> There was an error opening one of the files for reading
///     * 3 -> There was an error encountered when reading one of the files
fn compare_regular_files(config: &Config, first_path: &Path, second_path: &Path)
    -> Result<FileCmp, (i32, String)> {
    /* {{{ */

    /* Check if the files differ in size. If they do, they cannot be byte-for-byte identical */
    match first_path.metadata() {
        Ok(first_md) => match second_path.metadata() {
            Ok(second_md) => {
                if first_md.len() != second_md.len() {
                    return Ok(FileCmp::Mismatch);
                }
            },
            Err(_) => {
                let err_str: String = format!("{}{}", "Failed to get the metadata of file ",
                    second_path.display());
                return Err((1, err_str));
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to get the metadata of file ",
                first_path.display());
            return Err((1, err_str));
        }
    }

    /* If the config dictates that we do a shallow pass and the files match in size, then return
     * early that the files match skipping the part where we compare the contents of the regular
     * files */
    if config.shallow_pass {
        return Ok(FileCmp::Match);
    }

    const BYTE_COUNT: usize = 8192;
    let first_file_res = File::open(first_path);
    let second_file_res = File::open(second_path);
    let mut first_file: File;
    let mut second_file: File;
    let mut first_buf = [0; BYTE_COUNT];
    let mut second_buf = [0; BYTE_COUNT];

    if first_file_res.is_ok() {
        first_file = first_file_res.unwrap();
    } else {
        let err_str: String = format!("{}{}", "Failed to open a file for reading ",
            first_path.display());
        return Err((2, err_str));
    }
    if second_file_res.is_ok() { second_file = second_file_res.unwrap(); }
    else {
        let err_str: String = format!("{}{}", "Failed to open a file for reading ",
            second_path.display());
        return Err((2, err_str));
    }

    loop {
        match first_file.read(&mut first_buf) {
            Ok(first_bytes_read) => match second_file.read(&mut second_buf) {
                Ok(second_bytes_read) => {
                    /* One file ended before the other */
                    if first_bytes_read != second_bytes_read {
                        return Ok(FileCmp::Mismatch);
                    }
                    /* If both reads read 0 bytes, that means we have hit the end of both files and
                     * the two files are identical */
                    if first_bytes_read == 0 && second_bytes_read == 0 {
                        return Ok(FileCmp::Match);
                    }
                    /* This `if` statement is very important. The comparison here, done using the
                     * `==` operator is (as far as I understand) optimized in the same way
                     * `memcmp()` is in C and C++. What this means is that this operation can be
                     * vectorized (if the CPU architecture supports it) and if it is, it is a very
                     * fast way to compare chunks of memory. To those more familiar with other
                     * languages, this `if` statement may look flat out erroneous, but trust me, it
                     * is actually the secret sauce behind a lot of this programs speed. */
                    if first_buf != second_buf {
                    // if &first_buf[..first_bytes_read] != &second_buf[..second_bytes_read] {
                        return Ok(FileCmp::Mismatch);
                    }
                },
                Err(_) => {
                    let err_str: String = format!("{}{}",
                        "Encountered an error when reading a file ",
                        second_path.display());
                    return Err((3, err_str));
                }
            },
            Err(_) => {
                let err_str: String = format!("{}{}", "Encountered an error when reading a file ",
                    first_path.display());
                return Err((3, err_str));
            }
        }
    }
    /* }}} */
}


// TODO: remove:    DOC = updated 2025-06-04
/// Takes two paths and a result representing how the soft links compare. Both file paths must
/// point to soft links and both soft links must exist.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `first_path`: a file path that points to the first soft link we wish to compare.
/// * `second_path`: a file path that points to the second soft link we wish to compare.
/// #### Return:
/// * On success, a `FileCmp` that represents whether the two soft links are equivalent in terms
///     of their substance (`Match` or `Mismatch`).
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error reading the link path of one of the links
fn compare_soft_links(first_path: &Path, second_path: &Path) -> Result<FileCmp, (i32, String)> {
    /* {{{ */
    match read_link(first_path) {
        Ok(first_link_target) => match read_link(second_path) {
            Ok(second_link_target) => {
                /* If the two soft links point to the same file */
                if first_link_target == second_link_target {
                    return Ok(FileCmp::Match);
                /* If the two soft links point to a different file */
                } else {
                    return Ok(FileCmp::Mismatch);
                }
            },
            Err(_) => {
                let err_str: String = format!("{}{}", "Failed to read the link path of a link ",
                    second_path.display());
                return Err((1, err_str));
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to read the link path of a link ",
                first_path.display());
            return Err((1, err_str));
        }
    }
    /* }}} */
}


// TODO: remove:    DOC = updated 2025-06-04
/// A helper function for `compare_files()`. Takes two paths that point to two files of the same
/// file type and returns a `Result` that either contains a `FileCmp` that represents how the two
/// files (understood in the broad sense) compare in terms of their substance or an `Err`
/// indicating that an error occurred in the process of comparing the two files.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `config`: a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `representative_filetype`: a file type, usually derived from one of the two files, that is
///     the same between the two files pointed to by the two paths.
/// * `first_path`: a file path that points to the first file we wish to compare.
/// * `second_path`: a file path that points to the second file we wish to compare.
/// #### Return:
/// * On success, a `FileCmp` that represents whether the two files are equivalent in terms of
///      their substance (`Match` or `Mismatch`).
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error comparing the substance of the two files
fn compare_substance(config: &Config, representative_filetype: &SimpleFileType, first_path: &Path,
    second_path: &Path) -> Result<FileCmp, (i32, String)> {
    /* {{{ */

    /* TODO: The substance of directories are currently evaluated as being a match simply if both
     * directories exist. I'm not sure if there's anything else to evaluate with directories */
    match representative_filetype {
        SimpleFileType::Directory => {
            return Ok(FileCmp::Match);
        },
        SimpleFileType::RegFile => {
            match compare_regular_files(config, first_path, second_path) {
                Ok(file_cmp) => return Ok(file_cmp),
                Err((_, err_str)) => return Err((1, err_str)),
            }
        },
        SimpleFileType::SoftLink => {
            match compare_soft_links(first_path, second_path) {
                Ok(file_cmp) => return Ok(file_cmp),
                Err((_, err_str)) => return Err((1, err_str)),
            }
        }
        /* TODO: No other file types have support. At the moment, the commented out line below
         * would have them treated the same way directories are: if they both exist, and are of the
         * same type, return that they match. */
        // _ => return Ok(FileCmp::Match),
    }
    /* }}} */
}


// TODO: remove:    DOC = updated 2025-06-04
/// Takes two paths that point to two files of the same file type and returns a `Result` that
/// either contains a `FileCmp` that represents how the two files (understood in the broad sense)
/// compare in terms of their modification time or an `Err` indicating that an error occurred in
/// the process of comparing the two files.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `follow_softlinks`: a boolean that determines whether this function will dereference soft
///     links and follow them, or compare the modification time of the soft link itself.
/// * `representative_filetype`: the file type of the two files being compared. This function
///     assumes that `first_path` and `second_path` point to files that are of the same file type.
///     That file type is represented by `representative_filetype`.
/// * `first_path`: the file path to the first file we wish to compare.
/// * `second_path`: the file path to the second file we wish to compare.
/// #### Return:
/// * On success, a `FileCmp` that represents whether the two files are equivalent in terms of
///     their modification time (`Match` or `Mismatch`).
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error getting the metadata of one of the files
///     * 2 -> There was an error getting the modification time of one of the files
fn compare_modification_time(follow_softlinks: bool, representative_filetype: &SimpleFileType,
    first_path: &Path, second_path: &Path) -> Result<FileCmp, (i32, String)> {
    /* {{{ */

    /* Get the metadata of both files */
    let first_metadata_res;
    let second_metadata_res;
    /* If we shouldn't follow soft links and if both files are soft links, then get the symlink
     * metadata */
    if !follow_softlinks && *representative_filetype == SimpleFileType::SoftLink {
        first_metadata_res = first_path.symlink_metadata();
    } else {
        first_metadata_res = first_path.metadata();
    }
    if !follow_softlinks && *representative_filetype == SimpleFileType::SoftLink {
        second_metadata_res = second_path.symlink_metadata();
    } else {
        second_metadata_res = second_path.metadata();
    }
    let first_metadata: Metadata;
    let second_metadata: Metadata;
    match first_metadata_res {
        Ok(md) => first_metadata = md,
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to get metadata of file ",
                first_path.display());
            return Err((1, err_str));
        }
    }
    match second_metadata_res {
        Ok(md) => second_metadata = md,
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to get metadata of file ",
                second_path.display());
            return Err((1, err_str));
        }
    }

    /* Get the modifcation time of both files */
    match first_metadata.modified() {
        Ok(first_mod_time) => match second_metadata.modified() {
            Ok(second_mod_time) => {
                /* If the modification time of both files is not the same, return mismatch,
                 * otherwise continue */
                match first_mod_time == second_mod_time {
                    true => (),
                    false => return Ok(FileCmp::Mismatch),
                }
            },
            Err(_) => {
                let err_str: String = format!("{}{}", "Failed to get modification time of file ",
                    second_path.display());
                return Err((2, err_str));
            }
        },
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to get modification time of file ",
                first_path.display());
            return Err((2, err_str));
        }
    };

    return Ok(FileCmp::Match);
    /* }}} */
}


// TODO: remove:    DOC = updated 2025-06-04
/// On success, this function returns a `FileTypeCmp` representing the result of performing a file
/// type comparison between two `DirEntry`s. The result will indicate both whether the two
/// `DirEntry`s had a matching file type and what the `SimpleFileType` representation of each
/// `DirEntry`'s file type was determined to be for use in the next stages of `cmp-tree`.
///
/// If this function encounters an error during execution, it returns an `Err` containing an `i32`
/// which uniquely identifies the sort of error it encountered as well as a `String` explaining the
/// specifics of the error it encountered.
///
/// #### Parameters:
/// * `cc`: a `ComparisonContext` that contains information about the the current `cmp-tree`
///     directory tree comparison. This function uses this struct to get the full path to the files
///     being compared so that path can be mentioned in the error string, should there be an error.
/// * `first_de`: a `DirEntry` containing information about the first file we wish to compare.
/// * `second_de`: a `DirEntry` containing information about the second file we wish to compare.
/// #### Return:
/// * On success, a `FileTypeCmp` representing both the result of the file type comparison, but
///     also information that was learned during the comparison (such as the `SimpleFileType`
///     representation of each file's file type) saving callers of this function from accessing the
///     metadata of a file and parsing the file type repeatedly. name and file type) for all the
///     files in the directory located at `dir_path`.
/// * On failure, an error code and a string explaining the specifics of the error. Possible error
///     codes are:
///     * 1 -> There was an error getting the file type of one of the given files
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
fn compare_directory_missing_file(config: &Config, cc: &ComparisonContext,
    existent_file_de: &DirEntry, existent_is_first: bool) -> Result<(), (i32, String)> {
    /* {{{ */

    let full_path_to_file;
    let full_path_to_extension;
    if existent_is_first {
        full_path_to_file = cc.first_root.join(cc.extension.join(existent_file_de.file_name()));
        full_path_to_extension = cc.first_root.join(cc.extension);
    } else {
        full_path_to_file = cc.second_root.join(cc.extension.join(existent_file_de.file_name()));
        full_path_to_extension = cc.second_root.join(cc.extension);
    }

    /* If `existent_file_de` is a directory... */
    let is_dir: bool;
    match existent_file_de.file_type() {
        Ok(ft) => is_dir = ft.is_dir(),
        Err(_) => {
            let err_str: String = format!("{}{}", "Failed to get the file type of file ",
                full_path_to_file.display());
            return Err((1, err_str));
        }
    }
    if is_dir {
        /* Recurse on the lonely directory so we can print all the files that are missing */
        match files::relative_files_in_tree(&full_path_to_extension,
            Path::new(&existent_file_de.file_name())) {

            Ok(file_list) => {
                for f in file_list {
                    let first_fpath = cc.first_root.join(&f);
                    let second_fpath = cc.second_root.join(&f);

                    /* Only print about the missing file if silent mode is not on */
                    if !config.silent {
                        if existent_is_first {
                            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                                first_fpath.display(), second_fpath.display());
                        } else {
                            eprintln!("\"{}\" does NOT exist, but \"{}\" does",
                                first_fpath.display(), second_fpath.display());
                        }
                    }
                }
            },
            Err((_, err_str)) => {
                return Err((2, err_str));
            }
        }
    }

    return Ok(());
    /* }}} */
}


// TODO: rename?
// TODO: remove:    DOC = updated 2025-06-04
/// This function takes a `Config` and `ComparisonContext` and performs the comparison laid out in
/// `cc` with the configuration specified by `config`. This function returns an `i32` representing
/// an exit code that indicates whether the comparison represented by the `cc` was a match, a
/// mismatch, or experienced an error. This function is recursive, and will call itself until it
/// reaches the bottom of the directory tree specified by `cc`.
///
/// #### Parameters:
/// * `cc`: a `ComparisonContext` that contains information about the the current `cmp-tree`
///     directory tree comparison. This function uses this struct to get the full path to the files
///     being compared so that path can be mentioned in the error string, should there be an error.
/// * `config`: a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// #### Return:
/// * On success, this function returns 0.
/// * On failure, this function either returns 1, indicating that the two directory trees
///     mismatched, or 2, indicating that there an error was encountered and the full comparison
///     could not be completed.
// TODO: break up and simplify - the function is currently 190 lines long!
pub fn compare_directory(config: &Config, cc: &ComparisonContext) -> i32 {
    /* {{{ */
    /* Get the directory contents of both directories */
    let mut first_dir: Vec<DirEntry>;
    let mut second_dir: Vec<DirEntry>;
    let mut mismatch_occurred: bool = false;

    match files::dir_entries_of_dir(&cc.first_root.join(cc.extension)) {
        Ok(dir_entries) => first_dir = dir_entries,
        Err((_, err_str)) => {
            eprintln!("{}{}", "Error: ", err_str);
            return 2;
        },
    }

    match files::dir_entries_of_dir(&cc.second_root.join(cc.extension)) {
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
                        match compare_substance(&config, &ft_cmp.first_ft, &first_fpath, &second_fpath) {
                            Ok(FileCmp::Match) => {
                                if config.compare_modification_times {
                                    match compare_modification_time(
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
                                        Err((_, err_str)) => {
                                            eprintln!("Error: comparing the modification time of the two files \"{}\", and \"{}\"",
                                                cc.first_root.join(first_dir[i].file_name()).display(),
                                                cc.second_root.join(second_dir[j].file_name()).display());
                                            eprintln!("{}{}", "Error: ", err_str);
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
                            Err((_, err_str)) => {
                                eprintln!("Error: comparing the content of the two files \"{}\", and \"{}\"",
                                    cc.first_root.join(first_dir[i].file_name()).display(),
                                    cc.second_root.join(second_dir[j].file_name()).display());
                                eprintln!("{}{}", "Error: ", err_str);
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

            match compare_directory_missing_file(config, cc, &first_dir[i], true) {
                Ok(_) => (),
                Err((_, err_str)) => {
                    eprintln!("Error: {}", err_str);
                },
            }

            i += 1;
        /* If the file represented by `second_dir[j]` does not exist in the first dir tree */
        /* first_dir[i].file_name() > second_dir[j].file_name() */
        } else {
            eprintln!("\"{}\" exists, but \"{}\" does NOT",
                cc.second_root.join(second_dir[j].file_name()).display(),
                cc.first_root.join(second_dir[j].file_name()).display());

            mismatch_occurred = true;

            match compare_directory_missing_file(config, cc, &second_dir[j], false) {
                Ok(_) => (),
                Err((_, err_str)) => {
                    eprintln!("Error: {}", err_str);
                },
            }

            j += 1;
        }
    }

    while i < first_dir.len() {
        eprintln!("\"{}\" exists, but \"{}\" does NOT",
            cc.first_root.join(first_dir[i].file_name()).display(),
            cc.second_root.join(first_dir[i].file_name()).display());

        mismatch_occurred = true;

        match compare_directory_missing_file(config, cc, &first_dir[i], true) {
            Ok(_) => (),
            Err((_, err_str)) => {
                eprintln!("Error: {}", err_str);
            },
        }

        i += 1;
    }

    while j < second_dir.len() {
        eprintln!("\"{}\" does NOT exist, but \"{}\" does",
            cc.first_root.join(second_dir[j].file_name()).display(),
            cc.second_root.join(second_dir[j].file_name()).display());

        mismatch_occurred = true;

        match compare_directory_missing_file(config, cc, &second_dir[j], false) {
            Ok(_) => (),
            Err((_, err_str)) => {
                eprintln!("Error: {}", err_str);
            },
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


// TODO: look into unit tests for functions in this file
// /* Unit tests */
#[test]
fn ut_compare_regular_files_001() {
    /* {{{ */
    use crate::config::default_config;
    let conf = default_config();
    let first_file = Path::new("../../tests/001/first/Lorem.txt");
    let second_file = Path::new("../../tests/001/second/Lorem.txt");
    /* `expected_ret` would be `Ok(FileCmp::Match)` */
    let expected_ret_content = FileCmp::Match;

    let ret = compare_regular_files(&conf, &first_file, &second_file);
    match ret {
        Ok(ret_content) => {
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}

#[test]
fn ut_compare_regular_files_002() {
    /* {{{ */
    use crate::config::default_config;
    let conf = default_config();
    let first_file = Path::new("../../tests/001/first/cmp_man_pages.txt");
    let second_file = Path::new("../../tests/001/second/cmp_man_pages.txt");
    /* `expected_ret` would be `Ok(FileCmp::Match)` */
    let expected_ret_content = FileCmp::Match;

    let ret = compare_regular_files(&conf, &first_file, &second_file);
    match ret {
        Ok(ret_content) => {
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}

#[test]
fn ut_compare_regular_files_003() {
    /* {{{ */
    /* The two input files are text files with the same words, but every letter (but the very
     * first) in the first file is in lower case whereas every letter in the second file is
     * uppercase. */
    use crate::config::default_config;
    let conf = default_config();
    let first_file = Path::new("../../tests/002/first/Lorem.txt");
    let second_file = Path::new("../../tests/002/second/Lorem.txt");
    /* `expected_ret` would be `Ok(FileCmp::SubstanceRegFileContentMismatch)` */
    let expected_ret_content = FileCmp::Mismatch;

    let ret = compare_regular_files(&conf, &first_file, &second_file);
    match ret {
        Ok(ret_content) => {
            assert_eq!(ret_content, expected_ret_content);
        },
        Err(_) => assert!(false),
    }
    /* }}} */
}
