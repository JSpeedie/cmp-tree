# Specify included directories for `gcc`
INCS = -I.
# Specify libraries for `gcc`
LIBS = -lpthread
# Flags
CFLAGS = -Wall
# Compiler and linker
CC = gcc
# The name of the final executable we want to make
EXECUTABLENAME = cmp-tree
# The relative path to the source directory that contains all the source code
SRC = src

# Debug Build Settings
DEBUGDIR = debug
DEBUGCFLAGS = -ggdb

# Release Build Settings
RELEASEDIR = release
RELEASECFLAGS = -O2

# `debug` first because we want `make` to just compile the debug build by
# default, and the default target is always the the first one that doesn't
# begin with "."
.PHONY: debug
debug: prep-debug $(DEBUGDIR)/$(EXECUTABLENAME)

release: prep-release $(RELEASEDIR)/$(EXECUTABLENAME)

prep-debug:
	@mkdir -p $(DEBUGDIR)

prep-release:
	@mkdir -p $(RELEASEDIR)

clean:
	rm -rf $(DEBUGDIR)
	rm -rf $(RELEASEDIR)

####################
#    Debug Build   #
####################

# Note: The following 3 rules look like they can be abstracted into a single
# rule, but doing so would require that all future files put under that rule
# would have to have a corresponding .h file to go with the source and object
# file. This very well might not be the case. However, we can't just omit the
# .h file prerequisite because we want all files that have a corresponding .h
# file to have that file as a prerequisite since if something changes in the .h
# file we want the corresponding object file to be recompiled.

# Create the cmp-tree object file
$(DEBUGDIR)/cmp-tree.o: $(SRC)/cmp-tree.c $(SRC)/cmp-tree.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Create the dynamic array object file
$(DEBUGDIR)/dynamic-array.o: $(SRC)/dynamic-array.c $(SRC)/dynamic-array.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Create the better strings object file
$(DEBUGDIR)/better-strings.o: $(SRC)/better-strings.c $(SRC)/better-strings.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Take all generated object files and libraries, link and produce the final
# executable
$(DEBUGDIR)/$(EXECUTABLENAME): $(DEBUGDIR)/cmp-tree.o $(DEBUGDIR)/dynamic-array.o $(DEBUGDIR)/better-strings.o
	$(CC) $(DEBUGCFLAGS) $^ $(INCS) $(LIBS) -o $@

####################
#   Release Build  #
####################

# Note: The following 3 rules look like they can be abstracted into a single
# rule, but doing so would require that all future files put under that rule
# would have to have a corresponding .h file to go with the source and object
# file. This very well might not be the case. However, we can't just omit the
# .h file prerequisite because we want all files that have a corresponding .h
# file to have that file as a prerequisite since if something changes in the .h
# file we want the corresponding object file to be recompiled.

# Create the cmp-tree object file
$(RELEASEDIR)/cmp-tree.o: $(SRC)/cmp-tree.c $(SRC)/cmp-tree.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Create the dynamic array object file
$(RELEASEDIR)/dynamic-array.o: $(SRC)/dynamic-array.c $(SRC)/dynamic-array.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Create the better strings object file
$(RELEASEDIR)/better-strings.o: $(SRC)/better-strings.c $(SRC)/better-strings.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Take all generated object files and libraries, link and produce the final
# executable
$(RELEASEDIR)/$(EXECUTABLENAME): $(RELEASEDIR)/cmp-tree.o $(RELEASEDIR)/dynamic-array.o $(RELEASEDIR)/better-strings.o
	$(CC) $(RELEASECFLAGS) $^ $(INCS) $(LIBS) -o $@
