use std::str::FromStr;

/// Parse a string as a color in the RGB format.
pub fn parse_rgb_str(str: &str) -> (u8, u8, u8, f64) {
    let v_u8: Vec<u8> = str
        .trim()
        .replace(" ", "")
        .replace("rgb(", "")
        .replace(")", "")
        .split(",")
        .map(|s| s.parse::<f64>().unwrap().round() as u8)
        .collect();
    (v_u8[0], v_u8[1], v_u8[2], 1.)
}

/// Parse a string as a color in the RGBA format.
pub fn parse_rgba_str(str: &str) -> (u8, u8, u8, f64) {
    let v: Vec<String> = str
        .trim()
        .replace(" ", "")
        .replace("rgba(", "")
        .replace(")", "")
        .split(",")
        .map(|s| s.to_string())
        .collect();
    let (r, g, b) = (
        v[0].parse::<f64>().unwrap().round() as u8,
        v[1].parse::<f64>().unwrap().round() as u8,
        v[2].parse::<f64>().unwrap().round() as u8,
    );
    let alpha = f64::from_str(v[3].as_str()).unwrap();
    (r, g, b, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rgb_str() {
        let rgb = parse_rgb_str("rgb(0, 0, 0)");
        assert_eq!(rgb, (0, 0, 0, 1.));

        let rgb = parse_rgb_str("rgb(255, 255, 255)");
        assert_eq!(rgb, (255, 255, 255, 1.));

        let rgb = parse_rgb_str("rgb(254, 255, 255, 0.5)");
        assert_eq!(rgb, (254, 255, 255, 1.));
    }

    #[test]
    fn test_parse_rgba_str() {
        let rgba = parse_rgba_str("rgba(0, 0, 0, 0)");
        assert_eq!(rgba, (0, 0, 0, 0.));

        let rgba = parse_rgba_str("rgba(255, 255, 255, 1)");
        assert_eq!(rgba, (255, 255, 255, 1.));

        let rgba = parse_rgba_str("rgba(255, 255, 255, 0.5)");
        assert_eq!(rgba, (255, 255, 255, 0.5));
    }
}
