mod integration_tests {
    use std::path::Path;

    use cmp_tree;

    #[test]
    fn t001_1_level_trees_identical_regular_files() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/001/first");
        let second_dir = Path::new("../../tests/001/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t002_1_level_trees_single_differing_regular_file() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/002/first");
        let second_dir = Path::new("../../tests/002/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t003_2_level_trees_identical_regular_files() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/003/first");
        let second_dir = Path::new("../../tests/003/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t004_2_level_trees_single_differing_regular_file() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/004/first");
        let second_dir = Path::new("../../tests/004/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t005_1_level_trees_exclusively_differing_regular_files() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/005/first");
        let second_dir = Path::new("../../tests/005/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t006_4_level_trees_identical_dirs() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/006/first");
        let second_dir = Path::new("../../tests/006/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t007_4_level_trees_missing_a_subdir() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/007/first");
        let second_dir = Path::new("../../tests/007/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t008_2_level_trees_identical_soft_link_that_points_to_identical_directories() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/008/first");
        let second_dir = Path::new("../../tests/008/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 0);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t009_2_level_trees_differing_soft_links_that_point_to_differing_directories() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/009/first");
        let second_dir = Path::new("../../tests/009/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }

    #[test]
    fn t010_2_level_trees_differing_soft_links_that_point_to_identical_directories() {
        let conf = cmp_tree::default_config();
        let first_dir = Path::new("../../tests/010/first");
        let second_dir = Path::new("../../tests/010/second");

        let exit_code = cmp_tree::cmp_tree(&conf, &first_dir, &second_dir);
        assert_eq!(exit_code, 1);
        // TODO: this should also check more detailed output. How many files differed? Which files
        // differed? How did they differ?
    }
}
