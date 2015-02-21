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

use std::collections::vec_deque::RingBuf;

pub struct PopFrontItems<'a, C> where C: 'a {
    container: &'a mut C
}

pub trait PopFrontIter {
    fn pop_front_iter<'a>(&'a mut self) -> PopFrontItems<'a, Self>;
}

impl<T> PopFrontIter for RingBuf<T> {
    fn pop_front_iter<'a>(&'a mut self) -> PopFrontItems<'a, RingBuf<T>> {
        PopFrontItems {
            container: self
        }
    }
}

impl<'a, T> Iterator for PopFrontItems<'a, RingBuf<T>>
    where T: 'a {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.container.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.container.len(), Some(self.container.len()))
    }
}

#[test]
fn pop_front_iter_ring_buf() {
    let mut xs = RingBuf::new();
    xs.push_back(0u32);
    xs.push_back(1);
    xs.push_back(2);
    xs.push_back(3);
    xs.push_back(4);
    let ys: Vec<u32> = xs.pop_front_iter().take(3).collect();
    assert_eq!(xs[0], 3u32);
    assert_eq!(xs[1], 4u32);
    assert_eq!(&ys, &[0u32, 1, 2]);
}
