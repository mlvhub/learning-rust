extern crate tokio;
#[macro_use]
extern crate futures;
extern crate bytes;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use futures::sync::mpsc;
use futures::future::{self, Either};
use bytes::{BytesMut, Bytes, BufMut};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<Bytes>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<Bytes>;

struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}
impl Shared {
    /// Create a new, empty, instance of `Shared`.
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }
}

struct Peer {
    /// Name of the peer. This is the first line received from the client.
    name: BytesMut,

    /// The TCP socket wrapped with the `Lines` codec.
    lines: Lines,

    /// Handle to the shared chat state.
    state: Arc<Mutex<Shared>>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,

    /// Client socket address.
    ///
    /// The socket address is used as the key in the `peers` HashMap. The
    /// address is saved so that the `Peer` drop implementation can clean up its
    /// entry.
    addr: SocketAddr,
}
impl Peer {
    fn new(name: BytesMut,
           state: Arc<Mutex<Shared>>,
           lines: Lines) -> Peer
    {
        // Get the client socket address
        let addr = lines.socket.peer_addr().unwrap();

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().unwrap()
            .peers.insert(addr, tx);

        Peer {
            name,
            lines,
            state,
            rx,
            addr,
        }
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.state.lock().unwrap().peers
            .remove(&self.addr);
    }
}
impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        // Receive all messages from peers.
        loop {
            // Polling an `UnboundedReceiver` cannot fail, so `unwrap`
            // here is safe.
            match self.rx.poll().unwrap() {
                Async::Ready(Some(v)) => {
                    // Buffer the line. Once all lines are buffered,
                    // they will be flushed to the socket (right
                    // below).
                    self.lines.buffer(&v);
                }
                _ => break,
            }
        }

        // Flush the write buffer to the socket
        let _ = self.lines.poll_flush()?;

        // Read new lines from the socket
        while let Async::Ready(line) = self.lines.poll()? {
            println!("Received line ({:?}) : {:?}", self.name, line);

            if let Some(message) = line {
                // Append the peer's name to the front of the line:
                let mut line = self.name.clone();
                line.put(": ");
                line.put(&message);
                line.put("\r\n");

                // We're using `Bytes`, which allows zero-copy clones
                // (by storing the data in an Arc internally).
                //
                // However, before cloning, we must freeze the data.
                // This converts it from mutable -> immutable,
                // allowing zero copy cloning.
                let line = line.freeze();

                // Now, send the line to all other peers
                for (addr, tx) in &self.state.lock().unwrap().peers {
                    // Don't send the message to ourselves
                    if *addr != self.addr {
                        // The send only fails if the rx half has been
                        // dropped, however this is impossible as the
                        // `tx` half will be removed from the map
                        // before the `rx` is dropped.
                        tx.unbounded_send(line.clone()).unwrap();
                    }
                }
            } else {
                // EOF was reached. The remote client has disconnected.
                // There is nothing more to do.
                return Ok(Async::Ready(()));
            }
        }

        // As always, it is important to not just return `NotReady`
        // without ensuring an inner future also returned `NotReady`.
        //
        // We know we got a `NotReady` from either `self.rx` or
        // `self.lines`, so the contract is respected.
        Ok(Async::NotReady)
    }
}

fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
    // Wrap the socket with the `Lines` codec that we wrote above.
    let lines = Lines::new(socket);

    // The first line is treated as the client's name. The client
    // is not added to the set of connected peers until this line
    // is received.
    //
    // We use the `into_future` combinator to extract the first
    // item from the lines stream. `into_future` takes a `Stream`
    // and converts it to a future of `(first, rest)` where `rest`
    // is the original stream instance.
    let connection = lines.into_future()
    .map_err(|(e, _)| e)
    .and_then(|(name, lines)| {
        // If `name` is `None`, then the client disconnected without
        // actually sending a line of data.
        //
        // Since the connection is closed, there is no further work
        // that we need to do. So, we just terminate processing by
        // returning `future::ok()`.
        //
        // The problem is that only a single future type can be
        // returned from a combinator closure, but we want to
        // return both `future::ok()` and `Peer` (below).
        //
        // This is a common problem, so the `futures` crate solves
        // this by providing the `Either` helper enum that allows
        // creating a single return type that covers two concrete
        // future types.
        let name = match name {
            Some(name) => name,
            None => {
                // The remote client closed the connection without
                // sending any data.
                return Either::A(future::ok(()));
            }
        };

        println!("`{:?}` is joining the chat", name);

        // Create the peer.
        //
        // This is also a future that processes the connection, only
        // completing when the socket closes.
        let peer = Peer::new(
            name,
            state,
            lines);

        // Wrap `peer` with `Either::B` to make the return type fit.
        Either::B(peer)
    })
    // Task futures have an error of type `()`, this ensures we handle
    // the error. We do this by printing the error to STDOUT.
    .map_err(|e| {
        println!("connection error = {:?}", e);
    });
}

struct Lines {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl Lines {
    /// Create a new `Lines` codec backed by the socket
    fn new(socket: TcpStream) -> Self {
        Lines {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }
}
impl Stream for Lines {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        // First, read any new data that might have been received
        // off the socket
        //
        // We track if the socket is closed here and will be used
        // to inform the return value below.
        let sock_closed = self.fill_read_buf()?.is_ready();

        // Now, try finding lines
        let pos = self.rd.windows(2)
            .position(|bytes| bytes == b"\r\n");

        if let Some(pos) = pos {
            // Remove the line from the read buffer and set it
            // to `line`.
            let mut line = self.rd.split_to(pos + 2);

            // Drop the trailing \r\n
            line.split_off(pos);

            // Return the line
            return Ok(Async::Ready(Some(line)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl Lines {
    fn fill_read_buf(&mut self) -> Result<Async<()>, io::Error> {
        loop {
            // Ensure the read buffer has capacity.
            //
            // This might result in an internal allocation.
            self.rd.reserve(1024);

            // Read data into the buffer.
            //
            // The `read_buf` fn is provided by `AsyncRead`.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}
impl Lines {
    fn buffer(&mut self, line: &[u8]) {
        // Push the line onto the end of the write buffer.
        //
        // The `put` function is from the `BufMut` trait.
        self.wr.put(line);
    }

    fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to read some bytes from the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer.
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }
}

fn main() {
    let state = Arc::new(Mutex::new(Shared::new()));

    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket, state.clone());
        Ok(())
    })
    .map_err(|err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime (reactor, threadpool, etc...)
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);
}
