cmake_minimum_required(VERSION 3.28)
project(aoc2016)

file(GLOB main_src "./src/*.c" "./src/*.h")

add_executable(aoc ${main_src})