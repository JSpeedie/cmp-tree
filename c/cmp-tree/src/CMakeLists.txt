cmake_minimum_required(VERSION 3.16)
# Use C11
set(CMAKE_C_STANDARD 11)

project(cmp-tree)
add_executable(cmp-tree cmp-tree.c better-strings.c dynamic-array.c)

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)
target_link_libraries(${PROJECT_NAME} PRIVATE Threads::Threads)
