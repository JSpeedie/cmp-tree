#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"
	start_wd=$(pwd)

	mkdir -p "a/i/"
	# Return to start
	cd "$start_wd"

	mkdir -p "b/i/1/"
	mkdir -p "b/ii/2/"
	# Return to start
	cd "$start_wd"

	mkdir -p "c/i/1/a/"
	mkdir -p "c/ii/2/b/"
	mkdir -p "c/iii/3/c/"
	# Return to start
	cd "$start_wd"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	# Copy all the contents of the first/ to second/, preserving file
	# metadata.
	cp -arT "first/" "second/"
}

# The body of the script, build the first input directory tree and the second
build_first
build_second

