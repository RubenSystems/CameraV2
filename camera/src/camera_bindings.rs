use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};

#[repr(C)]
pub struct CameraGetFrameResult {
    pub success: bool,
    pub size: u64,
}

extern "C" {


    pub fn camera_init() -> *mut std::ffi::c_void;
    pub fn camera_setup(camera: *mut std::ffi::c_void, height: u64, width: u64, fps: u64);
    pub fn camera_get_frame(
        camera: *mut std::ffi::c_void,
        buffer: *mut u8,
        max_size: u64,
    ) -> CameraGetFrameResult;
}

pub struct CameraCapture {
    camera: *mut std::ffi::c_void,
    buffer: *mut u8, 
    max_size: u64
}

impl CameraCapture {
	pub fn new(camera: *mut std::ffi::c_void, buffer: &mut Vec<u8>,  max_size: u64) -> Self {
		CameraCapture {
			camera, 
			buffer: buffer.as_mut_ptr(), 
			max_size
		}
	}
}

impl Future for CameraCapture {
    type Output = u64; // Replace with your actual result type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implement your asynchronous logic here
        let res = unsafe{camera_get_frame(self.camera, self.buffer, self.max_size)};
        if res.success {
            Poll::Ready(res.size)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}