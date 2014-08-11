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

use intersection::Intersection;
use ray::Ray;
use vector3::{Vector3, dot};

/// Represents a surface that can be intersected with a ray.
pub trait Surface {
    /// Returns whether the surface was intersected, and if so, where.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

/// An infinitely large plane.
pub struct Plane {
    /// A unit vector perpendicular to the plane.
    normal: Vector3,

    /// A point in the plane.
    offset: Vector3
}

impl Plane {
    /// Creates a new plane with the specified `normal` through `offset`.
    pub fn new(normal: Vector3, offset: Vector3) -> Plane {
        Plane {
            normal: normal,
            offset: offset
        }
    }
}

impl Surface for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Transform the ray into the space where the plane is a linear
        // subspace (a plane through the origin).
        let origin = ray.origin - self.offset;

        let d = dot(self.normal, ray.direction);
        let t = dot(self.normal, origin) / d;

        // A ray has one direction, do not hit backwards.
        if t <= 0.0 {
            None
        } else {
            let intersection = Intersection {
                position: ray.origin + ray.direction * t,
                // Planes are two-sided.
                normal: if d < 0.0 { self.normal } else { -self.normal },
                // Tangent is not used here.
                tangent: Vector3::new(0.0, 0.0, 0.0),
                distance: t
            };
            Some(intersection)
        }
    }
}
