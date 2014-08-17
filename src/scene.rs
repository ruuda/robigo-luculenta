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

use camera::Camera;
use intersection::Intersection;
use object::Object;
use ray::Ray;

/// A collection of objects.
pub struct Scene {
    /// All the renderable objects in the scene.
    pub objects: Vec<Object>,

    /// A function that returns the camera through which the scene
    /// will be seen. The function takes one parameter, the time (in
    /// the range 0.0 - 1.0), which will be sampled randomly to create
    /// effects like motion blur and zoom blur.
    // TODO: apparently there is no such thing as an immutable closure
    // any more, but I'd prefer to be able to use a pure function here,
    // which might be a closure.
    pub get_camera_at_time: fn (f32) -> Camera
}

impl Scene {
    /// Intersects the specified ray with the scene.
    pub fn intersect(&self, ray: &Ray) -> Option<(Intersection, &Object)> {
        // Assume Nothing is found, and that Nothing is Very Far Away (tm).
        let mut result = None;
        let mut distance = 1.0e12f32;

        // Then intersect all surfaces.
        for obj in self.objects.iter() {
            match obj.surface.intersect(ray) {
                None => { },
                Some(isect) => {
                    // If there is an intersection, and if it is nearer than a
                    // previous one, use it.
                    if isect.distance < distance {
                        result = Some((isect, obj));
                        distance = isect.distance;
                    }
                }
            }
        }

        result
    }
}
