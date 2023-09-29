#include <camera.hpp>

extern "C" {


uint64_t camera_init() {
	return (uint64_t)(void *)(new rscamera::Camera());
}

void camera_setup(uint64_t camera, uint64_t width, uint64_t height, uint64_t fps ) {
	((rscamera::Camera *)camera)->setup(width, height, fps );
	((rscamera::Camera *)camera)->start();
}

uint32_t camera_get_stride(uint64_t camera) {
	return ((rscamera::Camera *)camera)->get_stride();
}

struct rscamera::GetLatestFrameRes camera_get_frame(uint64_t camera) {
	return ((rscamera::Camera *)camera)->get_latest_frame( );
}

void camera_next_frame(uint64_t camera, uint64_t request) {
	((rscamera::Camera *)camera)->next_frame(
		std::unique_ptr<rscamera::CompletedRequest>((rscamera::CompletedRequest *)request)
	);

}
}