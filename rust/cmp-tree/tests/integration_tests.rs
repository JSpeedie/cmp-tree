mod integration_tests {
    use std::path::Path;

    use cmp_tree;

    /* The naming convention for integration tests is to name each test like so:
     *
     *     it_[test-focus]_[test-focus-n]_[opt-differing/identical]_[opt-level_count]_[brief-description]
     *
     * where:
     *     `it` stands for "integration test"
     *     `test-focus` is a very short description of what condition we are testing the program
     *          for correctness under. Lots of tests will cleanly fit under one of the match
     *          criteria such as existence (do both files exist?), file-type (are the files of the
     *          same file-type?), substance, or modification time. Soem tests may not cleanly fit
     *          under one of these.
     *     `test-focus-n` represents the test id or number *within* the current `test-focus`. For
     *          example, the third test related to modification time correctness might be called
     *          `it_modification_times_003_...`
     *      `opt-differing/identical` is an optional specifier that is either `differing` or
     *          `identical` and appears in tests that evaluate whether `cmp-tree` correctly
     *          recognizes inputs as differing or identical.
     *      `opt-level_count` is an optional specifier that notes how many directory levels there
     *          are in the deepest input directory tree.
     *     `brief-description` a short description of what exactly the inputs to this test
     *          represent.
     */

    #[test]
    fn it_existence_001_differing_4l_only_directories_missing_multiple_subdirs() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/007/first");
        let second_dir = Path::new("../../tests/007/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_file_type_001_differing_1l_directory_vs_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/015/first");
        let second_dir = Path::new("../../tests/015/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_file_type_002_differing_2l_soft_link_vs_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/016/first");
        let second_dir = Path::new("../../tests/016/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_001_identical_1l_multiple_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/001/first");
        let second_dir = Path::new("../../tests/001/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_substance_002_differing_1l_single_differing_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/002/first");
        let second_dir = Path::new("../../tests/002/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_003_identical_2l_multiple_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/003/first");
        let second_dir = Path::new("../../tests/003/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_substance_004_differing_2l_single_differing_regular_file() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/004/first");
        let second_dir = Path::new("../../tests/004/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_005_differing_1l_three_differing_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/005/first");
        let second_dir = Path::new("../../tests/005/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_006_identical_4l_only_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/006/first");
        let second_dir = Path::new("../../tests/006/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_substance_007_identical_2l_identical_soft_links_that_points_to_identical_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/008/first");
        let second_dir = Path::new("../../tests/008/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_substance_008_differing_2l_differing_soft_links_that_point_to_differing_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/009/first");
        let second_dir = Path::new("../../tests/009/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_009_differing_2l_differing_soft_links_that_point_to_identical_directories() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/010/first");
        let second_dir = Path::new("../../tests/010/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_010_differing_6l_one_same_size_file_differing_in_content() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/013/first");
        let second_dir = Path::new("../../tests/013/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_011_identical_1l_two_0_byte_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/017/first");
        let second_dir = Path::new("../../tests/017/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

    #[test]
    fn it_substance_012_differing_1l_first_byte_differs_two_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/018/first");
        let second_dir = Path::new("../../tests/018/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_013_differing_1l_byte_in_middle_of_file_differs_two_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/019/first");
        let second_dir = Path::new("../../tests/019/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_substance_014_differing_1l_final_byte_differs_two_regular_files() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/020/first");
        let second_dir = Path::new("../../tests/020/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        /* }}} */
    }

    #[test]
    fn it_mtimes_001_identical_1l_two_reg_files() {
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
    fn it_mtimes_002_identical_2l_four_reg_files() {
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
    fn it_mtimes_003_differing_2l_differing_soft_links() {
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

    #[test]
    fn it_mtimes_004_differing_2l_four_reg_files_all_differing() {
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
    fn it_mtimes_005_differing_4l_nested_directories_one_differing_directory() {
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
    fn it_stability_001_identical_1l_soft_links_that_point_to_nonexistent_locations() {
        /* {{{ */
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/014/first");
        let second_dir = Path::new("../../tests/014/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        /* }}} */
    }

}
