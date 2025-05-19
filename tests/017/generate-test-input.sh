#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	touch empty_file.txt

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	# Copy all the contents of the first/ to second/, preserving file metadata.
	cp -arT first/ second/
}

# The body of the script, build the first directory tree, then the second
build_first
build_second

