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

use std::io::{File, Open, Write};
use std::mem::transmute;
use std::slice::bytes::copy_memory;
use vector3::Vector3;

pub struct GatherUnit {
    /// The buffer of tristimulus values.
    pub tristimulus_buffer: Vec<Vector3>,

    /// A buffer that contains compensation for rounding errors in summing.
    compensation_buffer: Vec<Vector3>
}

impl GatherUnit {
    /// Constructs a new GatherUnit that will gather a canvas
    /// of the specified size.
    pub fn new(width: uint, height: uint) -> GatherUnit {
        let mut unit = GatherUnit {
            tristimulus_buffer: Vec::from_elem(width * height, Vector3::zero()),
            compensation_buffer: Vec::from_elem(width * height, Vector3::zero())
        };

        // Try to continue a previous render.
        unit.read();

        unit
    }

    /// Add the results of the PlotUnit to the canvas.
    pub fn accumulate(&mut self, tristimuli: &[Vector3]) {
        let accs = self.tristimulus_buffer.mut_iter();
        let comps = self.compensation_buffer.mut_iter();
        let pixels = tristimuli.iter();

        // Loop through all the pixels, and add the values.
        for ((comp, acc), px) in comps.zip(accs).zip(pixels) {
            // What we want to add, is the real value to add (px),
            // minus compensation for previous errors.
            let extra = *px - *comp;
            let sum = *acc + extra;
            // The new compensation is the error in the accumulation.
            *comp = (sum - *acc) - extra;
            *acc = sum;
        }
    }

    /// Saves the tristimulus buffer to a file, so that rendering
    /// can be resumed later.
    pub fn save(&self) {
        let mut file = File::open_mode(&Path::new("buffer.raw"), Open, Write);
        let mut data = self.tristimulus_buffer.iter()
                           .chain(self.compensation_buffer.iter());
        for trist in data {
            let xyz: &[u8, ..12] = unsafe { transmute(trist) };
            file.write(xyz.as_slice()).ok().expect("failed to write raw buffer");
        }
    }

    /// Reads the tristimulus buffer from a file, to resume rendering.
    fn read(&mut self) {
        match File::open(&Path::new("buffer.raw")) {
            Ok(ref mut file) => {
                for trist in self.tristimulus_buffer.mut_iter() {
                    let xyz: &mut [u8, ..12] = &mut [0, ..12];
                    file.read(xyz.as_mut_slice()).ok().expect("failed to read raw buffer");
                    let trist: &mut [u8, ..12] = unsafe { transmute(trist) };
                    copy_memory(trist.as_mut_slice(), xyz.as_slice());
                }
            },
            Err(_) => { }
        }
    }

}
