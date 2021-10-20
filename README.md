# async-socket

This crate implements a general-purpose asynchronous socket.

The `Socket` implements [AsyncRead], [AsyncWrite], [Stream] and [Clone]
traits and thus mimics the functionality and the behaviour of the
[TcpStream] and [UnixStream] objects. These propertis makes it a perfect
tool for testing network activities and events.

[![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/async-socket)
[![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/async-socket-rs)

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

    spawn(async move {
        writer.write(b"Hello").await.unwrap();
    });

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

License: MIT
