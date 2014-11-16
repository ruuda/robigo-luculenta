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

use std::f32::consts::PI;
use std::num::{Float, FloatMath};
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

// Filter(ed) is implemented manually, because it is deprecated in the standard
// library, but it allows for some elegant code, so I wanted to keep it.
trait Filter<T> {
    fn filter(self, condition: |&T| -> bool) -> Self;
}

impl<T> Filter<T> for Option<T> {
    fn filter(self, condition: |&T| -> bool) -> Option<T> {
        match self {
            Some(x) => if condition(&x) { Some(x) } else { None },
            None => None
        }
    }
}

impl Surface for Circle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect_plane(&self.normal, &self.position, ray)
        .filter(|&(pos, _, _)| {
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

#[deriving(Clone)]
pub struct Paraboloid {
    /// The position of the focal point projected onto the plane.
    offset: Vector3,

    /// The normal of the plane, a vector of length 1 perpendicular to
    /// the plane (pointed towards the focal point).
    normal: Vector3,

    /// The position of the focal point, relative to the offset point.
    focal_point: Vector3
}

impl Paraboloid {
    /// Creates a new paraboloid, for the plane with the specified
    /// normal, the top centered between the specified offset, and
    /// specified focal distance (from the top, not the plane).
    pub fn new(normal: Vector3,
               offset: Vector3,
               focal_distance: f32)
               -> Paraboloid {
        Paraboloid {
            normal: normal,
            offset: offset - normal * focal_distance,
            focal_point: normal * (focal_distance * 2.0)
        }
    }
}

impl Surface for Paraboloid {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Transform the ray into the space where the plane is a linear
        // subspace (a plane through the origin).
        let origin = ray.origin - self.offset;
        let focal_offset = origin - self.focal_point;

        let n_dot_d = dot(self.normal, ray.direction);
        let n_dot_o = dot(self.normal, origin);
        let d_dot_f = dot(ray.direction, focal_offset);

        // Compute a, b, and c for the quadratic paraboloid equation.
        let a = n_dot_d * n_dot_d - 1.0;
        let b = 2.0 * n_dot_d * n_dot_o - 2.0 * d_dot_f;
        let c = n_dot_o * n_dot_o - focal_offset.magnitude_squared();

        // If a is zero, there is no quadratic equation,
        // and the solution is a simple line intersection.
        let t = if a == 0.0 {
            let t1 = -c / b;
            // For negative t, the paraboloid lies behind the ray.
            if t1 < 0.0 { return None; }
            else { t1 }
        } else {
            // The discriminant determines whether the equation
            // has a solution.
            let d = b * b - 4.0 * a * c;

            // If it is less than zero, there is no intersection.
            if d < 0.0 { return None; }

            // Otherwise, the equation can be solved for t.
            let sqrt_d = d.sqrt();
            let t1 = 0.5 * (-b + sqrt_d) / a;
            let t2 = 0.5 * (-b - sqrt_d) / a;

            // Pick the closest non-negative t.
            match (t1, t2) {
                (p, q) if p > 0.0 && (p < q || q < 0.0) => p,
                (_, q) if q > 0.0 => q,
                // For negative t, the paraboloid lies behind the ray entirely.
                _ => { return None; }
            }
        };

        // Compute the intersection details and normal.
        let pos = ray.origin + ray.direction * t;
        let local_pos = pos - self.offset;
        let plane_pr = local_pos - self.normal * dot(local_pos, self.normal);
        let normal = (self.focal_point - plane_pr).normalise();

        let intersection = Intersection {
            position: pos,
            normal: normal,
            tangent: Vector3::zero(), // Not used here.
            distance: t
        };

        Some(intersection)
    }
}

/// An intersection of two volumes/surfaces, the boolean ‘and’.
pub struct Compound<T1, T2> {
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

impl<T1, T2> Surface for Compound<T1, T2>
    where T1: Surface + Volume, T2: Surface + Volume {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let i1 = self.surface1.intersect(ray);
        let i2 = self.surface2.intersect(ray);

        // Invalidate intersections that do not lie in both volumes.
        let i1 = i1.filter(|i| { self.surface2.lies_inside(i.position) });
        let i2 = i2.filter(|i| { self.surface1.lies_inside(i.position) });

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

impl<T1, T2> Volume for Compound<T1, T2> where T1: Volume, T2: Volume {
    fn lies_inside(&self, p: Vector3) -> bool {
        self.surface1.lies_inside(p) && self.surface2.lies_inside(p)
    }
}

type InfinitePrism = Compound<Compound<SpacePartitioning, SpacePartitioning>,
                              SpacePartitioning>;

type ThickPlane = Compound<SpacePartitioning, SpacePartitioning>;

type Prism = Compound<InfinitePrism, ThickPlane>;

type HexagonalPrism = Compound<InfinitePrism, Prism>;

/// Constructs an equilateral triangle with specified edge length,
/// infinitely extruded along the axis vector,
/// rotated at the specified angle.
pub fn new_infinite_prism(axis: Vector3,
           offset: Vector3,
           edge_length: f32,
           angle: f32)
           -> InfinitePrism {
    let radius = 3.0_f32.sqrt() / 6.0 * edge_length;
    let a1 = angle;
    let a2 = angle + PI * 2.0 / 3.0;
    let a3 = angle + PI * 4.0 / 3.0;

    // Calculate the three unit vertices on the xy-plane.
    let p1 = Vector3::new(a1.cos(), a1.sin(), 0.0);
    let p2 = Vector3::new(a2.cos(), a2.sin(), 0.0);
    let p3 = Vector3::new(a3.cos(), a3.sin(), 0.0);

    // Now rotate the vertices, so they lie in the plane which
    // the axis vector is the normal.
    let p1 = p1.rotate_towards(axis);
    let p2 = p2.rotate_towards(axis);
    let p3 = p3.rotate_towards(axis);

    // Then the planes through the vertices can be constructed.
    let sp1 = SpacePartitioning::new(p1, p1 * radius + offset);
    let sp2 = SpacePartitioning::new(p2, p2 * radius + offset);
    let sp3 = SpacePartitioning::new(p3, p3 * radius + offset);

    // And finally combine the partitionings,
    // so that the extruded triangle is ‘carved out’.
    Compound::new(Compound::new(sp1, sp2), sp3)
}

/// Constructs a thick, infinite ‘wall’-like structure. One surface
/// passes through the specified offset, the other one is translated
/// along the normal for the specified thickness distance.
pub fn new_thick_plane(normal: Vector3,
                       offset: Vector3,
                       thickness: f32)
                       -> ThickPlane {
    // The plane is extruded along the normal, so the plane through
    // the offset vector should have the opposite normal.
    let sp1 = SpacePartitioning::new(-normal, offset);

    // And the extruded plane has the normal in the direction
    // of the extrusion.
    let sp2 = SpacePartitioning::new(normal, offset + normal * thickness);

    Compound::new(sp1, sp2)
}

/// Constructs a prism, oriented along the specified axis, its
/// equilateral base triangle with specified edge length at the
/// specified offset, rotated with the specified angle, and then
/// extruded along the axis for the specified height.
pub fn new_prism(axis: Vector3,
                 offset: Vector3,
                 edge_length: f32,
                 angle: f32,
                 height: f32)
                 -> Prism {
    // The infinite prism as before.
    let prism = new_infinite_prism(axis, offset, edge_length, angle);
    
    // But now clipped by a thick plane along the axis.
    let plane = new_thick_plane(axis, offset, height);

    Compound::new(prism, plane)
}

/// Constructs a prism, oriented along the specified axis, its
/// equilateral base triangle with specified edge length at the
/// specified offset, rotated with the specified angle, and then
/// extruded along the axis for the specified height. Then its corners
/// are capped, such that the effective edge length becomes the edge
/// length minus twice the bevel size.
pub fn new_hexagonal_prism(axis: Vector3,
                           offset: Vector3,
                           edge_length: f32,
                           bevel_size: f32,
                           angle: f32,
                           height: f32)
                           -> HexagonalPrism {
    // The ‘bevel edges’ (which is just an infinitely extruded
    // triangle; an infinte prism). It is rotated 180 degrees,
    // so it cuts off the corners. If the edge length is twice
    // the edge length of the desired prism, it does not cut the
    // corners at all.
    let iprism = new_infinite_prism(axis, offset,
                                    edge_length * 2.0 - bevel_size * 3.0,
                                    angle + PI);

    // And the normal prism, without bevel.
    let prism = new_prism(axis, offset, edge_length, angle, height);

    Compound::new(iprism, prism)
}
