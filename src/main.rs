use quaternion::{Quaternion};

mod quaternion;

fn main() {
    let a = Quaternion::new(1.0, 1.0, 1.0, 0.0);
    let b = a.normalise();
    println!("The quaternion a is {}.", a);
    println!("The quaternion a has magnitude {}.", a.magnitude());
    println!("The magnitude of b is {}.", b.magnitude());
}
