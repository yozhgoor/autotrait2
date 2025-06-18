# autotrait2

[![actions status][actions-badge]][actions-url]
[![crate version][crates-version-badge]][crates-url]
[![dependencies status][deps-badge]][deps-url]
![licenses][licenses-badge]

Same as the [`autotrait`](https://github.com/bearcove/autotrait) crate.

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

[actions-badge]: https://github.com/yozhgoor/autotrait2/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/yozhgoor/autotrait2/actions
[crates-version-badge]: https://img.shields.io/crates/v/autotrait2
[crates-url]: https://crates.io/crates/autotrait2
[deps-badge]: https://deps.rs/repo/github/yozhgoor/autotrait2/status.svg
[deps-url]: https://deps.rs/crate/autotrait2
[licenses-badge]: https://img.shields.io/crates/l/autotrait2
