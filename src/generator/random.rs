use crate::Color;

static DIGITS: &str = "0123456789abcdef";

/// Generate a random color.
pub fn random() -> Color {
    let mut code = "#".to_string();
    for _ in 0..6 {
        let index = rand::random::<usize>() % DIGITS.len();
        code.push(DIGITS.chars().nth(index).unwrap());
    }
    dbg!(&code);
    Color::from(code.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let color = random();

        assert!(color.rgba.0.ge(&0));
        assert!(color.rgba.0.le(&255));
        assert!(color.rgba.1.ge(&0));
        assert!(color.rgba.1.le(&255));
        assert!(color.rgba.2.ge(&0));
        assert!(color.rgba.2.le(&255));
        assert!(color.rgba.3.eq(&1.));
    }
}
