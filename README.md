# Memory Reordering Caught in the Act

This is a quick demo of how to break Rust's guarantees in every way. It's a
reimplementation of Preshing's demo of how memory reordering occurs using C++.
See the original article at
http://preshing.com/20120515/memory-reordering-caught-in-the-act/.

This is easy to write in C/C++, but a bit more difficult in Rust because the
language works so hard to stop you. This could have been done with atomics and
setting the memory ordering to `Ordering::Relaxed`, but I wanted to do it
without that. To match the original's semantics:

1. Wrap an `UnsafeCell<u8>` in a tuple struct (simple type) and implement the
   two traits needed to send it between threads without copying,
   `std::marker::Send`and `std::marker::Sync`.
2. Wrap that in an `Arc` so Rust knows who has a reference to it and doesn't
   destroy it, and share cloned `Arc`s with the two competing threads.
3. Use `unsafe` blocks to obtain, deref and modify mutable references to `r{1,
   2}`, `X` and `Y`.

That's a lot of effort to recreate the default semantics of a top-level `int`
declared in a C++ file! Other notes:

1. Used `Barrier` instead of semaphores to synchronise the start/end of the competing
   regions, because semaphores were dropped from Rust `std` in 1.8.0.
2. Used nightly Rust and the `asm!` macro to insert either a memory clobbering to
   prevent compiler reodering, or an `mfence` instruction to prevent memory
   reordering.

How to run this:

1. Use rust nightly for `#![feature(asm)]`.
2. `cargo run` should print out reorderings & total iterations each time
   a reordering is detected.
2. If you want to insert `mfence` and prove that it prevents reordering the
   `*X=1;`/ `*r1 = *Y;` statements, swap in the commented `asm!` in both the
   competing threads. This should prevent any reordering, so you won't see any
   output in `cargo run`, it'll just go forever.
