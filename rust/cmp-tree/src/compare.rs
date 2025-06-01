use std::fs::{File,Metadata,read_link};
use std::io::Read; // For getting the SHA256 hash of a file
use std::path::Path;


use crate::data_structures::{FileCmp,SimpleFileType};


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
                    return Ok(FileCmp::Mismatch);
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
                    return Ok(FileCmp::Mismatch);
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
pub fn compare_substance(representative_filetype: &SimpleFileType, first_path: &Path,
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


// TODO: update to describe the follow_softlinks, representative_filetype parameters
// TODO: update to describe the Err return
//
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
pub fn compare_modification_time(follow_softlinks: bool, representative_filetype: &SimpleFileType,
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


// /* Unit tests */
// #[test]
// fn ut_compare_regular_files_001() {
//     /* {{{ */
//     let first_file = Path::new("../../tests/001/first/Lorem.txt");
//     let second_file = Path::new("../../tests/001/second/Lorem.txt");
//     /* `expected_ret` would be `Ok(FileCmp::Match)` */
//     let expected_ret_content = FileCmp::Match;
// 
//     let ret = compare_regular_files(&first_file, &second_file);
//     match ret {
//         Ok(ret_content) => {
//             assert_eq!(ret_content, expected_ret_content);
//         },
//         Err(_) => assert!(false),
//     }
//     /* }}} */
// }
// 
// #[test]
// fn ut_compare_regular_files_002() {
//     /* {{{ */
//     let first_file = Path::new("../../tests/001/first/cmp_man_pages.txt");
//     let second_file = Path::new("../../tests/001/second/cmp_man_pages.txt");
//     /* `expected_ret` would be `Ok(FileCmp::Match)` */
//     let expected_ret_content = FileCmp::Match;
// 
//     let ret = compare_regular_files(&first_file, &second_file);
//     match ret {
//         Ok(ret_content) => {
//             assert_eq!(ret_content, expected_ret_content);
//         },
//         Err(_) => assert!(false),
//     }
//     /* }}} */
// }
// 
// #[test]
// fn ut_compare_regular_files_003() {
//     /* {{{ */
//     /* The two input files are text files with the same words, but every letter (but the very
//      * first) in the first file is in lower case whereas every letter in the second file is
//      * uppercase. */
//     let first_file = Path::new("../../tests/002/first/Lorem.txt");
//     let second_file = Path::new("../../tests/002/second/Lorem.txt");
//     /* `expected_ret` would be `Ok(FileCmp::SubstanceRegFileContentMismatch)` */
//     let expected_ret_content = FileCmp::SubstanceRegFileContentMismatch;
// 
//     let ret = compare_regular_files(&first_file, &second_file);
//     match ret {
//         Ok(ret_content) => {
//             assert_eq!(ret_content, expected_ret_content);
//         },
//         Err(_) => assert!(false),
//     }
//     /* }}} */
// }
