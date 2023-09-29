use std::collections::VecDeque;

pub struct BufferQueue {
    buffers: VecDeque<Vec<u8>>,
    buffer_size: usize,
}

impl BufferQueue {
    pub fn new(buffer_size: usize) -> Self {
        BufferQueue {
            buffers: VecDeque::new(),
            buffer_size,
        }
    }

    pub fn new_buffer(&mut self) -> Vec<u8> {
        if let Some(buffer) = self.buffers.pop_front() {
            buffer
        } else {
            println!("ALLOC");
            vec![0_u8; self.buffer_size]
        }
    }

    pub fn return_buffer(&mut self, buffer: Vec<u8>) {
        self.buffers.push_back(buffer);
    }
}
