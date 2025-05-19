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
	mkdir -p first/
	cd first/
	man ls > ls_man_pages.txt

	mkdir -p 02/
	cd 02/
	write_lorem_to_file Lorem.txt

	mkdir -p 03/
	cd 03/
	magick rose: rose.png

	mkdir -p 04/
	cd 04/
	magick -size 256x256 gradient: linear_gradient.png

	mkdir -p 05/
	cd 05/
	ffmpeg -y -f lavfi -i testsrc=duration=10:size=1280x720:rate=30 test.mp4

	mkdir -p 06/
	cd 06/
	ffmpeg -y -f lavfi -i gradients=duration=60:nb_colors=2:c0=green:c1=yellow:x0=500:x1=1500:y0=500:y1=1500:type=linear:speed=0.0125:size=2000x2000:rate=60 -pix_fmt yuv420p -g 1 -preset slow gradients.mov

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	# Copy all the contents of the first/ to second/, preserving file
	# metadata...
	cp -arT first/ second/

	# But then overwrite one of the files to be different
	cd second/02/03/04/05/06/
	ffmpeg -y -f lavfi -i gradients=duration=60:nb_colors=2:c0=yellow:c1=orange:x0=500:x1=1500:y0=500:y1=1500:type=linear:speed=0.0125:size=2000x2000:rate=60 -pix_fmt yuv420p -g 1 -preset slow gradients.mov

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# I cannot guarantee that this won't cause issues with some file formats, but
# it does seem to work with these .mov files although I have not thoroughly
# tested that.
pad_differing_file () {
	size_first=$(stat -c%s "first/02/03/04/05/06/gradients.mov")
	size_second=$(stat -c%s "second/02/03/04/05/06/gradients.mov")

	if [[ "$size_first" -lt "$size_second" ]]; then
		dd if=/dev/zero bs=1 count=$((size_second - size_first)) status=none >> "first/02/03/04/05/06/gradients.mov"
	elif [[ "$size_first" -gt "$size_second" ]]; then
		dd if=/dev/zero bs=1 count=$((size_first - size_second)) status=none >> "second/02/03/04/05/06/gradients.mov"
	fi
}

# The body of the script, build the first directory tree, then the second
build_first
build_second
# We also want to make sure the two differing files are of the exact same size
# in terms of bytes, so we need to pad the smaller one.
pad_differing_file

