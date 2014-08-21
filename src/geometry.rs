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
use vector3::{Vector3, cross, dot};

/// Represents a surface that can be intersected with a ray.
pub trait Surface {
    /// Returns whether the surface was intersected, and if so, where.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

/// Represents a part of space.
pub trait Volume {
    /// Returns whether the specified point `p` lies inside the volume.
    fn lies_inside(&self, p: Vector3) -> bool;
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

/// Intersects a plane, and returns the position, distance, and the dot
/// product of the normal with the ray.
fn intersect_plane(normal: &Vector3, offset: &Vector3, ray: &Ray)
                   -> Option<(Vector3, f32, f32)> {
    // Transform the ray into the space where the plane is a linear
    // subspace (a plane through the origin).
    let origin = ray.origin - *offset;

    let d = dot(*normal, ray.direction);
    if d == 0.0 { return None; }
    let t = - dot(*normal, origin) / d;

    // A ray has one direction, do not hit backwards.
    if t <= 0.0 {
        None
    } else {
        Some((ray.origin + ray.direction * t, t, d))
    }
}

impl Surface for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect_plane(&self.normal, &self.offset, ray)
        .map(|(pos, t, d)| {
            Intersection {
                position: pos,
                // Planes are two-sided.
                normal: if d < 0.0 { self.normal } else { -self.normal },
                // Tangent is not used here.
                tangent: Vector3::zero(),
                distance: t
            }
        })
    }
}

/// An infinitely large one-sided plane that cuts space in half.
pub struct SpacePartitioning {
    /// A unit vector perpendicular to the plane.
    normal: Vector3,

    /// A point in the plane.
    offset: Vector3
}

impl SpacePartitioning {
    /// Creates a new space partitioning with the specified `normal` through
    /// `offset`. The side that the normal points to is outside.
    pub fn new(normal: Vector3, offset: Vector3) -> SpacePartitioning {
        SpacePartitioning {
            normal: normal,
            offset: offset
        }
    }
}

impl Surface for SpacePartitioning {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect_plane(&self.normal, &self.offset, ray)
        .map(|(pos, t, _)| {
            Intersection {
                position: pos,
                normal: self.normal,
                // Tangent is not used here.
                tangent: Vector3::zero(),
                distance: t
            }
        })
    }
}

impl Volume for SpacePartitioning {
    fn lies_inside(&self, p: Vector3) -> bool {
        dot(p - self.offset, self.normal) < 0.0
    }
}

pub struct Circle {
    /// A unit vector perpendicular to the circle.
    normal: Vector3,

    /// The centre of the circle.
    position: Vector3,

    /// The square of the radius of the circle.
    radius_squared: f32
}

impl Circle {
    pub fn new(normal: Vector3, position: Vector3, radius: f32) -> Circle {
        Circle {
            normal: normal,
            position: position,
            radius_squared: radius * radius
        }
    }
}

impl Surface for Circle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect_plane(&self.normal, &self.position, ray)
        .filtered(|&(pos, _, _)| {
            // Allow only indersections that lie inside the circle.
            (pos - self.position).magnitude_squared() <= self.radius_squared
        })
        .map(|(pos, t, d)| {
            Intersection {
                position: pos,
                // Planes are two-sided.
                normal: if d < 0.0 { self.normal } else { -self.normal },
                // Tangent is not used here.
                tangent: Vector3::zero(),
                distance: t
            }
        })
    }
}

pub struct Sphere {
    /// The position of the centre of the sphere.
    position: Vector3,

    /// The square of the radius of the spere.
    radius_squared: f32
}

impl Sphere {
    pub fn new(position: Vector3, radius: f32) -> Sphere {
        Sphere {
            position: position,
            radius_squared: radius * radius
        }
    }

    /// Returns whether a ray intersects the sphere, and if it does,
    /// the distances along the ray.
    fn get_intersections(&self, ray: &Ray) -> Option<(f32, f32)> {
        // Compute the a, b, c factors of the quadratic equation.
        let a = 1.0f32;
        let centre_offset = self.position - ray.origin;
        let b = 2.0 * dot(ray.direction, centre_offset);
        let c = centre_offset.magnitude_squared() - self.radius_squared;

        // The discriminant determines whether the equation has a solution.
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // For a negative discriminant, there is no solution.
            None
        } else {
            let d = discriminant.sqrt();
            let t1 = -0.5 * (-b + d) / a;
            let t2 = -0.5 * (-b - d) / a;
            Some((t1, t2))
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // TODO: is there a macro for this, like try!?
        // Maybe write it myself?
        let (t1, t2) = match self.get_intersections(ray) {
            None => return None,
            Some(x) => x
        };

        // One of the ts must be positive at least, for an intersection.
        let mut t: f32;
        if t1 > 0.0 && t1 < t2 { t = t1; }
        else if t2 > 0.0 && t2 < t1 { t = t2; }
        // For negative t, the sphere lies behind the ray entirely.
        else { return None; }

        // The intersection point can be calculated from the distance.
        let position = ray.origin + ray.direction * t;

        // The normal points radially outward everywhere.
        let normal = (position - self.position).normalise();

        // The tangent vector is perpendicular to the surface normal
        // and the up vector. The choice is quite arbitrary.
        let up = Vector3::new(0.0, 1.0, 0.0);
        let tangent = cross(up, normal).normalise();

        let intersection = Intersection {
            position: position,
            normal: normal,
            tangent: tangent,
            distance: t
        };
        Some(intersection)
    }
}

impl Volume for Sphere {
    fn lies_inside(&self, p: Vector3) -> bool {
        (p - self.position).magnitude_squared() < self.radius_squared
    }
}

/// An intersection of two volumes/surfaces, the boolean ‘and’.
struct Compound<T1, T2> {
    /// The first of the two surfaces.
    surface1: T1,

    /// The second of the two surfaces.
    surface2: T2
}

impl<T1, T2> Compound<T1, T2> {
    /// Creates a new object which is the intersection of the two
    /// specified objects.
    pub fn new(s1: T1, s2: T2) -> Compound<T1, T2> {
        Compound {
            surface1: s1,
            surface2: s2
        }
    }
}

impl<T1: Surface + Volume, T2: Surface + Volume> Surface
for Compound<T1, T2> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let i1 = self.surface1.intersect(ray);
        let i2 = self.surface2.intersect(ray);

        // Invalidate intersections that do not lie in both volumes.
        let i1 = i1.filtered(|i| { self.surface2.lies_inside(i.position) });
        let i2 = i2.filtered(|i| { self.surface1.lies_inside(i.position) });

        // If both intersections are valid, pick the closest one.
        if i1.is_some() && i2.is_some() {
            if i1.unwrap().distance < i2.unwrap().distance {
                return i1;
            } else {
                return i2;
            }
        }

        i1.or(i2)
    }
}

impl <T1: Volume, T2: Volume> Volume for Compound<T1, T2> {
    fn lies_inside(&self, p: Vector3) -> bool {
        self.surface1.lies_inside(p) && self.surface2.lies_inside(p)
    }
}
