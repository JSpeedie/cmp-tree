GLOBAL_INSTALL="/usr/local/bin/"
LOCAL_INSTALL="/usr/local/bin/"
MAN_INSTALL="/usr/share/man/man1/"

sudo mkdir -p ${GLOBAL_INSTALL}
sudo cp -f target/release/cmp-tree ${GLOBAL_INSTALL}/cmp-tree
sudo chmod 755 ${GLOBAL_INSTALL}/cmp-tree
sudo mkdir -p ${MAN_INSTALL}
sudo cp -f cmp-tree.1.gz ${MAN_INSTALL}/cmp-tree.1.gz
