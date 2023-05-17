[![dependency status](https://deps.rs/repo/github/akanalytics/kateno/status.svg)](https://deps.rs/repo/github/akanalytics/kateno)

`kateno` provides a library for parsing unicode text. It aims to have a gentle and intuitive API, without sacrificing performance (it can be zero-copy). Being a library, rather than a framework means that it can be used alongside and complement other parsing toolkits.

## Synopsis

```rust
use kateno::prelude::*;
use std::str::FromStr;

struct Time {
    hours: u32,
    mins: u32,
}

impl FromStr for Time {
    type Err = kateno::prelude::ParseError;

    /// eg "09:23" or "23:59"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_cursor, hours, mins) = Cursor::from(s)
            .digits(2..=2)             // matching also sets the selection
            .parse_selection::<u32>()  // kateno will use u32::FromStr
            .text(":")
            .digits(2..=2)
            .parse_selection()         // often no need to specify type explicitly
            .end_of_stream()           // ensure we are at end-of-string
            .validate()?;
        Ok(Time { hours, mins })
    }
}
```

## License

`kateno` is distributed under the terms of either the MIT license or the
Apache License (Version 2.0)



See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.