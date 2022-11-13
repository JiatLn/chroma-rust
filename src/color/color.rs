use crate::utils::conversion;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub rgba: (u8, u8, u8, f64),
}

impl From<&str> for Color {
    fn from(str: &str) -> Self {
        let (r, g, b, a) = match str {
            str if str.starts_with("#") => conversion::hex::hex2rgb(str),
            str if str.starts_with("rgba") => Color::parse_rgba_str(str),
            str if str.starts_with("rgb") => Color::parse_rgb_str(str),
            str if str.starts_with("lab") => {
                let (l, a, b) = Color::parse_lab_str(str);
                conversion::lab::lab2rgb((l, a, b))
            }
            _ => todo!(),
        };
        Color { rgba: (r, g, b, a) }
    }
}

impl Iterator for Color {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let (r, g, b, a) = self.rgba;
        vec![r as f64, g as f64, b as f64, a].into_iter().next()
    }
}

impl Color {
    pub fn get_mode(self, mode: &str) -> Vec<f64> {
        match mode {
            "rgb" => vec![self.rgba.0 as f64, self.rgba.1 as f64, self.rgba.2 as f64],
            "rgba" => vec![
                self.rgba.0 as f64,
                self.rgba.1 as f64,
                self.rgba.2 as f64,
                self.rgba.3,
            ],
            "lab" => {
                let (l, a, b) = conversion::lab::rgb2lab(self.rgba);
                vec![l, a, b]
            }
            _ => todo!(),
        }
    }
}

impl Color {
    fn parse_rgb_str(str: &str) -> (u8, u8, u8, f64) {
        let v_u8: Vec<u8> = str
            .trim()
            .replace(" ", "")
            .replace("rgb(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        (v_u8[0], v_u8[1], v_u8[2], 1.)
    }
    fn parse_lab_str(str: &str) -> (f64, f64, f64) {
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
    fn parse_rgba_str(str: &str) -> (u8, u8, u8, f64) {
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
        (r, g, b, alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        let hex_color = Color::from("#abcdef");
        assert_eq!(
            hex_color,
            Color {
                rgba: (171, 205, 239, 1.)
            }
        );

        let rgb_color = Color::from("rgb(255, 255, 255)");
        assert_eq!(
            rgb_color,
            Color {
                rgba: (255, 255, 255, 1.)
            }
        );

        let rgba_color = Color::from("rgba(255, 255, 255, 0.6)");
        assert_eq!(
            rgba_color,
            Color {
                rgba: (255, 255, 255, 0.6)
            }
        );

        let lab_color = Color::from("lab(100, 0, 0)");
        assert_eq!(
            lab_color,
            Color {
                rgba: (255, 255, 255, 1.)
            }
        );
    }
}
