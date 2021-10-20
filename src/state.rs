use std::task::Waker;

/// A central shared state of the socket where all socket data are stored.
pub struct State {
    /// A buffer holding buffered bytes of the socket.
    pub buf: Vec<u8>,

    /// The Waker reference of the root socket. The socket stores this reference
    /// to enable async notifications across cloned socket instances.
    pub waker: Option<Waker>,

    /// The maximum size of a chunk returned by a stream.
    pub chunk_size: usize,
}

impl State {
    /// Returns a new instance with a specific chunk size.
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        let mut this = Self::default();
        this.chunk_size = chunk_size;
        this
    }

    /// Triggers the wake event if the waker has been set by the socket.
    pub fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            buf: vec![],
            waker: None,
            chunk_size: 10,
        }
    }
}
