cmake_minimum_required(VERSION 3.0)
PROJECT(LibBadMath C)
add_library(badmath STATIC badmath.c)
install(TARGETS badmath DESTINATION .)