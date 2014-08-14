use camera::Camera;
use geometry::{Volume, Plane, Sphere};
use material::{EmissiveMaterial, BlackBodyMaterial, DiffuseColouredMaterial};
use object::{Object, Emissive, Reflective};
use quaternion::Quaternion;
use scene::Scene;
use trace_unit::TraceUnit;
use plot_unit::PlotUnit;
use vector3::Vector3;

mod camera;
mod cie1931;
mod constants;
mod geometry;
mod intersection;
mod material;
mod monte_carlo;
mod object;
mod plot_unit;
mod quaternion;
mod ray;
mod scene;
mod trace_unit;
mod vector3;

fn main() {
    fn make_camera(_: f32) -> Camera {
        Camera {
            position: Vector3::new(0.0, 1.0, -10.0),
            field_of_view: Float::frac_pi_2(),
            focal_distance: 10.0,
            depth_of_field: 10.0,
            chromatic_abberation: 1.0,
            orientation: Quaternion::rotation(1.0, 0.0, 0.0, 0.5)
        }
    }
    let red = DiffuseColouredMaterial::new(0.9, 700.0, 60.0);
    let plane = Plane::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0);
    let black_body = BlackBodyMaterial::new(6504.0, 1.0);
    let reflective = Object::new(box plane, Emissive(box black_body));
    let emissive = Object::new(box sphere, Reflective(box red));
    let scene = Scene {
        objects: vec!(reflective, emissive),
        get_camera_at_time: make_camera
    };
    println!("Is (1,0,0) inside sphere? {}.", sphere.lies_inside(Vector3::new(1.0, 0.0, 0.0)));
    println!("Is (2,1,0) inside sphere? {}.", sphere.lies_inside(Vector3::new(2.0, 1.0, 0.0)));
    println!("Black body intensity at 400 and 600 nm is {} and {}.", black_body.get_intensity(400.0), black_body.get_intensity(600.0));

    let mut trace_unit = box TraceUnit::new(&scene, 1280, 720);
    trace_unit.render();
    println!("First three intensities are:.");
    for i in trace_unit.mapped_photons.slice(0, 3).iter().map(|mp| { mp.probability }) {
        println!("{}", i);
    }
}
