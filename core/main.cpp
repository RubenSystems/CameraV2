#include <camera.hpp>
#include <pipeline.hpp>
#include <iostream>
#include <chrono>
#include <thread>
#include <c_bindings.hpp>

using namespace libcamera;
using namespace std::chrono_literals;

int main() {
	using std::chrono::high_resolution_clock;
	using std::chrono::duration_cast;
	using std::chrono::duration;
	using std::chrono::milliseconds;

	rscamera::Camera cam = rscamera::Camera();

	cam.setup( 2328, 1748, 30 );
	cam.start();

	uint8_t * buffer = new uint8_t[2328 * 1748 * 4];
	auto start = high_resolution_clock::now();
	size_t count = 1;

	while ( 1 ) {
		rscamera::GetLatestFrameRes res = cam.get_latest_frame( buffer, 2328 * 1748 * 4 );
		bool success = res.indicator ==
			       rscamera::GetLatestFrameResultIndicator::GET_LATEST_FRAME_SUCCESS;
		
		
		// std::unique_ptr<Request> req = video_stream.pop();
		// std::cout<<video_stream.count() << std::endl;

		// cam.next_frame(std::move(req));
		if ( success ) {
			uint64_t size = res.size; 
			std::cout << res.size << std::endl;
			// auto current = high_resolution_clock::now();
			// std::cout
			// 	<< duration_cast<milliseconds>( current - start ).count() / count++
			// 	<< "\n";
		}
	}

	return 0;
}