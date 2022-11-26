use std::str::FromStr;

/// Parse a string as a color in the cmyk format.
pub fn parse_cmyk_str(str: &str) -> (f64, f64, f64, f64) {
    let v: Vec<f64> = str
        .trim()
        .replace(" ", "")
        .replace("cmyk(", "")
        .replace(")", "")
        .split(",")
        .map(|s| {
            if s.contains('%') {
                f64::from_str(s.replace("%", "").as_str()).unwrap() / 100.
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect();
    (v[0], v[1], v[2], v[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmyk_str() {
        let cmyk = parse_cmyk_str("cmyk(95%, 80%, 0, 0)");
        assert_eq!(cmyk, (0.95, 0.8, 0., 0.));

        let cmyk = parse_cmyk_str("cmyk(0.95, 0.8, 0, 0)");
        assert_eq!(cmyk, (0.95, 0.8, 0., 0.));

        let cmyk = parse_cmyk_str("cmyk(0, 0, 0, 0)");
        assert_eq!(cmyk, (0., 0., 0., 0.));

        let cmyk = parse_cmyk_str("cmyk(0, 0, 0, 1)");
        assert_eq!(cmyk, (0., 0., 0., 1.));
    }
}
