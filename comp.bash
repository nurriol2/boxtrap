#!/bin/bash 

# This is a convenience script to build the project with CMake
# To use, run `bash comp.bash <executable_name>` in the terminal

read -p "What is the name of the executable?    " executable_name

echo "Buliding " $executable_name "..."
# Move into the build directory
cd ./build/ 

# Run CMake using the top level CMakeLists.txt
cmake ../

# Build in the curring directory using CMake
cmake --build .

echo "Running " $executable_name "..."
# Run the executable
./$executable_name
