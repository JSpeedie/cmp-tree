#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

write_filler_text_file () {
	# {{{
	printf "This is a file with some text.

It should be identical to the corresponding file in the other directory \
tree.\n" > "$1"
	# }}}
}

write_link_text_file () {
	# {{{
	printf "https://www. not a valid url .com/page\n" > "$1"
	# }}}
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	mkdir -p "adirectory/"
	cd "adirectory/"
	mkdir -p "dir/"
	write_filler_text_file "file.txt"
	cd ..

	ln -Tfs "adirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	mkdir -p "second/"
	cd "second/"

	mkdir -p "adirectory/"
	cd "adirectory/"
	mkdir -p "dir/"
	write_filler_text_file "file.txt"
	cd ..

	write_link_text_file "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# The body of the script, build the first input directory tree and the second
build_first
build_second

