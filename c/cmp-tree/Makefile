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

# Debug Build Settings
DEBUGDIR = debug
DEBUGEXECUTABLE = $(DEBUGDIR)/$(EXECUTABLENAME)
DEBUGCFLAGS = -ggdb

# Release Build Settings
RELEASEDIR = release
RELEASEEXECUTABLE = $(RELEASEDIR)/$(EXECUTABLENAME)
RELEASECFLAGS = -O2

# `debug` first because we want `make` to just compile the debug build by
# default, and the default target is always the the first one that doesn't
# begin with "."
.PHONY: debug
debug: prep-debug $(DEBUGDIR)/cmp-tree

release: prep-release $(RELEASEDIR)/cmp-tree

prep-debug:
	@mkdir -p $(DEBUGDIR)

prep-release:
	@mkdir -p $(RELEASEDIR)

####################
#    Debug Build   #
####################

# Note: The following 3 rules look like they can be abstracted into a single
# rule, but doing so would require that all future files put under that rule
# would have to have a corresponding .h file to go with the source and object
# file. This very well might not be the case. However, we want all files that
# have a corresponding .h file to have that file as a prerequisite, since if
# something changes in the .h file, we want to recompile our object files.

# Create the cmp-tree object file
$(DEBUGDIR)/cmp-tree.o: cmp-tree.c cmp-tree.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Create the dynamic array object file
$(DEBUGDIR)/dynamic-array.o: dynamic-array.c dynamic-array.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Create the better strings object file
$(DEBUGDIR)/better-strings.o: better-strings.c better-strings.h
	$(CC) $(DEBUGCFLAGS) $< -c -o $@

# Take all generated object files and libraries, link and produce the final
# executable
$(DEBUGDIR)/cmp-tree: $(DEBUGDIR)/cmp-tree.o $(DEBUGDIR)/dynamic-array.o $(DEBUGDIR)/better-strings.o
	$(CC) $(DEBUGCFLAGS) $^ $(INCS) $(LIBS) -o $@

####################
#   Release Build  #
####################

# Note: The following 3 rules look like they can be abstracted into a single
# rule, but doing so would require that all future files put under that rule
# would have to have a corresponding .h file to go with the source and object
# file. This very well might not be the case. However, we want all files that
# have a corresponding .h file to have that file as a prerequisite, since if
# something changes in the .h file, we want to recompile our object files.

# Create the cmp-tree object file
$(RELEASEDIR)/cmp-tree.o: cmp-tree.c cmp-tree.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Create the dynamic array object file
$(RELEASEDIR)/dynamic-array.o: dynamic-array.c dynamic-array.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Create the better strings object file
$(RELEASEDIR)/better-strings.o: better-strings.c better-strings.h
	$(CC) $(RELEASECFLAGS) $< -c -o $@

# Take all generated object files and libraries, link and produce the final
# executable
$(RELEASEDIR)/cmp-tree: $(RELEASEDIR)/cmp-tree.o $(RELEASEDIR)/dynamic-array.o $(RELEASEDIR)/better-strings.o
	$(CC) $(RELEASECFLAGS) $^ $(INCS) $(LIBS) -o $@
