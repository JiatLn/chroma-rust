use crate::Color;

pub struct Chroma;

impl Chroma {
    /// mix two colors together using the given ratio (0.0 - 1.0)
    fn _mix(color1: &Color, color2: &Color, mode: Option<&str>, ratio: Option<f64>) -> Color {
        let mode = mode.unwrap_or("rgba");
        let ratio = ratio.unwrap_or(0.5);
        let v1 = color1.mode(mode);
        let v2 = color2.mode(mode);
        let mut v3 = Vec::new();
        for i in 0..v1.len() {
            v3.push(v1[i] + (v2[i] - v1[i]) * ratio);
        }
        Color::vec_mode2color(v3, mode)
    }

    pub fn mix(color1: &Color, color2: &Color) -> Color {
        Chroma::_mix(color1, color2, None, None)
    }

    pub fn mix_mode(color1: &Color, color2: &Color, mode: &str) -> Color {
        Chroma::_mix(color1, color2, Some(mode), None)
    }

    pub fn mix_ratio(color1: &Color, color2: &Color, ratio: f64) -> Color {
        Self::_mix(color1, color2, None, Some(ratio))
    }

    pub fn mix_mode_and_ratio(color1: &Color, color2: &Color, mode: &str, ratio: f64) -> Color {
        Self::_mix(color1, color2, Some(mode), Some(ratio))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let color1 = Color::from("rgb(255, 0, 0)");
        let color2 = Color::from("rgb(0, 0, 255)");
        let color3 = Chroma::mix(&color1, &color2);
        assert_eq!(color3.hex(), "#800080");

        let color1 = Color::from("rgb(255, 0, 0)");
        let color2 = Color::from("rgb(0, 0, 255)");
        let color3 = Chroma::mix_mode(&color1, &color2, "lab");
        assert_eq!(color3.hex(), "#ca0088");

        let color1 = Color::from("red");
        let color2 = Color::from("green");
        let color3 = Chroma::mix_mode(&color1, &color2, "hsl");
        assert_eq!(color3.hex(), "#c0c000");

        let color1 = Color::from("red");
        let color2 = Color::from("green");
        let color3 = Chroma::mix(&color1, &color2);
        assert_eq!(color3.hex(), "#804000");

        let color1 = Color::from("red");
        let color2 = Color::from("green");
        let color3 = Chroma::mix_mode(&color1, &color2, "lab");
        assert_eq!(color3.hex(), "#a16b00");
    }
}
