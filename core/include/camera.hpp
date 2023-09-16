#pragma once

#include "pipeline.hpp"
#include "completed_request.hpp"
#include <iostream>
#include <memory>
#include <sys/mman.h>

#include <libcamera/camera.h>
#include <libcamera/camera_manager.h>
#include <libcamera/control_ids.h>
#include <libcamera/controls.h>
#include <libcamera/formats.h>
#include <libcamera/framebuffer_allocator.h>
#include <libcamera/framebuffer_allocator.h>
#include <libcamera/stream.h>
#include <string.h>

using namespace libcamera;

namespace rscamera {

enum StreamType { NORMAL, LORES };

enum GetLatestFrameResultIndicator { GET_LATEST_FRAME_SUCCESS = 1, GET_LATEST_FRAME_FAIL = 0 };

struct GetLatestFrameRes {
	GetLatestFrameResultIndicator indicator;
	size_t size;
};

class Camera {
    public:
	typedef rscamera::Pipeline<std::unique_ptr<CompletedRequest> > pipeline_t;

	Camera()
		: pipe_( std::make_unique<pipeline_t>() ) {
	}

	~Camera();

	void setup( uint32_t width, uint32_t height, uint32_t fps );

	void start();

	void next_frame( std::unique_ptr<CompletedRequest> req );

	uint32_t get_stride();

	GetLatestFrameRes get_latest_frame(uint8_t packet_index, uint8_t * buffer );

    private:
	void alloc();

	void setup_camera();

	void setup_streams( uint32_t width, uint32_t height );

	void setup_controls();

	void set_framerate( uint64_t fps );

	void create_buffer_allocator();

	void configure_requests_for_stream( StreamType stream_name );

	void request_complete( libcamera::Request * request );

    private:
	std::unique_ptr<libcamera::CameraManager> manager_;
	std::shared_ptr<libcamera::Camera> camera_ = nullptr;
	libcamera::ControlList controls_;
	std::unique_ptr<libcamera::CameraConfiguration> config_;
	std::unique_ptr<libcamera::FrameBufferAllocator> allocator_;
	std::vector<std::unique_ptr<libcamera::Request> > requests_;
	std::map<StreamType, libcamera::Stream *> streams_;
	std::unordered_map<libcamera::FrameBuffer *, std::vector<libcamera::Span<uint8_t> > >
		mapped_buffers_;
	std::unique_ptr<pipeline_t> pipe_;
	uint32_t stride_;
};
}