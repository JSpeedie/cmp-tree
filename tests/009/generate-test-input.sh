#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

write_filler_text_file_one () {
	# {{{
	printf "This is a file with some text.






Here is some more text.\n" > "$1"
	# }}}
}

write_filler_text_file_two () {
	# {{{
	printf "This is a file with some text.

It is in the \`anotherdirectory\` directory and will be different from
its corresponding file.\n" > "$1"
	# }}}
}

build_adirectory() {
	cur_wd=$(pwd)
	mkdir -p "adirectory/"
	cd "adirectory/"

	mkdir -p "dir/"
	write_filler_text_file_one "file.txt"
	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_anotherdirectory() {
	cur_wd=$(pwd)
	mkdir -p "anotherdirectory/"
	cd "anotherdirectory/"

	write_filler_text_file_two "file.txt"
	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	ln -Tfs "../adirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	mkdir -p "second/"
	cd "second/"

	ln -Tfs "../anotherdirectory/" "link"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# The body of the script, build the two "helper" directory trees, and then the
# first input directory tree and the second
build_adirectory
build_anotherdirectory
build_first
build_second

