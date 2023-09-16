use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[repr(C)]
pub struct CameraGetFrameResult {
    pub success: bool,
    pub size: u64,
}

extern "C" {

    pub fn camera_init() -> *mut std::ffi::c_void;
    pub fn camera_setup(camera: *mut std::ffi::c_void, width: u64, height: u64, fps: u64);
    pub fn camera_get_stride(camera: *mut std::ffi::c_void) -> u32;
    pub fn camera_get_frame(
        camera: *mut std::ffi::c_void,
        frame_id: u8,
        buffer: *mut u8,
    ) -> CameraGetFrameResult;
}

pub struct CameraCapture {
    camera: *mut std::ffi::c_void,
    frame_id: u8,
    buffer: *mut u8,
}

impl CameraCapture {
    pub fn new(
        camera: *mut std::ffi::c_void,
        frame_id: u8,
        buffer: &mut Vec<u8>,
    ) -> Self {
        CameraCapture {
            camera,
            frame_id,
            buffer: buffer.as_mut_ptr(),
        }
    }
}

impl Future for CameraCapture {
    type Output = u64; // Replace with your actual result type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implement your asynchronous logic here
        let res =
            unsafe { camera_get_frame(self.camera, self.frame_id, self.buffer) };
        if res.success {
            Poll::Ready(res.size)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
