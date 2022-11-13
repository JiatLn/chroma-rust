pub fn rgb2hex(color: (u8, u8, u8, f64)) -> String {
    let (r, g, b, _) = color;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn hex2rgb(hex: &str) -> (u8, u8, u8, f64) {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    (r, g, b, 1.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hex() {
        let color = (255, 255, 255, 1.);
        assert_eq!(rgb2hex(color), "#ffffff");
    }

    #[test]
    fn test_hex2rgb() {
        let hex = "#ffffff";
        assert_eq!(hex2rgb(hex), (255, 255, 255, 1.));
    }
}
