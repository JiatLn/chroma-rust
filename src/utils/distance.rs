use crate::Color;

/// calc simple Euclidean distance between two colors with the same mode
///
/// the default mode is `lab`
pub fn distance(c1: &Color, c2: &Color, mode: Option<&str>) -> f64 {
    let mode = match mode {
        Some(mode) => mode,
        None => "lab",
    };

    let c1 = c1.get_mode(mode);
    let c2 = c2.get_mode(mode);

    let mut sum_sq = 0.0;
    c1.iter().zip(c2.iter()).for_each(|(a, b)| {
        sum_sq += (a - b).powi(2);
    });
    sum_sq.sqrt()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_rgb_distance() {
        let c1 = Color::from("rgb(255, 0, 0)");
        let c2 = Color::from("rgb(0, 255, 0)");

        assert_eq!(distance(&c1, &c2, Some("rgb")), 360.62445840513925);
    }

    #[test]
    fn test_calc_lab_distance() {
        let c1 = Color::from("rgb(255, 0, 0)");
        let c2 = Color::from("rgb(0, 255, 0)");

        // default is lab
        assert_eq!(distance(&c1, &c2, None), 170.56524200601007);
        assert_eq!(distance(&c1, &c2, Some("lab")), 170.56524200601007);
    }
}
