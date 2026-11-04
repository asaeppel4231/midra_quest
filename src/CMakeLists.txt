target_include_directories(midra_quest INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

add_subdirectory(basics)
#add_subdirectory(physics) does currently not have a CMake file
#add_subdirectory(rt)

target_link_libraries(midra_quest INTERFACE mq_basic)

# Not implemented yet
# add_subdirectory(window)
# add_subdirectory(sound)

