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

use std::num::Float;
use quaternion::Quaternion;

#[deriving(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x
    }
}

pub fn dot(a: Vector3, b: Vector3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude_squared(self) -> f32 {
        dot(self, self)
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalise(self) -> Vector3 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            self
        } else {
            Vector3 {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude
            }
        }
    }

    pub fn rotate_towards(self, normal: Vector3) -> Vector3 {
        let dot = normal.z;

        // No rotation necessary.
        if dot > 0.9999 { return self; }

        // Mirror along the z-axis.
        if dot < -0.9999 { return Vector3::new(self.x, self.y, -self.z) }

        let up = Vector3::new(0.0, 0.0, 1.0);
        let a1 = cross(up, normal).normalise();
        let a2 = cross(a1, normal).normalise();

        a1 * self.x + a2 * self.y + normal * self.z
    }

    pub fn rotate(self, q: Quaternion) -> Vector3 {
        let p = Quaternion::new(self.x, self.y, self.z, 0.0);
        let r = q * p * q.conjugate();
        Vector3::new(r.x, r.y, r.z)
    }

    pub fn reflect(self, normal: Vector3) -> Vector3 {
        self - normal * 2.0 * dot(normal, self)
    }
}

impl Add<Vector3, Vector3> for Vector3 {
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vector3, Vector3> for Vector3 {
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg<Vector3> for Vector3 {
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f32, Vector3> for Vector3 {
    fn mul(self, f: f32) -> Vector3 {
        Vector3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f
        }
    }
}
