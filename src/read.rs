// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2015 Ruud van Asseldonk
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

use std::io::{Read, Result};

/// Read into the buffer until it is full, regardless of how many calls it takes.
pub fn read_into<R: Read>(reader: &mut R, buf: &mut [u8]) -> Result<()> {
    let mut n = 0;
    loop {
        let progress = try!(reader.read(&mut buf[n ..]));
        if progress > 0 {
            n += progress;
        } else {
            break;
        }
    }

    Ok(())
}
