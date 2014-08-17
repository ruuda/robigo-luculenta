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

    }
}
