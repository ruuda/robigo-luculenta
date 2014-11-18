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

use std::io::{BufferedWriter, IoResult};
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
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
