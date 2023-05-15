use chainsaw::prelude::*;

/// cargo test --example intro
///     
/// RUST_LOG=trace cargo test --example intro -- --nocapture
///
///

fn main() {
    let _ = Some(" Hello").ws().text("He");
}

#[cfg(test)]
mod tests {
    use chainsaw::prelude::*;
    use test_log::test;

    #[test]
    fn test_intro() {
        // whitespace - 0zero or more whitespace characters
        assert_eq!(Some(" Hello").ws(), Some("Hello"));
        assert_eq!(Some("\nHello").ws(), Some("Hello"));
        assert_eq!(None.ws(), None);

        // horizontal whitespace doesn't match a '\n'
        assert_eq!(Some(" Hello").hws(), Some("Hello"));
        assert_eq!(Some("\nHello").hws(), Some("\nHello"));
        assert_eq!(None.hws(), None);

        // match end-of-string/end-of-stream
        assert_eq!(Some("Hello").is_eos(), None);
        assert_eq!(Some("").is_eos(), Some(""));
        assert_eq!(None.is_eos(), None);

        // match end-of-line (end-of-stream is treated as eol too!)
        assert_eq!(Some("\nHello").is_eol(), Some("Hello"));
        assert_eq!(Some("\n\nHello").is_eol(), Some("\nHello"));
        assert_eq!(Some("\r\nHello").is_eol(), Some("Hello"));
        assert_eq!(Some("").is_eol(), Some(""));
        assert_eq!(None.is_eol(), None);

        // match by character
        let chars: Vec<_> = "Hle".chars().collect();
        assert_eq!(Some("Hello").chars_in(1.., chars.as_slice()), Some("o"));
        assert_eq!(Some("Hello").chars_in(1.., &['H', 'l', 'e']), Some("o"));
        assert_eq!(
            Some("Hello").chars_match(1.., |c| c.is_uppercase()),
            Some("ello")
        );

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
