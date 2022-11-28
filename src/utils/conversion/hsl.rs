/// [RGB to HSL color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsl.html)
pub fn rgb2hsl(color: (u8, u8, u8)) -> (f64, f64, f64) {
    let (r, g, b) = color;

    let r = r as f64 / 255.;
    let g = g as f64 / 255.;
    let b = b as f64 / 255.;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let mut s = 0.0;
    let mut h = 0.0;
    let l = (max + min) / 2.0;

    let delta = max - min;

    if delta != 0.0 {
        s = delta / (1.0 - (2.0 * l - 1.0).abs());

        h = match max {
            x if x == r => 60.0 * (((g - b) / delta) % 6.0),
            x if x == g => 60.0 * (((b - r) / delta) + 2.0),
            x if x == b => 60.0 * (((r - g) / delta) + 4.0),
            _ => 0.0,
        };
    }

    (h, s, l)
}

/// [HSL to RGB color conversion](https://www.rapidtables.com/convert/color/hsl-to-rgb.html)
pub fn hsl2rgb(color: (f64, f64, f64)) -> (u8, u8, u8) {
    let (h, s, l) = color;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h >= 0.0 && h < 60.0 => (c, x, 0.0),
        h if h >= 60.0 && h < 120.0 => (x, c, 0.0),
        h if h >= 120.0 && h < 180.0 => (0.0, c, x),
        h if h >= 180.0 && h < 240.0 => (0.0, x, c),
        h if h >= 240.0 && h < 300.0 => (x, 0.0, c),
        h if h >= 300.0 && h < 360.0 => (c, 0.0, x),
        _ => panic!(),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hsl() {
        let color = (65, 164, 221);
        let hsl = rgb2hsl(color);
        assert!(hsl.0 - 202.0 < 0.01);
        assert!(hsl.1 - 0.70 < 0.01);
        assert!(hsl.2 - 0.56 < 0.01);

        let color = (0, 128, 0);
        let hsl = rgb2hsl(color);
        assert!(hsl.0 - 120.0 < 0.01);
        assert!(hsl.1 - 1.0 < 0.01);
        assert!(hsl.2 - 0.25 < 0.01);
    }

    #[test]
    fn test_hsl2rgb() {
        let color = (202.0, 0.70, 0.56);
        let rgb = hsl2rgb(color);
        assert_eq!(rgb, (64, 164, 221));

        let color = (0.0, 0.0, 0.0);
        let rgb = hsl2rgb(color);
        assert_eq!(rgb, (0, 0, 0));
    }
}
