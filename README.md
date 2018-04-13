Multi-read no_std ring buffer
=============================

The wheelbuffer crate offers a ringbuffer-like structure without a read pointer, making multiple reads of a buffer possible. The store behind the buffer can be a static array, a vector or any other structure that can be converted into a slice.

Its iterator goes through the latest `n` added items, in order, where `n` is the size of the underlying store.

```rust
let mut wheel = wheelbuf::WheelBuf::new(['x'; 3]);

assert!(wheel.is_empty());

wheel.push('a');
wheel.push('b');
wheel.push('c');
wheel.push('d'); // Capacity 3: 'a' gets pushed out.

assert!(wheel.iter().eq(['b', 'c', 'd'].iter()));
```

Documentation is available at [docs.rs](https://docs.rs/wheelbuf).
