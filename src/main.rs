use quaternion::{Quaternion};

mod quaternion;

fn main() {
    let a = Quaternion { w: 0.0, x: 1.0, y: 1.0, z: 1.0 };
    println!("The quaternion a is {}.", a);
}
