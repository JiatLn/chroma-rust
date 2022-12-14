use crate::{utils::conversion, Color};

impl Color {
    /// Return the color as hexadecimal string.
    ///
    /// The hex string will include the alpha channel if it's less than 1.
    ///
    /// For example:
    /// ```
    /// use chroma_rust::Color;
    /// let color = Color::from("#abcdef");
    /// assert_eq!(color.hex(), "#abcdef");
    /// let color = Color::from("rgba(255, 255, 255, 0.6)");
    /// assert_eq!(color.hex(), "#ffffff99");
    /// ```
    pub fn hex(&self) -> String {
        conversion::hex::rgb2hex(self.rgba())
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        let (r, g, b, _) = self.rgba;
        (r, g, b)
    }

    pub fn rgba(&self) -> (u8, u8, u8, f64) {
        self.rgba
    }

    pub fn hsl(&self) -> (f64, f64, f64) {
        conversion::hsl::rgb2hsl(self.rgb())
    }

    pub fn hsv(&self) -> (f64, f64, f64) {
        conversion::hsv::rgb2hsv(self.rgb())
    }

    pub fn hsla(&self) -> (f64, f64, f64, f64) {
        let (h, s, l) = self.hsl();
        let a = self.alpha();
        (h, s, l, a)
    }

    pub fn lab(&self) -> (f64, f64, f64) {
        conversion::lab::rgb2lab(self.rgb())
    }

    /// CMYK (cyan, magenta, yellow, black)
    ///
    /// <https://en.wikipedia.org/wiki/CMYK_color_model>
    ///
    /// <https://www.rapidtables.com/convert/color/rgb-to-cmyk.html>
    ///
    /// Each value is in the range [0, 1]
    pub fn cmyk(&self) -> (f64, f64, f64, f64) {
        conversion::cmyk::rgb2cmyk(self.rgb())
    }

    /// Returns the numeric representation of the hexadecimal RGB color.
    ///
    /// For example:
    /// ```
    /// use chroma_rust::Color;
    /// let color = Color::from("#abcdef");
    /// assert_eq!(color.num(), 11259375);
    /// ```
    pub fn num(&self) -> u32 {
        conversion::num::rgb2num(self.rgb())
    }

    /// Returns the named color.
    ///
    /// Falls back to hexadecimal RGB string, if the color isn't present.
    ///
    /// Named color from [*w3cx11*](http://www.w3.org/TR/css3-color/#svg-color)
    ///
    /// ```
    /// use chroma_rust::Color;
    ///
    /// let color = Color::from("#ff0");
    /// assert_eq!(color.name(), "yellow");
    ///
    /// let color = Color::from("#abcdef");
    /// assert_eq!(color.name(), "#abcdef");
    /// ```
    pub fn name(&self) -> String {
        let hex = self.hex();

        let result = crate::W3CX11
            .clone()
            .into_iter()
            .find(|(_k, v)| v.to_string() == hex);

        match result {
            Some((k, _v)) => String::from(k),
            None => hex,
        }
    }

    /// Estimate the temperature in Kelvin of any given color, though this makes the only sense for colors from the temperature gradient above.
    /// More see [color-temperature](https://github.com/neilbartlett/color-temperature).
    pub fn temperature(&self) -> f64 {
        conversion::temperature::rgb2temperature(self.rgb())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let color = Color::new(255, 255, 255, 1.0);
        assert_eq!(color.hex(), "#ffffff");

        let color = Color::new(255, 255, 255, 0.5);
        assert_eq!(color.hex(), "#ffffff80");
    }

    #[test]
    fn test_rgb() {
        let color = Color::from("orange");
        assert_eq!(color.rgb(), (255, 165, 0));
    }

    #[test]
    fn test_rgba() {
        let color = Color::new(255, 255, 255, 1.0);
        assert_eq!(color.rgba(), (255, 255, 255, 1.0));
    }

    #[test]
    fn test_hsl() {
        let color = Color::from("cyan");
        let (h, s, l) = color.hsl();
        assert_eq!(h, 180.0);
        assert_eq!(s, 1.0);
        assert_eq!(l, 0.5);
    }

    #[test]
    fn test_hsv() {
        let color = Color::from("cyan");
        let (h, s, v) = color.hsv();
        assert_eq!(h, 180.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_hsla() {
        let color = Color::new(255, 255, 255, 1.0);
        let (h, s, l, a) = color.hsla();
        assert_eq!(h, 0.0);
        assert_eq!(s, 0.0);
        assert_eq!(l, 1.0);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_lab() {
        let color = Color::from("orange");
        let (l, a, b) = color.lab();
        // [74.94,23.93,78.95]
        assert!(l - 74.94 < 0.01);
        assert!(a - 23.93 < 0.01);
        assert!(b - 78.95 < 0.01);
    }

    #[test]
    fn test_name() {
        let color = Color::from("#abcdef");
        assert_eq!(color.name(), "#abcdef");

        let color = Color::from("rgb(0, 250, 154)");
        assert_eq!(color.name(), "mediumspringgreen");

        let color = Color::from("#00fa9a");
        assert_eq!(color.name(), "mediumspringgreen");
    }

    #[test]
    fn test_num() {
        let color = Color::from("#abcdef");
        assert_eq!(color.num(), 11259375);

        let color = Color::from("rgb(255, 128, 44)");
        assert_eq!(color.num(), 16744492);
    }

    #[test]
    fn test_temperature() {
        let color = Color::from("#abcdef");
        assert_eq!(color.temperature(), 14098.);

        let color = Color::from("rgb(255, 128, 44)");
        assert_eq!(color.temperature(), 2208.);

        let color = Color::from("#b3ccff");
        assert_eq!(color.temperature(), 15005.);

        let color = Color::from("#ff0000");
        assert_eq!(color.temperature(), 1000.);

        let color = Color::from("#00ff00");
        assert_eq!(color.temperature(), 40000.);

        let color = Color::from("#0000ff");
        assert_eq!(color.temperature(), 40000.);

        let color = Color::from("#000000");
        assert_eq!(color.temperature(), 40000.);
    }

    #[test]
    fn test_cmyk() {
        let color = Color::from("#cc33ff");
        assert_eq!(color.cmyk(), (0.2, 0.8, 0., 0.));

        let color = Color::from("rgb(255, 128, 44)");
        assert_eq!(color.cmyk(), (0.0, 0.5, 0.83, 0.0));
    }
}
