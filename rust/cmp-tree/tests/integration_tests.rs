mod integration_tests {
    use std::path::Path;

    use cmp_tree;

    /* The naming convention for integration tests is to name each test like so:
     *
     *     it_[test-focus]_[test-focus-n]_[brief-description]
     *
     * where:
     *     `it` stands for "integration test"
     *     `test-focus` is a very short description of what condition we are testing the program
     *         for correctness under.
     *     `test-focus-n` represents the test id or number *within* the current `test-focus`. For
     *         example, the third test related to modification time correctness might be called
     *         `it_modification_times_003_...`
     *     `brief-description` a short description of what exactly the inputs to this test
     *         represent.
     */

    #[test]
    fn it_general_001_identical_1l_with_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/001/first");
        let second_dir = Path::new("../../tests/001/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_general_002_differing_1l_single_differing_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/002/first");
        let second_dir = Path::new("../../tests/002/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_general_003_identical_2l_with_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/003/first");
        let second_dir = Path::new("../../tests/003/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_general_004_differing_2l_single_differing_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/004/first");
        let second_dir = Path::new("../../tests/004/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_general_005_differing_1l_two_differing_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/005/first");
        let second_dir = Path::new("../../tests/005/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_general_006_identical_4l_only_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/006/first");
        let second_dir = Path::new("../../tests/006/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_general_007_differing_4l_only_directories_missing_a_subdir() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/007/first");
        let second_dir = Path::new("../../tests/007/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_general_008_differing_6l_one_same_size_file_differing_in_content() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/013/first");
        let second_dir = Path::new("../../tests/013/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_file_types_001_directory_vs_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/015/first");
        let second_dir = Path::new("../../tests/015/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_file_types_002_soft_link_vs_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/016/first");
        let second_dir = Path::new("../../tests/016/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_soft_links_001_identical_soft_link_that_points_to_identical_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/008/first");
        let second_dir = Path::new("../../tests/008/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_soft_links_002_differing_soft_links_that_point_to_differing_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/009/first");
        let second_dir = Path::new("../../tests/009/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_soft_links_003_differing_soft_links_that_point_to_identical_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/010/first");
        let second_dir = Path::new("../../tests/010/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_soft_links_004_links_that_point_to_nonexistent_locations() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/014/first");
        let second_dir = Path::new("../../tests/014/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_modification_times_001_two_reg_files_with_identical_mtimes() {
        /* {{{ */
        let mut conf = cmp_tree::default_config();
        // By default, `cmp-tree` does not check that the modification times of the files are
        // identical as well. Enable modification time comparison.
        conf.compare_modification_times = true;
        let first_dir = Path::new("../../tests/001/first");
        let second_dir = Path::new("../../tests/001/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_modification_times_002_four_reg_files_with_identical_mtimes() {
        /* {{{ */
        let mut conf = cmp_tree::default_config();
        // By default, `cmp-tree` does not check that the modification times of the files are
        // identical as well. Enable modification time comparison.
        conf.compare_modification_times = true;
        let first_dir = Path::new("../../tests/003/first");
        let second_dir = Path::new("../../tests/003/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_modification_times_003_four_reg_files_all_with_differing_mtimes() {
        /* {{{ */
        let mut conf = cmp_tree::default_config();
        // By default, `cmp-tree` does not check that the modification times of the files are
        // identical as well. Enable modification time comparison.
        conf.compare_modification_times = true;
        let first_dir = Path::new("../../tests/011/first");
        let second_dir = Path::new("../../tests/011/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_modification_times_004_nested_directories_with_one_directory_differing_in_mtime() {
        /* {{{ */
        let mut conf = cmp_tree::default_config();
        // By default, `cmp-tree` does not check that the modification times of the files are
        // identical as well. Enable modification time comparison.
        conf.compare_modification_times = true;
        let first_dir = Path::new("../../tests/012/first");
        let second_dir = Path::new("../../tests/012/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_modification_times_005_soft_links_with_differing_mtimes() {
        /* {{{ */
        let mut conf = cmp_tree::default_config();
        // By default, `cmp-tree` does not check that the modification times of the files are
        // identical as well. Enable modification time comparison.
        conf.compare_modification_times = true;
        let first_dir = Path::new("../../tests/008/first");
        let second_dir = Path::new("../../tests/008/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }
}
