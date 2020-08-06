cmake_minimum_required(VERSION 3.9)

if (WIN32)
  set (CMAKE_GENERATOR_PLATFORM "x64")
  set (CMAKE_CXX_FLAGS "/Ox /EHsc /Zi /Zo /std:c++17")
endif (WIN32)

set (CMAKE_CXX_STANDARD 17)

project(jdsd_dsiii_practice_tool C CXX)
set( CMAKE_VERBOSE_MAKEFILE on )

include_directories(jdsd_dsiii_practice_tool PUBLIC include)
link_directories(jdsd_dsiii_practice_tool PUBLIC lib)
file(GLOB_RECURSE jdsd_dsiii_practice_tool_SRC src/dll/*.h src/dll/*.cpp)

add_library(jdsd_dsiii_practice_tool MODULE ${jdsd_dsiii_practice_tool_SRC})

target_link_libraries(jdsd_dsiii_practice_tool libMinHook-x64-v141-mt.lib)
target_link_libraries(jdsd_dsiii_practice_tool shlwapi.lib)
target_link_libraries(jdsd_dsiii_practice_tool d3d11.lib)
target_compile_definitions(jdsd_dsiii_practice_tool PUBLIC BUILD_CONFIG="${CMAKE_BUILD_TYPE}")
set_target_properties(jdsd_dsiii_practice_tool PROPERTIES PREFIX "")

project(jdsd_dsiii_practice_tool_injector C CXX)

file(GLOB_RECURSE jdsd_dsiii_practice_tool_injector_SRC src/injector/*.cpp src/injector/*.h src/injector/resources.rc)
add_executable(jdsd_dsiii_practice_tool_injector WIN32 ${jdsd_dsiii_practice_tool_injector_SRC})
set_target_properties(jdsd_dsiii_practice_tool_injector PROPERTIES OUTPUT_NAME "jdsd_dsiii_practice_tool")
