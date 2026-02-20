cmake_minimum_required(VERSION 3.28)

project(globals LANGUAGES C)
set(CMAKE_BUILD_TYPE Debug)

add_library(globals SHARED)
target_sources(globals PRIVATE globals.c globals.h)
