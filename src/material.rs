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
use constants::{BOLTZMANNS_CONSTANT, SPEED_OF_LIGHT, PLANCKS_CONSTANT, WIENS_CONSTANT};

/// Models the behaviour of a ray when it bounces off a surface.
pub trait Material {
    /// Returns the ray that continues the light path, backwards from the
    /// camera to the light source.
    fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray;
}

/// Models the behavior of a light-emitting surface. Light-emitting surfaces
/// are handled independently of reflecting surfaces.
pub trait EmissiveMaterial {
    /// Returns the light intensity at the specified `wavelength`.
    fn get_intensity(&self, wavelength: f32) -> f32;
}

/// Returns a ray as if reflected by a perfectly diffuse white material.
fn get_diffuse_ray(incoming_ray: &Ray, intersection: &Intersection) -> Ray {
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

    Ray {
        origin: intersection.position,
        direction: direction,
        wavelength: incoming_ray.wavelength,
        probability: 1.0
    }
}

/// The Boltzmann distribution.
fn boltzmann(wavelength: f64, temperature: f64) -> f64 {
    // Use double precision here, the numbers are quite large/small,
    // which might cause precision loss.
    let h = PLANCKS_CONSTANT;
    let k = BOLTZMANNS_CONSTANT;
    let c = SPEED_OF_LIGHT;

    // Multiply by 1e-9 (nano), because the wavelength is specified in nm,
    // while m is the standard unit.
    let f = c / (wavelength * 1.0e-9);

    // Then evaluate the Boltzmann distribution.
    (2.0 * h * f * f * f) / (c * c * ((h * f / (k * temperature)).exp() - 1.0))
}

/// Has the spectrum of a black body.
pub struct BlackBodyMaterial {
    /// The temperature of the black body, in Kelvin. 6504 is a warm white,
    /// higher values are blue-ish, lower are red-ish.
    temperature: f32,

    /// Bodies with lower temperature also have a lower intensity,
    /// but for the purposes of a light source, only the distribution
    /// is important, not the intensity, so the distribution must be
    /// normalised.
    normalisation_factor: f32
}

impl BlackBodyMaterial {
    /// Constructs a black body material with the specified
    /// temperature in Kelvin.
    pub fn new(kelvins: f32, intensity: f32) -> BlackBodyMaterial {
        BlackBodyMaterial {
            temperature: kelvins,
            normalisation_factor: intensity
                / boltzmann((WIENS_CONSTANT / kelvins as f64) * 1.0e9, kelvins as f64) as f32
        }
    }
}

impl EmissiveMaterial for BlackBodyMaterial {
    fn get_intensity(&self, wavelength: f32) -> f32 {
        boltzmann(wavelength as f64, self.temperature as f64) as f32 * self.normalisation_factor
    }
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
        let mut ray = get_diffuse_ray(incoming_ray, intersection);

        // The probability that the ray was reflected is the reflectance.
        ray.probability = self.reflectance;
        ray
    }
}

/// Reflects light of a certain wavelength better than others,
/// with a normal distribution.
pub struct DiffuseColouredMaterial {
    /// How much the material reflects; 0.0 is black, 1.0 is white.
    reflectance: f32,

    /// The wavelength that is best reflected, in nm.
    wavelength: f32,

    /// The standard deviation of the reflectance distribution.
    deviation: f32
}

impl DiffuseColouredMaterial {
    pub fn new(refl: f32, wavel: f32, dev: f32) -> DiffuseColouredMaterial {
        DiffuseColouredMaterial {
            reflectance: refl,
            wavelength: wavel,
            deviation: dev
        }
    }
}

impl Material for DiffuseColouredMaterial {
    fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray {
        // Compute the probability using Gaussian falloff.
        let p = (self.wavelength - incoming_ray.wavelength) / self.deviation;
        let q = (-0.5 * p * p).exp();

        let mut ray = get_diffuse_ray(incoming_ray, intersection);
        
        // The probablity is a combination of reflectance, and the probability
        // based on the wavelength.
        ray.probability = self.reflectance * q;
        ray
    }
}

/// Blends between perfect reflection and diffuse.
pub struct GlossyMirrorMaterial {
    /// The amount of 'gloss', where 1.0 equals diffuse,
    /// and 0.0 is a perfect mirror.
    glossiness: f32
}

impl GlossyMirrorMaterial {
    pub fn new(gloss: f32) -> GlossyMirrorMaterial {
        GlossyMirrorMaterial {
            glossiness: gloss
        }
    }
}

impl Material for GlossyMirrorMaterial {
    fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray {
        // The diffuse component is as usual.
        let mut ray = get_diffuse_ray(incoming_ray, intersection);

        // Then blend between diffuse and reflection, and re-normalise.
        let reflection = incoming_ray.direction.reflect(intersection.normal);
        ray.direction = (ray.direction * self.glossiness
                         + reflection * (1.0 - self.glossiness)).normalise();
        ray
    }
}
