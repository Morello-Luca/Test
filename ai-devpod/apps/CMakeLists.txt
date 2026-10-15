cmake_minimum_required(VERSION 3.20)

project(Omni
    VERSION 1.0
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Shared dependency
add_subdirectory(
    "${CMAKE_CURRENT_SOURCE_DIR}/../third_party/llama.cpp"
    "${CMAKE_CURRENT_BINARY_DIR}/third_party/llama.cpp"
    EXCLUDE_FROM_ALL
)

# Discover applications
file(GLOB APP_DIRS CONFIGURE_DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/*"
)

foreach(APP_DIR ${APP_DIRS})
    if(IS_DIRECTORY "${APP_DIR}" AND EXISTS "${APP_DIR}/CMakeLists.txt")
        add_subdirectory("${APP_DIR}")
    endif()
endforeach()