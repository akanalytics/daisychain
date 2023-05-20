
//! running examples
//!
//! cargo test --example intro
//!     
//! RUST_LOG=trace cargo test --example intro -- --nocapture
//!
//!
//! add to Cargo.toml
//! 
//! [dev-dependencies]
//! env_logger = "0.9"
//! test-log = {version = "0.2"}
//! 

//! 
//!```rust
//! assert_eq!(Some(" Hello").ws(), Some("Hello"));
//! assert_eq!(Some("\nHello").ws(), Some("Hello"));
//! assert_eq!(None.ws(), None);
//!```
//!
#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use test_log::test;

    ///
    /// 
    /// 
    /// Option<&str> is a simple cursor, offering matching functions but no
    /// selection. Handy for unit testing.
    /// None indicates that no match mas made
    ///
    #[test]
    fn test_intro() {
        // whitespace - zero or more whitespace characters
        assert_eq!(Some(" Hello").ws(), Some("Hello"));
        assert_eq!(Some("\nHello").ws(), Some("Hello"));
        assert_eq!(None.ws(), None);

        // horizontal whitespace doesn't match a '\n'
        assert_eq!(Some(" Hello").hws(), Some("Hello"));
        assert_eq!(Some("\nHello").hws(), Some("\nHello"));
        assert_eq!(None.hws(), None);

        // match end-of-string/end-of-stream
        assert_eq!(Some("Hello").end_of_stream(), None);
        assert_eq!(Some("").end_of_stream(), Some(""));
        assert_eq!(None.end_of_stream(), None);

        // match end-of-line (end-of-stream is treated as eol too!)
        assert_eq!(Some("Hello").end_of_line(), None);
        assert_eq!(Some("\nHello").end_of_line(), Some("Hello"));
        assert_eq!(Some("\n\nHello").end_of_line(), Some("\nHello"));
        assert_eq!(Some("\r\nHello").end_of_line(), Some("Hello"));
        assert_eq!(Some("").end_of_line(), Some(""));
        assert_eq!(None.end_of_line(), None);

        // match by character
        let chars: Vec<_> = "Hle".chars().collect();
        assert_eq!(Some("Hello").chars_in(1.., chars.as_slice()), Some("o"));
        assert_eq!(Some("Hello").chars_in(1.., &['H', 'l', 'e']), Some("o"));
        assert_eq!(
            Some("Hello").chars_match(1.., |c| c.is_uppercase()),
            Some("ello")
        );

        assert_eq!(Some("Hello").chars_in(1.., &['H', 'e']), Some("llo"));


        // text - match a word
        // text-alt match one of series of words
        assert_eq!(Some("Hello").text("He"), Some("llo"));
        assert_eq!(Some("Hello").text("He"), Some("llo"));
        assert_eq!(Some("Hello").text("Bye"), None);
        assert_eq!(Some("Hello").text_alt(&[""]), Some("Hello"));
        assert_eq!(Some("Hello").text_alt(&["Bye", "He"]), Some("llo"));
        assert_eq!(Some("Hello").text_alt(&["He", "Hello"]), Some("llo"));
        assert_eq!(Some("Hello").text_alt(&["Hello", "He"]), Some(""));
        assert_eq!(Some("Hello").text_alt(&["Bye1", "Bye2"]), None);

        // finds (locate start of the needle being searched for)
        assert_eq!(Some("Hello").find("llo"), Some("llo"));
        assert_eq!(Some("Hello").find("Bye"), None);
        assert_eq!(Some("Hello").find(""), Some("Hello"));
        assert_eq!(Some("Hello").find("eH"), None);

        // scans (locates after the end of the needle)
        assert_eq!(Some("Hello").scan_text("llo"), Some(""));
        assert_eq!(Some("Hello").scan_text("Bye"), None);
        assert_eq!(Some("Hello").scan_text(""), Some("Hello"));
        assert_eq!(Some("Hello").scan_text("eH"), None);
        assert_eq!(Some("Hello").scan_text("Hel"), Some("lo"));

        assert_eq!(Some("Hello\nWorld").scan_eol(), Some("World"));
        assert_eq!(Some("Hello").scan_eol(), Some(""));
        assert_eq!(Some("Hello\n\n").scan_eol(), Some("\n"));
        assert_eq!(Some("Hello\r\nWorld").scan_eol(), Some("World"));
    }
}
