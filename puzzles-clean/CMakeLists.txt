cmake_minimum_required(VERSION 3.5)

project(puzzles
  LANGUAGES C)

include(cmake/setup.cmake)

add_library(core_obj OBJECT
  combi.c divvy.c drawing.c dsf.c findloop.c grid.c latin.c
  laydomino.c loopgen.c malloc.c matching.c midend.c misc.c penrose.c
  penrose-legacy.c ps.c random.c sort.c tdq.c tree234.c version.c
  ${platform_common_sources})
add_library(core $<TARGET_OBJECTS:core_obj>)
add_library(common $<TARGET_OBJECTS:core_obj> hat.c spectre.c)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})

cliprogram(loopygenerator loopy.c
  COMPILE_DEFINITIONS STANDALONE_GENERATOR)

if(build_cli_programs)
  write_generated_games_header()
endif()

build_extras()
