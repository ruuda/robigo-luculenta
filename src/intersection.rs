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

use vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Intersection {
    /// The position at which the intersection occurred.
    pub position: Vector3,

    /// The surface normal at the intersection.
    pub normal: Vector3,

    /// The surface tangent at the intersection.
    pub tangent: Vector3,

    /// The distance between the intersection point and the ray origin.
    pub distance: f32
}
