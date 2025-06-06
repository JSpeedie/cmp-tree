#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	# Write a file of 50,000 bytes pulled from /dev/urandom
	dd if="/dev/urandom" bs=1 count=50000 of="random_file" status=none
	# Change the 25,000th byte to be 'a'
	printf "a" \
		| dd bs=1 count=1 seek=24999 of="random_file" conv=notrunc status=none

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	# Copy all the contents of the first/ to second/, preserving file
	# metadata...
	cp -arT "first/" "second/"

	# But then change one byte in `random_file` so that it differs
	cd "second/"
	# Change the 25,000th byte to be 'b'
	printf "b" \
		| dd bs=1 count=1 seek=24999 of="random_file" conv=notrunc status=none

	# Return to the working directory where this function started
	cd "$cur_wd"
}


# The body of the script, build the first directory tree and the second
build_first
build_second

