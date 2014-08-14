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

use trace_unit::MappedPhoton;

/// Handles plotting the result of a `TraceUnit`.
pub struct PlotUnit {
    /// The width of the canvas (in pixels).
    image_width: uint,

    /// The height of the canvas (in pixels).
    image_height: uint,

    /// Width of the canvas divided by its height.
    aspect_ratio: f32,

    /// The buffer of tristimulus values.
    pub tristimulus_buffer: Vec<f32>
}

impl PlotUnit {
    /// Constructs a new plot unit that will plot to a canvas
    /// of the specified size.
    pub fn new(width: uint, height: uint) -> PlotUnit {
        PlotUnit {
            image_width: width,
            image_height: height,
            aspect_ratio: width as f32 / height as f32,
            tristimulus_buffer: Vec::from_elem(width * height * 3, 0.0)
        }
    }

    /// Plots a pixel, anti-aliased into the buffer
    /// (adding it to existing content).
    fn plot_pixel(&mut self, x: f32, y: f32, cie: (f32, f32, f32)) {

    }

    /// Plots the result of the specified TraceUnit onto the canvas.
    pub fn plot(&mut self, photons: &[MappedPhoton]) {

    }

    /// Resets the tristimulus buffer to black.
    pub fn clear(&mut self) {

    }
}
