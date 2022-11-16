/// Convert RGB to HSL
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
        if l > 0.0 && l <= 0.5 {
            s = delta / (max + min);
        } else {
            s = delta / (2.0 - max - min);
        }

        match max {
            x if x == r => {
                if g >= b {
                    h = 60.0 * ((g - b) / delta);
                } else {
                    h = 60.0 * ((g - b) / delta) + 360.0;
                }
            }
            x if x == g => h = 60.0 * ((b - r) / delta) + 120.0,
            x if x == b => h = 60.0 * ((r - g) / delta) + 240.0,
            _ => (),
        }
    }

    (h, s, l)
}

/// Convert HSL to RGB
pub fn hsl2rgb(color: (f64, f64, f64)) -> (u8, u8, u8) {
    let (h, s, l) = color;

    if s == 0.0 {
        let v = (l * 255.).round() as u8;
        return (v, v, v);
    }

    let t2 = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let t1 = 2.0 * l - t2;
    let h = h / 360.0;

    let mut t3 = vec![h + 1.0 / 3.0, h, h - 1.0 / 3.0];
    let mut c = vec![0.0, 0.0, 0.0];

    for i in 0..3 {
        if t3[i] < 0.0 {
            t3[i] += 1.0;
        } else if t3[i] > 1.0 {
            t3[i] -= 1.0;
        }
        if 6.0 * t3[i] < 1.0 {
            c[i] = t1 + (t2 - t1) * 6.0 * t3[i];
        } else if 2.0 * t3[i] < 1.0 {
            c[i] = t2;
        } else if 3.0 * t3[i] < 2.0 {
            c[i] = t1 + (t2 - t1) * (2.0 / 3.0 - t3[i]) * 6.0;
        } else {
            c[i] = t1;
        }
    }

    (
        (c[0] * 255.).round() as u8,
        (c[1] * 255.).round() as u8,
        (c[2] * 255.).round() as u8,
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
