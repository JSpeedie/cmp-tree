use std::cmp::Ordering;
use std::fs::{File,FileType,Metadata,read_link};
use std::io::Read; // For getting the SHA256 hash of a file
use std::path::{Path, PathBuf};
use std::thread::{available_parallelism,scope,ScopedJoinHandle};


#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
enum FileCmp {
    /* (1) For Existence Comparisons */
    /* For when neither of the two files (understood in the broad sense) exist. */
    ExistenceNeitherFile,
    /* For when only the first of the two files (understood in the broad sense) exists. */
    ExistenceOnlyFirstFile,
    /* For when only the second of the two files (understood in the broad sense) exists. */
    ExistenceOnlySecondFile,
    /* (2) For File Type Comparisons */
    /* For when the two files (understood in the broad sense) mismatch in their type (e.g. one is a
    * directory, one is a regular file). */
    FileTypeTypeMismatch,
    /* (3) For Substance Comparisons */
    /* For when the two files mismatch in their content (i.e. they are not byte-for-byte
    * identical). */
    SubstanceRegFileContentMismatch,
    /* For when the two soft links mismatch in their link path */
    SubstanceSoftLinkLinkMismatch,
    /* (4) For Metadata Comparisons */
    MetadataAccessTimeMismatch,
    MetadataCreationTimeMismatch,
    MetadataModificationTimeMismatch,
    /* (5) For complete matches */
    /* For when the two files (understood in the broad sense) match don't mismatch in any of the
    * possible ways represented above */
    Match,
}


#[derive(Debug,PartialEq,Eq,Clone,PartialOrd,Ord)]
enum SimpleFileType {
    RegFile,
    Directory,
    SoftLink,
}


#[derive(Debug,PartialEq,Eq,PartialOrd)]
struct PartialFileComparison {
    file_cmp: FileCmp,
    first_ft: Option<SimpleFileType>,
    second_ft: Option<SimpleFileType>,
}


impl Ord for PartialFileComparison {
    /* {{{ */
    fn cmp(&self, other: &Self) -> Ordering {
        /* Compare the `file_cmp` member. If that comparison returns a `Less` or `Greater`
         * Ordering, our work is done, otherwise proceed to compare the next member */
        match (&(self.file_cmp)).cmp(&(other.file_cmp)) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                /* Provide a closure for calculating the "value" of a given
                 * `Option<SimpleFileType>` */
                let calculate_ft_value = |input: &Option<SimpleFileType>| {
                    match &input {
                        None => return 0,
                        Some(ft) => match ft {
                            SimpleFileType::RegFile => return 1,
                            SimpleFileType::Directory => return 2,
                            SimpleFileType::SoftLink => return 3,
                        }
                    }
                };

                /* Compare the `first_ft` member. If that comparison returns a `Less` or `Greater`
                 * Ordering, our work is done, otherwise proceed to compare the next member */
                match calculate_ft_value(&(self.first_ft))
                    .cmp(&calculate_ft_value(&(other.first_ft))) {

                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {
                        /* Compare the `second_ft` member. Since this is the last member to compare,
                         * return whatever it evaluates to. */
                        return calculate_ft_value(&(self.second_ft))
                            .cmp(&calculate_ft_value(&(other.second_ft)));
                    }
                }
            }
        }
    }
    /* }}} */
}


#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
struct FullFileComparison {
    partial_cmp: PartialFileComparison,
    first_path: PathBuf,
    second_path: PathBuf,
}


/* A struct used to define the configuration `cmp-tree` functions will run under. Many functions
* within `cmp-tree` will require a Config struct and the values of said struct will affect how
* they work or run. */
pub struct Config {
    pub compare_metadata: bool,
    pub matches: bool,
    pub pretty: bool,
    pub silent: bool,
    pub totals: bool,
}


/* A struct used to keep count of the max number and the found number of files, directories, and
* soft links in a given directory tree comparison */
struct Totals {
    max_file_matches: u128,
    max_dir_matches: u128,
    max_softlink_matches: u128,
    file_matches: u128,
    dir_matches: u128,
    softlink_matches: u128,
}


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


/// Returns the default config for `cmp-tree`.
///
/// #### Return:
/// * a `Config` struct will all its values set to the default values for `cmp-tree`.
pub fn default_config() -> Config {
    /* {{{ */
    return Config {
        compare_metadata: false,
        matches: false,
        pretty: false,
        silent: false,
        totals: false,
    };
    /* }}} */
}


/// Returns a freshly initialized Totals struct.
///
/// #### Return:
/// * a `Totals` struct will all its values set to suitable defaults.
fn default_totals() -> Totals {
    /* {{{ */
    return Totals {
        max_file_matches: 0,
        max_dir_matches: 0,
        max_softlink_matches: 0,
        file_matches: 0,
        dir_matches: 0,
        softlink_matches: 0,
    };
    /* }}} */
}


/// Takes a `std::fs::FileType` and maps it to a `SimpleFileType` if possible.
///
/// #### Parameters:
/// * `fs_filetype` the `std::fs::FileType` we wish to map to a `SimpleFileType`.
/// #### Return:
/// * a `SimpleFileType` representing one of the file types this library supports on success, and
///     on error, a unit type (`()`).
fn fs_filetype_to_simplefiletype(fs_filetype: &FileType) -> Result<SimpleFileType, ()> {
    /* {{{ */
    if fs_filetype.is_file() {
        return Ok(SimpleFileType::RegFile);
    } else if fs_filetype.is_dir() {
        return Ok(SimpleFileType::Directory);
    } else if fs_filetype.is_symlink() {
        return Ok(SimpleFileType::SoftLink);
    } else {
        return Err(());
    }
    /* }}} */
}


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


/// Returns an unsorted vector list of file paths for all the files (in the broad sense of the
/// word, including links and directories, as well as hidden files) in a directory tree rooted at
/// the directory pointed to by 'root'.
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


/// Takes two paths and a result representing how the files compare. Both file paths must point to
/// regular files and both regular files must exist.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first file we wish to compare.
/// * `second_path` a file path that points to the second file we wish to compare.
/// #### Return:
/// * `Ok(FileCmp)` on success and `Err(())` on failure.
fn compare_regular_files(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()> {
    /* {{{ */
    const BYTE_COUNT: usize = 8192;

    let first_file_res = File::open(first_path);
    let second_file_res = File::open(second_path);
    let mut first_file: File;
    let mut second_file: File;
    let mut first_buf = [0; BYTE_COUNT];
    let mut second_buf = [0; BYTE_COUNT];

    /* Check if the files differ in size. If they do, they cannot be byte-for-byte identical */
    match first_path.metadata() {
        Ok(first_md) => match second_path.metadata() {
            Ok(second_md) => {
                if first_md.len() != second_md.len() {
                    return Ok(FileCmp::SubstanceRegFileContentMismatch);
                }
            },
            Err(_) => return Err(()),
        },
        Err(_) => return Err(()),
    }

    if first_file_res.is_ok() && second_file_res.is_ok() {
        first_file = first_file_res.unwrap();
        second_file = second_file_res.unwrap();
    } else {
        return Err(());
    }

    loop {
        match first_file.read(&mut first_buf) {
            Ok(first_bytes_read) => match second_file.read(&mut second_buf) {
                Ok(second_bytes_read) => {
                    /* One file ended before the other */
                    if first_bytes_read != second_bytes_read {
                        return Ok(FileCmp::SubstanceRegFileContentMismatch);
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
                        return Ok(FileCmp::SubstanceRegFileContentMismatch);
                    }
                },
                Err(_) => {
                    return Err(());
                }
            },
            Err(_) => {
                return Err(());
            }
        }
    }
    /* }}} */
}


/// Takes two paths and a result representing how the soft links compare. Both file paths must
/// point to soft links and both soft links must exist.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first soft link we wish to compare.
/// * `second_path` a file path that points to the second soft link we wish to compare.
/// #### Return:
/// * `Ok(FileCmp)` on success and `Err(())` on failure.
fn compare_soft_links(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()> {
    /* {{{ */
    match read_link(first_path) {
        Ok(first_link_target) => match read_link(second_path) {
            Ok(second_link_target) => {
                /* If the two soft links point to the same file */
                if first_link_target == second_link_target {
                    return Ok(FileCmp::Match);
                /* If the two soft links point to a different file */
                } else {
                    return Ok(FileCmp::SubstanceSoftLinkLinkMismatch);
                }
            },
            Err(_) => {
                return Err(());
            }
        },
        Err(_) => {
            return Err(());
        }
    }
    /* }}} */
}


/// Takes two paths and returns a `Result` that either contains a `FileCmp` that represents how the
/// two files (understood in the broad sense) pointed to by the two paths compare in terms of their
/// existence or an `Err` indicating that an error occurred in the process of comparing the two
/// files.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first file we wish to compare.
/// * `second_path` a file path that points to the second file we wish to compare.
/// #### Return:
/// * a `FileCmp` that represents whether the two files are equivalent in terms of existence and
///     how they are different in this regard if they are.
fn compare_files_compare_existences(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()> {
    /* {{{ */

    /* If neither path points to files that exist, return that neither exists */
    if !first_path.exists() && !second_path.exists() {
        return Ok(FileCmp::ExistenceNeitherFile);
    /* If only one of the file paths points to an existing file, note which file exists */
    } else if first_path.exists() && !second_path.exists() {
        return Ok(FileCmp::ExistenceOnlyFirstFile);
    } else if !first_path.exists() && second_path.exists() {
        return Ok(FileCmp::ExistenceOnlySecondFile);
    }

    return Ok(FileCmp::Match);
    /* }}} */
}


/// A helper function for `compare_files()`. Takes two paths and returns a `Result` that either
/// contains a tuple of two `Option<Metadata>`s (representing possibly the metadata of the two files being compared)
/// or an `Err` indicating that an error occurred in the process of acquiring the metadata.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first file whose metadata we wish to get.
/// * `second_path` a file path that points to the second file whose metadata we wish to get.
/// #### Return:
/// * a `Result<(Option<Metadata>, Option<Metadata>), ()>` that either contains possibly the
///     metadata of the two files or an Err indicating that this function failed to get the
///     metadata on the two files successfully.
fn compare_files_get_metadata(first_path: &Path, second_path: &Path) ->
    Result<(Option<Metadata>, Option<Metadata>), ()> {
    /* {{{ */

    let mut first_file_metadata: Option<Metadata> = None;
    let mut second_file_metadata: Option<Metadata> = None;

    if first_path.exists() {
        let first_file_metadata_res;

        /* Collect the metadata on the current files. By default, Path.metadata() follows symlinks,
         * so we need to check if the files we're looking at are symlinks and gets their metadata
         * appropriately */
        match first_path.is_symlink() {
            true => first_file_metadata_res = first_path.symlink_metadata(),
            false => first_file_metadata_res = first_path.metadata(),
        }

        if let Ok(md) = first_file_metadata_res {
            first_file_metadata = Some(md);
        } else {
            return Err(());
        }
    }

    if second_path.exists() {
        let second_file_metadata_res;

        /* Collect the metadata on the current files. By default, Path.metadata() follows symlinks,
         * so we need to check if the files we're looking at are symlinks and gets their metadata
         * appropriately */
        match second_path.is_symlink() {
            true => second_file_metadata_res = second_path.symlink_metadata(),
            false => second_file_metadata_res = second_path.metadata(),
        }

        if let Ok(md) = second_file_metadata_res {
            second_file_metadata = Some(md);
        } else {
            return Err(());
        }
    }


    return Ok((first_file_metadata, second_file_metadata));
    /* }}} */
}


/// A helper function for `compare_files()`. Takes two paths that point to two files of the same
/// file type and returns a `Result` that either contains a `FileCmp` that represents how the two
/// files (understood in the broad sense) compare in terms of their substance or an `Err`
/// indicating that an error occurred in the process of comparing the two files.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first file we wish to compare.
/// * `representative_filetype` a file type, usually derived from one of the two files, that is the
///     same between the two files pointed to by the two paths.
/// * `second_path` a file path that points to the second file we wish to compare.
/// #### Return:
/// * a `FileCmp` that represents whether the two files are equivalent in terms of substance and
///     how they are different in this regard if they are.
fn compare_files_compare_substance(first_path: &Path, representative_filetype: SimpleFileType,
    second_path: &Path) -> Result<FileCmp, ()> {
    /* {{{ */

    /* TODO: The substance of directories are currently evaluated as being a match simply if both
    * directories exist. I'm not sure if there's anything else to evaluate with directories */
    match representative_filetype {
        SimpleFileType::Directory => return Ok(FileCmp::Match),
        SimpleFileType::RegFile => return compare_regular_files(first_path, second_path),
        SimpleFileType::SoftLink => return compare_soft_links(first_path, second_path),
        /* TODO: No other file types have support. At the moment, the commented out line below
         * would have them treated the same way directories are: if they both exist, and are of the
         * same type, return that they match. */
        // _ => return Ok(FileCmp::Match),
    }
    /* }}} */
}


/// A helper function for `compare_files()`. Takes two paths that point to two files of the same
/// file type and returns a `Result` that either contains a `FileCmp` that represents how the two
/// files (understood in the broad sense) compare in terms of their metadata or an `Err`
/// indicating that an error occurred in the process of comparing the two files.
///
/// #### Parameters:
/// * `first_metadata` the file metadata of the first file we wish to compare.
/// * `second_metadata` the file metadata of the second file we wish to compare.
/// #### Return:
/// * a `FileCmp` that represents whether the two files are equivalent in terms of metadata and
///     how they are different in this regard if they are.
fn compare_files_compare_metadata(first_metadata: &Metadata, second_metadata: &Metadata) ->
    Result<FileCmp, ()> {
    /* {{{ */
    match first_metadata.accessed() {
        Ok(first_acc_time) => match second_metadata.accessed() {
            Ok(second_acc_time) => {
                match first_acc_time == second_acc_time {
                    true => (),
                    false => return Ok(FileCmp::MetadataAccessTimeMismatch),
                }
            },
            Err(_) => return Err(()),
        },
        Err(_) => return Err(()),
    };

    match first_metadata.created() {
        Ok(first_create_time) => match second_metadata.created() {
            Ok(second_create_time) => {
                match first_create_time == second_create_time {
                    true => (),
                    false => return Ok(FileCmp::MetadataCreationTimeMismatch),
                }
            },
            Err(_) => return Err(()),
        },
        Err(_) => return Err(()),
    };

    match first_metadata.modified() {
        Ok(first_mod_time) => match second_metadata.modified() {
            Ok(second_mod_time) => {
                match first_mod_time == second_mod_time {
                    true => (),
                    false => return Ok(FileCmp::MetadataModificationTimeMismatch),
                }
            },
            Err(_) => return Err(()),
        },
        Err(_) => return Err(()),
    };

    return Ok(FileCmp::Match);
    /* }}} */
}


/// Takes two paths and returns a `Result` that either contains a `PartialFileComparison` that
/// represents how the two files (understood in the broad sense) pointed to by the two paths
/// compare or an `Err` indicating that an error occurred in the process of comparing the two
/// files.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `first_path` a file path that points to the first file we wish to compare.
/// * `second_path` a file path that points to the second file we wish to compare.
/// #### Return:
/// * a `PartialFileComparison` that represents whether the two files are equivalent, if they
///     differ and how they differ, as well as the two file types of the files.
fn compare_files(config: &Config, first_path: &Path, second_path: &Path) ->
    Result<PartialFileComparison, ()> {
    /* {{{ */

    let mut ret_partial_cmp: PartialFileComparison;

    /* 1. Compare the existence of both files */
    match compare_files_compare_existences(first_path, second_path) {
        Ok(existence_cmp) => {
            ret_partial_cmp = PartialFileComparison {
                first_ft: None,
                second_ft: None,
                file_cmp: existence_cmp,
            };
            /* If both files don't exist at this point, we can return that they experienced a
             * mismatch. However, so long as one of them exists, we want to get the file type of
             * that existing file. Normally we would perform an early return here if there was a
             * mismatch, but we will delay the return until we get the file types */
        },
        Err(_) => return Err(()),
    }

    /* INTERMEDIATE: Get the metadata of the two files. We will need this metadata for several
    * types of comparisons coming up. */
    let first_metadata: Metadata;
    let second_metadata: Metadata;

    match compare_files_get_metadata(first_path, second_path) {
        /* If we were able to successfully get the metadata from both files, save the metadata
         * and continue execution */
        Ok((Some(first_meta), Some(second_meta))) => {
            first_metadata = first_meta;
            second_metadata = second_meta;
        },
        /* If we were only able to get the metadata for one file (which is what would happen if
         * the existence comparison resulted in a mismatch), save the file type of the existing
         * file and return early */
        Ok((Some(first_meta), None)) => {
            match fs_filetype_to_simplefiletype(&first_meta.file_type()) {
                Ok(ft) => ret_partial_cmp.first_ft = Some(ft),
                /* If we weren't able to get a `SimpleFileType` for the first file, return early
                 * with an error */
                Err(_) => return Err(()),
            }
            ret_partial_cmp.second_ft = None;
            return Ok(ret_partial_cmp);
        },
        /* Same as previous comment */
        Ok((None, Some(second_meta))) => {
            ret_partial_cmp.first_ft = None;
            match fs_filetype_to_simplefiletype(&second_meta.file_type()) {
                Ok(ft) => ret_partial_cmp.second_ft = Some(ft),
                /* If we weren't able to get a `SimpleFileType` for the second file, return early
                 * with an error */
                Err(_) => return Err(()),
            }
            return Ok(ret_partial_cmp);
        },
        /* Extremely unlikely edge case (should only get hit if something like a TOCTOU happens) */
        Ok((None, None)) => {
            ret_partial_cmp.first_ft = None;
            ret_partial_cmp.second_ft = None;
            return Ok(ret_partial_cmp);
        },
        Err(_) => return Err(()),
    }

    /* 2. Compare the file types of both files. */
    let first_filetype = first_metadata.file_type();
    let second_filetype = second_metadata.file_type();
    /* Update the file types in our return struct now that we have them */
    ret_partial_cmp.first_ft =
        match fs_filetype_to_simplefiletype(&first_filetype) {
            Ok(ft) => Some(ft),
            Err(_) => None,
        };
    ret_partial_cmp.second_ft =
        match fs_filetype_to_simplefiletype(&second_filetype) {
            Ok(ft) => Some(ft),
            Err(_) => None,
        };
    /* If the two paths point to files that are of different types (e.g. a directory vs. a symlink,
     * a directory vs a regular file) then return early */
    if ret_partial_cmp.first_ft != ret_partial_cmp.second_ft {
        ret_partial_cmp.file_cmp = FileCmp::FileTypeTypeMismatch;
        return Ok(ret_partial_cmp);
    }

    /* 3. Compare the substance of both files. */
    /* We know the unwrap call won't fail because of the large match statement above will return
     * early on any case where it was not able to get a `SimpleFileType` representation of both
     * files' file types. */
    match compare_files_compare_substance(first_path, ret_partial_cmp.first_ft.clone().unwrap(),
        second_path) {

        Ok(substance_cmp) => {
            ret_partial_cmp.file_cmp = substance_cmp;
            /* If the two files did not have identical substance, return early */
            match ret_partial_cmp.file_cmp {
                FileCmp::Match => (),
                _ => return Ok(ret_partial_cmp),
            }
        },
        Err(_) => return Err(()),
    }

    /* 3. Compare the metadata of both files. */
    /* Comparing metadata is optional, and by default is not enabled */
    if config.compare_metadata {
        match compare_files_compare_metadata(&first_metadata, &second_metadata) {
            Ok(metadata_cmp) => {
                ret_partial_cmp.file_cmp = metadata_cmp;
                /* If the two files did not have identical metadata, return early */
                match ret_partial_cmp.file_cmp {
                    FileCmp::Match => (),
                    _ => return Ok(ret_partial_cmp),
                }
            },
            Err(_) => return Err(()),
        }
    }

    /* If we make it to this point, that means all the types of comparisons have resulted in a
    * Match. We can return return struct. */
    return Ok(ret_partial_cmp);
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
///     the `Vec<FullFileComparison>`, then the caller is given a list of all the file comparisons
///     that were performed during the larger, directory tree comparison. If the `Result` turns out
///     to be an `Err`, then this function experienced some sort of error.
fn compare_directory_trees(config: &Config, first_root: &Path, second_root: &Path) -> 
    Result<Vec<FullFileComparison>, ()> {
    /* {{{ */

    let mut ret: Vec<FullFileComparison> = Vec::new();
    /* Get the first directory file list and the second directory file list:
    * the list of files in each directory */
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

                    let cmp_res = compare_files(config, &first_file, &second_file);

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


/// Takes a `Totals` struct `totals_count` and increments the relevant members inside it based on
/// the result of a file comparison represented by `p_cmp`.
///
/// #### Parameters:
/// * `totals_count` a `Totals` representing running totals for a given directory tree comparison.
/// * `p_cmp` a `PartialFileComparison` containing only the necessary the information about the 2
///     files that were compared.
/// TODO: simplify this function or break it up. 99 lines is too long.
fn update_totals(totals_count: &mut Totals, p_cmp: &PartialFileComparison) {
    /* {{{ */

    /* First we determine how the given PartialFileComparison should affect the max file,
    * directory, etc. match counts in the Totals struct */
    match &p_cmp.first_ft {
        Some(f_ft) => match f_ft {
            SimpleFileType::Directory => totals_count.max_dir_matches += 1,
            _ => {
                match &p_cmp.second_ft {
                    Some(s_ft) => match s_ft {
                        SimpleFileType::Directory => totals_count.max_dir_matches += 1,
                        _ => (),
                    },
                    None => (),
                }
            }
        },
        None => match &p_cmp.second_ft {
            Some(s_ft) => match s_ft {
                SimpleFileType::Directory => totals_count.max_dir_matches += 1,
                _ => (),
            },
            None => (),
        },
    }
    match &p_cmp.first_ft {
        Some(f_ft) => match f_ft {
            SimpleFileType::RegFile => totals_count.max_file_matches += 1,
            _ => match &p_cmp.second_ft {
                Some(s_ft) => match s_ft {
                    SimpleFileType::RegFile => totals_count.max_file_matches += 1,
                    _ => (),
                },
                None => (),
            }
        },
        None => {
            match &p_cmp.second_ft {
                Some(s_ft) => match s_ft {
                    SimpleFileType::RegFile => totals_count.max_file_matches += 1,
                    _ => (),
                },
                None => (),
            }
        },
    }
    match &p_cmp.first_ft {
        Some(f_ft) => match f_ft {
            SimpleFileType::SoftLink => totals_count.max_softlink_matches += 1,
            _ => match &p_cmp.second_ft {
                Some(s_ft) => match s_ft {
                    SimpleFileType::SoftLink => totals_count.max_softlink_matches += 1,
                    _ => (),
                },
                None => (),
            }
        },
        None => {
            match &p_cmp.second_ft {
                Some(s_ft) => match s_ft {
                    SimpleFileType::SoftLink => totals_count.max_softlink_matches += 1,
                    _ => (),
                },
                None => (),
            }
        },
    }

    /* Second, we determine how the given PartialFileComparison should affect the actual file,
    * directory, etc. match counts in the Totals struct */
    match &p_cmp.file_cmp {
        FileCmp::Match => match p_cmp.first_ft.clone().unwrap() {
            SimpleFileType::RegFile => totals_count.file_matches += 1,
            SimpleFileType::Directory => totals_count.dir_matches += 1,
            SimpleFileType::SoftLink => totals_count.softlink_matches += 1,
        },
        /* If the file comparison is anything but a match, do nothing to the totals */
        _ => (),
    }
    /* }}} */
}


/// Takes a `FullFileComparison` and prints out the necessary information about it. What
/// information is printed will depend on the values of `config`.
///
/// #### Parameters:
/// * `config` a `Config` representing a configuration for executing `cmp-tree`, usually modified
///     through command line arguments to the program.
/// * `full_comp` a `FullFileComparison` containing all the information about the 2 files that
///     were compared.
fn print_one_comparison(config: &Config, full_comp: &FullFileComparison) {
    /* {{{ */
    match full_comp.partial_cmp.file_cmp {
        FileCmp::ExistenceNeitherFile => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("Neither {:?} nor {:?} exist", full_comp.first_path, full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::ExistenceOnlyFirstFile => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} exists, but {:?} does NOT exist", full_comp.first_path, full_comp.second_path);
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
        FileCmp::SubstanceRegFileContentMismatch | FileCmp::SubstanceSoftLinkLinkMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} differs from {:?}", full_comp.first_path, full_comp.second_path);
            if config.pretty { print!("{NORMAL}"); }
        },
        FileCmp::MetadataAccessTimeMismatch | FileCmp::MetadataCreationTimeMismatch |
            FileCmp::MetadataModificationTimeMismatch => {
            if config.pretty { print!("{BOLD}{RED}"); }
            println!("{:?} has different metadata to {:?}", full_comp.first_path, full_comp.second_path);
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
fn print_output(config: &Config, directory_tree_comparison: Result<Vec<FullFileComparison>, ()>) {
    /* {{{ */
    let mut totals_count = default_totals();

    match directory_tree_comparison {
        Ok(list) => {
            for e in list {
                /* If we are going to print totals, update our totals count struct */
                if config.totals { update_totals(&mut totals_count, &e.partial_cmp); }

                /* Print what needs to be printed for the current comparison. This function call
                * may very well print nothing */
                print_one_comparison(&config, &e);
            }
        },
        Err(_) => {
            println!("ERROR: Failed to get the list of comparisons between the two directory trees.");
            return;
        }
    }

    if config.totals {
        println!("All done!");
        println!("File byte-for-byte matches: {0}/{1}", totals_count.file_matches, totals_count.max_file_matches);
        println!("Directory matches: {0}/{1}", totals_count.dir_matches, totals_count.max_dir_matches);
        println!("Soft link matches: {0}/{1}", totals_count.softlink_matches, totals_count.max_softlink_matches);
    }
    /* }}} */
}


/// Takes a `Result` possibly containing a `Vec` of `FullFileComparison`s and returns a `Result`
/// either containing a boolean representing whether or not the file comparison list received as
/// input contains any mismatches or an empty `Err` indicating that the provided `Result` file
/// comparison list had an error.
///
/// #### Parameters:
/// * `directory_tree_comparison` a `Result` possibly containing a `Vec` of `FullFileComparison`s.
///     Typically, this parameter is the result of a call to `compare_directory_trees()`.
/// #### Return:
/// * a `Result<bool, ()>` that represents whether this function was able to successfully evaluate
///     its input. If the input itself did not have an error, then the return value will contain a
///     boolean that will be `true` if there were any mismatches in the directory tree comparison
///     and `false` if the comparison found the two directory trees to be identical.
fn directory_tree_comparison_contains_mismatch(
    directory_tree_comparison: &Result<Vec<FullFileComparison>, ()>) -> Result<bool, ()> {
    /* {{{ */

    /* Go through the directory tree comparison result */
    match directory_tree_comparison {
        Ok(list) => {
            /* For every comparison in the list... */
            for e in list {
                /* If the comparison found a mismatch of any kind between the two files, return
                * early */
                match e.partial_cmp.file_cmp {
                    FileCmp::Match => (),
                    _ => return Ok(true),
                }
            }
            /* If we make it here, that means no mismatches of any kind were found in the file
            * comparison list. */
            return Ok(false);
        },
        Err(_) => {
            return Err(());
        }
    };
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
    /* Check if any mismatches occurred (this is needed to determine the exit code of this program
    * */
    let mismatch_occurred =
        directory_tree_comparison_contains_mismatch(&directory_tree_comparison_res);
    /* Print the appropriate output, provided silent mode is off */
    if !config.silent {
        print_output(&config, directory_tree_comparison_res);
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


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn ut_files_in_tree_001() {
        /* {{{ */
        let root_dir = Path::new("../../tests/001/first");
        let mut expected_ret = Vec::from([
            Path::new("Lorem.md"),
            Path::new("cmp_man_pages.txt"),
        ]);
        expected_ret.sort();

        let mut ret = files_in_tree(&root_dir);
        ret.sort();
        assert_eq!(ret, expected_ret);
        /* }}} */
    }

    #[test]
    fn ut_files_in_tree_002() {
        /* {{{ */
        let root_dir = Path::new("../../tests/003/second");
        let mut expected_ret = Vec::from([
            Path::new("Lorem.md"),
            Path::new("cmp_man_pages.txt"),
            Path::new("subdir"),
            Path::new("subdir/linear_gradient.png"),
            Path::new("subdir/rose.png"),
        ]);
        expected_ret.sort();

        let mut ret = files_in_tree(&root_dir);
        ret.sort();
        assert_eq!(ret, expected_ret);
        /* }}} */
    }

    #[test]
    fn ut_files_in_tree_003() {
        /* {{{ */
        let root_dir = Path::new("../../tests/006/second");
        let mut expected_ret = Vec::from([
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
        expected_ret.sort();

        let mut ret = files_in_tree(&root_dir);
        ret.sort();
        assert_eq!(ret, expected_ret);
        /* }}} */
    }

    #[test]
    fn ut_files_in_tree_004() {
        /* {{{ */
        let root_dir = Path::new("../../tests/007/second");
        let mut expected_ret = Vec::from([
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
        expected_ret.sort();

        let mut ret = files_in_tree(&root_dir);
        ret.sort();
        assert_eq!(ret, expected_ret);
        /* }}} */
    }

    #[test]
    fn ut_compare_regular_files_001() {
        /* {{{ */
        let first_file = Path::new("../../tests/001/first/Lorem.md");
        let second_file = Path::new("../../tests/001/second/Lorem.md");
        /* `expected_ret` would be `Ok(FileCmp::Match)` */
        let expected_ret_content = FileCmp::Match;

        let ret = compare_regular_files(&first_file, &second_file);
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
        let first_file = Path::new("../../tests/001/first/cmp_man_pages.txt");
        let second_file = Path::new("../../tests/001/second/cmp_man_pages.txt");
        /* `expected_ret` would be `Ok(FileCmp::Match)` */
        let expected_ret_content = FileCmp::Match;

        let ret = compare_regular_files(&first_file, &second_file);
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
        let first_file = Path::new("../../tests/002/first/Lorem.md");
        let second_file = Path::new("../../tests/002/second/Lorem.md");
        /* `expected_ret` would be `Ok(FileCmp::SubstanceRegFileContentMismatch)` */
        let expected_ret_content = FileCmp::SubstanceRegFileContentMismatch;

        let ret = compare_regular_files(&first_file, &second_file);
        match ret {
            Ok(ret_content) => {
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    // TODO: Add tests for:
    // fn compare_soft_links(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()>

    // TODO: Add tests for:
    // fn compare_files_compare_existences(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()>

    // TODO: Add tests for:
    // fn compare_files_get_metadata(first_path: &Path, second_path: &Path) -> Result<(Option<Metadata>, Option<Metadata>), ()>

    // TODO: Add tests for:
    // fn compare_files_compare_substance(first_path: &Path, representative_filetype: &FileType, second_path: &Path) -> Result<FileCmp, ()>

    // TODO: Add tests for:
    // fn compare_files_compare_metadata(first_metadata: &Metadata, second_metadata: &Metadata) -> Result<FileCmp, ()>

    // TODO: Add tests for:
    // fn compare_files(config: &Config, first_path: &Path, second_path: &Path) -> Result<PartialFileComparison, ()>

    #[test]
    fn ut_compare_directory_trees_001() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/001/first");
        let second_dir = Path::new("../../tests/001/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("cmp_man_pages.txt")),
                second_path: PathBuf::from(second_dir.join("cmp_man_pages.txt")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("Lorem.md")),
                second_path: PathBuf::from(second_dir.join("Lorem.md")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_002() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/002/first");
        let second_dir = Path::new("../../tests/002/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("cmp_man_pages.txt")),
                second_path: PathBuf::from(second_dir.join("cmp_man_pages.txt")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceRegFileContentMismatch,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("Lorem.md")),
                second_path: PathBuf::from(second_dir.join("Lorem.md")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_003() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/003/first");
        let second_dir = Path::new("../../tests/003/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("cmp_man_pages.txt")),
                second_path: PathBuf::from(second_dir.join("cmp_man_pages.txt")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("Lorem.md")),
                second_path: PathBuf::from(second_dir.join("Lorem.md")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("subdir")),
                second_path: PathBuf::from(second_dir.join("subdir")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("subdir/linear_gradient.png")),
                second_path: PathBuf::from(second_dir.join("subdir/linear_gradient.png")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("subdir/rose.png")),
                second_path: PathBuf::from(second_dir.join("subdir/rose.png")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_004() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/004/first");
        let second_dir = Path::new("../../tests/004/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("cmp_man_pages.txt")),
                second_path: PathBuf::from(second_dir.join("cmp_man_pages.txt")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("Lorem.md")),
                second_path: PathBuf::from(second_dir.join("Lorem.md")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("subdir")),
                second_path: PathBuf::from(second_dir.join("subdir")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("subdir/linear_gradient.png")),
                second_path: PathBuf::from(second_dir.join("subdir/linear_gradient.png")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceRegFileContentMismatch,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("subdir/rose.png")),
                second_path: PathBuf::from(second_dir.join("subdir/rose.png")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_005() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/005/first");
        let second_dir = Path::new("../../tests/005/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceRegFileContentMismatch,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("linear_gradient.png")),
                second_path: PathBuf::from(second_dir.join("linear_gradient.png")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceRegFileContentMismatch,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("Lorem.md")),
                second_path: PathBuf::from(second_dir.join("Lorem.md")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceRegFileContentMismatch,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("rose.png")),
                second_path: PathBuf::from(second_dir.join("rose.png")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_006() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/006/first");
        let second_dir = Path::new("../../tests/006/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("a")),
                second_path: PathBuf::from(second_dir.join("a")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("a/i")),
                second_path: PathBuf::from(second_dir.join("a/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b")),
                second_path: PathBuf::from(second_dir.join("b")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/i")),
                second_path: PathBuf::from(second_dir.join("b/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/i/1")),
                second_path: PathBuf::from(second_dir.join("b/i/1")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/ii")),
                second_path: PathBuf::from(second_dir.join("b/ii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/ii/2")),
                second_path: PathBuf::from(second_dir.join("b/ii/2")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c")),
                second_path: PathBuf::from(second_dir.join("c")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/i")),
                second_path: PathBuf::from(second_dir.join("c/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/i/1")),
                second_path: PathBuf::from(second_dir.join("c/i/1")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/i/1/a")),
                second_path: PathBuf::from(second_dir.join("c/i/1/a")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/ii")),
                second_path: PathBuf::from(second_dir.join("c/ii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/ii/2")),
                second_path: PathBuf::from(second_dir.join("c/ii/2")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/ii/2/b")),
                second_path: PathBuf::from(second_dir.join("c/ii/2/b")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/iii")),
                second_path: PathBuf::from(second_dir.join("c/iii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/iii/3")),
                second_path: PathBuf::from(second_dir.join("c/iii/3")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("c/iii/3/c")),
                second_path: PathBuf::from(second_dir.join("c/iii/3/c")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_007() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/007/first");
        let second_dir = Path::new("../../tests/007/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("a")),
                second_path: PathBuf::from(second_dir.join("a")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("a/i")),
                second_path: PathBuf::from(second_dir.join("a/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b")),
                second_path: PathBuf::from(second_dir.join("b")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/i")),
                second_path: PathBuf::from(second_dir.join("b/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/i/1")),
                second_path: PathBuf::from(second_dir.join("b/i/1")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/ii")),
                second_path: PathBuf::from(second_dir.join("b/ii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory)
                },
                first_path: PathBuf::from(first_dir.join("b/ii/2")),
                second_path: PathBuf::from(second_dir.join("b/ii/2")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c")),
                second_path: PathBuf::from(second_dir.join("c")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/i")),
                second_path: PathBuf::from(second_dir.join("c/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/i/1")),
                second_path: PathBuf::from(second_dir.join("c/i/1")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/i/1/a")),
                second_path: PathBuf::from(second_dir.join("c/i/1/a")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/ii")),
                second_path: PathBuf::from(second_dir.join("c/ii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/ii/2")),
                second_path: PathBuf::from(second_dir.join("c/ii/2")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/ii/2/b")),
                second_path: PathBuf::from(second_dir.join("c/ii/2/b")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/iii")),
                second_path: PathBuf::from(second_dir.join("c/iii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/iii/3")),
                second_path: PathBuf::from(second_dir.join("c/iii/3")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlyFirstFile,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: None
                },
                first_path: PathBuf::from(first_dir.join("c/iii/3/c")),
                second_path: PathBuf::from(second_dir.join("c/iii/3/c")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d")),
                second_path: PathBuf::from(second_dir.join("d")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/i")),
                second_path: PathBuf::from(second_dir.join("d/i")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/i/1")),
                second_path: PathBuf::from(second_dir.join("d/i/1")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/i/1/a")),
                second_path: PathBuf::from(second_dir.join("d/i/1/a")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/ii")),
                second_path: PathBuf::from(second_dir.join("d/ii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/ii/2")),
                second_path: PathBuf::from(second_dir.join("d/ii/2")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/ii/2/b")),
                second_path: PathBuf::from(second_dir.join("d/ii/2/b")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iii")),
                second_path: PathBuf::from(second_dir.join("d/iii")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iii/3")),
                second_path: PathBuf::from(second_dir.join("d/iii/3")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iii/3/c")),
                second_path: PathBuf::from(second_dir.join("d/iii/3/c")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iv")),
                second_path: PathBuf::from(second_dir.join("d/iv")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iv/4")),
                second_path: PathBuf::from(second_dir.join("d/iv/4")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::ExistenceOnlySecondFile,
                    first_ft: None,
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("d/iv/4/d")),
                second_path: PathBuf::from(second_dir.join("d/iv/4/d")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                println!(" ret: {:#?}\n exp: {:#?}", ret_content, expected_ret_content);
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_008() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/008/first");
        let second_dir = Path::new("../../tests/008/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("adirectory")),
                second_path: PathBuf::from(second_dir.join("adirectory")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::Directory),
                    second_ft: Some(SimpleFileType::Directory),
                },
                first_path: PathBuf::from(first_dir.join("adirectory/dir")),
                second_path: PathBuf::from(second_dir.join("adirectory/dir")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::RegFile),
                    second_ft: Some(SimpleFileType::RegFile),
                },
                first_path: PathBuf::from(first_dir.join("adirectory/file.txt")),
                second_path: PathBuf::from(second_dir.join("adirectory/file.txt")),
            },
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::Match,
                    first_ft: Some(SimpleFileType::SoftLink),
                    second_ft: Some(SimpleFileType::SoftLink),
                },
                first_path: PathBuf::from(first_dir.join("link")),
                second_path: PathBuf::from(second_dir.join("link")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }

    #[test]
    fn ut_compare_directory_trees_010() {
        /* {{{ */
        let conf = default_config();
        let first_dir = Path::new("../../tests/010/first");
        let second_dir = Path::new("../../tests/010/second");
        /* `expected_ret` would be `Ok(expected_ret_content)` */
        let mut expected_ret_content = Vec::from([
            FullFileComparison {
                partial_cmp: PartialFileComparison {
                    file_cmp: FileCmp::SubstanceSoftLinkLinkMismatch,
                    first_ft: Some(SimpleFileType::SoftLink),
                    second_ft: Some(SimpleFileType::SoftLink),
                },
                first_path: PathBuf::from(first_dir.join("link")),
                second_path: PathBuf::from(second_dir.join("link")),
            },
        ]);
        expected_ret_content.sort();

        match compare_directory_trees(&conf, &first_dir, &second_dir) {
            Ok(mut ret_content) => {
                ret_content.sort();
                assert_eq!(ret_content, expected_ret_content);
            },
            Err(_) => assert!(false),
        }
        /* }}} */
    }
}
