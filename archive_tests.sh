cd tests/

# For all directories in the dir at depth=1, sorted
for i in $(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | sort); do
	tar -cJf ${i}.tar.xz ${i}
	if [ $? -eq 0 ]; then
		echo "Archived ${i}.tar.xz"
	else
		echo "Error archiving ${i}.tar.xz!"
	fi
done
