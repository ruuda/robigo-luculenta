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
    println!("The vector has magnitude {}.", a.magnitude());
    println!("The normalised y-component is {}.", b.y);
    println!("Their dot product is {}.", dot);
    println!("The y-component of the sum is {}.", sum.y);
    println!("The y-component of the difference is {}.", diff.y);
    println!("The y-component of the negation is {}.", neg.y);
    println!("The y-component of a * 2 is {}.", mul_right.y);
    println!("The z-component of the cross product of a and d is {}.", cross.z);
}
