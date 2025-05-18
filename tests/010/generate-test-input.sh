#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

write_filler_text_file () {
	# {{{
	printf "This is a file with some text.

	It should be identical to the corresponding file in the other directory\
	tree." > "$1"
	# }}}
}

build_adirectory() {
	cur_wd=$(pwd)
	mkdir -p "adirectory/"
	cd "adirectory/"
	mkdir -p "dir/"
	write_filler_text_file "file.txt"
	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_anotherdirectory() {
	# Copy all the contents of the adirectory/ to anotherdirectory/, preserving
	# file metadata...
	cp -arT "adirectory/" "anotherdirectory/"
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	ln -s "../adirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	mkdir -p "second/"
	cd "second/"

	ln -s "../anotherdirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# The body of the script, build the two "helper" directory trees, and then the
# first input directory tree and the second
build_adirectory
build_anotherdirectory
build_first
build_second

