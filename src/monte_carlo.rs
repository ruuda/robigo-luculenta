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

use std::rand;
use std::rand::Closed01;

// Note that it is safe to just use rand::random: it uses a task-local rng.

/// Returns a random number in the range [0, 1].
pub fn get_unit() -> f32 {
    let Closed01(x) = rand::random::<Closed01<f32>>();
    x
}

/// Returns a random number in the range [-1, 1].
pub fn get_bi_unit() -> f32 {
    get_unit() * 2.0 - 1.0
}

/// Returns a random number in the range [0, 2pi).
pub fn  get_longitude() -> f32 {
    rand::random::<f32>() * Float::two_pi()
}

/// Returns a random number in the range [-pi/2, pi/2].
pub fn get_latitude() -> f32 {
    get_bi_unit() * Float::frac_pi_2()
}

/// Returns a random number in the range [-380, 780].
pub fn get_wavelength() -> f32 {
    get_unit() * 400.0 + 380.0
}
