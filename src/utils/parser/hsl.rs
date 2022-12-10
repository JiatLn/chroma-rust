use std::str::FromStr;

/// Parse a string as a color in the HSL format.
pub fn parse_hsl_str(str: &str) -> (f64, f64, f64) {
    let v: Vec<f64> = str
        .trim()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsl(", "")
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
    (v[0], v[1], v[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsl_str() {
        let hsl = parse_hsl_str("hsl(0, 0%, 0%)");
        assert_eq!(hsl, (0., 0., 0.));

        let hsl = parse_hsl_str("hsl(0, 0, 0)");
        assert_eq!(hsl, (0., 0., 0.));

        let hsl = parse_hsl_str("hsl(0, 100%, 100%)");
        assert_eq!(hsl, (0., 1., 1.));

        let hsl = parse_hsl_str("hsl(0, 100, 100)");
        assert_eq!(hsl, (0., 100., 100.));
    }
}
