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

use std::collections::Deque;

pub struct PopFrontItems<'a, C> where C: 'a {
    container: &'a mut C
}

pub trait PopFrontIter {
    fn pop_front_iter<'a>(&'a mut self) -> PopFrontItems<'a, Self>;
}

impl<'a, T, C> Iterator<T> for PopFrontItems<'a, C>
    where C: PopFrontIter + Collection + Deque<T> {
    fn next(&mut self) -> Option<T> {
        self.container.pop_front()
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (self.container.len(), Some(self.container.len()))
    }
}

impl<T, C> PopFrontIter for C where C: Collection + Deque<T> {
    fn pop_front_iter<'a>(&'a mut self) -> PopFrontItems<'a, C> {
        PopFrontItems {
            container: self
        }
    }
}

#[test]
fn pop_front_iter_ring_buf() {
    use std::collections::RingBuf;

    let mut xs = RingBuf::new();
    xs.push(0u); xs.push(1); xs.push(2); xs.push(3); xs.push(4);
    let ys: Vec<uint> = xs.pop_front_iter().take(3).collect();
    assert_eq!(xs[0], 3u);
    assert_eq!(xs[1], 4u);
    assert_eq!(ys[], [0u, 1, 2][]);
}

pub struct PopItems<'a, C> where C: 'a {
    container: &'a mut C
}

pub trait PopIter {
    fn pop_iter<'a>(&'a mut self) -> PopItems<'a, Self>;
}

impl<'a, T, C> Iterator<T> for PopItems<'a, C>
    where C: PopIter + Collection + MutableSeq<T> {
    fn next(&mut self) -> Option<T> {
        self.container.pop()
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (self.container.len(), Some(self.container.len()))
    }
}

impl<T, C> PopIter for C where C: Collection + MutableSeq<T> {
    fn pop_iter<'a>(&'a mut self) -> PopItems<'a, C> {
        PopItems {
            container: self
        }
    }
}

#[test]
fn pop_iter_vec() {
    let mut xs = vec!(0u, 1, 2, 3, 4);
    let ys: Vec<uint> = xs.pop_iter().take(3).collect();
    assert_eq!(xs[], [0u, 1][]);
    assert_eq!(ys[], [4u, 3, 2][]);
}

#[test]
fn pop_iter_ring_buf() {
    use std::collections::RingBuf;

    let mut xs = RingBuf::new();
    xs.push(0u); xs.push(1); xs.push(2); xs.push(3); xs.push(4);
    let ys: Vec<uint> = xs.pop_iter().take(3).collect();
    assert_eq!(xs[0], 0u);
    assert_eq!(xs[1], 1u);
    assert_eq!(ys[], [4u, 3, 2][]);
}
