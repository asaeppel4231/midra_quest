add_library(mq_basic STATIC)

target_include_directories(mq_basic PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_subdirectory(misc)
add_subdirectory(position)
add_subdirectory(safe)
add_subdirectory(vector)
add_subdirectory(line)
add_subdirectory(shapes)
