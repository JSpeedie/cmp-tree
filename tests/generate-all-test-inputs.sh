#!/usr/bin/env bash

# Exit immediately if a command fails
set -e

cur_wd=$(pwd)
# For all directories in the dir at depth=1, sorted, with names made only of 3 digits
for i in $(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | sort | grep "^[0-9]\{3\}$"); do
	cd "$i"
	echo "##################################################"
	echo "Generating test input \"$i\""
	echo "##################################################"
	./generate-test-input.sh

	# Return to the working directory where this loop started
	cd "$cur_wd"
done

