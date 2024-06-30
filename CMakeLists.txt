# TODO: Add source files to list

cmake_minimum_required(VERSION 3.1...3.22)
project(
    boxtrap
    DESCRIPTION "An Old School Runescape Fan-game about Chinchompas"
    LANGUAGES C CXX 
)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# === Dependencies ===

# === raylib ===
# Check GitHub tags for raylib version 
set(RAYLIB_VERSION 5.0)

# TODO: Refactor pattern to fetch and build third party deps.
#   This is a deprecated pattern
#   (see https://cmake.org/cmake/help/latest/module/FetchContent.html#command:fetchcontent_getproperties 
find_package(raylib QUIET)
# If specified version not found, fetch and build raylib
if (NOT raylib_FOUND) 
    include(FetchContent)
    FetchContent_Declare(
        raylib
        DOWNLOAD_EXTRACT_TIMESTAMP OFF
        URL https://github.com/raysan5/raylib/archive/refs/tags/${RAYLIB_VERSION}.tar.gz
        #GIT_REPOSITORY https://github.com/raysan5/raylib.git
        #GIT_TAG ae50bfa
    )
    FetchContent_GetProperties(raylib)

    # Is raylib downloaded yet?
    if (NOT raylib_POPULATED) 
        set(FETCHCONTENT_QUIET NO)
        FetchContent_Populate(raylib)
        # Do not build the raylib supplied examples or games
        set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
        set(BUILD_GAMES OFF CACHE BOOL "" FORCE)
        add_subdirectory(${raylib_SOURCE_DIR} ${raylib_BINARY_DIR})
    endif()
endif()

# === flecs ===
find_package(flecs QUIET)
# If specified version not found, fetch and build flecs 
if (NOT flecs_FOUND) 
    include(FetchContent)
    FetchContent_Declare(
        flecs
        DOWNLOAD_EXTRACT_TIMESTAMP OFF
        GIT_REPOSITORY https://github.com/SanderMertens/flecs.git
        GIT_TAG 198607d 
    )

    FetchContent_MakeAvailable(flecs)

endif()

# === The Project ===
# TODO: Add source files
set(
    SOURCE_FILES
    src/main.cpp
    vendor/flecs/flecs.c
    #TODO: Add remaining source files 
    #${CMAKE_CURRENT_SOURCE_DIR}/src/components/*.cpp
    #${CMAKE_CURRENT_SOURCE_DIR}/src/entities/*.cpp
    #${CMAKE_CURRENT_SOURCE_DIR}/src/systems/*.cpp
)
        
# Tell CMake what to build
add_executable(${PROJECT_NAME} ${SOURCE_FILES})

target_link_libraries(${PROJECT_NAME} raylib)
target_link_libraries(${PROJECT_NAME} flecs::flecs_static)

# Tell CMake where to find header files (*.h)
target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/vendor/
)

# Compile flecs.c as C code
set_source_files_properties(
    vendor/flecs/flecs.c
    PROPERTIES
    LANGUAGE C
    COMPILE_OPTIONS "-std=gnu99" 
)

# Pass flags to compiler
target_compile_features(
    ${PROJECT_NAME} 
    PRIVATE
    cxx_std_20
)

# The simplest possible build command includes tese compiler options (Linux)
# (See raylib Wiki)
set(EXTRA_RAYLIB_FLAGS lraylib lGL lm lpthread ldl lrt)
add_compile_options(${EXTRA_RAYLIB_FLAGS})

# Check if OSX and links appropriate frameworks (Only required on MacOS)
if (APPLE)
    target_link_libraries(${PROJECT_NAME} "-framework IOKit")
    target_link_libraries(${PROJECT_NAME} "-framework CoCoa")
    target_link_libraries(${PROJECT_NAME} "-framework OpenGL")
endif()
