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

extern "C" {
	struct GetLatestFrameRes {
		bool success;
		uint8_t * data;
		uint64_t request;
		size_t size;
	};

	struct CameraDimensionConfig {
		uint32_t h_height, h_width, h_buffercount; 
		uint32_t l_height, l_width, l_buffercount; 
		uint32_t fps;
	};
}

class Camera {
    public:
	typedef rscamera::Pipeline<std::unique_ptr<CompletedRequest> > pipeline_t;

	Camera();

	~Camera();

	void setup( CameraDimensionConfig );

	void start();

	void next_frame( std::unique_ptr<CompletedRequest> req );

	GetLatestFrameRes get_latest_h_frame();

	GetLatestFrameRes get_latest_l_frame();

    private:
	
	GetLatestFrameRes get_frame_for_stream(std::unique_ptr<pipeline_t> & cpipe, StreamType type );

	void alloc();

	void setup_camera();

	void setup_streams( CameraDimensionConfig config );

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
	
	std::unordered_map<StreamType, libcamera::Stream *> streams_;
	std::unordered_map<libcamera::FrameBuffer *, std::vector<libcamera::Span<uint8_t> > >
		mapped_buffers_;


	std::vector<std::unique_ptr<libcamera::Request> > lrequests_;
	std::vector<std::unique_ptr<libcamera::Request> > hrequests_;
	std::unique_ptr<pipeline_t> hpipe_;
	std::unique_ptr<pipeline_t> lpipe_;
};
}