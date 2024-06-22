# === CMake Template ===
#
# TODO: Change <PROJECT-NAME> 
# TODO: Change executable name  
# TODO: Add source files to list

cmake_minimum_required(VERSION 3.1...3.7)
project(
    MyTemplateRaylibProject
    DESCRIPTION "raylib Template"
    LANGUAGES CXX
)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# === Dependencies ===

# raylib.h gives the raylib version
set(RAYLIB_VERSION 5.0)

find_package(raylib ${RAYLIB_VERSION} QUIET)
# If specified version not found, fetch and build raylib
if (NOT raylib_FOUND) 
    include(FetchContent)
    FetchContent_Declare(
        raylib
        DOWNLOAD_EXTRACT_TIMESTAMP OFF
        URL https://github.com/raysan5/raylib/archive/refs/tags/${RAYLIB_VERSION}.tar.gz
    )
    FetchContent_GetProperties(raylib)

    # Is raylib downloaded yet?
    if (NOT raylib_POPULATED) 
        set(FETCHCONTENT_QUIET NO)
        FetchCOntent_Populate(raylib)
        # Do not build the raylib supplied examples or games
        set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
        set(BUILD_GAMES OFF CACHE BOOL "" FORCE)
        add_subdirectory(${raylib_SOURCE_DIR} ${raylib_BINARY_DIR})
    endif()
endif()

# Fetch raylib from the master branch on GitHub
# TODO: Do not set RAYLIB_VERSION variable when using this method
# FetchContent_Declare
# (
#     raylib
#     GIT_REPOSITORY "https://github.com/raysan5/raylib.git"
#     GIT_TAG "master"
#     GIT_PROGRESS TRUE
# )


# === The Project ===
set(EXECUTABLE_BINARY_NAME game)

# TODO: Add source files
set(
    SOURCE_FILES
    src/main.cpp
    src/Foo.cpp
)
        
# Tell CMake what to build
add_executable(${EXECUTABLE_BINARY_NAME} ${SOURCE_FILES})

target_link_libraries(${EXECUTABLE_BINARY_NAME} raylib)
# Tell CMake where to find header files (*.h)
target_include_directories(
    ${EXECUTABLE_BINARY_NAME}
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Pass flags to compiler
target_compile_features(
    ${EXECUTABLE_BINARY_NAME} 
    PRIVATE
    cxx_std_20
)

# The simplest possible build command includes tese compiler options (Linux)
# (See raylib Wiki)
set(EXTRA_RAYLIB_LIBRARIES lraylib lGL lm lpthread ldl lrt)
add_compile_options(${EXTRA_RAYLIB_LIBRARIES})

# Check if OSX and links appropriate frameworks (Only required on MacOS)
if (APPLE)
    target_link_libraries(${EXECUTABLE_BINARY_NAME} "-framework IOKit")
    target_link_libraries(${EXECUTABLE_BINARY_NAME} "-framework CoCoa")
    target_link_libraries(${EXECUTABLE_BINARY_NAME} "-framework OpenGL")
endif()
