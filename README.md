# async-socket

This crate implements a general purpose asynchronous socket.

The `AsyncSocket` implements [AsyncRead], [AsyncWrite], [Stream] and [Clone]
traits and thus mimic the functionality and the behaviour of the [TcpStream]
or [UnixStream]. These propertis makes it a perfect tool for testing network
activities and events.

#### Usage

**Example:**

```rust
use async_socket::Socket;
use async_std::task::spawn;
use futures::io::AsyncWriteExt;
use futures::stream::StreamExt;

async fn example() {
    let mut stream = Socket::default();
    let mut writer = stream.clone();

    // writing
    spawn(async move {
        writer.write(b"Hello").await.unwrap();
    });

    // reading
    while let Some(bytes) = stream.next().await {
        // ...
    }
}
```

[AsyncRead]: https://docs.rs/futures/latest/futures/prelude/trait.AsyncRead.html
[AsyncWrite]: https://docs.rs/futures/latest/futures/prelude/trait.AsyncWrite.html
[Stream]: https://docs.rs/futures/latest/futures/prelude/trait.Stream.html
[Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[TcpStream]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html
[UnixStream]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html

[![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-protos)
[![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/protos)

License: MIT
