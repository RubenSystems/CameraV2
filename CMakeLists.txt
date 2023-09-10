cmake_minimum_required(VERSION 3.12)
project (Camera)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_BUILD_TYPE Release)


include_directories(core/include)
include_directories(/usr/include/libcamera)

add_library(libcamera SHARED IMPORTED)
set_property(TARGET libcamera PROPERTY IMPORTED_LOCATION "/lib/aarch64-linux-gnu/libcamera.so")

add_library(libcamera_base SHARED IMPORTED)
set_property(TARGET libcamera_base PROPERTY IMPORTED_LOCATION "/lib/aarch64-linux-gnu/libcamera-base.so")

add_library(rscamera STATIC
    core/c_bindings.cpp core/camera.cpp
)

target_compile_options(rscamera PRIVATE -O3)


target_link_libraries(rscamera libcamera)
target_link_libraries(rscamera libcamera_base)


set(INSTALL_DIR "/home/RubenSystems/CameraV2/camera/lib")
install(TARGETS rscamera DESTINATION ${INSTALL_DIR})
