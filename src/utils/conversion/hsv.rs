/// [RGB to HSV color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsv.html)
pub fn rgb2hsv(color: (u8, u8, u8)) -> (f64, f64, f64) {
    let (r, g, b) = color;

    let r = r as f64 / 255.;
    let g = g as f64 / 255.;
    let b = b as f64 / 255.;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h = 0.0;
    let mut s = 0.0;
    let v = max;

    if delta != 0.0 {
        h = match max {
            x if x == r => {
                let h = 60.0 * (((g - b) / delta) % 6.0);
                if h < 0.0 {
                    h + 360.0
                } else {
                    h
                }
            }
            x if x == g => 60.0 * (((b - r) / delta) + 2.0),
            x if x == b => 60.0 * (((r - g) / delta) + 4.0),
            _ => 0.0,
        };

        s = delta / max;
    }

    (h, s, v)
}

/// [HSV to RGB color conversion](https://www.rapidtables.com/convert/color/hsv-to-rgb.html)
pub fn hsv2rgb(color: (f64, f64, f64)) -> (u8, u8, u8) {
    let (h, s, v) = color;

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

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
    fn test_rgb2hsv() {
        assert_eq!(rgb2hsv((255, 0, 0)), (0.0, 1.0, 1.0));
        assert_eq!(rgb2hsv((0, 255, 0)), (120.0, 1.0, 1.0));
        assert_eq!(rgb2hsv((0, 0, 255)), (240.0, 1.0, 1.0));
        assert_eq!(rgb2hsv((255, 255, 255)), (0.0, 0.0, 1.0));
        assert_eq!(rgb2hsv((0, 0, 0)), (0.0, 0.0, 0.0));
        assert_eq!(rgb2hsv((255, 255, 0)), (60.0, 1.0, 1.0));
        assert_eq!(rgb2hsv((255, 0, 255)), (300.0, 1.0, 1.0));
        assert_eq!(rgb2hsv((0, 255, 255)), (180.0, 1.0, 1.0));
        let (h, s, v) = rgb2hsv((128, 128, 128));
        assert_eq!(h, 0.0);
        assert_eq!(s, 0.0);
        assert_eq!(v - 0.5 < 0.1, true);
        let (h, s, v) = rgb2hsv((255, 127, 0));
        assert_eq!(h - 30.0 < 0.1, true);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_hsv2rgb() {
        assert_eq!(hsv2rgb((0.0, 1.0, 1.0)), (255, 0, 0));
        assert_eq!(hsv2rgb((120.0, 1.0, 1.0)), (0, 255, 0));
        assert_eq!(hsv2rgb((240.0, 1.0, 1.0)), (0, 0, 255));
        assert_eq!(hsv2rgb((0.0, 0.0, 1.0)), (255, 255, 255));
        assert_eq!(hsv2rgb((0.0, 0.0, 0.0)), (0, 0, 0));
        assert_eq!(hsv2rgb((60.0, 1.0, 1.0)), (255, 255, 0));
        assert_eq!(hsv2rgb((300.0, 1.0, 1.0)), (255, 0, 255));
        assert_eq!(hsv2rgb((180.0, 1.0, 1.0)), (0, 255, 255));
        let (r, g, b) = hsv2rgb((0.0, 0.0, 0.5));
        assert_eq!(r, 128);
        assert_eq!(g, 128);
        assert_eq!(b, 128);
        let (r, g, b) = hsv2rgb((30.0, 1.0, 1.0));
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 0);
    }
}
