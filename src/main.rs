use camera::Camera;
use ray::Ray;
use quaternion::Quaternion;
use vector3::Vector3;

mod camera;
mod constants;
mod monte_carlo;
mod ray;
mod quaternion;
mod vector3;

fn main() {
    let a = Quaternion::new(1.0, 1.0, 1.0, 0.0);
    let b = a.normalise();
    let c = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let sum = a + c;
    let diff = a - c;
    let d = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let v = Vector3::new(1.0, 0.0, 0.0);
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
        chromatic_abberation: 1.0,
        orientation: Quaternion::rotation(0.0, 0.0, 1.0, 0.0)
    };
    println!("The quaternion a is {}.", a);
    println!("The quaternion a has magnitude {}.", a.magnitude());
    println!("The magnitude of b is {}.", b.magnitude());
    println!("The conjugate of c is {}.", c.conjugate());
    println!("The sum a + c is {}.", sum);
    println!("The difference a - c is {}.", diff);
    println!("The negation of a is {}.", -a);
    println!("The quaternion c * 2 is {}.", c * 2.0f32);
    println!("The product ac is {}.", a * c);
    println!("The product ad is {}.", a * d);
    println!("The vector v rotated by 90° around the z-axis is {}.", v.rotate(rotation));
    println!("The ray is {}.", ray);
    println!("A random number in [-1, 1] is {}.", monte_carlo::get_bi_unit());
    println!("A random number in [0, 1] is {}.", monte_carlo::get_unit());
    println!("A random sphere coordinate is ({}, {}).", monte_carlo::get_longitude(), monte_carlo::get_latitude());
    println!("A random visible wavelength is {}.", monte_carlo::get_wavelength());
    println!("A random hemisphere vector is {}.", monte_carlo::get_hemisphere_vector());
    println!("The speed of light is {} m/s.", constants::SPEED_OF_LIGHT);
}
