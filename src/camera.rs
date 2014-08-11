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

use quaternion::Quaternion;
use ray::Ray;
use vector3::Vector3;

pub struct Camera {
    /// Location of the camera in the scene.
    pub position: Vector3,

    /// Horizontal field of view, in radians.
    pub field_of_view: f32,

    /// The distance along the optical axis that is perfectly in focus.
    pub focal_distance: f32,

    /// The amount of chromatic abberation. 0 indicates no chromatic
    /// abberation, larger values result in more chromatic abberation.
    pub chromatic_abberation: f32,

    /// The direction in which the camera is looking.
    pub orientation: Quaternion
}

impl Camera {
    
    fn get_screen_ray(&self,
                      x: f32,
                      y: f32,
                      chromatic_abberation_factor: f32,
                      dof_angle: f32,
                      dof_radius: f32)
                      -> Ray {
        // The smaller the FOV, the further the screen is away;
        // the larger the FOV, the closer the screen is.
        let screen_distance = 1.0 / (self.field_of_view * 0.5).tan();
        
        // Then apply some wavelength dependent zoom to create chromatic
        // abberation. Please note, this is not a physically correct model of
        // chromatic abberation, for a correct response, you can place a lens
        // in front of the camera, with a dispersive glass material.
        let xs = x * chromatic_abberation_factor;
        let ys = y * chromatic_abberation_factor;

        let direction = Vector3::new(xs, screen_distance, -ys).normalise();

        // Now find the intersection with the focal plane (which is trivial as
        // long as the ray has not been transformed yet).
        let focus_point = direction * (self.focal_distance / direction.y);

        // Then take a new point on the camera 'lens' (this is of course not
        // accurate, but then again, the pinhole camera does not have depth of
        // field at all, so it is a hack anyway).
        let lens_point = Vector3 {
            x: dof_angle.cos() * dof_radius,
            y: 0.0,
            z: dof_angle.sin() * dof_radius
        };

        // Then construct the new ray, from the lens point,
        // through the focus point.
        Ray {
            origin: self.position + lens_point.rotate(self.orientation),
            direction: (focus_point - lens_point)
                .normalise()
                .rotate(self.orientation),
            wavelength: 0.0,
            probability: 1.0
        }
    }
}       
