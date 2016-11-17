#![no_std]

use core::cmp;

#[derive(Debug)]
pub struct WheelBuf<'a, T: 'a> {
    /// Reference to backend store
    data: &'a mut [T],

    /// Insert position
    pos: usize,

    /// Total items written
    total: usize,
}

#[derive(Debug)]
pub struct WheelBufIter<'a, T: 'a> {
    buffer: &'a WheelBuf<'a, T>,
    cur: usize,
}

impl<'a, T> WheelBuf<'a, T> {
    #[inline]
    pub fn new(data: &'a mut [T]) -> WheelBuf<'a, T> {
        WheelBuf {
            data: data,
            pos: 0,
            total: 0,
        }
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.total
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.data[self.pos] = item;
        self.total += 1;
        self.pos = (self.pos + 1) % self.data.len();
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        cmp::min(self.total, self.capacity())
    }

    pub fn iter(&'a self) -> WheelBufIter<'a, T> {
        WheelBufIter {
            buffer: &self,
            cur: 0,
        }
    }

    fn read_start(&self) -> usize {
        self.pos - (self.len() % self.capacity())
    }
}

impl<'a, T> Iterator for WheelBufIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.cur >= self.buffer.len() {
            return None;
        }

        let cur = self.cur;
        self.cur += 1;
        Some(&self.buffer.data[(self.buffer.read_start() + cur) % self.buffer.capacity()])
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let max_idx = cmp::min(self.buffer.total, self.buffer.capacity());

        if n > 0 {
            self.cur += cmp::min(n, max_idx);
        }

        self.next()
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use std::string::String;
    use super::*;

    #[test]
    fn test_basics() {
        let mut buf = ['x'; 8];
        let mut wheel = WheelBuf::new(&mut buf);

        wheel.push('H');
        wheel.push('e');
        wheel.push('l');
        assert_eq!(wheel.len(), 3);
        assert_eq!(*wheel.iter().next().unwrap(), 'H');

        wheel.push('l');
        wheel.push('o');
        wheel.push(' ');
        wheel.push('W');
        wheel.push('o');
        wheel.push('r');
        wheel.push('l');
        wheel.push('d');
        assert_eq!(wheel.len(), 8);

        let s: String = wheel.iter().cloned().collect();
        assert_eq!(s.as_str(), "lo World");
    }

    #[test]
    fn test_nth() {
        let mut buf = ['x'; 8];
        let mut wheel = WheelBuf::new(&mut buf);

        wheel.push('H');
        wheel.push('e');
        wheel.push('l');

        assert_eq!(*wheel.iter().nth(1).unwrap(), 'e');
    }
}
