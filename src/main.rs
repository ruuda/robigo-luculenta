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

#![feature(slicing_syntax)]

extern crate time;
extern crate image;

use app::App;
use mode::{AppMode, get_mode};

mod app;
mod camera;
mod cie1931;
mod constants;
mod gather_unit;
mod geometry;
mod intersection;
mod material;
mod mode;
mod monte_carlo;
mod network;
mod object;
mod plot_unit;
mod pop_iter;
mod quaternion;
mod ray;
mod scene;
mod srgb;
mod task_scheduler;
mod tonemap_unit;
mod trace_unit;
mod vector3;

fn main() {
    // Determine how the application should behave with respect to networking.
    let mode = get_mode();
    match mode {
        AppMode::Single => println!("running a single local instance"),
        AppMode::Slave(master) => println!("running in slave mode, master is at {}", master),
        AppMode::Master(port) => println!("running in master mode, listening at port {}", port)
    }

    // Start up the path tracer. It begins rendering immediately.
    let width = 1280u;
    let height = 720u;
    let app = App::new(mode, width, height);
    let images = app.images;

    println!("press ctrl+c to stop rendering");

    // Then wait for news from other tasks: when an image has been rendered,
    // write it out. Loop forever; the application must be stopped by
    // terminating it.
    loop {
        let img = images.recv();

        // Write the image to output.png.
        match image::save_buffer(&Path::new("output.png"), img[],
                                 width as u32, height as u32, image::RGB(8)) {
            Ok(_) => println!("wrote image to output.png"),
            Err(reason) => println!("failed to write output png: {}", reason)
        }
    }
}
