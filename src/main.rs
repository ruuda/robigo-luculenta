use vector3::{Vector3, dot};

mod vector3;

fn main() {
    let a = Vector3::new(0.0, 2.0, 0.0);
    let b = a.normalised();
    let d = dot(a, b);
    println!("The vector has magnitude {}.", a.magnitude());
    println!("The normalised y-component is {}.", b.y);
    println!("Their dot product is {}.", d);
}
