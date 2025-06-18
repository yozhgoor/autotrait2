# autotrait2

Same as the [`autotrait`][https://github.com/bearcove/autotrait] crate.

Use this crate if the first one didn't work.

## Usage

This crate allow you to define traits without repeating yourself with a `trait` block.

So you only need the following:
```rust,ignore
struct Stuff;

#[autotrait2::autotrait]
impl MyTrait for Stuff {
    fn a_trait_function(&self) -> String {
        // ...
    }
}
```
