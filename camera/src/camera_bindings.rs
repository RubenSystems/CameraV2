use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type VoidPointer = u64;

#[repr(C)]
pub struct CameraGetFrameResult {
    success: bool,
    data: *mut u8,
    request: VoidPointer,
    size: usize,
}

pub struct FrameData {
    pub data: Vec<u8>,
    pub request: VoidPointer,
}

extern "C" {

    pub fn camera_init() -> VoidPointer;
    pub fn camera_setup(camera: VoidPointer, width: u64, height: u64, fps: u64);
    pub fn camera_get_stride(camera: VoidPointer) -> u32;
    pub fn camera_get_frame(camera: VoidPointer) -> CameraGetFrameResult;
    pub fn camera_next_frame(camera: VoidPointer, frame: VoidPointer);
}

pub struct CameraCapture {
    camera: VoidPointer,
}

impl CameraCapture {
    pub fn new(camera: VoidPointer) -> Self {
        CameraCapture { camera }
    }
}

impl Future for CameraCapture {
    type Output = std::mem::ManuallyDrop<FrameData>; // Replace with your actual result type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implement your asynchronous logic here
        let res = unsafe { camera_get_frame(self.camera) };

        if res.success {
            let vector = unsafe { Vec::from_raw_parts(res.data, res.size, res.size) };
            Poll::Ready(std::mem::ManuallyDrop::new(FrameData {
                data: vector,
                request: res.request,
            }))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
