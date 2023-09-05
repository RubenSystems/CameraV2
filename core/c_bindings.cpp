#include <camera.hpp>

extern "C" {
struct camera_get_frame_res {
	bool success;
	uint64_t size; 
};

void * camera_init() {
	return (void *)(new rscamera::Camera());
}

void camera_setup( void * camera, uint64_t height, uint64_t width, uint64_t fps ) {
	((rscamera::Camera *)camera)->setup( height, width, fps );
	((rscamera::Camera *)camera)->start();
}

struct camera_get_frame_res camera_get_frame( void * camera, uint8_t * buffer,
					      uint64_t max_size ) {
	rscamera::GetLatestFrameRes res = ((rscamera::Camera *)camera)->get_latest_frame( buffer, max_size );

	struct camera_get_frame_res ret = {
		.success = res.indicator ==
			   rscamera::GetLatestFrameResultIndicator::GET_LATEST_FRAME_SUCCESS,
		.size = (uint64_t)res.size
	};
	return ret;
}
}