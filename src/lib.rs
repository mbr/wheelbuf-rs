// #![no_std]

extern crate core;

use core::cmp;

pub struct WheelBuf<'a, T: 'a> {
    /// Reference to backend store
    data: &'a mut [T],

    /// Insert position
    pos: usize,

    /// Total items written
    total: usize,
}

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
}

impl<'a, T> Iterator for WheelBufIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.cur >= self.buffer.total || self.cur >= self.buffer.capacity() {
            return None;
        }

        let rv = Some(&self.buffer.data[(self.buffer.pos + self.cur) % self.buffer.capacity()]);
        self.cur += 1;
        rv
    }
}


#[cfg(test)]
mod tests {
    use std;
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
}
