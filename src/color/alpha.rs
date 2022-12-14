use crate::Color;

impl Color {
    /// Get color alpha.
    ///
    /// # Examples
    /// ```
    /// use chroma_rust::Color;
    /// let color = Color::from("#abcdef");
    /// assert_eq!(color.alpha(), 1.);
    /// ```
    pub fn alpha(&self) -> f64 {
        self.rgba.3
    }
    /// Set color alpha.
    ///
    /// - If alpha is less than 0, it will be set to 0.
    /// - If alpha is greater than 1, it will be set to 1.
    ///
    /// # Example
    /// ```
    /// use chroma_rust::Color;
    /// let mut color = Color::from("#7760BF");
    /// let color = color.set_alpha(0.5);
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
