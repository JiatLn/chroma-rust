/// checking if a color can be parsed by chroma-rust
pub fn valid(str: &str) -> bool {
    let valid = match str {
        str if str.starts_with("#") => true,
        str if str.starts_with("rgba") => true,
        str if str.starts_with("rgb") => true,
        str if str.starts_with("lab") => true,
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
