
#include <camera.hpp>

using namespace rscamera;

rscamera::Camera::~Camera() {
	camera_->stop();
	camera_->release();
	camera_.reset();
	manager_->stop();
}

void rscamera::Camera::setup( uint32_t width, uint32_t height, uint32_t fps ) {
	alloc();
	setup_camera();
	setup_streams( width, height );
	setup_controls();
	set_framerate( fps );
	create_buffer_allocator();
	configure_requests_for_stream( NORMAL );
}

void rscamera::Camera::start() {
	camera_->start( &controls_ );
	for ( std::unique_ptr<libcamera::Request> & request : requests_ )
		camera_->queueRequest( request.get() );
}

void rscamera::Camera::next_frame( std::unique_ptr<CompletedRequest> req ) {
	libcamera::Request * request = req->request;
	libcamera::Request::BufferMap buffers = std::move( req->buffers );

	for ( auto const & buffer : buffers ) {
		if ( request->addBuffer( buffer.first, buffer.second ) < 0 )
			throw std::runtime_error( "[BUFFER] - could not readd buffer to req" );
	}

	request->reuse( libcamera::Request::ReuseBuffers );
	camera_->queueRequest( request );
}

GetLatestFrameRes rscamera::Camera::get_latest_frame( uint8_t * buffer, size_t max_copy_size ) {
	std::unique_ptr<CompletedRequest> req = nullptr;
	GetLatestFrameRes ret;
	if ( !pipe_->pop( req ) ) {
		ret.indicator = GET_LATEST_FRAME_FAIL;
		ret.size = 0;
		return ret;
	}
	libcamera::FrameBuffer * raw_buffer = req->buffers.find( streams_[NORMAL] )->second;
	std::vector<libcamera::Span<uint8_t> > frame_buffer = mapped_buffers_[raw_buffer];
	size_t max_size_to_copy = std::min( max_copy_size, frame_buffer.size() );
	// memmove(buffer, frame_buffer[0].data(), max_size_to_copy * sizeof(uint8_t));
	next_frame( std::move( req ) );
	ret.indicator = GET_LATEST_FRAME_SUCCESS;
	ret.size = max_size_to_copy;
	return ret;
}

void rscamera::Camera::alloc() {
	manager_ = std::unique_ptr<CameraManager>( new libcamera::CameraManager() );
	manager_->start();
}

void rscamera::Camera::setup_camera() {
	camera_ = manager_->cameras().at( 0 );
	camera_->acquire();
	camera_->requestCompleted.connect( this, &rscamera::Camera::request_complete );
}

void rscamera::Camera::setup_streams( uint32_t width, uint32_t height ) {
	config_ = camera_->generateConfiguration( { libcamera::StreamRole::VideoRecording } );

	libcamera::StreamConfiguration & video_stream_config = config_->at( 0 );

	video_stream_config.size = { width, height };
	video_stream_config.bufferCount = 1;
	video_stream_config.pixelFormat = libcamera::formats::YUV420;
	video_stream_config.colorSpace = libcamera::ColorSpace::Sycc;

	config_->validate();
	camera_->configure( config_.get() );

	streams_[StreamType::NORMAL] = config_->at( 0 ).stream();
}

void rscamera::Camera::setup_controls() {
	controls_.set( controls::AwbMode, controls::AwbAuto );
	controls_.set( controls::AeMeteringMode, controls::MeteringMatrix );
	controls_.set( controls::AfMode, controls::AfModeEnum::AfModeContinuous );
}

void rscamera::Camera::set_framerate( uint64_t fps ) {
	int64_t frame_time = 1000000 / fps;
	controls_.set( libcamera::controls::FrameDurationLimits,
		       libcamera::Span<const int64_t, 2>( { frame_time, frame_time } ) );
}

void rscamera::Camera::create_buffer_allocator() {
	allocator_ = std::unique_ptr<FrameBufferAllocator>( new FrameBufferAllocator( camera_ ) );

	for ( libcamera::StreamConfiguration & cfg : *config_ ) {
		if ( allocator_->allocate( cfg.stream() ) < 0 ) {
			throw std::runtime_error( "[CAMERA] - can't allocate buffers" );
		}

		for ( const std::unique_ptr<libcamera::FrameBuffer> & buffer :
		      allocator_->buffers( cfg.stream() ) ) {
			size_t buffer_size = 0;
			for ( unsigned i = 0; i < buffer->planes().size(); i++ ) {
				const libcamera::FrameBuffer::Plane & plane = buffer->planes()[i];
				buffer_size += plane.length;
				if ( i == buffer->planes().size() - 1 ||
				     plane.fd.get() != buffer->planes()[i + 1].fd.get() ) {
					void * memory = mmap( NULL, buffer_size,
							      PROT_READ | PROT_WRITE, MAP_SHARED,
							      plane.fd.get(), 0 );

					mapped_buffers_[buffer.get()].push_back(
						libcamera::Span<uint8_t>(
							static_cast<uint8_t *>( memory ),
							buffer_size ) );
					buffer_size = 0;
				}
			}
		}
	}
}

void rscamera::Camera::configure_requests_for_stream( StreamType stream_name ) {
	libcamera::Stream * stream_ = streams_[stream_name];
	const std::vector<std::unique_ptr<libcamera::FrameBuffer> > & buffers =
		allocator_->buffers( stream_ );

	for ( unsigned int i = 0; i < buffers.size(); ++i ) {
		std::unique_ptr<libcamera::Request> request = camera_->createRequest();
		if ( !request )
			throw std::runtime_error( "[CAMERA] - cannot make request" );

		libcamera::FrameBuffer * buffer = buffers[i].get();
		int ret = request->addBuffer( stream_, buffer );
		if ( ret < 0 )
			throw std::runtime_error( "[CAMERA] - cannot set buffer" );

		requests_.push_back( std::move( request ) );
	}
}

void rscamera::Camera::request_complete( libcamera::Request * request ) {
	if ( request->status() == libcamera::Request::RequestCancelled )
		return;

	pipe_->push( std::make_unique<CompletedRequest>( request ) );
}
