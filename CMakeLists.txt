cmake_minimum_required(VERSION 3.10)

cmake_policy(SET CMP0079 NEW)
cmake_policy(SET CMP0076 NEW)

project(midra_quest_library VERSION 0.0.1 LANGUAGES C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

add_library(midra_quest INTERFACE)

add_subdirectory(src)

