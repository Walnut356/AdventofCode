cmake_minimum_required(VERSION 3.28)
project(aoc2016)

file(GLOB main_src "./src/*.c" "./src/*.h" "./src/md5.h" "./src/md5.c")

add_executable(aoc ${main_src})