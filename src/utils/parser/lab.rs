use std::str::FromStr;

/// Parse a string as a color in the LAB format.
pub fn parse_lab_str(str: &str) -> (f64, f64, f64) {
    let v: Vec<f64> = str
        .trim()
        .replace(" ", "")
        .replace("lab(", "")
        .replace(")", "")
        .split(",")
        .map(|s| f64::from_str(s).unwrap())
        .collect();
    (v[0], v[1], v[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lab_str() {
        let lab = parse_lab_str("lab(0, 0, 0)");
        assert_eq!(lab, (0., 0., 0.));

        let lab = parse_lab_str("lab(100, 0, 0)");
        assert_eq!(lab, (100., 0., 0.));
    }
}
