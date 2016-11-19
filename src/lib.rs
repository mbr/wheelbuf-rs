#![no_std]

use core::cmp;
use core::convert::AsRef;
use core::marker::PhantomData;
use core::fmt::Write;

#[derive(Debug)]
pub struct WheelBuf<C, I>
    where C: AsMut<[I]> + AsRef<[I]>
{
    /// Backend store
    data: C,

    /// Insert position
    pos: usize,

    /// Total items written
    total: usize,

    _pd: PhantomData<I>,
}

#[derive(Debug)]
pub struct WheelBufIter<'a, C, I>
    where C: AsMut<[I]> + AsRef<[I]>,
          I: 'a,
          C: 'a
{
    buffer: &'a WheelBuf<C, I>,
    cur: usize,
}

impl<C, I> WheelBuf<C, I>
    where C: AsMut<[I]> + AsRef<[I]>
{
    #[inline]
    pub fn new(data: C) -> WheelBuf<C, I> {
        WheelBuf {
            data: data,
            pos: 0,
            total: 0,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.total
    }

    #[inline]
    pub fn push(&mut self, item: I) {
        self.data.as_mut()[self.pos] = item;
        self.total += 1;
        self.pos = (self.pos + 1) % self.data.as_ref().len();
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.as_ref().len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        cmp::min(self.total, self.capacity())
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> WheelBufIter<'a, C, I> {
        WheelBufIter {
            buffer: &self,
            cur: 0,
        }
    }

    #[inline]
    fn read_start(&self) -> usize {
        self.pos - (self.len() % self.capacity())
    }
}

impl<'a, C, I> Iterator for WheelBufIter<'a, C, I>
    where C: AsMut<[I]> + AsRef<[I]>,
          I: 'a,
          C: 'a
{
    type Item = &'a I;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.buffer.len() {
            return None;
        }

        let cur = self.cur;
        self.cur += 1;
        Some(&self.buffer.data.as_ref()[(self.buffer.read_start() + cur) % self.buffer.capacity()])
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let max_idx = cmp::min(self.buffer.total, self.buffer.capacity());

        if n > 0 {
            self.cur += cmp::min(n, max_idx);
        }

        self.next()
    }
}

impl<C> Write for WheelBuf<C, char>
    where C: AsMut<[char]> + AsRef<[char]>
{
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.chars() {
            self.push(c)
        }
        Ok(())
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use core::fmt::Write;
    use std::string::String;
    use super::*;

    #[test]
    fn basics() {
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
    fn nth() {
        let mut buf = ['x'; 8];
        let mut wheel = WheelBuf::new(&mut buf);

        wheel.push('H');
        wheel.push('e');
        wheel.push('l');

        assert_eq!(*wheel.iter().nth(0).unwrap(), 'H');
        assert_eq!(*wheel.iter().nth(1).unwrap(), 'e');
        assert_eq!(*wheel.iter().nth(2).unwrap(), 'l');
        assert!(wheel.iter().nth(3).is_none());
    }

    #[test]
    fn write() {
        let mut buf = ['x'; 8];
        let mut wheel = WheelBuf::new(&mut buf);

        write!(wheel, "Hello, World! {}", 123).unwrap();
        let s: String = wheel.iter().cloned().collect();
        assert_eq!(s.as_str(), "rld! 123");
    }

    #[test]
    fn using_vec() {
        let mut buf = vec!['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'];
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
}
