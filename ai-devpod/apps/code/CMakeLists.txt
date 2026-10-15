add_executable(Loader
    src/main.cpp
    src/LLMLoader.cpp
    src/LLMSession.cpp
    src/ChatCLI.cpp
)
target_include_directories(Loader PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(Loader PRIVATE llama)