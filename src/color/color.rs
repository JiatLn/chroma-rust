use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
    Hex(String),
    Hsl(u8, u8, u8),
    Rgba(u8, u8, u8, f64),
    Unknown,
}

impl From<&str> for Color {
    fn from(str: &str) -> Self {
        match str {
            str if str.starts_with("#") => Self::Hex(str.to_string()),
            str if str.starts_with("rgba") => Color::parse_rgba(str),
            str if str.starts_with("rgb") => Color::parse_rgb(str),
            str if str.starts_with("hsv") => todo!(),
            str if str.starts_with("hsl") => todo!(),
            _ => Self::Unknown,
        }
    }
}

impl Color {
    fn parse_rgb(str: &str) -> Self {
        let v_u8: Vec<u8> = str
            .trim()
            .replace(" ", "")
            .replace("rgb(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Self::Rgb(v_u8[0], v_u8[1], v_u8[2])
    }
    fn parse_rgba(str: &str) -> Self {
        let v: Vec<String> = str
            .trim()
            .replace(" ", "")
            .replace("rgba(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.to_string())
            .collect();
        let (r, g, b) = (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        );
        let alpha = f64::from_str(v[3].as_str()).unwrap();
        Self::Rgba(r, g, b, alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        let hex_color = Color::from("#abcdef");
        assert_eq!(hex_color, Color::Hex(String::from("#abcdef")));

        let rgb_color = Color::from("rgb(255, 255, 255)");
        assert_eq!(rgb_color, Color::Rgb(255, 255, 255));

        let rgba_color = Color::from("rgba(255, 255, 255, 0.6)");
        assert_eq!(rgba_color, Color::Rgba(255, 255, 255, 0.6));
    }
}
