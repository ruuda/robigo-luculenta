// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014-2015 Ruud van Asseldonk
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

use std::sync::mpsc::{Sender, Receiver, channel};
use std::f32::consts::PI;
use std::old_io::timer::sleep;
use std::num::Float;
use std::os::num_cpus;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::time::Duration;
use camera::Camera;
use constants::GOLDEN_RATIO;
use gather_unit::GatherUnit;
use geometry::{Circle, Paraboloid, Plane, Sphere, Surface, new_hexagonal_prism};
use material::{BlackBodyMaterial,
               DiffuseGreyMaterial,
               DiffuseColouredMaterial,
               GlossyMirrorMaterial,
               Sf10GlassMaterial,
               SoapBubbleMaterial};
use object::Object;
use plot_unit::PlotUnit;
use quaternion::Quaternion;
use ray::Ray;
use scene::Scene;
use task_scheduler::{Task, TaskScheduler};
use tonemap_unit::TonemapUnit;
use trace_unit::TraceUnit;
use vector3::Vector3;

pub type Image = Vec<u8>;

pub struct App {
    /// Channel that produces a rendered image periodically.
    pub images: Receiver<Image>
}

impl App {
    /// Constructs and starts a new path tracer that renders to a canvas of
    /// the specified size.
    pub fn new(image_width: u32, image_height: u32) -> App {
        let concurrency = num_cpus();
        let ts = TaskScheduler::new(concurrency, image_width, image_height);
        let task_scheduler = Arc::new(Mutex::new(ts));

        // Channel for communicating back to the main task.
        let (img_tx, img_rx) = channel();

        // Set up the scene that will be rendered.
        let scene = Arc::new(App::set_up_scene());

        // Spawn as many workers as cores.
        for _ in (0us .. concurrency) {
            App::start_worker(task_scheduler.clone(),
                              scene.clone(),
                              img_tx.clone());
        }

        App { images: img_rx }
    }

    #[cfg(test)]
    pub fn new_test(image_width: u32, image_height: u32) -> App {
        // Set up a task scheduler and scene with no concurrency.
        let mut ts = TaskScheduler::new(1, image_width, image_height);
        let (mut img_tx, img_rx) = channel();
        let scene = Arc::new(App::set_up_scene());

        // Run 5 tasks serially, on this thread.
        let mut task = Task::Sleep;
        for _ in 0u8 .. 5 {
            task = ts.get_new_task(task);
            App::execute_task(&mut task, &*scene, &mut img_tx);
        }

        App { images: img_rx }
    }

    fn start_worker(task_scheduler: Arc<Mutex<TaskScheduler>>,
                    scene: Arc<Scene>,
                    img_tx: Sender<Image>) {
        Thread::spawn(move || {
            // Move img_tx into the proc.
            let mut owned_img_tx = img_tx;

            // There is no task yet, but the task scheduler expects
            // a completed task. Therefore, this worker is done sleeping.
            let mut task = Task::Sleep;

            // Continue rendering forever, unless the application is terminated.
            loop {
                // Ask the task scheduler for a new task, complete the old one.
                // Then execute it.
                task = task_scheduler.lock().unwrap().get_new_task(task);
                App::execute_task(&mut task, &*scene, &mut owned_img_tx);
            }
        });
    }

    fn execute_task(task: &mut Task, scene: &Scene, img_tx: &mut Sender<Image>) {
        match *task {
            Task::Sleep =>
                App::execute_sleep_task(),
            Task::Trace(ref mut trace_unit) =>
                App::execute_trace_task(scene, &mut **trace_unit),
            Task::Plot(ref mut plot_unit, ref mut units) =>
                App::execute_plot_task(&mut **plot_unit, &mut units[]),
            Task::Gather(ref mut gather_unit, ref mut units) =>
                App::execute_gather_task(&mut **gather_unit, &mut units[]),
            Task::Tonemap(ref mut tonemap_unit, ref mut gather_unit) =>
                App::execute_tonemap_task(img_tx, &mut **tonemap_unit, &mut **gather_unit)
        }
    }

    fn execute_sleep_task() {
        sleep(Duration::milliseconds(100));
    }

    fn execute_trace_task(scene: &Scene, trace_unit: &mut TraceUnit) {
        trace_unit.render(scene);
    }

    fn execute_plot_task(plot_unit: &mut PlotUnit,
                         units: &mut[Box<TraceUnit>]) {
        for unit in units.iter_mut() {
            plot_unit.plot(&unit.mapped_photons[]);
        }
    }

    fn execute_gather_task(gather_unit: &mut GatherUnit,
                           units: &mut[Box<PlotUnit>]) {
        for unit in units.iter_mut() {
            gather_unit.accumulate(&unit.tristimulus_buffer[]);
            unit.clear();
        }

        // Save the gather state, so that rendering can be continued later.
        gather_unit.save();
    }

    fn execute_tonemap_task(img_tx: &mut Sender<Image>,
                            tonemap_unit: &mut TonemapUnit,
                            gather_unit: &mut GatherUnit) {
        tonemap_unit.tonemap(&gather_unit.tristimulus_buffer[]);

        // Copy the rendered image.
        let img = tonemap_unit.rgb_buffer.clone();

        // And send it to the UI / main task.
        img_tx.send(img).unwrap();
    }

    fn set_up_scene() -> Scene {
        use object::MaterialBox::{Emissive, Reflective};

        let mut objects = Vec::new();

        // Sphere in the centre.
        let sun_radius: f32 = 5.0;
        let sun_position = Vector3::zero();
        let sun_sphere = Box::new(Sphere::new(sun_position, sun_radius));
        let sun_emissive = Box::new(BlackBodyMaterial::new(6504.0, 1.0));
        let sun = Object::new(sun_sphere, Emissive(sun_emissive));
        objects.push(sun);

        // Floor paraboloid.
        let floor_normal = Vector3::new(0.0, 0.0, -1.0);
        let floor_position = Vector3::new(0.0, 0.0, -sun_radius);
        let floor_paraboloid = Paraboloid::new(floor_normal, floor_position,
                                               sun_radius.powi(2));
        let grey = Box::new(DiffuseGreyMaterial::new(0.8));
        let floor = Object::new(Box::new(floor_paraboloid.clone()), Reflective(grey));
        objects.push(floor);

        // Floorwall paraboloid (left).
        let wall_left_normal = Vector3::new(0.0, 0.0, 1.0);
        let wall_left_position = Vector3::new(1.0, 0.0, -sun_radius.powi(2));
        let wall_left_paraboloid = Box::new(Paraboloid::new(wall_left_normal,
                                                            wall_left_position,
                                                            sun_radius.powi(2)));
        let green = Box::new(DiffuseColouredMaterial::new(0.9, 550.0, 40.0));
        let wall_left = Object::new(wall_left_paraboloid, Reflective(green));
        objects.push(wall_left);

        // Floorwall paraboloid (right).
        let wall_right_normal = Vector3::new(0.0, 0.0, 1.0);
        let wall_right_position = Vector3::new(-1.0, 0.0, -sun_radius.powi(2));
        let wall_right_paraboloid = Box::new(Paraboloid::new(wall_right_normal,
                                                             wall_right_position,
                                                             sun_radius.powi(2)));
        let red = Box::new(DiffuseColouredMaterial::new(0.9, 660.0, 60.0));
        let wall_right = Object::new(wall_right_paraboloid, Reflective(red));
        objects.push(wall_right);

        // Sky light 1.
        let sky_height: f32 = 30.0;
        let sky1_radius: f32 = 5.0;
        let sky1_position = Vector3::new(-sun_radius, 0.0, sky_height);
        let sky1_circle = Box::new(Circle::new(floor_normal, sky1_position, sky1_radius));
        let sky1_emissive = Box::new(BlackBodyMaterial::new(7600.0, 0.6));
        let sky1 = Object::new(sky1_circle, Emissive(sky1_emissive));
        objects.push(sky1);

        let sky2_radius: f32 = 15.0;
        let sky2_position = Vector3 {
            x: -sun_radius * 0.5, y: sun_radius * 2.0 + sky2_radius, z: sky_height
        };
        let sky2_circle = Box::new(Circle::new(floor_normal, sky2_position, sky2_radius));
        let sky2_emissive = Box::new(BlackBodyMaterial::new(5000.0, 0.6));
        let sky2 = Object::new(sky2_circle, Emissive(sky2_emissive));
        objects.push(sky2);

        // Ceiling plane (for more interesting light).
        let ceiling_position = Vector3::new(0.0, 0.0, sky_height * 2.0);
        let ceiling_plane = Box::new(Plane::new(floor_normal, ceiling_position));
        let blue = Box::new(DiffuseColouredMaterial::new(0.5, 470.0, 25.0));
        let ceiling = Object::new(ceiling_plane, Reflective(blue));
        objects.push(ceiling);

        // Spiral sunflower seeds.
        let gamma: f32 = PI * 2.0 * (1.0 - 1.0 / GOLDEN_RATIO as f32);
        let seed_size: f32 = 0.8;
        let seed_scale: f32 = 1.5;
        let first_seed = ((sun_radius / seed_scale + 1.0).powi(2) + 0.5) as isize;
        let seeds = 100;
        for i in (first_seed .. first_seed + seeds) {
            let phi = i as f32 * gamma;
            let r = (i as f32).sqrt() * seed_scale;
            let position = Vector3 {
                x: phi.cos() * r,
                y: phi.sin() * r,
                z: (r - sun_radius) * -0.5
            } + sun_position;
            let sphere = Box::new(Sphere::new(position, seed_size));
            let mat = Box::new(DiffuseColouredMaterial::new(0.9,
                              (i - first_seed) as f32 / seeds as f32
                              * 130.0 + 600.0, 60.0));
            let object = Object::new(sphere, Reflective(mat));
            objects.push(object);
        }

        // Seeds in between.
        for i in (first_seed .. first_seed + seeds) {
            let phi = (i as f32 + 0.5) * gamma;
            let r = (i as f32 + 0.5).sqrt() * seed_scale;
            let position = Vector3 {
                x: phi.cos() * r,
                y: phi.sin() * r,
                z: (r - sun_radius) * -0.25
            } + sun_position;
            let sphere = Box::new(Sphere::new(position, seed_size * 0.5));
            let mat = Box::new(GlossyMirrorMaterial::new(0.1));
            let object = Object::new(sphere, Reflective(mat));
            objects.push(object);
        }

        // Soap bubbles above.
        for i in (first_seed / 2 .. first_seed + seeds) {
            let phi = -i as f32 * gamma;
            let r = (i as f32).sqrt() * seed_scale * 1.5;
            let position = Vector3 {
                x: phi.cos() * r,
                y: phi.sin() * r,
                z: (r - sun_radius) * 1.5 + sun_radius * 2.0
            } + sun_position;
            let sphere = Box::new(Sphere::new(position, seed_size
                                             * (0.5 + (i as f32).sqrt() * 0.2)));
            let mat = Box::new(SoapBubbleMaterial);
            let object = Object::new(sphere, Reflective(mat));
            objects.push(object);
        }

        // Prisms along the walls.
        let prisms: isize = 11;
        let prism_angle: f32 = PI * 2.0 / prisms as f32;
        let prism_radius: f32 = 17.0;
        let prism_height: f32 = 8.0;
        for i in (0 .. prisms) {
            for &(ofs, radius, phi_ofs, h) in vec!(
                    (0.0f32, 1.0f32, 0.0f32, 1.0f32),
                    (0.5 * prism_angle, 1.2, PI * 0.5, 1.5)
                ).iter() {
                let phi = i as f32 * prism_angle + ofs;
                // Get an initial position.
                let mut position = Vector3 {
                    x: phi.cos() * prism_radius * radius,
                    y: phi.sin() * prism_radius * radius,
                    z: 0.0
                };
                let mut normal = Vector3::new(0.0, 0.0, -1.0);

                // Intersect with the floor to get the normal.
                let ray = Ray {
                    origin: position,
                    direction: normal,
                    wavelength: 0.0,
                    probability: 1.0
                };

                if let Some(intersection) = floor_paraboloid.intersect(&ray) {
                    // The parabola focus is on the other side of the paraboloid.
                    normal = -intersection.normal;
                    position = intersection.position + normal * 2.0 * h;
                }

                let prism = Box::new(new_hexagonal_prism(normal, position, 3.0, 1.0,
                                                         phi + phi_ofs, prism_height * h));
                let glass = Box::new(Sf10GlassMaterial);
                let object = Object::new(prism, Reflective(glass));
                objects.push(object);
            }
        }

        fn make_camera(t: f32) -> Camera {
            // Orbit around (0, 0, 0) based on the time.
            let phi = PI * (1.0 + 0.01 * t);
            let alpha = PI * (0.3 - 0.01 * t);

            // Also zoom in a bit. (Or actually, it is a dolly roll.)
            let distance = 50.0 - 0.5 * t;

            let position = Vector3 {
                x: alpha.cos() * phi.sin() * distance,
                y: alpha.cos() * phi.cos() * distance,
                z: alpha.sin() * distance
            };

            // Compensate for the displacement of the camera by rotating
            // such that (0, 0, 0) remains fixed. The camera is aimed
            // downward with angle alpha.
            let orientation = Quaternion::rotation(0.0, 0.0, -1.0, phi + PI)
                * Quaternion::rotation(1.0, 0.0, 0.0, -alpha);

            Camera {
                position: position,
                field_of_view: PI * 0.35,
                focal_distance: distance * 0.9,
                // A slight blur, not too much, but enough to demonstrate the effect.
                depth_of_field: 2.0,
                // A subtle amount of chromatic abberation.
                chromatic_abberation: 0.012,
                orientation: orientation
            }
        }

        Scene {
            objects: objects,
            get_camera_at_time: make_camera
        }
    }
}
