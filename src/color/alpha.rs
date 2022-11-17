use crate::Color;

impl Color {
    /// get color alpha
    ///
    /// # Examples
    /// ```
    /// use color::Color;
    /// let color = Color::from("#abcdef");
    /// assert_eq!(color.alpha(), 1.);
    /// ```
    pub fn alpha(&self) -> f64 {
        self.rgba.3
    }
    /// set color alpha
    ///
    /// # Example
    /// ```
    /// use color::Color;
    /// let color = Color::from("#7760BF");
    /// let color = color.alpha(0.5);
    /// assert_eq!(color.alpha(), 0.5);
    /// ```
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_alpha() {
        let color = Color::from("#abcdef");
        assert_eq!(color.alpha(), 1.);
    }

    #[test]
    fn test_color_set_alpha() {
        let mut color = Color::from("#7760BF");
        let color = color.set_alpha(0.5);
        assert_eq!(color.alpha(), 0.5);
    }
}
