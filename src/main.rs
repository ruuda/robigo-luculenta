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

extern crate time;
extern crate lodepng;

use std::comm::channel;
use std::io::stdin;
use app::App;

mod app;
mod camera;
mod cie1931;
mod constants;
mod gather_unit;
mod geometry;
mod intersection;
mod material;
mod monte_carlo;
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
    // Start up the path tracer. It begins rendering immediately.
    let width = 1280u;
    let height = 720u;
    let app = App::new(width, height);

    // Spawn a new proc that will signal stop when enter is pressed.
    let (stop_tx, stop_rx) = channel();
    spawn(proc() {
        println!("press enter to stop rendering");
        stdin().read_line().unwrap();
        stop_tx.send(());
    });

    let images = app.images;

    // Then wait for news from other tasks: when an image has been rendered,
    // write it out, and when stop is signalled, stop the app.
    loop {
        select! {
            img = images.recv() => {
                // Write the image to output.png using lodepng.
                match lodepng::encode24_file(&Path::new("output.png"),
                                             img.as_slice(), width as u32, height as u32) {
                    Ok(_) => println!("wrote image to output.png"),
                    Err(reason) => println!("failed to write output png: {}", reason)
                }
            },
            () = stop_rx.recv() => {
                // Tell the app to stop.
                app.stop.send(());

                // Stop the main loop.
                break;
            }
        }
    }
}
