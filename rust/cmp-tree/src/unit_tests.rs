#[cfg(test)]
mod unit_tests {
    use super::super::*;

    #[test]
    fn ut_files_in_tree_001() {
        /* {{{ */
        let root_dir = Path::new("../../tests/001/first");
        let mut expected_ret = Vec::from([
            Path::new("Lorem.txt"),
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
            Path::new("Lorem.txt"),
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
                first_path: PathBuf::from(first_dir.join("Lorem.txt")),
                second_path: PathBuf::from(second_dir.join("Lorem.txt")),
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
                first_path: PathBuf::from(first_dir.join("Lorem.txt")),
                second_path: PathBuf::from(second_dir.join("Lorem.txt")),
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
                first_path: PathBuf::from(first_dir.join("Lorem.txt")),
                second_path: PathBuf::from(second_dir.join("Lorem.txt")),
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
                first_path: PathBuf::from(first_dir.join("Lorem.txt")),
                second_path: PathBuf::from(second_dir.join("Lorem.txt")),
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
                first_path: PathBuf::from(first_dir.join("Lorem.txt")),
                second_path: PathBuf::from(second_dir.join("Lorem.txt")),
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
