#pragma once
#include "camera.hpp"
#include <stdint.h>

extern "C" {

	struct camera_get_frame_res {
		bool success;
		uint64_t size; 
	};

	void * camera_init();

	void camera_setup(void * camera, uint64_t height, uint64_t width, uint64_t fps);

	struct camera_get_frame_res camera_get_frame(void * camera, uint8_t * buffer, uint64_t max_size);

}