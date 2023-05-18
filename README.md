[![Crates.io](https://img.shields.io/crates/v/daisychain.svg?style=flat-square)](https://crates.io/crates/daisychain)
[![dependency status](https://deps.rs/repo/github/akanalytics/daisychain/status.svg)](https://deps.rs/repo/github/akanalytics/daisychain)
[![Documentation](https://docs.rs/daisychain/badge.svg)](https://docs.rs/daisychain/)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.65.0+-lightgray.svg)](#rust-version-requirements-msrv)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![LICENSE](https://img.shields.io/badge/license-APACHE-lightblue.svg)](LICENSE)




Placeholder crate - `DaisyChain` is a work-in-progress currently, and not quite ready for use!


`DaisyChain` provides a library for parsing unicode text. It aims to have a gentle and intuitive API, without sacrificing performance (it can be zero-copy). Being a library, rather than a framework means that it can be used alongside and complement other parsing toolkits.

## Synopsis

```rust
use daisychain::prelude::*;
use std::str::FromStr;

struct Time {
    hours: u32,
    mins: u32,
}

impl FromStr for Time {
    type Err = daisychain::prelude::ParseError;

    /// eg "09:23" or "23:59"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_cursor, hours, mins) = Cursor::from(s)
            .digits(2..=2)             // matching also sets the selection
            .parse_selection::<u32>()  // daisychain will use u32::FromStr
            .text(":")
            .digits(2..=2)
            .parse_selection()         // often no need to specify type explicitly
            .end_of_stream()           // ensure we are at end-of-string
            .validate()?;
        Ok(Time { hours, mins })
    }
}
```
See [The DaisyChain Cookbook](https://docs.rs/crate/daisychain/latest/source/examples/cookbook/) for more examples

## License

`daisychain` is distributed under the terms of either the MIT license or the
Apache License (Version 2.0)



See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.