import importlib  # So we can import cmp-tree, a module that has a hyphen in its filename
cmptree = importlib.import_module("cmp-tree")  # Import cmp-tree
from pathlib import Path # For Paths


# Integration tests

def test_001_1_level_trees_identical_regular_files():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/001/first")
    second_dir = Path("../../tests/001/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 0


def test_002_1_level_trees_single_differing_regular_file():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/002/first")
    second_dir = Path("../../tests/002/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1


def test_003_2_level_trees_identical_regular_files():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/003/first")
    second_dir = Path("../../tests/003/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 0


def test_004_2_level_trees_single_differing_regular_file():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/004/first")
    second_dir = Path("../../tests/004/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1


def test_005_1_level_trees_exclusively_differing_regular_files():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/005/first")
    second_dir = Path("../../tests/005/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1


def test_006_4_level_trees_identical_dirs():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/006/first")
    second_dir = Path("../../tests/006/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 0


def test_007_4_level_trees_missing_a_subdir():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/007/first")
    second_dir = Path("../../tests/007/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1


def test_008_2_level_trees_identical_soft_link_that_points_to_identical_directories():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/008/first")
    second_dir = Path("../../tests/008/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 0


def test_009_2_level_trees_differing_soft_links_that_point_to_differing_directories():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/009/first")
    second_dir = Path("../../tests/009/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1


def test_010_2_level_trees_differing_soft_links_that_point_to_identical_directories():
    conf = cmptree.default_config()
    first_dir = Path("../../tests/010/first")
    second_dir = Path("../../tests/010/second")

    exit_code = cmptree.cmp_tree(conf, first_dir, second_dir)
    assert exit_code == 1
