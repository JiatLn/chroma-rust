use std::str::FromStr;

/// Parse a string as a color in the HSV format.
pub fn parse_hsv_str(str: &str) -> (f64, f64, f64) {
    let v: Vec<f64> = str
        .trim()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsv(", "")
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
    fn test_parse_hsv_str() {
        let hsv = parse_hsv_str("hsv(0, 0%, 0%)");
        assert_eq!(hsv, (0., 0., 0.));

        let hsv = parse_hsv_str("hsv(0, 0, 0)");
        assert_eq!(hsv, (0., 0., 0.));

        let hsv = parse_hsv_str("hsv(300°, 100%, 100%)");
        assert_eq!(hsv, (300., 1., 1.));

        let hsv = parse_hsv_str("hsv(0, 100, 100)");
        assert_eq!(hsv, (0., 100., 100.));
    }
}
