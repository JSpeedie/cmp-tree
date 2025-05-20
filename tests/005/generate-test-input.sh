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

write_lorem_allcaps_to_file () {
	# {{{
	printf "\
LOREM IPSUM DOLOR SIT AMET, CONSECTETUR ADIPISCING ELIT, SED DO EIUSMOD TEMPOR
INCIDIDUNT UT LABORE ET DOLORE MAGNA ALIQUA. UT ENIM AD MINIM VENIAM, QUIS
NOSTRUD EXERCITATION ULLAMCO LABORIS NISI UT ALIQUIP EX EA COMMODO CONSEQUAT.
DUIS AUTE IRURE DOLOR IN REPREHENDERIT IN VOLUPTATE VELIT ESSE CILLUM DOLORE EU
FUGIAT NULLA PARIATUR. EXCEPTEUR SINT OCCAECAT CUPIDATAT NON PROIDENT, SUNT IN
CULPA QUI OFFICIA DESERUNT MOLLIT ANIM ID EST LABORUM.\n" > $1
	# }}}
}

build_first () {
	cur_wd=$(pwd)
	mkdir -p "first/"
	cd "first/"

	write_lorem_to_file "Lorem.txt"
	magick rose: "rose.png"
	magick -size 256x256 gradient: "linear_gradient.png"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

build_second () {
	cur_wd=$(pwd)
	mkdir -p "second/"
	cd "second/"

	write_lorem_allcaps_to_file "Lorem.txt"
	# Create a horizontally flipped `rose.png`
	magick rose: -flop "rose.png"
	# Create a vertically flipped `linear_gradient.png`
	magick -size 256x256 gradient: -flip "linear_gradient.png"

	# Return to the working directory where this function started
	cd "$cur_wd"
}

# The body of the script, build the first input directory tree and the second
build_first
build_second

