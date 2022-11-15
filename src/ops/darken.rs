use crate::{utils::conversion, Color};

// Corresponds roughly to RGB brighter/darker
static KN: f64 = 18.;

impl Color {
    /// Darken a color by a given amount.
    pub fn darken(&self, amount: Option<f64>) -> Color {
        let amount = amount.unwrap_or(1.);
        let lab = self.get_mode("lab");
        let alpha = self.get_alpha();
        let (l, a, b) = (lab[0] - KN * amount, lab[1], lab[2]);
        let (r, g, b, _) = conversion::lab::lab2rgb((l, a, b));
        Color::new(r, g, b, alpha)
    }
    /// alias for darken
    pub fn darker(&self, amount: Option<f64>) -> Color {
        let amount = amount.unwrap_or(1.);
        Color::darken(&self, Some(amount))
    }

    /// Brighten a color by a given amount.
    pub fn brighten(&self, amount: Option<f64>) -> Color {
        let amount = amount.unwrap_or(1.);
        Color::darken(&self, Some(amount * -1.))
    }
    /// alias for brighten
    pub fn brighter(&self, amount: Option<f64>) -> Color {
        let amount = amount.unwrap_or(1.);
        Color::brighten(&self, Some(amount))
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn test_darken_color() {
        let color = Color::from("#7760BF");
        let darkened = color.darken(None);
        assert_eq!(darkened.name(), "#46358e"); // #483584
    }

    #[test]
    fn test_brighten_color() {
        let color = Color::from("#7760BF");
        let brightened = color.brighten(None);
        assert_eq!(brightened.name(), "#a98ef2"); // #b5a9dc
    }
}
