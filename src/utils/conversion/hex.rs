use crate::Color;

pub fn rgb2hex(color: Color) -> Color {
    if let Color::Rgb(r, g, b) = color {
        let str = format!("#{:02x}{:02x}{:02x}", r, g, b);
        return Color::Hex(str);
    }
    Color::Unknown
}

pub fn hex2rgb(color: Color) -> Color {
    if let Color::Hex(str) = color {
        let r = u8::from_str_radix(&str[1..3], 16).unwrap();
        let g = u8::from_str_radix(&str[3..5], 16).unwrap();
        let b = u8::from_str_radix(&str[5..7], 16).unwrap();
        return Color::Rgb(r, g, b);
    }
    Color::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hex() {
        let color = Color::Rgb(255, 255, 255);
        let color = rgb2hex(color);
        assert_eq!(color, Color::Hex("#ffffff".to_string()));
    }

    #[test]
    fn test_hex2rgb() {
        let color = Color::Hex("#ffffff".to_string());
        let color = hex2rgb(color);
        assert_eq!(color, Color::Rgb(255, 255, 255));
    }
}
