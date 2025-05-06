/* C++ includes */
#include <algorithm>
#include <array>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <vector>

/* C includes */
#include <dirent.h>
#include <fcntl.h>
#include <getopt.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

/* Local includes */
#include "cmp-tree.hpp"

namespace fs = std::filesystem;


/* For printing coloured output to the terminal */
char NOTHING[] = { "" };
char BOLD[] = { "\x1B[1m" };
char NORMAL[] = { "\x1B[0m" };
char RED[] = { "\x1B[31m" };
char GREEN[] = { "\x1B[32m" };
char YELLOW[] = { "\x1B[33m" };
char BLUE[] = { "\x1B[34m" };
char MAGENTA[] = { "\x1B[35m" };
char CYAN[] = { "\x1B[36m" };
char WHITE[] = { "\x1B[37m" };


/** Returns the default config for `cmp-tree`.
 *
 *  \return a `Config` struct with all its values set to the default values for
 *      `cmp-tree`.
 */
Config default_config() {
    /* {{{ */
    return (Config) {
        .matches = false,
        .pretty = false,
        .totals = false,
    };
    /* }}} */
}


/** Returns an unsorted vector list of relative file paths for all files (in
 * the broad sense of the word, including links and directories, as well as
 * hidden files) in a directory tree rooted at the directory pointed to by the
 * path `&root` / `&extension`. The file paths included in the list will all
 * begin with `&extension`, but not `&root`.
 *
 * \param `&root` the beginning of the file path to the directory for which we
 *     wish to get a list of all the files in the directory tree. It will be
 *     combined with `&extension` to produce the complete path.
 * \param `&extension` the end of the file path to the directory for which we
 *     wish to get a list of all the files in the directory tree. It will be
 *     combined with `&root` to produce the complete path.
 * \return an unsorted vector list of the relative file paths for all the files
 *     in the directory tree rooted at `&root` / `&extension`. The file paths
 *     included in the list will omit `&root` from their path, but include
 *     `&extension`.
 */
std::vector<fs::path> relative_files_in_tree( \
	fs::path &root, fs::path &extension) {
	/* {{{ */

	std::vector<fs::path> ret;
	fs::path dir_path = root / extension;

	DIR *dir;
	/* If we are able to open the directory successfully */
	if ((dir = opendir(dir_path.c_str())) != NULL) {
		struct dirent *dir_entry;
		while ((dir_entry = readdir(dir)) != NULL) {
			fs::path file_name(dir_entry->d_name);
			/* Only includes files that are not the special "." and ".."
			 * entries */
			if (0 != file_name.compare(".") && 0 != file_name.compare("..")) {
				fs::path file_fp = dir_path / file_name;
				fs::path file_rp = extension / file_name;
				ret.push_back(file_rp);

				/* If the current element is a directory... */
				if (fs::is_directory(file_fp)) {
					/* Recurse and append the sub directory relative file
					 * paths */
					std::vector<fs::path> sub_dir_files = \
						relative_files_in_tree(root, file_rp);
					ret.insert(ret.end(), sub_dir_files.begin(), sub_dir_files.end());
				}
			}
		}
		closedir(dir);
	/* If we are NOT able to open the directory successfully */
	} else {
		std::cout << "Was not able to open the directory\n";
	}

	return ret;
	/* }}} */
}


/** Returns an unsorted vector list of file paths for all the files (in the
 * broad sense of the word, including links and directories, as well as hidden
 * files) in a directory tree rooted at the directory pointed to by `&root`.
 *
 * \param `&dir_path` the file path to a directory that roots a directory tree
 *     which we wish to get a list of all files contained within.
 * \return an unsorted vector list of the relative file paths for all the files
 *     in the directory tree rooted at `&root`.
 */
std::vector<fs::path> files_in_tree(fs::path &root) {
	/* {{{ */
	fs::path extension = "";
	return relative_files_in_tree(root, extension);
	/* }}} */
}


/** Takes two paths and returns 0 if the files are byte-for-byte identical,
 * and -1 if they are not. Both file paths must point to regular files and
 * both regular files must exist.
 *
 * \param `&first_path` a file path that points to the first file we wish to
 *     compare.
 * \param `&second_path` a file path that points to the second file we wish to
 *     compare.
 * \return 0 if the files are byte-for-byte identical, -1 otherwise.
 */
int compare_files(fs::path &first_path, fs::path &second_path) {
	/* {{{ */
	/* Check if the files differ in size. If they do, they cannot be
	 * byte-for-byte identical */
	struct stat first_file_info;
	struct stat second_file_info;

	if(stat(first_path.c_str(), &first_file_info) != 0) {
		/* stat() failed, return -1 */
		return -1;
	}

	if(stat(second_path.c_str(), &second_file_info) != 0) {
		/* stat() failed, return -1 */
		return -1;
	}

	if (first_file_info.st_size != second_file_info.st_size) {
		return -1;
	}

	/* Read through both files simultaneously, comparing their bytes. If at any
	 * point two bytes at the same location in the files differ, return -1 */
	std::ifstream first_stream(first_path.c_str(), std::ifstream::binary);
	std::ifstream second_stream(second_path.c_str(), std::ifstream::binary);
	first_stream.sync_with_stdio(false);
	second_stream.sync_with_stdio(false);
	/* Create a buffer of 8192 chars, all initialized to 0(?) */
	std::vector<char> first_buf(8192, 0);
	std::vector<char> second_buf(8192, 0);

	while(first_stream.good() && second_stream.good()) {
		first_stream.read(first_buf.data(), first_buf.size());
		second_stream.read(second_buf.data(), second_buf.size());
		std::streamsize first_bytes_read = first_stream.gcount();
		std::streamsize second_bytes_read = second_stream.gcount();

		/* One file ended before the other */
		if (first_bytes_read != second_bytes_read) {
			return -1;
		}

		if (0 != std::memcmp(first_buf.data(), second_buf.data(), \
			first_bytes_read)) {

			return -1;
		}
	}

	return 0;
	/* }}} */
}


/** Takes two paths and returns a `PartialFileComparison` that represents
 * whether the two files pointed to by the two paths are the same or different.
 *
 * \param `&first_path` a file path that points to the first file we wish to
 *     compare.
 * \param `&second_path` a file path that points to the second file we wish to
 *     compare.
 * \return a `PartialFileComparison` that will represents whether the two files
 *     are equivalent, if they differ and how they differ, as well as the two
 *     file types of the files.
 */
PartialFileComparison compare_path( \
	fs::path &first_path, fs::path &second_path) {
	/* {{{ */

	PartialFileComparison ret;

	/* Check file existences first. If neither path points to files that exist,
	 * return that neither exists. If one file exists, but the other does not,
	 * get the file mode/type of the existing file and return, setting the
	 * comparison member so that the caller knows which file does not exist */
	if (!fs::exists(first_path) && !fs::exists(second_path)) {
		ret.file_cmp = MISMATCH_NEITHER_EXISTS;
		return ret;
	} else if (fs::exists(first_path) && !fs::exists(second_path)) {
		ret.first_ft = fs::status(first_path).type();
		ret.file_cmp = MISMATCH_ONLY_FIRST_EXISTS;
		return ret;
	} else if (!fs::exists(first_path) && fs::exists(second_path)) {
		ret.second_ft = fs::status(second_path).type();
		ret.file_cmp = MISMATCH_ONLY_SECOND_EXISTS;
		return ret;
	}

	/* Check file modes/types. At this point we know both files exist, but if
	 * they are of different types (e.g. a fifo vs a regular file) then
	 * return with the two file modes/types and setting the comparison member
	 * so the caller knows the types of the two files */
	ret.first_ft = fs::status(first_path).type();
	ret.second_ft = fs::status(second_path).type();

	if (ret.first_ft != ret.second_ft) {
		ret.file_cmp = MISMATCH_TYPE;
		return ret;
	}

	/* Check that the two files are equivalent. At this point we know both
	 * files exist and that they are of the same type. The various types the
	 * files could both be need individual methods for checking equivalence.
	 * Regular files will use 'cmp', directories will simply return a match
	 * since both files are directories */

	/* If the two paths point to files that are of different types (e.g. a
	 * directory vs. a symlink, a fifo vs a regular file) then return early,
	 * with the match member set to false */
	if (ret.first_ft != ret.second_ft) {
		return ret;
	}

	/* Check that the two files are equivalent. At this point we know both
	 * files exist and that they are of the same type. The various types the
	 * files could both be need individual methods for checking equivalence.
	 * Regular files will use 'cmp', directories will simply return a match
	 * since both files are directories */
	if (ret.first_ft == fs::file_type::directory) {
		ret.file_cmp = MATCH;
		return ret;
	} else if (ret.first_ft == fs::file_type::regular) {
		/* If the file comparison succeeded we know that this means the two
		 * files are byte-for-byte identical. Return with the comparison
		 * member set to match */
		if (compare_files(first_path, second_path) == 0) {
			ret.file_cmp = MATCH;
			return ret;
		} else {
			ret.file_cmp = MISMATCH_CONTENT;
			return ret;
		}
	/* TODO: Other file types do not yet have support. At the moment, they are
	 * treated the same way directories are: if they both exist, and are of
	 * the same type, return that they match. */
	} else {
		ret.file_cmp = MATCH;
		return ret;
	}
	/* }}} */
}


/** Returns a sorted vector list of `FullFileComparisons` representing
 * comparisons between every file contained in one of the root directories and
 * the file of the same relative path in the other root directory. This
 * includes comparisons between a file and its non-existent equivalent if there
 * is no equivalent in the other root directory. The list is sorted by the
 * relative path of each `FullFileComparison`.
 *
 * \param `&first_root` the file path to the root of the first directory tree.
 * \param `&second_root` the file path to the root of the second directory
 *     tree.
 * \return a vector list of `FullFileComparisons` representing the comparisons
 *     between every file contained in both root directories.
 */
std::vector<FullFileComparison> compare_directory_trees( \
	fs::path &first_root, fs::path &second_root) {
	/* {{{ */

	std::vector<FullFileComparison> ret;
	/* Get the first directory file list and the second directory file list:
	 * the list of files in each directory */
	std::vector<fs::path> first_ft = files_in_tree(first_root);
	std::vector<fs::path> second_ft = files_in_tree(second_root);

	/* Create a vector that contains both the files from the first directory
	 * tree and the files from the second directory tree */
	std::vector<fs::path> combined_ft(first_ft.begin(), first_ft.end());
	combined_ft.insert(combined_ft.end(), second_ft.begin(), second_ft.end());
	/* Sort the combined file tree and remove duplicate items */
	std::sort(combined_ft.begin(), combined_ft.end());
	auto last = std::unique(combined_ft.begin(), combined_ft.end());
	combined_ft.erase(last, combined_ft.end());

	/* Go through all the files in the combined  file list, create two full
	 * paths to the file, one rooted at '&first_root', one rooted at
	 * '&second_root', and compare them */
	for (auto &e: combined_ft) {
		FullFileComparison res;
		res.first_path = first_root / e;
		res.second_path = second_root / e;
		res.partial_cmp = compare_path(res.first_path, res.second_path);
		ret.push_back(res);
	}

	return ret;
	/* }}} */
}


/** Takes a `std::vector<FullFileComparison>` and prints out the necessary
 * information about the list of file comparisons. What information is printed
 * will depend on the values of `&config`.
 *
 * \param `&config` a `Config` representing a configuration for executing
 *     `cmp-tree`, usually modified through command line arguments to the
 *     program.
 * \param `directory_tree_comparison` a `std::vector<FullFileComparison>`.
 *     Typically, this parameter is the result of a call to
 *     `compare_directory_trees()`.
 */
void print_output(Config &config, \
	std::vector<FullFileComparison> directory_tree_comparison) {
	/* {{{ */

	long max_num_file_matches = 0;
	long max_num_dir_matches = 0;
	long num_file_matches = 0;
	long num_dir_matches = 0;

	for (auto e: directory_tree_comparison) {
		if (config.totals) {
			if (e.partial_cmp.first_ft == fs::file_type::directory \
				|| e.partial_cmp.second_ft == fs::file_type::directory) {

				max_num_dir_matches++;
			}
			if (e.partial_cmp.first_ft == fs::file_type::regular \
				|| e.partial_cmp.second_ft == fs::file_type::regular) {

				max_num_file_matches++;
			}
		}

		switch (e.partial_cmp.file_cmp) {
			case MATCH:
				if (config.matches) {
					if (config.pretty) printf("%s%s", BOLD, GREEN);
					printf("\"%s\" == \"%s\"\n",
						e.first_path.c_str(), e.second_path.c_str());
					if (config.pretty) printf("%s", NORMAL);
				}
				if (e.partial_cmp.first_ft == fs::file_type::regular) {
					num_file_matches++;
				} else if (e.partial_cmp.first_ft == fs::file_type::directory) {
					num_dir_matches++;
				}
				break;
			case MISMATCH_TYPE:
				if (config.pretty) printf("%s%s", BOLD, RED);
				printf("\"%s\" is not of the same type as \"%s\"\n",
					e.first_path.c_str(), e.second_path.c_str());
				if (config.pretty) printf("%s", NORMAL);
				break;
			case MISMATCH_CONTENT:
				if (config.pretty) printf("%s%s", BOLD, RED);
				printf("\"%s\" differs from \"%s\"\n",
					e.first_path.c_str(), e.second_path.c_str());
				if (config.pretty) printf("%s", NORMAL);
				break;
			case MISMATCH_NEITHER_EXISTS:
				if (config.pretty) printf("%s%s", BOLD, RED);
				printf("Neither \"%s\" nor \"%s\" exist\n",
					e.first_path.c_str(), e.second_path.c_str());
				if (config.pretty) printf("%s", NORMAL);
				break;
			case MISMATCH_ONLY_FIRST_EXISTS:
				if (config.pretty) printf("%s%s", BOLD, RED);
				printf("\"%s\" exists, but \"%s\" does NOT exist\n",
					e.first_path.c_str(), e.second_path.c_str());
				if (config.pretty) printf("%s", NORMAL);
				break;
			case MISMATCH_ONLY_SECOND_EXISTS:
				if (config.pretty) printf("%s%s", BOLD, RED);
				printf("\"%s\" does NOT exist, but \"%s\" does exist\n",
					e.first_path.c_str(), e.second_path.c_str());
				if (config.pretty) printf("%s", NORMAL);
				break;
		}
	}

	if (config.totals) {
		fprintf(stdout, "All done!\n");
		fprintf(stdout, "File byte-for-byte matches: %ld/%ld\n", \
			num_file_matches, max_num_file_matches);
		fprintf(stdout, "Directory matches: %ld/%ld\n", num_dir_matches, \
			max_num_dir_matches);
	}
	/* }}} */
}


/** Takes a `std::vector<FullFileComparison>` and returns a boolean
 * representing whether or not the file comparison list received as input
 * contains any mismatches or not.
 *
 * \param `directory_tree_comparison` a `std::vector<FullFileComparison>`.
 *     Typically, this parameter is the result of a call to
 *     `compare_directory_trees()`.
 * \return a `bool` that will be `true` if there were any mismatches in the
 *     directory tree comparison and `false` if the comparison found the two
 *     directory trees to be identical.
 */
bool directory_tree_comparison_contains_mismatch(
	std::vector<FullFileComparison> directory_tree_comparison) {
	/* {{{ */

	/* For every comparison in the list... */
	for (auto &e: directory_tree_comparison) {
		/* If the comparison found a mismatch of any kind between the two
		 * files, return early */
		if (e.partial_cmp.file_cmp != MATCH) {
			return true;
		}
	}
	/* If we make it here, that means no mismatches of any kind were found in
	 * the file comparison list. */
	return false;
	/* }}} */
}


/** Takes a `Config` and two `fs::path`s that point to two directory trees, and
 * compares the two directory trees, returning an `int32_t` representing the
 * appropriate exit code for this program given how the execution went.
 *
 * \param `&config` a `Config` representing a configuration for executing
 *     `cmp-tree`, usually modified through command line arguments to the
 *     program.
 * \param `&first_dir` a file path that points to the root directory of the
 *     first directory tree we wish to compare. This function assumes that this
 *     path points to a directory and that the directory exists.
 * \param `&second_dir` a file path that points to the root directory of the
 *     second directory tree we wish to compare. This function assumes that
 *     this path points to a directory and that the directory exists.
 * \return an `int32_t` that represents how execution of the directory tree
 *     comparison went. If there was an error during execution, 2 is returned.
 *     If the comparison proceeded without error, but mismatches between files
 *     were found, 1 is returned. If the comparison proceeeded without error
 *     and no mismatches were found, 0 is returned.
 */
int32_t cmp_tree(Config &config, fs::path &first_dir, fs::path &second_dir) {
	/* {{{ */
	/* Perform the comparison between the two directory trees */
	std::vector<FullFileComparison> directory_tree_comparison = \
		compare_directory_trees(first_dir, second_dir);

	/* Check if any mismatches occurred (this is needed to determine the exit
	 * code of this program */
	bool mismatch_occurred = directory_tree_comparison_contains_mismatch( \
		directory_tree_comparison);
	/* Print the appropriate output, provided silent mode is off */
	print_output(config, directory_tree_comparison);

	/* If a mismatch occurred during the comparison, exit with exit code 1. If
	 * there were no mismatches, and the directory trees are identical, exit
	 * with exit code 0. */
	if (mismatch_occurred) {
		return 1;
	} else {
		return 0;
	}
	/* }}} */
}


int main(int argc, char **argv) {

	Config conf = default_config();

	int opt;
	struct option opt_table[] = {
		{ "matches",  no_argument,  NULL,  'm' },
		{ "pretty",   no_argument,  NULL,  'p' },
		{ "totals",   no_argument,  NULL,  't' },
		{ 0, 0, 0, 0 }
	};
	char opt_string[] = { "mpt" };

	while ((opt = getopt_long(argc, argv, opt_string, opt_table, NULL)) != -1) {
		switch (opt) {
			case 'm':
				conf.matches = true;
				break;
			case 'p':
				conf.pretty = true;
				break;
			case 't':
				conf.totals = true;
				break;
		}
	}

	/* If after parsing all the flags there aren't 2 arguments left */
	if (optind + 2 != argc) {
		fprintf(stdout, "Expected 2 arguments, received %d\n", argc - 1);
	}

	fs::path first_path(argv[optind]);
	optind++;
	fs::path second_path(argv[optind]);
	/* Create an array of arguments that specify directories so that
	 * we can check their validity */
	std::array<fs::path, 2> directory_args = { first_path, second_path };

	/* Loop through all the arguments that specify directories and check that
	 * they are valid */
	for (auto &e: directory_args) {
		/* Check if the given argument is a file path that points to something
		* that exists... */
		if (!fs::exists(e)) {
			std::cout << "Provided directory (" << e << \
				") does not exist. Exiting...\n";
			return -1;
		} else {
			/* ... and that it points to a directory */
			if (!fs::is_directory(e)) {
				std::cout << "Provided directory (" << e << \
					") is not a directory. Exiting...\n";
				return -1;
			}
		}
	}

	int32_t exit_code = cmp_tree(conf, first_path, second_path);
	exit(exit_code);
}
