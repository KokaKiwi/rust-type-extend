type-extend
===========

This rustc plugin add the `extend!` macro and allow to extend Rust type with traits easily.

:warning: As rustc plugins are unstable, you need a nightly version of the Rust toolchain, you can use multirust with `multirust override nightly-2016-02-17` (You need a specific version as the `syntax` library change often)

Usage
-----

Add the plugin in `Cargo.toml`:

```toml
[dependencies]
type-extend = { git = "<This git URL>" }
```

Use the plugin in your crate:

```rust
#![feature(plugin)]
#![plugin(type_extend)]
```

Extend the type you want

```rust
extend! {
    pub impl<T: ::std::fmt::Debug> VecExt<T> for Vec<T> {
        fn print(&self) {
            println!("{:?}", self);
        }
    }
}
```

expand to

```rust
pub trait VecExt<T: ::std::fmt::Debug> {
    fn print(&self);
}

impl<T: ::std::fmt::Debug> VecExt<T> for Vec<T> {
    fn print(&self) {
        println!("{:?}", self);
    }
}
```

Example
-------

You can find an example in the `sample` directory.
