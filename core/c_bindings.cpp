#include <camera.hpp>

extern "C" {


uint64_t camera_init() {
	return (uint64_t)(void *)(new rscamera::Camera());
}

void camera_setup(uint64_t camera, rscamera::CameraDimensionConfig config ) {
	((rscamera::Camera *)camera)->setup(config);
	((rscamera::Camera *)camera)->start();
}

struct rscamera::GetLatestFrameRes camera_get_h_frame(uint64_t camera) {
	return ((rscamera::Camera *)camera)->get_latest_h_frame( );
}

struct rscamera::GetLatestFrameRes camera_get_l_frame(uint64_t camera) {
	return ((rscamera::Camera *)camera)->get_latest_l_frame( );
}

void camera_next_frame(uint64_t camera, uint64_t request) {
	((rscamera::Camera *)camera)->next_frame(
		std::unique_ptr<rscamera::CompletedRequest>((rscamera::CompletedRequest *)request)
	);

}
}