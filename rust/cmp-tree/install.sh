GLOBAL_INSTALL="/usr/local/bin/"
LOCAL_INSTALL="/usr/local/bin/"
MAN_INSTALL="/usr/share/man/man1/"

sudo mkdir -p ${GLOBAL_INSTALL}
# If the release build has been compiled...
if [ -f "target/release/cmp-tree" ]; then
	# ... then install it.
	sudo cp -f target/release/cmp-tree ${GLOBAL_INSTALL}/cmp-tree
	sudo chmod 755 ${GLOBAL_INSTALL}/cmp-tree
else
	echo "Error: could not find the cmp-tree binary. Perhaps you haven't compiled the release build yet?"
fi
sudo mkdir -p ${MAN_INSTALL}
sudo cp -f cmp-tree.1.gz ${MAN_INSTALL}/cmp-tree.1.gz
