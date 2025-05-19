#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

write_lorem_to_file () {
	# {{{
	printf "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in
culpa qui officia deserunt mollit anim id est laborum.\n" > $1
	# }}}
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	man cmp > "cmp_man_pages.txt"
	write_lorem_to_file "Lorem.txt"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	# Copy all the contents of the first/ to second/, preserving file
	# metadata.
	cp -arT first/ second/
}

# The body of the script, build the first input directory tree and the second
build_first
build_second

