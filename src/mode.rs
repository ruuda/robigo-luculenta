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

use std::io::net::ip::{Port, SocketAddr};
use std::os;

pub enum AppMode {
    Master(Port),
    Slave(SocketAddr),
    Single
}

pub fn get_mode() -> AppMode {
    let args = os::args();
    let mut iter = args.iter();

    // First argument is the program name.
    iter.next();

    match iter.next().map(|x| x[]) {
        // If --master is specified, try po parse the port.
        Some("--master") => match iter.next() {
            Some(port_str) => match from_str(port_str[]) {
                Some(port) => AppMode::Master(port),
                None => panic!("invalid port")
            },
            None => panic!("no port specified")
        },

        // If --slave is specified, try to parse the master address.
        Some("--slave") => match iter.next() {
            Some(master_str) => match from_str(master_str[]) {
                Some(master) => AppMode::Slave(master),
                None => panic!("invalid master address")
            },
            None => panic!("no master address specified")
        },
        Some(param) => panic!("unrecognised parameter {}", param),
        None => AppMode::Single
    }
}
