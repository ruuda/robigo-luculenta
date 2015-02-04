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

use std::cmp::PartialOrd;
use std::iter::{repeat, AdditiveIterator};
use std::num::Float;
use vector3::Vector3;

/// Converts the result of a `GatherUnit` into an sRGB image.
pub struct TonemapUnit {
    /// The width of the canvas (in pixels).
    image_width: u32,

    /// THe height of the canvas (in pixels).
    image_height: u32,

    /// The buffer of sRGB values.
    pub rgb_buffer: Vec<u8>
}

/// Clamps `x` to the interval [0, 1].
fn clamp(x: f32) -> f32 {
    if x.lt(&0.0) { 0.0 }
    else if 1.0f32.lt(&x) { 1.0 }
    else { x }
}

impl TonemapUnit {
    /// Constructs a new tonemap unit that will tonemap a canvas
    /// of the specified size.
    pub fn new(width: u32, height: u32) -> TonemapUnit {
        let sz = (width * height) as usize;
        TonemapUnit {
            image_width: width,
            image_height: height,
            rgb_buffer: repeat(0).take(sz * 3).collect()
        }
    }

    /// Returns an exposure estimate based on the average cieY value.
    /// The returned value is the maximum acceptable intensity, the
    /// intensity that should become (nearly) white.
    fn find_exposure(&self, tristimuli: &[Vector3]) -> f32 {
        let n = (self.image_width * self.image_height) as f32;

        // Compute the average intensity.
        // Calculations are based on the CIE Y value,
        // which corresponds to lightness.
        let mean = tristimuli.iter().map(|cie| cie.y).sum() / n;

        // Then compute the standard deviation.
        let sqr_mean = tristimuli.iter().map(|cie| cie.y * cie.y).sum() / n;
        let variance = sqr_mean - mean * mean;

        // The desired 'white' is one standard deviation above average.
        mean + variance.sqrt()
    }

    /// Converts the unweighted CIE XYZ values in the buffer
    /// to tonemapped sRGB values.
    pub fn tonemap(&mut self, tristimuli: &[Vector3]) {
        let max_intensity = self.find_exposure(tristimuli);
        let buffer = (&mut self.rgb_buffer[]).chunks_mut(3);
        let ln_4 = 4.0f32.ln();

        // Loop through all pixels.
        for (px, cie) in buffer.zip(tristimuli.iter()) {
            // Apply exposure correction.
            let cie = Vector3 {
                x: (cie.x / max_intensity + 1.0).ln() / ln_4,
                y: (cie.y / max_intensity + 1.0).ln() / ln_4,
                z: (cie.z / max_intensity + 1.0).ln() / ln_4
            };

            // Then convert to sRGB.
            let rgb = ::srgb::transform(cie);

            // Clamp colours to saturate.
            let r = clamp(rgb.x);
            let g = clamp(rgb.y);
            let b = clamp(rgb.z);

            // Then convert to integers.
            px[0] = (r * 255.0) as u8;
            px[1] = (g * 255.0) as u8;
            px[2] = (b * 255.0) as u8;
        }
    }
}
