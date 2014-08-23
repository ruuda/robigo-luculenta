// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#![allow(dead_code)]

#[deriving(Show)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w:f32) -> Quaternion {
        Quaternion { x: x, y: y, z: z, w: w }
    }

    /// Returns a quaternion that represents a rotation of `angle` radians
    /// around the axis specified by `x`, `y` and `z`.
    pub fn rotation(x: f32, y: f32, z: f32, angle: f32) -> Quaternion {
        Quaternion {
            x: (angle * 0.5).sin() * x,
            y: (angle * 0.5).sin() * y,
            z: (angle * 0.5).sin() * z,
            w: (angle * 0.5).cos()
        }
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalise(self) -> Quaternion {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            self
        } else {
            Quaternion {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
                w: self.w / magnitude
            }
        }
    }

    pub fn conjugate(self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }
}

impl Add<Quaternion, Quaternion> for Quaternion {
    fn add(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub<Quaternion, Quaternion> for Quaternion {
    fn sub(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Neg<Quaternion> for Quaternion {
    fn neg(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

trait MulQuaternion {
    fn mul(&self, lhs: &Quaternion) -> Quaternion;
}

impl MulQuaternion for f32 {
    fn mul(&self, lhs: &Quaternion) -> Quaternion {
        Quaternion {
            x: lhs.x * *self,
            y: lhs.y * *self,
            z: lhs.z * *self,
            w: lhs.w * *self
        }
    }
}

impl MulQuaternion for Quaternion {
    fn mul(&self, lhs: &Quaternion) -> Quaternion {
        Quaternion {
            x: lhs.w * self.x + lhs.x * self.w + lhs.y * self.z - lhs.z * self.y,
            y: lhs.w * self.y - lhs.x * self.z + lhs.y * self.w + lhs.z * self.x,
            z: lhs.w * self.z + lhs.x * self.y - lhs.y * self.x + lhs.z * self.w,
            w: lhs.w * self.w - lhs.x * self.x - lhs.y * self.y - lhs.z * self.z
        }
    }
}

impl<T: MulQuaternion> Mul<T, Quaternion> for Quaternion {
    fn mul(&self, other: &T) -> Quaternion {
        other.mul(self)
    }
}
