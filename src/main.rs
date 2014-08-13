use camera::Camera;
use geometry::{Surface, Volume, Plane, SpacePartitioning, Sphere};
use intersection::Intersection;
use material::{EmissiveMaterial, BlackBodyMaterial, DiffuseColouredMaterial};
use object::{Object, Emissive, Reflective};
use quaternion::Quaternion;
use ray::Ray;
use vector3::Vector3;
use scene::Scene;

mod camera;
mod constants;
mod geometry;
mod intersection;
mod material;
mod monte_carlo;
mod object;
mod quaternion;
mod ray;
mod scene;
mod vector3;

fn main() {
    let ray = Ray {
        origin: Vector3::new(0.0, 0.0, -10.0),
        direction: Vector3::new(0.0, 0.0, 1.0),
        wavelength: 550.0,
        probability: 1.0
    };
    let camera = Camera {
        position: Vector3::new(0.0, 0.0, 0.0),
        field_of_view: Float::frac_pi_2(),
        focal_distance: 10.0,
        depth_of_field: 10.0,
        chromatic_abberation: 1.0,
        orientation: Quaternion::rotation(0.0, 0.0, 1.0, 0.0)
    };
    let intersection = Intersection {
        position: Vector3::new(0.0, 1.0, 2.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tangent: Vector3::new(0.0, 1.0, 0.0),
        distance: 1.0
    };
    let red = DiffuseColouredMaterial::new(0.9, 700.0, 60.0);
    let plane = Plane::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    let sp = SpacePartitioning::new(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0);
    let black_body = BlackBodyMaterial::new(6504.0, 1.0);
    let reflective = Object::new(box plane, Reflective(box red));
    let emissive = Object::new(box sphere, Emissive(box black_body));
    let scene = Scene {
        objects: vec!(reflective, emissive),
        get_camera_at_time: |_| { camera }
    };
    println!("Is (0,0,0) inside sp? {}.", sp.lies_inside(Vector3::new(0.0, 0.0, 0.0)));
    println!("Is (0,0,2) inside sp? {}.", sp.lies_inside(Vector3::new(0.0, 0.0, 2.0)));
    println!("Is (1,0,0) inside sphere? {}.", sphere.lies_inside(Vector3::new(1.0, 0.0, 0.0)));
    println!("Is (2,1,0) inside sphere? {}.", sphere.lies_inside(Vector3::new(2.0, 1.0, 0.0)));
    println!("Black body intensity at 400 and 600 nm is {} and {}.", black_body.get_intensity(400.0), black_body.get_intensity(600.0));
    println!("Intersecting ray with the scene yields {}.", scene.intersect(&ray));
}
