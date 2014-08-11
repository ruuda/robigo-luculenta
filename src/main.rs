use camera::Camera;
use intersection::Intersection;
use material::{Material, GlossyMirrorMaterial, DiffuseColouredMaterial, DiffuseGreyMaterial};
use ray::Ray;
use quaternion::Quaternion;
use vector3::Vector3;

mod camera;
mod constants;
mod intersection;
mod material;
mod monte_carlo;
mod ray;
mod quaternion;
mod vector3;

fn main() {
    let rotation = Quaternion::rotation(0.0, 0.0, 1.0, Float::frac_pi_2());
    let ray = Ray {
        origin: Vector3::new(5.0, 7.0, 11.0),
        direction: Vector3::new(1.0, 2.0, 3.0).normalise(),
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
    let diffuse_grey = DiffuseGreyMaterial::new(0.8);
    let red = DiffuseColouredMaterial::new(0.9, 700.0, 60.0);
    let mirror = GlossyMirrorMaterial::new(0.1);
    let new_ray = diffuse_grey.get_new_ray(&ray, &intersection);
    println!("The ray is {}.", ray);
    println!("A random number in [-1, 1] is {}.", monte_carlo::get_bi_unit());
    println!("A random number in [0, 1] is {}.", monte_carlo::get_unit());
    println!("A random sphere coordinate is ({}, {}).", monte_carlo::get_longitude(), monte_carlo::get_latitude());
    println!("A random visible wavelength is {}.", monte_carlo::get_wavelength());
    println!("A random hemisphere vector is {}.", monte_carlo::get_hemisphere_vector());
    println!("The speed of light is {} m/s.", constants::SPEED_OF_LIGHT);
    println!("A camera ray through (0,0) is {}.", camera.get_ray(0.0, 0.0, 550.0));
    println!("The reflected ray is {}.", new_ray);
    println!("The ray reflected on red: {}.", red.get_new_ray(&ray, &intersection));
    println!("The mirrored ray is {}.", mirror.get_new_ray(&ray, &intersection));
}
