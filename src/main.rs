use vector3::{Vector3, cross, dot};

mod vector3;

fn main() {
    let a = Vector3::new(0.0, 2.0, 0.0);
    let b = a.normalised();
    let c = Vector3::new(1.0, 3.0, 1.0);
    let d = Vector3::new(1.0, 0.0, 0.0);
    let dot = dot(a, b);
    let sum = a + c;
    let diff = a - c;
    let neg = -a;
    let mul_right = a * 2.0;
    let cross = cross(a, d);
    let rotated = a.rotate_towards(d);
    println!("The vector a has magnitude {}.", a.magnitude());
    println!("The vector a normalised is {}.", b);
    println!("Their dot product is {}.", dot);
    println!("The sum a + c is {}.", sum);
    println!("The difference a - c is {}.", diff);
    println!("The negation of a is {}.", neg);
    println!("The scalar product a * 2 is {}.", mul_right);
    println!("The cross product of a and d is {}.", cross);
    println!("The vector a rotated towards d is {}.", rotated);
}
