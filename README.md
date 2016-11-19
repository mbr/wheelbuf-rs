Multi-read no_std ring buffer
=============================

The wheelbuffer crate offers a ringbuffer-like structure without a read pointer, making multiple reads of a buffer possible. The store behind the buffer is flexible and can be a static array, a vector or any other structure that can be converted into a slice.

Documentation is available at [docs.rs](https://docs.rs/wheelbuf).
