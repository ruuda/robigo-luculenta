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

pub struct Ray {
    /// The 'position' of the ray.
    pub origin: Vector3,

    /// The normalised direction in which the ray is pointing.
    pub direction: Vector3,

    /// The wavelength of the light ray in nm (so in the range 380-780).
    pub wavelength: f32,

    /// The probability that a photon followed this light path. Note that
    /// this can also be compensated for, if the probability of the ray being
    /// generated is not uniform.
    pub probability: f32
}
