use crate::Color;

/// calc simple Euclidean distance between two colors with the same mode
pub fn distance(c1: &str, c2: &str, mode: Option<&str>) -> f64 {
    let mode = match mode {
        Some(mode) => mode,
        None => "lab",
    };

    let c1 = Color::from(c1).get_mode(mode);
    let c2 = Color::from(c2).get_mode(mode);

    let mut sum_sq = 0.0;
    for (c1, c2) in c1.into_iter().zip(c2.into_iter()) {
        sum_sq += (c1 - c2).powi(2);
    }
    sum_sq.sqrt()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_rgb_distance() {
        let c1 = "rgb(255, 0, 0)";
        let c2 = "rgb(0, 255, 0)";

        assert_eq!(distance(c1, c2, Some("rgb")), 360.62445840513925);
    }

    #[test]
    fn test_calc_lab_distance() {
        let c1 = "rgb(255, 0, 0)";
        let c2 = "rgb(0, 255, 0)";

        // default is lab
        assert_eq!(distance(c1, c2, None), 170.56524200601007);
        assert_eq!(distance(c1, c2, Some("lab")), 170.56524200601007);
    }
}
