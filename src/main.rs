use quaternion::{Quaternion};

mod quaternion;

fn main() {
    let a = Quaternion::new(1.0, 1.0, 1.0, 0.0);
    println!("The quaternion a is {}.", a);
    println!("The quaternion a has magnitude {}.", a.magnitude());
}
