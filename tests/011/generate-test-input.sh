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
	first_wd=$(pwd)

	man cmp > cmp_man_pages.txt
	write_lorem_to_file Lorem.txt

	mkdir -p "subdir/"
	cd "subdir/"
	magick -size 256x256 gradient: linear_gradient.png
	magick rose: rose.png

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	# Copy all the contents of the first/ to second/, preserving file
	# metadata...
	cp -arT first/ second/

	# ... but then overwrite the file metadata of all the regular files
	# in second/ to have their access and modification times set to 1 day earlier
	# than their current modification time
	for cur_file in $(find second/ -type f | sort); do
		cur_mod_time=$(stat -c%y ${cur_file})
		touch -d "${cur_mod_time} - 1 day" ${cur_file}
	done
}

# The body of the script, build the first directory tree, then the second
build_first
build_second

