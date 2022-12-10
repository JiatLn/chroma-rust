/// Checking if a color can be parsed by chroma-rust
///
/// you can use `chroma_rust::valid` to try if a color argument can be correctly parsed as color by `chroma_rust`.
///
/// ```rust
/// chroma_rust::valid("red");
/// chroma_rust::valid("bread");
/// chroma_rust::valid("#F0000D");
/// chroma_rust::valid("#FOOOOD");
/// ```
pub fn valid(str: &str) -> bool {
    let valid = match str {
        str if str.starts_with("#") => {
            if str.len() == 4 || str.len() == 7 {
                str.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            } else {
                false
            }
        }
        // TODO: check if rgb/rgba/hsl/hsla/hsv/hsva are valid
        str if str.starts_with("rgba") => true,
        str if str.starts_with("rgb") => true,
        str if str.starts_with("lab") => true,
        str if str.starts_with("hsl") => true,
        str if str.starts_with("hsv") => true,
        str if str.starts_with("cmyk") => true,
        _ => match crate::W3CX11.get(str) {
            Some(_) => true,
            None => false,
        },
    };
    valid
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_hex() {
        assert!(valid("#abcdef"));
    }

    #[test]
    fn test_valid_hex_short() {
        assert!(valid("#abc"));
    }

    #[test]
    fn test_valid_hex_invalid() {
        assert!(!valid("#FOOOOD"));
    }

    #[test]
    fn test_valid_hex_invalid_length() {
        assert!(!valid("#abcde"));
    }

    #[test]
    fn test_valid_rgb() {
        assert!(valid("rgb(255, 255, 255)"));
    }

    #[test]
    fn test_valid_rgba() {
        assert!(valid("rgba(255, 255, 255, 0.6)"));
    }

    #[test]
    fn test_valid_lab() {
        assert!(valid("lab(100, 0, 0)"));
    }

    #[test]
    fn test_valid_name() {
        assert!(valid("mediumspringgreen"));
    }

    #[test]
    fn test_invalid() {
        assert!(!valid("invalid"));
    }
}
