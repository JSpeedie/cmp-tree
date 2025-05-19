#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	ln -Tfs "../adirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	# Copy all the contents of the first/ to second/, preserving file metadata.
	cp -arT first/ second/
}

# The body of the script, build the two "helper" directory trees, and then the
# first input directory tree and the second
build_first
build_second

