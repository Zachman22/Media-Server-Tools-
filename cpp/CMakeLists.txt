cmake_minimum_required(VERSION 3.15)
project(MediaServerDashboard VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add executable
add_executable(media-dashboard
    src/main.cpp
    src/dashboard.cpp
    src/utils.cpp
)

target_include_directories(media-dashboard PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Enable all warnings
if(MSVC)
    target_compile_options(media-dashboard PRIVATE /W4)
else()
    target_compile_options(media-dashboard PRIVATE -Wall -Wextra -pedantic)
endif()

# Set output directory
set_target_properties(media-dashboard PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Installation
install(TARGETS media-dashboard
    RUNTIME DESTINATION bin
)
