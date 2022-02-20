//! An example of hooking up stdin/stdout to either a TCP or UDP stream.
//!
//! This example will connect to a socket address specified in the argument list
//! and then forward all data read on stdin to the server, printing out all data
//! received on stdout. An optional `--udp` argument can be passed to specify
//! that the connection should be made over UDP instead of TCP, translating each
//! line entered on stdin to a UDP packet to be sent to the remote address.
//!
//! Note that this is not currently optimized for performance, especially
//! around buffer management. Rather it's intended to show an example of
//! working with a client.
//!
//! This example can be quite useful when interacting with the other examples in
//! this repository! Many of them recommend running this as a simple "hook up
//! stdin/stdout to a server" to get up and running.

#![warn(rust_2018_idioms)]

use futures::StreamExt;
use tokio::io;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use std::env;
use std::error::Error;
use std::net::SocketAddr;

//
// todo x:
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Determine if we're going to run in TCP or UDP mode
    ///
    /// todo x: 命令行传参
    ///
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    ///
    /// todo x:
    ///
    let tcp = match args.iter().position(|a| a == "--udp") {
        Some(i) => {
            args.remove(i);
            false
        }
        None => true,
    };

    // Parse what address we're going to connect to
    let addr = args
        .first()
        .ok_or("this program requires at least one argument")?;
    let addr = addr.parse::<SocketAddr>()?;

    let stdin = FramedRead::new(io::stdin(), BytesCodec::new());
    let stdin = stdin.map(|i| i.map(|bytes| bytes.freeze()));
    let stdout = FramedWrite::new(io::stdout(), BytesCodec::new());

    ///
    /// todo x: connect
    ///
    if tcp {
        tcp::connect(&addr, stdin, stdout).await?; // todo x: TCP 方式, 后面实现
    } else {
        udp::connect(&addr, stdin, stdout).await?; // todo x: UDP 方式, 后面实现
    }

    Ok(())
}

///
/// todo x: 子包实现:
///
mod tcp {
    use bytes::Bytes;
    use futures::{future, Sink, SinkExt, Stream, StreamExt};
    use std::{error::Error, io, net::SocketAddr};
    use tokio::net::TcpStream;
    use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

    ///
    /// todo x:
    ///
    pub async fn connect(
        addr: &SocketAddr,
        mut stdin: impl Stream<Item = Result<Bytes, io::Error>> + Unpin,
        mut stdout: impl Sink<Bytes, Error = io::Error> + Unpin,
    ) -> Result<(), Box<dyn Error>> {
        ///
        /// todo x: TCP
        ///
        let mut stream = TcpStream::connect(addr).await?;

        ///
        /// todo x: reader/writer
        ///
        let (r, w) = stream.split();

        ///
        /// todo x: writer
        ///
        let mut sink = FramedWrite::new(w, BytesCodec::new());
        // filter map Result<BytesMut, Error> stream into just a Bytes stream to match stdout Sink
        // on the event of an Error, log the error and end the stream
        ///
        /// todo x: reader
        ///
        let mut stream = FramedRead::new(r, BytesCodec::new())
            .filter_map(|i| match i {
                //BytesMut into Bytes
                Ok(i) => future::ready(Some(i.freeze())),
                Err(e) => {
                    println!("failed to read from socket; error={}", e);
                    future::ready(None)
                }
            })
            .map(Ok);

        ///
        /// todo x:
        ///
        match future::join(sink.send_all(&mut stdin), stdout.send_all(&mut stream)).await {
            (Err(e), _) | (_, Err(e)) => Err(e.into()),
            _ => Ok(()),
        }
    }
}

mod udp {
    use bytes::Bytes;
    use futures::{Sink, SinkExt, Stream, StreamExt};
    use std::error::Error;
    use std::io;
    use std::net::SocketAddr;
    use tokio::net::UdpSocket;

    ///
    /// TODO X: UDP connect
    ///
    pub async fn connect(
        addr: &SocketAddr,
        stdin: impl Stream<Item = Result<Bytes, io::Error>> + Unpin,
        stdout: impl Sink<Bytes, Error = io::Error> + Unpin,
    ) -> Result<(), Box<dyn Error>> {
        // We'll bind our UDP socket to a local IP/port, but for now we
        // basically let the OS pick both of those.
        let bind_addr = if addr.ip().is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };

        let socket = UdpSocket::bind(&bind_addr).await?;
        socket.connect(addr).await?;

        ///
        /// todo x: send/recv
        ///
        tokio::try_join!(send(stdin, &socket), recv(stdout, &socket))?;

        Ok(())
    }

    ///
    /// todo x: send
    ///
    async fn send(
        mut stdin: impl Stream<Item = Result<Bytes, io::Error>> + Unpin,
        writer: &UdpSocket,
    ) -> Result<(), io::Error> {
        ///
        ///
        ///
        while let Some(item) = stdin.next().await {
            let buf = item?;

            ///
            /// todo x: send
            ///
            writer.send(&buf[..]).await?;
        }

        Ok(())
    }

    ///
    /// todo x: recv
    ///
    async fn recv(
        mut stdout: impl Sink<Bytes, Error = io::Error> + Unpin,
        reader: &UdpSocket,
    ) -> Result<(), io::Error> {
        loop {
            let mut buf = vec![0; 1024];
            let n = reader.recv(&mut buf[..]).await?;

            if n > 0 {
                stdout.send(Bytes::from(buf)).await?;
            }
        }
    }
}
