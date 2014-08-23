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

use geometry::Surface;
use material::{Material, EmissiveMaterial};

pub enum MaterialBox {
    Reflective(Box<Material + Sync + Send>),
    Emissive(Box<EmissiveMaterial + Sync + Send>)
}

/// Represents a surface with a material.
pub struct Object {
    /// The surface that defines the geometry of the object.
    pub surface: Box<Surface + Sync + Send>,
    /// Either an emissive or a reflective material.
    pub material: MaterialBox
}

impl Object {
    /// Creates an object with the specified `surface` and `material`.
    pub fn new(surface: Box<Surface + Sync + Send>,
               material: MaterialBox)
               -> Object {
        Object {
            surface: surface,
            material: material
        }
    }
}
