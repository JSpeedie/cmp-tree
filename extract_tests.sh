cd tests/

# For all files in the dir at depth=1, sorted, and ending in ".tar.xz"
for i in $(find . -mindepth 1 -maxdepth 1 -type f -printf '%f\n' | sort | grep ".tar.xz$"); do
	# If a directory of that name already exists, don't extract the .tar.xz
	dir_name=$(echo "$i" | sed "s/.tar.xz$//g")
	if [ -d "$dir_name" ]; then
		echo "Error extracting ${i}: a directory of name \"$dir_name\" already exists!"
	else
		tar -xf ${i}
		if [ $? -eq 0 ]; then
			echo "Extracted ${i}"
		else
			echo "Error extracting ${i}: the tar command did not succeed!"
		fi
	fi
done
