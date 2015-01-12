// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014-2015 Ruud van Asseldonk
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

use std::cmp::{min, max};
use std::iter::repeat;
use std::num::Float;
use trace_unit::MappedPhoton;
use vector3::Vector3;

/// Handles plotting the result of a `TraceUnit`.
pub struct PlotUnit {
    /// The width of the canvas (in pixels).
    image_width: u32,

    /// The height of the canvas (in pixels).
    image_height: u32,

    /// Width of the canvas divided by its height.
    aspect_ratio: f32,

    /// The buffer of tristimulus values.
    pub tristimulus_buffer: Vec<Vector3>,

    /// An ID for identifying this unit in the UI.
    pub id: usize
}

impl PlotUnit {
    /// Constructs a new plot unit that will plot to a canvas
    /// of the specified size.
    pub fn new(id: usize, width: u32, height: u32) -> PlotUnit {
        let sz = (width * height) as usize;
        PlotUnit {
            image_width: width,
            image_height: height,
            aspect_ratio: width as f32 / height as f32,
            tristimulus_buffer: repeat(Vector3::zero()).take(sz).collect(),
            id: id
        }
    }

    /// Plots a pixel, anti-aliased into the buffer
    /// (adding it to existing content).
    fn plot_pixel(&mut self, x: f32, y: f32, cie: Vector3) {
        // Map the position to pixels.
        let w = self.image_width as isize;
        let h = self.image_height as isize;
        let px = (x * 0.5 + 0.5) * (w as f32 - 1.0);
        let py = (y * self.aspect_ratio * 0.5 + 0.5) * (h as f32 - 1.0);

        // Then map them to discrete pixels.
        let px1 = max(0is, min(w - 1, px.floor() as isize)) as usize;
        let px2 = max(0is, min(w - 1, px.ceil() as isize)) as usize;
        let py1 = max(0is, min(h - 1, py.floor() as isize)) as usize;
        let py2 = max(0is, min(h - 1, py.ceil() as isize)) as usize;

        // Compute pixel coefficients.
        let cx = px - px1 as f32;
        let cy = py - py1 as f32;
        let c11 = (1.0 - cx) * (1.0 - cy);
        let c12 = (1.0 - cx) * cy;
        let c21 = cx * (1.0 - cy);
        let c22 = cx * cy;

        // Then plot the four pixels.
        let buffer = self.tristimulus_buffer.as_mut_slice();
        let w = self.image_width as usize;
        buffer[py1 * w + px1] = buffer[py1 * w + px1] + cie * c11;
        buffer[py1 * w + px2] = buffer[py1 * w + px2] + cie * c21;
        buffer[py2 * w + px1] = buffer[py2 * w + px1] + cie * c12;
        buffer[py2 * w + px2] = buffer[py2 * w + px2] + cie * c22;
    }

    /// Plots the result of the specified TraceUnit onto the canvas.
    pub fn plot(&mut self, photons: &[MappedPhoton]) {
        for ref photon in photons.iter() {
            // Calculate the CIE tristimulus values, given the wavelength.
            let cie = ::cie1931::get_tristimulus(photon.wavelength);

            // Then plot the pixel into the buffer.
            self.plot_pixel(photon.x, photon.y, cie * photon.probability);
        }
    }

    /// Resets the tristimulus buffer to black.
    pub fn clear(&mut self) {
        for x in self.tristimulus_buffer.iter_mut() {
            *x = Vector3::zero();
        }
    }
}
