use crate::utils::conversion;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub rgba: (u8, u8, u8, f64),
}

/// Color is a struct that represents a color in RGBA format.
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
            _ => {
                let found = crate::W3CX11.get(str);
                match found {
                    Some(hex) => {
                        let (r, g, b, _) = conversion::hex::hex2rgb(hex);
                        (r, g, b, 1.0)
                    }
                    None => todo!(),
                }
            }
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
    /// get color with mode
    ///
    /// mode can be `rgb`, `rgba`, `hex`, `lab`
    pub fn get_mode(&self, mode: &str) -> Vec<f64> {
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
    /// get color name
    ///
    /// it will return a name of color if found in [*w3cx11*](http://www.w3.org/TR/css3-color/#svg-color), otherwise return hex code
    pub fn name(&self) -> String {
        let hex = conversion::hex::rgb2hex(self.rgba);

        let result = crate::W3CX11
            .clone()
            .into_iter()
            .find(|(_k, v)| v.to_string() == hex);

        match result {
            Some((k, _v)) => String::from(k),
            None => hex,
        }
    }

    /// get color alpha
    pub fn get_alpha(&self) -> f64 {
        self.rgba.3
    }
    /// set color alpha
    pub fn set_alpha(&mut self, alpha: f64) -> &mut Self {
        if alpha > 1.0 {
            self.rgba.3 = 1.0;
        } else if alpha < 0.0 {
            self.rgba.3 = 0.0;
        } else {
            self.rgba.3 = alpha;
        }
        self
    }

    pub fn new(r: u8, g: u8, b: u8, alpha: f64) -> Color {
        Color {
            rgba: (r, g, b, alpha),
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

        let name_color = Color::from("mediumspringgreen");
        assert_eq!(
            name_color,
            Color {
                rgba: (0, 250, 154, 1.)
            }
        );
    }

    #[test]
    fn test_get_color_name() {
        let color = Color::from("#abcdef");
        assert_eq!(color.name(), "#abcdef");

        let color = Color::from("rgb(0, 250, 154)");
        assert_eq!(color.name(), "mediumspringgreen");

        let color = Color::from("#00fa9a");
        assert_eq!(color.name(), "mediumspringgreen");
    }

    #[test]
    fn test_get_and_set_alpha() {
        let mut color = Color::from("#abcdef");
        color.set_alpha(1.5);
        assert_eq!(color.get_alpha(), 1.0);
        color.set_alpha(-0.5);
        assert_eq!(color.get_alpha(), 0.0);
        color.set_alpha(0.5);
        assert_eq!(color.get_alpha(), 0.5);
        assert_eq!(color.rgba, (171, 205, 239, 0.5));
    }
}
