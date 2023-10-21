
#include <camera.hpp>

using namespace rscamera;

rscamera::Camera::Camera()  {
	hpipe_ = std::make_unique<pipeline_t>();
	lpipe_ = std::make_unique<pipeline_t>();
}

rscamera::Camera::~Camera() {
	camera_->stop();
	camera_->release();
	camera_.reset();
	manager_->stop();
}

void rscamera::Camera::setup( CameraDimensionConfig config ) {
	alloc();
	setup_camera();
	setup_streams( config );
	setup_controls();
	set_framerate( config.fps );
	create_buffer_allocator();
	configure_requests_for_stream( NORMAL );
	configure_requests_for_stream( LORES );
}

void rscamera::Camera::start() {
	camera_->start( &controls_ );
	for ( std::unique_ptr<libcamera::Request> & request : lrequests_ )
		camera_->queueRequest( request.get() );
	for ( std::unique_ptr<libcamera::Request> & request : hrequests_ )
		camera_->queueRequest( request.get() );
}

void rscamera::Camera::next_frame( std::unique_ptr<CompletedRequest> req ) {
	libcamera::Request * request = req->request;
	libcamera::Request::BufferMap buffers = req->buffers;

	for ( auto const & buffer : buffers ) {
		if ( request->addBuffer( buffer.first, buffer.second ) < 0 )
			throw std::runtime_error( "[BUFFER] - could not readd buffer to req" );
	}

	request->reuse( libcamera::Request::ReuseBuffers );
	camera_->queueRequest( request );
}

GetLatestFrameRes rscamera::Camera::get_latest_l_frame() {
	return get_frame_for_stream(lpipe_, StreamType::LORES );
}

GetLatestFrameRes rscamera::Camera::get_latest_h_frame() {
	return get_frame_for_stream(hpipe_, StreamType::NORMAL );
	
}

GetLatestFrameRes rscamera::Camera::get_frame_for_stream(std::unique_ptr<pipeline_t> & cpipe, StreamType type ) {
	
	std::unique_ptr<CompletedRequest> req = nullptr;
	GetLatestFrameRes ret;
	if ( !cpipe->pop( req ) ) {
		ret.success = false;
		return ret;
	}
	libcamera::FrameBuffer * raw_buffer = req->buffers.find( streams_[type] )->second;
	libcamera::Span<uint8_t> & frame_buffer = mapped_buffers_.find( raw_buffer )->second[0];
	ret.success = true;
	ret.data = frame_buffer.data();
	ret.size = frame_buffer.size();
	ret.request = (uint64_t)req.release();

	return ret;
}

void rscamera::Camera::alloc() {
	manager_ = std::make_unique<CameraManager>();
	manager_->start();
}

void rscamera::Camera::setup_camera() {
	camera_ = manager_->cameras().at( 0 );
	camera_->acquire();
	camera_->requestCompleted.connect( this, &rscamera::Camera::request_complete );
}

void rscamera::Camera::setup_streams( CameraDimensionConfig config ) {
	{
		std::unique_ptr<libcamera::CameraConfiguration> configuration =
			camera_->generateConfiguration( { libcamera::StreamRole::VideoRecording,
							  libcamera::StreamRole::Viewfinder } );
		libcamera::StreamConfiguration * video_stream_config = &( configuration->at( 0 ) );
		video_stream_config->size.width = config.h_width;
		video_stream_config->size.height = config.h_height;
		video_stream_config->bufferCount = config.h_buffercount;
		video_stream_config->pixelFormat = libcamera::formats::YUV420;
		video_stream_config->colorSpace = libcamera::ColorSpace::Sycc;

		libcamera::StreamConfiguration * lores_stream_config = &( configuration->at( 1 ) );
		lores_stream_config->size.width = config.l_width;
		lores_stream_config->size.height = config.l_height;
		lores_stream_config->bufferCount = config.l_buffercount;
		lores_stream_config->pixelFormat = libcamera::formats::YUV420;

		configuration->validate();
		camera_->configure( configuration.get() );
		streams_[StreamType::NORMAL] = video_stream_config->stream();
		streams_[StreamType::LORES] = lores_stream_config->stream();

		std::cout << "H and L streams setup" 
		<< "\nHHeight: " << video_stream_config->size.height
		<< "\nHWidth: " << video_stream_config->size.width
		<< "\nLHeight: " << lores_stream_config->size.width
		<< "\nLHeight: " << lores_stream_config->size.width
		<< "\nHPitch: " << video_stream_config->stride
		<< "\nLPitch: " << lores_stream_config->stride << std::endl;

		config_ = std::move( configuration );
	}
}

void rscamera::Camera::setup_controls() {
	controls_.set( controls::AwbMode, controls::AwbAuto );
	// controls_.set( controls::ExposureMode, controls::AwbAuto );
	controls_.set( controls::AeMeteringMode, controls::MeteringMatrix );
	controls_.set( controls::AfMode, controls::AfModeEnum::AfModeContinuous );
}

void rscamera::Camera::set_framerate( uint64_t fps ) {
	int64_t frame_time = 1000000 / fps;
	controls_.set( libcamera::controls::FrameDurationLimits,
		       libcamera::Span<const int64_t, 2>( { frame_time, frame_time } ) );
}

void rscamera::Camera::create_buffer_allocator() {
	allocator_ = std::make_unique<FrameBufferAllocator>( camera_ );

	for ( libcamera::StreamConfiguration & cfg : *config_ ) {
		if ( allocator_->allocate( cfg.stream() ) < 0 )
			throw std::runtime_error( "[CAMERA] - can't allocate buffers" );

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

	for ( unsigned int i = 0; i < buffers.size(); i++ ) {
		std::unique_ptr<libcamera::Request> request = camera_->createRequest();
		if ( !request )
			throw std::runtime_error( "[CAMERA] - cannot make request" );

		libcamera::FrameBuffer * buffer = buffers[i].get();
		int ret = request->addBuffer( stream_, buffer );
		if ( ret < 0 )
			throw std::runtime_error( "[CAMERA] - cannot set buffer" );
		
		switch (stream_name) {
			case StreamType::LORES: 
				lrequests_.push_back( std::move( request ) );
				break; 
			case StreamType::NORMAL:
				hrequests_.push_back( std::move( request ) );
				break;
		}
		
	}
}

void rscamera::Camera::request_complete( libcamera::Request * request ) {
	if ( request->status() == libcamera::Request::RequestCancelled )
		return;
	
	for (int i = 0; i < lrequests_.size(); i ++) {
		if (request == lrequests_[i].get()) {
			lpipe_->push( std::make_unique<CompletedRequest>( request ) );
			return;
		}
	}
	hpipe_->push( std::make_unique<CompletedRequest>( request ) );
}
