use crate::utils::round;

pub fn rgb2cmyk(color: (u8, u8, u8)) -> (f64, f64, f64, f64) {
    let (r, g, b) = color;
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let k = 1.0 - f64::max(f64::max(r, g), b);
    let c = (1.0 - r - k) * (1. - k);
    let m = (1.0 - g - k) * (1. - k);
    let y = (1.0 - b - k) * (1. - k);
    (round(c, 2), round(m, 2), round(y, 2), round(k, 2))
}

pub fn cmyk2rgb(color: (f64, f64, f64, f64)) -> (u8, u8, u8) {
    let (c, m, y, k) = color;
    let r = (1. - c) * (1. - k) * 255.;
    let g = (1. - m) * (1. - k) * 255.;
    let b = (1. - y) * (1. - k) * 255.;
    (r.round() as u8, g.round() as u8, b.round() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rgb2cmyk() {
        let color = (255, 255, 255);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (0.0, 0.0, 0.0, 0.0));

        let color = (0, 0, 0);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (0.0, 0.0, 0.0, 1.0));

        let color = (255, 0, 0);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (0.0, 1.0, 1.0, 0.0));

        let color = (0, 255, 0);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (1.0, 0.0, 1.0, 0.0));

        let color = (0, 0, 255);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (1.0, 1.0, 0.0, 0.0));

        let color = (255, 255, 0);
        let (c, m, y, k) = rgb2cmyk(color);
        assert_eq!((c, m, y, k), (0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn test_cmyk2rgb() {
        let color = (0.0, 0.0, 0.0, 1.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (0, 0, 0));

        let color = (0.0, 0.0, 0.0, 0.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (255, 255, 255));

        let color = (0.0, 1.0, 1.0, 0.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (255, 0, 0));

        let color = (1.0, 0.0, 1.0, 0.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (0, 255, 0));

        let color = (1.0, 1.0, 0.0, 0.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (0, 0, 255));

        let color = (0.0, 0.0, 1.0, 0.0);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (255, 255, 0));

        let color = (0.0, 0.0, 0.0, 0.5);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (128, 128, 128));

        let color = (0.0, 0.0, 0.0, 0.75);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (64, 64, 64));

        let color = (0.0, 0.0, 0.0, 0.25);
        let (r, g, b) = cmyk2rgb(color);
        assert_eq!((r, g, b), (191, 191, 191));
    }
}
