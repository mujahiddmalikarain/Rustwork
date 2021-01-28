/*
    Borrowing and Ownership in Rust
*/

/*
  Stack
  - Extremely Fast
  - Values must have Fixed Sizes
  - Always puts data in on Top
  - data pushed down as new data comes in
*/

/*
  Heap
  - Less Organized and Slower
  - Accepts Dynamicly Sized Data or Data that can Grow
  - Returns a Pointer which goes on the stack
  - Pointer points to where the data is on the Heap
*/

/*
Ownership Rules:

1) Each value has a variable which is its owner.
2) There can only be one owner at any given time.
3) When the owner goes out of scope, the value will be dropped out of memory.
*/

/*
Borrowing Rules:
    1) Allowed infinite borrows for readonly access.
    2) Readonly borrows make the original data immutable for their duration.
    3) Only allowed to pass one borrow at a time for write access/mutability.
*/

/* Rust */Stack | Copy Types
    bool
    character
    numbers
    slices
    fix sized arrays
    tuples containing primitives
    function pointers
