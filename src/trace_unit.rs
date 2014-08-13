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

use scene::Scene;

/// The number of paths to trace in one batch.
// TODO: Use 1024 * 64 for debug, 1024 * 512 in release.
static number_of_paths: i32 = 1024 * 64;

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

/// Handles ray tracing.
pub struct TraceUnit<'a> {
    /// The scene that will be rendered.
    scene: &'a Scene<'a>,

    /// The aspect ratio of the image that will be rendered.
    aspect_ratio: f32,

    /// THe photons that were rendered.
    pub mapped_photons: [MappedPhoton, ..number_of_paths]
}
