# Iterator Investigation
Iterator in Rust allows us to go through each value as an immutable reference. In rust, we called it as `Iterator Trait` since it is an *An interface for dealing with iterators.* (according to the Rust-lang).
Iterator traits comes with multiple methods and this document explains how to use it and when to use it in the right circumstance. 

## iter, into_iter, iter_mut... What is the diference? 

### iter
Iter() provides iteration over immutable references (&T); it always returns an iterator which generates shared references to its items.

### IntoIterator ( or into_iter)
We use this when you need to specify how a specific type has to be converted into an iterator. 


### iter_mut.