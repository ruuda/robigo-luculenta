use quaternion::{Quaternion};

mod quaternion;

fn main() {
    let a = Quaternion::new(1.0, 1.0, 1.0, 0.0);
    let b = a.normalise();
    let c = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    println!("The quaternion a is {}.", a);
    println!("The quaternion a has magnitude {}.", a.magnitude());
    println!("The magnitude of b is {}.", b.magnitude());
    println!("The conjugate of c is {}.", c.conjugate());
}
