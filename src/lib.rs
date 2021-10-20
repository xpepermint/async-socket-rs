//! This crate implements a general-purpose asynchronous socket.
//! 
//! The `AsyncSocket` implements [AsyncRead], [AsyncWrite], [Stream] and [Clone]
//! traits and thus mimic the functionality and the behaviour of the [TcpStream]
//! or [UnixStream]. These propertis makes it a perfect tool for testing network
//! activities and events.
//! 
//! ### Usage
//! 
//! **Example:**
//! 
//! ```rust
//! use async_socket::Socket;
//! use async_std::task::spawn;
//! use futures::io::AsyncWriteExt;
//! use futures::stream::StreamExt;
//! 
//! async fn example() {
//!     let mut stream = Socket::default();
//!     let mut writer = stream.clone();
//! 
//!     spawn(async move {
//!         writer.write(b"Hello").await.unwrap();
//!     });
//! 
//!     while let Some(bytes) = stream.next().await {
//!         // ...
//!     }
//! }
//! ```
//! 
//! [AsyncRead]: https://docs.rs/futures/latest/futures/prelude/trait.AsyncRead.html
//! [AsyncWrite]: https://docs.rs/futures/latest/futures/prelude/trait.AsyncWrite.html
//! [Stream]: https://docs.rs/futures/latest/futures/prelude/trait.Stream.html
//! [Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [TcpStream]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html 
//! [UnixStream]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html
//! 
//! [![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-protos)
//! [![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/protos)
mod socket;
mod state;

pub use socket::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task::spawn;
    use futures::io::AsyncWriteExt;
    use futures::stream::StreamExt;

    #[async_std::test]
    async fn performs_common_tasks() {

        let mut stream = Socket::default();
        let mut writer = stream.clone();

        spawn(async move {
            writer.write(b"Hello").await.unwrap();
            writer.write(b" ").await.unwrap();
            writer.write(b"World!").await.unwrap();
        });

        

        let mut data = vec![];
        while let Some(mut chunk) = stream.next().await {
            data.append(&mut chunk);

            if data.len() >= 12 {
                break;
            }
        }

        assert_eq!(data, b"Hello World!");
    }
}