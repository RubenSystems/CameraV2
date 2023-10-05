use std::cell::RefCell;
use std::collections::VecDeque;
pub struct BufferPool {
    buffer_size: usize,
    pool: RefCell<VecDeque<Vec<u8>>>,
}

impl BufferPool {
    pub fn new(buffer_size: usize, pool_size: usize) -> Self {
        let mut pool = VecDeque::with_capacity(pool_size);
        for _ in 0..pool_size {
            pool.push_back(vec![0; buffer_size]);
        }

        BufferPool {
            buffer_size,
            pool: RefCell::new(pool),
        }
    }

    pub fn alloc(&self) -> Option<Vec<u8>> {
        self.pool.borrow_mut().pop_front()
    }

    pub fn free(&self, buffer: Vec<u8>) {
        if buffer.len() != self.buffer_size {
            panic!("Invalid buffer size");
        }
        self.pool.borrow_mut().push_back(buffer);
    }
}
