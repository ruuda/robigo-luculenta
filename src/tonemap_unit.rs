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

use std::cmp::PartialOrd;
use std::iter::AdditiveIterator;
use vector3::Vector3;

/// Converts the result of a `GatherUnit` into an sRGB image.
pub struct TonemapUnit {
    /// The width of the canvas (in pixels).
    image_width: uint,

    /// THe height of the canvas (in pixels).
    image_height: uint,

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
    pub fn new(width: uint, height: uint) -> TonemapUnit {
        TonemapUnit {
            image_width: width,
            image_height: height,
            rgb_buffer: Vec::from_elem(width * height * 3, 0)
        }
    }

    /// Returns an exposure estimate based on the average cieY value.
    /// The returned value is the maximum acceptable intensity, the
    /// intensity that should become (nearly) white.
    fn find_exposure(&self, tristimuli: &[Vector3]) -> f32 {
        let mut intensities = tristimuli.iter()
        // Iterate over triplets of CIE XYZ values.
        // Calculations are based on the CIE Y value (which corresponds
        // to lightness), but X and Z are also taken into account slightly
        // to avoid weird situations.
        .map(|cie| { cie.x + cie.y * 5.0 + cie.z });

        // Compute the average intensity. Divide by 7 to compensate
        // for the coefficients above.
        let average = intensities.sum() /
        (self.image_width as f32 * self.image_height as f32 * 7.0);

        // TODO: I have not found a good way to re-use an iterator.
        intensities = tristimuli.iter()
        .map(|cie| { cie.x + cie.y * 5.0 + cie.z });

        // Then compute the standard deviation. Divide by 49 = 7 * 7
        // to compensate for the coefficients above.
        let variance = intensities.by_ref().map(|i| { i * i }).sum() /
        (self.image_width as f32 * self.image_height as f32 * 49.0) -
        (average * average);
        let standard_deviation = variance.sqrt();

        // The desired 'white' is one standard deviation above average.
        average + standard_deviation
    }

    /// Converts the unweighted CIE XYZ values in the buffer
    /// to tonemapped sRGB values.
    pub fn tonemap(&mut self, tristimuli: &[Vector3]) {
        let max_intensity = self.find_exposure(tristimuli);
        let buffer = self.rgb_buffer.as_mut_slice().mut_chunks(3);
        let pixels = tristimuli.iter();
        let ln_4 = 4.0f32.ln();

        // Loop through all pixels.
        for (buf, px) in buffer.zip(pixels) {
            // Apply exposure correction.
            let cie = Vector3 {
                x: (px.x / max_intensity + 1.0).ln() / ln_4,
                y: (px.y / max_intensity + 1.0).ln() / ln_4,
                z: (px.z / max_intensity + 1.0).ln() / ln_4
            };

            // Then convert to sRGB.
            let rgb = ::srgb::transform(cie);

            // Clamp colours to saturate.
            let r = clamp(rgb.x);
            let g = clamp(rgb.y);
            let b = clamp(rgb.z);

            // Then convert to integers.
            buf[0] = (r * 255.0) as u8;
            buf[1] = (g * 255.0) as u8;
            buf[2] = (b * 255.0) as u8;
        }
    }
}
