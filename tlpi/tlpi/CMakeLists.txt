cmake_minimum_required(VERSION 3.28)
project(tlpi LANGUAGES C)

option(TLPI_SHARED "Enable shared library (dynamic linking)" OFF)

set(TLPI_LIB_TYPE STATIC)
if(TLPI_SHARED)
  set(TLPI_LIB_TYPE SHARED)
endif()

add_library(${PROJECT_NAME} ${TLPI_LIB_TYPE})

target_sources(${PROJECT_NAME} PRIVATE 
  error_functions.c
  ename.c.inc
  get_num.c
)

target_include_directories(
  ${PROJECT_NAME}
  PUBLIC
    "$<BUILD_LOCAL_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../lib/;${CMAKE_CURRENT_BINARY_DIR}/>"
)
target_compile_features(${PROJECT_NAME} PRIVATE c_std_17)


