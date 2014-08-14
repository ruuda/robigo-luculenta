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

use ray::Ray;
use scene::Scene;

/// The number of paths to trace in one batch.
// TODO: Use 1024 * 64 for debug, 1024 * 512 in release.
static number_of_photons: uint = 1024 * 64;

/// Represents a photon that has been traced.
pub struct MappedPhoton {
    /// The screen position x-coordinate.
    pub x: f32,

    /// The screen position y-coordinate.
    pub y: f32,

    /// The probability that a simulated photon hit the screen
    /// at this position.
    pub probability: f32,

    /// The wavelength of the simulated photon (in nm).
    pub wavelength: f32
}

impl MappedPhoton {
    fn new() -> MappedPhoton {
        MappedPhoton {
            x: 0.0,
            y: 0.0,
            probability: 0.0,
            wavelength: 0.0
        }
    }
}

/// Handles ray tracing.
pub struct TraceUnit<'a> {
    /// The scene that will be rendered.
    scene: &'a Scene<'a>,

    /// The aspect ratio of the image that will be rendered.
    aspect_ratio: f32,

    /// The photons that were rendered.
    pub mapped_photons: [MappedPhoton, ..number_of_photons]
}

impl<'a> TraceUnit<'a> {
    /// Creates a new trace unit that renders the given scene.
    pub fn new<'b>(scene: &'b Scene<'b>, width: i32, height: i32) -> TraceUnit<'b> {
        TraceUnit {
            scene: scene,
            aspect_ratio: width as f32 / height as f32,
            mapped_photons: [MappedPhoton::new(), ..number_of_photons]
        }
    }

    /// Return the contribution of a photon travelling backwards
    /// the specified ray.
    fn render_ray(&self, ray: &Ray) -> f32 {
        0.0
    }

    /// Returns the contribution of a ray
    /// through the specified creen coordinate.
    fn render_camera_ray(&self, x: f32, y: f32, wavelength: f32) -> f32 {
        // Get a random time to sample at.
        let t = ::monte_carlo::get_unit();

        // Get the camera at that time.
        let camera = (self.scene.get_camera_at_time)(t);

        // Create a camera ray for the specified pixel and wavelength.
        let ray = camera.get_ray(x, y, wavelength);

        // And render this camera ray.
        self.render_ray(&ray)
    }

    /// Filss the buffer of mapped photons once.
    pub fn render(&mut self) {
        for &mut mapped_photon in self.mapped_photons.iter() {
            // Pick a wavelength for this photon.
            let wavelength = ::monte_carlo::get_wavelength();

            // Pick a screen coordinate for the photon.
            let x = ::monte_carlo::get_bi_unit();
            let y = ::monte_carlo::get_bi_unit() / self.aspect_ratio;

            // Store the coordinates already.
            mapped_photon.wavelength = wavelength;
            mapped_photon.x = x;
            mapped_photon.y = y;

            // And then trace the scene at this wavelength.
            mapped_photon.probability = self.render_camera_ray(x, y, wavelength);
        }
    }
}
