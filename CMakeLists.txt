cmake_minimum_required(VERSION 3.29)
project(RayTracing)

set(CMAKE_CXX_STANDARD 20)

find_package(imgui CONFIG REQUIRED)
find_package(glfw3 CONFIG REQUIRED)
find_package(fmt CONFIG REQUIRED)

add_executable(RayTracing src/main.cpp
)

target_include_directories(RayTracing PUBLIC src/public)

target_link_libraries(RayTracing PRIVATE imgui::imgui)
target_link_libraries(RayTracing PRIVATE fmt::fmt)