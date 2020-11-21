# easy-ext

[![crates.io](https://img.shields.io/crates/v/easy-ext.svg?style=flat-square&logo=rust)](https://crates.io/crates/easy-ext)
[![docs.rs](https://img.shields.io/badge/docs.rs-easy--ext-blue?style=flat-square)](https://docs.rs/easy-ext)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/easy-ext/CI/master?style=flat-square)](https://github.com/taiki-e/easy-ext/actions?query=workflow%3ACI+branch%3Amaster)

An attribute macro for easily writing [extension trait pattern](https://github.com/rust-lang/rfcs/blob/master/text/0445-extension-trait-conventions.md).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easy-ext = "0.2"
```

*Compiler support: requires rustc 1.31+*

## Examples

```rust
use easy_ext::ext;

#[ext(ResultExt)]
impl<T, E> Result<T, E> {
    pub fn err_into<U>(self) -> Result<T, U>
    where
        E: Into<U>,
    {
        self.map_err(Into::into)
    }
}
```

Code like this will be generated:

```rust
pub trait ResultExt<T, E> {
    fn err_into<U>(self) -> Result<T, U>
    where
        E: Into<U>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn err_into<U>(self) -> Result<T, U>
    where
        E: Into<U>,
    {
        self.map_err(Into::into)
    }
}
```

You can elide the trait name. Note that in this case, `#[ext]` assigns a random name, so you cannot import/export the generated trait.

```rust
use easy_ext::ext;

#[ext]
impl<T, E> Result<T, E> {
    fn err_into<U>(self) -> Result<T, U>
    where
        E: Into<U>,
    {
        self.map_err(Into::into)
    }
}
```

### Visibility

The visibility of the generated extension trait inherits the visibility of the item in the original `impl`.

For example, if the method is `pub` then the trait will also be `pub`:

```rust
use easy_ext::ext;

#[ext(ResultExt)] // generate `pub trait ResultExt`
impl<T, E> Result<T, E> {
    pub fn err_into<U>(self) -> Result<T, U>
    where
        E: Into<U>,
    {
        self.map_err(Into::into)
    }
}
```

See [documentation](https://docs.rs/easy-ext) for more details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
