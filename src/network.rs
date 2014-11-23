// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::io::{Acceptor, BufferedReader, BufferedWriter, IoResult, Listener, Reader};
use std::io::net::ip::{Port, SocketAddr};
use std::io::net::tcp::{TcpListener, TcpStream};
use vector3::Vector3;

/// Attempts to connect to the master instance, and sends the buffer.
pub fn send(master_addr: SocketAddr, tristimuli: &[Vector3]) -> IoResult<()> {
    let tcp_stream = try!(TcpStream::connect(master_addr));

    // Buffer writes, we would not want to issue a syscall for every pixel.
    let mut writer = BufferedWriter::new(tcp_stream);

    for tri in tristimuli.iter() {
        try!(writer.write_le_f32(tri.x));
        try!(writer.write_le_f32(tri.y));
        try!(writer.write_le_f32(tri.z));
    }

    Ok(())
}

/// Listens for incoming connections on the specified port, and calls the
/// handler when data has been received.
pub fn listen(port: Port) -> Receiver<Vec<Vector3>> {
    let listener = TcpListener::bind(("localhost", port));
    let mut acceptor = listener.listen();

    // Allow multiple images to be queued, but block after the fourth. (In that
    // case, a task must complete and collect the images first. Because send
    // will block, this will also block accepting new incoming connections
    // until the queue is emptied.)
    let (tx, rx) = sync_channel(4);

    // Accept incoming connections from another thread.
    spawn(proc() {
        for incoming in acceptor.incoming() {
            match incoming {
                Err(_) => println!("bad incoming connection"),
                Ok(stream) => handle_incoming(stream, &tx)
            }
        }
        // Currently, listening cannot be stopped, except by killing the program.
    });

    rx
}

/// Reads a `Vector3` from a stream.
fn read_vector3(stream: &mut Reader) -> IoResult<Vector3> {
    let x = try!(stream.read_le_f32());
    let y = try!(stream.read_le_f32());
    let z = try!(stream.read_le_f32());
    Ok(Vector3::new(x, y, z))
}

/// Handles an incoming connection by reading the image and sending it through
/// the sending part of a channel.
fn handle_incoming(stream: TcpStream, tx: &SyncSender<Vec<Vector3>>) {
    // Buffer reads, we would not want to issue a syscall for every pixel.
    let mut reader = BufferedReader::new(stream);
    let mut image = Vec::new();

    loop {
        match read_vector3(&mut reader) {
            Ok(pixel) => image.push(pixel),
            Err(_) => break
        }
    }

    tx.send(image);
}
