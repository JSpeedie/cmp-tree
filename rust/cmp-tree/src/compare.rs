use std::fs::{File,Metadata,read_link};
use std::io::Read; // For getting the SHA256 hash of a file
use std::path::Path;


// Use statements to get rid of the `config::` prefix
use crate::config::Config;

// Use statements to get rid of the `data_structures::` prefix
use crate::data_structures::FileCmp;
use crate::data_structures::SimpleFileType;
use crate::data_structures::PartialFileComparison;


/// Takes two paths and returns a result representing how the files compare. Both file paths must
/// point to regular files and both regular files must exist.
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
fn compare_existences(first_path: &Path, second_path: &Path) -> Result<FileCmp, ()> {
    /* {{{ */
    let first_existence: bool;
    let second_existence: bool;

    /* Checking existences is a little bit trickier with symlinks. First, we need to check if our
     * paths point to symlinks and the link itself exists (rather than whether the stuff it points
     * to exists) */
    if first_path.is_symlink() {
        first_existence = true;
    } else {
        first_existence = first_path.exists();
    }
    if second_path.is_symlink() {
        second_existence = true;
    } else {
        second_existence = second_path.exists();
    }

    if first_existence && second_existence {
        return Ok(FileCmp::Match);
    /* If only one of the file paths points to an existing file, note which file exists */
    } else if first_existence && !second_existence {
        return Ok(FileCmp::ExistenceOnlyFirstFile);
    } else if !first_existence && second_existence {
        return Ok(FileCmp::ExistenceOnlySecondFile);
    /* !first_existence && !second_existence */
    } else {
        return Ok(FileCmp::ExistenceNeitherFile);
    }
    /* }}} */
}


/// A helper function for `compare_files()`. Takes two paths and returns a `Result` that either
/// contains a tuple of two `Option<Metadata>`s (representing possibly the metadata of the two
/// files being compared) or an `Err` indicating that an error occurred in the process of acquiring
/// the metadata.
///
/// #### Parameters:
/// * `first_path` a file path that points to the first file whose metadata we wish to get.
/// * `second_path` a file path that points to the second file whose metadata we wish to get.
/// #### Return:
/// * a `Result<(Option<Metadata>, Option<Metadata>), ()>` that either contains possibly the
///     metadata of the two files or an Err indicating that this function failed to get the
///     metadata on the two files successfully.
fn get_metadata(first_path: &Path, second_path: &Path) ->
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
fn compare_substance(first_path: &Path, representative_filetype: SimpleFileType,
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
/// files (understood in the broad sense) compare in terms of their modification time or an `Err`
/// indicating that an error occurred in the process of comparing the two files.
///
/// #### Parameters:
/// * `first_metadata` the file metadata of the first file we wish to compare.
/// * `second_metadata` the file metadata of the second file we wish to compare.
/// #### Return:
/// * a `FileCmp` that represents whether the two files are equivalent in terms of their
///     modification time.
fn compare_modification_time(first_metadata: &Metadata,
    second_metadata: &Metadata) -> Result<FileCmp, ()> {
    /* {{{ */

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
pub fn compare_files(config: &Config, first_path: &Path, second_path: &Path) ->
    Result<PartialFileComparison, ()> {
    /* {{{ */

    let mut ret_partial_cmp: PartialFileComparison;

    /* 1. Compare the existence of both files */
    match compare_existences(first_path, second_path) {
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

    match get_metadata(first_path, second_path) {
        /* If we were able to successfully get the metadata from both files, save the metadata
         * and continue execution */
        Ok((Some(first_meta), Some(second_meta))) => {
            first_metadata = first_meta;
            second_metadata = second_meta;
        },
        /* If we were only able to get the metadata for one file (which is what would happen if the
         * existence comparison resulted in a mismatch), save the file type of the existing file
         * and return early */
        Ok((Some(first_meta), None)) => {
            match SimpleFileType::try_from(&first_meta.file_type()) {
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
            match SimpleFileType::try_from(&second_meta.file_type()) {
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
        match SimpleFileType::try_from(&first_filetype) {
            Ok(ft) => Some(ft),
            Err(_) => None,
        };
    ret_partial_cmp.second_ft =
        match SimpleFileType::try_from(&second_filetype) {
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
    match compare_substance(first_path, ret_partial_cmp.first_ft.clone().unwrap(),
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
    if config.compare_modification_times {
        match compare_modification_time(&first_metadata, &second_metadata) {
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


/* Unit tests */
#[test]
fn ut_compare_regular_files_001() {
    /* {{{ */
    let first_file = Path::new("../../tests/001/first/Lorem.txt");
    let second_file = Path::new("../../tests/001/second/Lorem.txt");
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
    let first_file = Path::new("../../tests/002/first/Lorem.txt");
    let second_file = Path::new("../../tests/002/second/Lorem.txt");
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
