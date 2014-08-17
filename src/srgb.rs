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

use vector3::Vector3;

/// Applies the sRGB gamma correction to the component.
fn gamma_correct(f: f32) -> f32 {
    if f <= 0.0031308 {
        12.92 * f
    } else {
        1.055 * f.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts a CIE XYZ tristimulus to an sRGB colour.
pub fn transform(cie: Vector3) -> Vector3 {
    // Apply the sRGB matrix.
    let r =  3.2406 * cie.x - 1.5372 * cie.y - 0.4986 * cie.z;
    let g = -0.9689 * cie.x + 1.8758 * cie.y + 0.0415 * cie.z;
    let b =  0.0557 * cie.x - 0.2040 * cie.y + 1.0570 * cie.z;

    // Then do gamma correction.
    Vector3 {
        x: gamma_correct(r),
        y: gamma_correct(g),
        z: gamma_correct(b)
    }
}
