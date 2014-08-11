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

pub trait Material {
    /// Returns the ray that continues the light path, backwards from the
    /// camera to the light source.
    fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray;
}

/// A perfectly diffuse material that reflects all wavelengths perfectly,
/// but absorbes some energy.
pub struct DiffuseGreyMaterial {
    /// How much the material reflects; 0.0 is black, 1.0 is white.
    reflectance: f32
}

impl DiffuseGreyMaterial {
    pub fn new(refl: f32) -> DiffuseGreyMaterial {
        DiffuseGreyMaterial {
            reflectance: refl
        }
    }
}

impl Material for DiffuseGreyMaterial {
    fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray {
        // Generate a ray in a random direction,
        // originating from the intersection.
        let hemi_vec = ::monte_carlo::get_hemisphere_vector();

        // However, the new ray is now facing in the wrong direction,
        // it must be rotated towards the surface normal.
        let normal = if dot(incoming_ray.direction, intersection.normal) < 0.0 {
            intersection.normal
        } else {
            -intersection.normal
        };
        let direction = hemi_vec.rotate_towards(normal);

        // The probability of the new ray is determined by the reflectance.
        Ray {
            origin: intersection.position,
            direction: direction,
            wavelength: incoming_ray.wavelength,
            probability: self.reflectance
        }
    }
}
