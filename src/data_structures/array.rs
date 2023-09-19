#[derive(Debug)]
pub struct RingBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    size: usize,
    head: usize,
    tail: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            buffer: vec![None; size],
            size,
            head: 0,
            tail: 0,
        }
    }
    pub fn is_full(&self) -> bool {
        self.head == self.tail && self.buffer[self.head].is_some()
    }
    pub fn is_empty(&self) -> bool {
        self.head == self.tail && self.buffer[self.head].is_none()
    }
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Ring buffer is full");
        }
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.size;
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let val = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % self.size;
        val
    }
}
