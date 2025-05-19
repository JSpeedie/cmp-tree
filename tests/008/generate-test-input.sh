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
	# Copy all the contents of the first/ to second/, preserving file
	# metadata...
	cp -arT first/ second/

	# ... but then overwrite the file metadata of one specific file to be
	# different
	touch -m -h -d "$(stat -c%y 'second/link') + 1 hour" 'second/link'
}

# The body of the script, build the two "helper" directory trees, and then the
# first input directory tree and the second
build_first
build_second

