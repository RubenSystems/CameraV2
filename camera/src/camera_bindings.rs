use std::future::Future;
use std::pin::Pin;
use std::slice;
use std::task::{Context, Poll};

pub type VoidPointer = u64;

#[repr(C)]
pub struct CameraGetFrameResult {
    success: bool,
    data: *mut u8,
    request: VoidPointer,
    size: usize,
}

#[repr(C)]
pub struct CameraDimensionConfig {
    h_height: u32,
    h_width: u32,
    h_buffercount: u32,
    l_height: u32,
    l_width: u32,
    l_buffercount: u32,
    fps: u32,
}

impl CameraDimensionConfig {
    pub fn new(
        h_height: u64,
        h_width: u64,
        h_buffercount: u64,
        l_height: u64,
        l_width: u64,
        l_buffercount: u64,
        fps: u64,
    ) -> Self {
        return Self {
            h_height: h_height as u32,
            h_width: h_width as u32,
            h_buffercount: h_buffercount as u32,
            l_height: l_height as u32,
            l_width: l_width as u32,
            l_buffercount: l_buffercount as u32,
            fps: fps as u32,
        };
    }
}

pub struct FrameData {
    pub data: &'static [u8],
    pub request: VoidPointer,
}

extern "C" {

    pub fn camera_init() -> VoidPointer;
    pub fn camera_setup(camera: VoidPointer, config: CameraDimensionConfig);
    pub fn camera_get_h_frame(camera: VoidPointer) -> CameraGetFrameResult;
    pub fn camera_get_l_frame(camera: VoidPointer) -> CameraGetFrameResult;
    pub fn camera_next_frame(camera: VoidPointer, frame: VoidPointer);
}

pub struct HiResCameraCapture {
    camera: VoidPointer,
}

impl HiResCameraCapture {
    pub fn new(camera: VoidPointer) -> Self {
        Self { camera }
    }
}

impl Future for HiResCameraCapture {
    type Output = FrameData; // Replace with your actual result type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implement your asynchronous logic here
        let res = unsafe { camera_get_h_frame(self.camera) };

        if res.success {
            let image = unsafe { slice::from_raw_parts(res.data, res.size) };
            Poll::Ready(FrameData {
                data: image,
                request: res.request,
            })
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub struct LoResCameraCapture {
    camera: VoidPointer,
}

impl LoResCameraCapture {
    pub fn new(camera: VoidPointer) -> Self {
        Self { camera }
    }
}

impl Future for LoResCameraCapture {
    type Output = FrameData; // Replace with your actual result type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implement your asynchronous logic here
        let res = unsafe { camera_get_l_frame(self.camera) };

        if res.success {
            let image = unsafe { slice::from_raw_parts(res.data, res.size) };
            Poll::Ready(FrameData {
                data: image,
                request: res.request,
            })
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
