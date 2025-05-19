#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

build_common () {
	cur_wd=$(pwd)
	mkdir -p "$1"
	cd "$1"
	dir_wd=$(pwd)

	mkdir -p "a/i/"
	# Return to dir/
	cd "$dir_wd"

	mkdir -p "b/i/1/"
	mkdir -p "b/ii/2/"
	# Return to dir/
	cd "$dir_wd"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

copy_first_to_second () {
	# Copy all the contents of the first/ to second/, preserving file
	# metadata...
	cp -arT first/ second/
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"
	first_wd=$(pwd)

	mkdir -p "c/i/1/a/"
	mkdir -p "c/ii/2/b/"
	mkdir -p "c/iii/3/c/"
	# Return to first/
	cd "$first_wd"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	mkdir -p "second/"
	cd "second/"
	second_wd=$(pwd)

	mkdir -p "d/i/1/a/"
	mkdir -p "d/ii/2/b/"
	mkdir -p "d/iii/3/c/"
	mkdir -p "d/iv/4/d/"
	# Return to second/
	cd "$second_wd"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# The body of the script. build the first directory tree, then the second First
# build the parts of the first directory tree common to both directory trees,
# then duplicate the partially finished first directory tree to create the
# partially finished second directory tree. Lastly, Finalize both directory
# trees.
build_common "first/"
copy_first_to_second
build_first
build_second

