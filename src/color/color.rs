use crate::utils::{conversion, parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub rgba: (u8, u8, u8, f64),
}

/// Color is a struct that represents a color in RGBA format.
impl From<&str> for Color {
    fn from(str: &str) -> Self {
        let (r, g, b, a) = match str {
            str if str.starts_with("#") => conversion::hex::hex2rgb(str),
            str if str.starts_with("rgba") => parser::parse_rgba_str(str),
            str if str.starts_with("rgb") => parser::parse_rgb_str(str),
            str if str.starts_with("lab") => {
                let (l, a, b) = parser::parse_lab_str(str);
                conversion::lab::lab2rgb((l, a, b))
            }
            str if str.starts_with("hsl") => {
                let (h, s, l) = parser::parse_hsl_str(str);
                let (r, g, b) = conversion::hsl::hsl2rgb((h, s, l));
                (r, g, b, 1.0)
            }
            _ => {
                let found = crate::W3CX11.get(str);
                match found {
                    Some(hex) => conversion::hex::hex2rgb(hex),
                    None => panic!("Color not found"),
                }
            }
        };
        Color { rgba: (r, g, b, a) }
    }
}

impl From<u32> for Color {
    fn from(num: u32) -> Self {
        let (r, g, b) = conversion::num::num2rgb(num);
        Color {
            rgba: (r, g, b, 1.0),
        }
    }
}

impl Iterator for Color {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let (r, g, b, a) = self.rgba();
        vec![r as f64, g as f64, b as f64, a].into_iter().next()
    }
}

impl Color {
    /// Get color with mode
    ///
    /// mode can be `rgb`, `rgba`, `lab`, `hsl`
    pub fn mode(&self, mode: &str) -> Vec<f64> {
        match mode {
            "rgb" => {
                let (r, g, b) = self.rgb();
                vec![r as f64, g as f64, b as f64]
            }
            "rgba" => {
                let (r, g, b, a) = self.rgba();
                vec![r as f64, g as f64, b as f64, a]
            }
            "lab" => {
                let (l, a, b) = self.lab();
                vec![l, a, b]
            }
            "hsl" => {
                let (h, s, l) = self.hsl();
                vec![h, s, l]
            }
            _ => todo!(),
        }
    }

    pub fn new(r: u8, g: u8, b: u8, alpha: f64) -> Color {
        Color {
            rgba: (r, g, b, alpha),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex_str() {
        let hex_color = Color::from("#ff0000");
        assert_eq!(hex_color.rgba, (255, 0, 0, 1.0));

        let hex_color = Color::from("#abcdef");
        assert_eq!(
            hex_color,
            Color {
                rgba: (171, 205, 239, 1.)
            }
        );
    }

    #[test]
    fn test_color_from_rgb_str() {
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
    }

    #[test]
    fn test_color_from_lab_str() {
        let lab_color = Color::from("lab(100, 0, 0)");
        assert_eq!(
            lab_color,
            Color {
                rgba: (255, 255, 255, 1.)
            }
        );
    }

    #[test]
    fn test_color_from_name_str() {
        let name_color = Color::from("mediumspringgreen");
        assert_eq!(
            name_color,
            Color {
                rgba: (0, 250, 154, 1.)
            }
        );
    }

    #[test]
    fn test_color_from_hsl_str() {
        let hsl_color = Color::from("hsl(0, 100%, 50%)");
        assert_eq!(
            hsl_color,
            Color {
                rgba: (255, 0, 0, 1.)
            }
        );

        let hsl_color = Color::from("hsl(120, 100%, 50%)");
        assert_eq!(
            hsl_color,
            Color {
                rgba: (0, 255, 0, 1.)
            }
        );

        let hsl_color = Color::from("hsl(240, 100%, 50%)");
        assert_eq!(
            hsl_color,
            Color {
                rgba: (0, 0, 255, 1.)
            }
        );
    }

    #[test]
    fn test_color_from_num() {
        let num_color = Color::from(0xff0000);
        assert_eq!(
            num_color,
            Color {
                rgba: (255, 0, 0, 1.)
            }
        );

        let num_color = Color::from(0xffff00);
        assert_eq!(
            num_color,
            Color {
                rgba: (255, 255, 0, 1.)
            }
        );

        let num_color = Color::from(0x00ff00);
        assert_eq!(
            num_color,
            Color {
                rgba: (0, 255, 0, 1.)
            }
        );

        let num_color = Color::from(11259375);
        assert_eq!(num_color.hex(), "#abcdef");
    }
}
