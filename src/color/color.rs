use crate::utils::conversion;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
    Hex(String),
    Hsl(u8, u8, u8),
    Rgba(u8, u8, u8, f64),
    Lab(f64, f64, f64),
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

impl IntoIterator for Color {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Rgb(r, g, b) => vec![r as f64, g as f64, b as f64].into_iter(),
            Self::Hsv(_, _, _) => todo!(),
            Self::Hex(_) => todo!(),
            Self::Hsl(_, _, _) => todo!(),
            Self::Rgba(r, g, b, a) => vec![r as f64, g as f64, b as f64, a].into_iter(),
            Self::Lab(l, a, b) => vec![l, a, b].into_iter(),
            Self::Unknown => todo!(),
        }
    }
}

impl Color {
    pub fn get_mode(self, mode: &str) -> Self {
        match mode {
            "hex" => match self {
                Self::Hex(..) => self,
                Self::Rgb(..) => conversion::hex::rgb2hex(self),
                _ => todo!(),
            },
            "rgb" => match self {
                Self::Rgb(..) => self,
                Self::Hex(..) => conversion::hex::hex2rgb(self),
                Self::Lab(..) => conversion::lab::lab2rgb(self),
                _ => todo!(),
            },
            "lab" => match self {
                Self::Lab(..) => self,
                Self::Rgb(..) => conversion::lab::rgb2lab(self),
                _ => todo!(),
            },
            _ => todo!(),
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
