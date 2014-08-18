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

use std::comm::{Handle, Select, Sender, Receiver, channel};
use std::os::num_cpus;
use std::sync::{Arc, Mutex};
use std::vec::unzip;
use camera::Camera;
use scene::Scene;
use quaternion::Quaternion;
use task_scheduler::TaskScheduler;
use vector3::Vector3;

pub type Image = Vec<u8>;

/// Width of the canvas.
pub static image_width: uint = 1280;

/// Height of the canvas.
pub static image_height: uint = 720;

/// Canvas aspect ratio.
static aspect_ratio: f32 = image_width as f32 / image_height as f32;

pub struct App {
    /// The scene that is rendered.
    scene: Scene,

    /// Channel that can be used to signal the application to stop.
    pub stop: Sender<()>,

    /// Channel that produces a rendered image periodically.
    pub images: Receiver<Image>
}

impl App {
    pub fn new() -> App {
        // Build the scene that will be rendered.
        let scene = App::set_up_scene();
        let concurrency = num_cpus();
        let ts = TaskScheduler::new(concurrency, image_width, image_height);
        let task_scheduler = Arc::new(Mutex::new(ts));

        // Channels for communicating back to the main task.
        let (stop_tx, stop_rx) = channel::<()>();
        let (img_tx, img_rx) = channel();

        // Then spawn a supervisor task that will start the workers.
        spawn(proc() {
            // Spawn as many workers as cores.
            let (stop_workers, images) = unzip(
            range(0u, concurrency)
            .map(|_| { App::start_worker(task_scheduler.clone()) }));
            
            // Combine values so we can recv one at a time.
            let select = Select::new();
            let mut worker_handles: Vec<Handle<Image>> = images
            .iter().map(|worker_rx| {
                let mut handle = select.handle(worker_rx);
                unsafe { handle.add(); }
                handle
            }).collect();
            let mut stop_handle = select.handle(&stop_rx);
            unsafe { stop_handle.add(); }
            
            // Then go into the supervising loop: broadcast a stop signal to
            // all workers, or route a rendered image to the main task.
            loop {
                let id = select.wait();

                // Was the source a worker?
                for handle in worker_handles.mut_iter() {
                    // When a new image arrives, route it to the main task.
                    if id == handle.id() {
                        let img = handle.recv();
                        img_tx.send(img);
                    }
                }

                // Or the stop channel perhaps?
                if id == stop_handle.id() {
                    // Broadcast to all workers that they should stop.
                    for stop in stop_workers.iter() {
                        stop.send(());
                    }
                    // Then also stop the supervising loop.
                    break;
                }
            }
        });

        App {
            scene: scene,
            stop: stop_tx,
            images: img_rx
        }
    }

    fn start_worker(scheduler: Arc<Mutex<TaskScheduler>>)
                    -> (Sender<()>, Receiver<Image>) {
        let (stop_tx, stop_rx) = channel::<()>();
        let (img_tx, img_rx) = channel::<Image>();

        // TODO: spawn proc.
        (stop_tx, img_rx)
    }

    fn set_up_scene() -> Scene {
        fn get_camera_at_time(_: f32) -> Camera {
            Camera {
                position: Vector3::new(0.0, 1.0, -10.0),
                field_of_view: Float::frac_pi_2(),
                focal_distance: 10.0,
                depth_of_field: 1.0,
                chromatic_abberation: 0.1,
                orientation: Quaternion::rotation(1.0, 0.0, 0.0, 1.531)
            }
        }

        Scene {
            objects: Vec::new(),
            get_camera_at_time: get_camera_at_time 
        }
    }
}
