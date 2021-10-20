use futures::Future;
use futures::lock::Mutex;
use futures::prelude::{AsyncRead, AsyncWrite};
use futures::stream::Stream;
use futures::task::Context;
use std::clone::Clone;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use crate::State;

/// The asynchronous socket that mimics the network stream.
#[derive(Debug)]
pub struct Socket {
    /// A central socket state which is shared among all the cloned instances.
    pub state: Arc<Mutex<State>>,
}

impl Socket {
    /// Returns a new instance with a specific chunk size.
    pub fn with_chunk_size(csize: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::with_chunk_size(csize))),
        }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
        }
    }
}

impl Clone for Socket {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl AsyncRead for Socket {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
        match Pin::new(&mut self.state.lock()).poll(cx) {
            Poll::Ready(mut state) => {

                let dsize = state.buf.len();
                let bsize = buf.len();
                if dsize < bsize {
                    return Poll::Pending;
                }

                let data = state.buf.drain(0..bsize).as_slice().to_vec();
                buf[..bsize].copy_from_slice(&data);
                Poll::Ready(Ok(bsize))
            },
            Poll::Pending => {
                Poll::Pending
            },
        }
    }
}

impl AsyncWrite for Socket {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, data: &[u8]) -> Poll<std::io::Result<usize>> {
        match Pin::new(&mut self.state.lock()).poll(cx) {
            Poll::Ready(mut state) => {
                state.buf.append(&mut data.to_vec());
                state.wake();
                Poll::Ready(Ok(state.buf.len()))
            },
            Poll::Pending => {
                Poll::Pending
            },
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl Stream for Socket {
    type Item = Vec<u8>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.state.lock()).poll(cx) {
            Poll::Ready(mut state) => {
                state.waker = Some(cx.waker().clone());
                
                let max = std::cmp::min(state.chunk_size, state.buf.len());
                let data = state.buf.drain(0..max).as_slice().to_vec();
                if data.is_empty() {
                    Poll::Pending
                } else {
                    Poll::Ready(Some(data))
                }
            },
            _ => {
                Poll::Pending
            },
        }
    }
}
