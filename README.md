# Memory Reordering Caught in the Act

This is a quick demo of how to break Rust's guarantees and show some memory
reordering in action. It's a reimplementation of Preshing's demo of how memory
reordering occurs using C++. See the original article at
http://preshing.com/20120515/memory-reordering-caught-in-the-act/.

Differences:

1. Used `Barrier` instead of semaphores to synchronise the start/end of the competing
   regions, because semaphores were dropped from Rust `std` in 1.8.0. This has
   nothing to do with memory barriers.
2. Used nightly Rust and the `asm!` macro to insert either a memory clobbering to
   prevent compiler reordering, or an `mfence` instruction to prevent memory
   reordering.

How to run this:

1. Use rust nightly for `#![feature(asm)]`.
2. `cargo run` should print out reordering count & total iterations each time
   a reordering is detected.
2. If you want to insert `mfence` and prove that it prevents reordering the
   `*X=1;`/ `*r1 = *Y;` statements, swap in the commented `asm!` in both the
   competing threads. This should prevent any reordering, so you won't see any
   output in `cargo run`, it'll just go forever.
