cmake_minimum_required(VERSION 3.12)
project (Camera)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_BUILD_TYPE Release)


include_directories(src/include)
include_directories(/usr/include/libcamera)

add_library(libcamera SHARED IMPORTED)
set_property(TARGET libcamera PROPERTY IMPORTED_LOCATION "/lib/aarch64-linux-gnu/libcamera.so")

# add_library(libcamera_app SHARED IMPORTED)
# set_property(TARGET libcamera_app PROPERTY IMPORTED_LOCATION "/lib/aarch64-linux-gnu/libcamera_app.so")

add_library(libcamera_base SHARED IMPORTED)
set_property(TARGET libcamera_base PROPERTY IMPORTED_LOCATION "/lib/aarch64-linux-gnu/libcamera-base.so")


# add_executable(camera src/main.cpp src/camera.cpp)
add_library(rscamera STATIC
    src/c_bindings.cpp
)

target_link_libraries(rscamera libcamera)
target_link_libraries(rscamera libcamera_base)

include(GNUInstallDirs)
install(TARGETS rscamera
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
