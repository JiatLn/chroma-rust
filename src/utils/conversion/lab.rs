// Corresponds roughly to RGB brighter/darker
// static KN: f64 = 18.;
// D65 standard referent
static XN: f64 = 0.950470;
static YN: f64 = 1.;
static ZN: f64 = 1.088830;
static LAB_CONSTANTS_T0: f64 = 4. / 29.;
static LAB_CONSTANTS_T1: f64 = 6. / 29.;
static LAB_CONSTANTS_T2: f64 = 3. * LAB_CONSTANTS_T1 * LAB_CONSTANTS_T1;
static LAB_CONSTANTS_T3: f64 = LAB_CONSTANTS_T1 * LAB_CONSTANTS_T1 * LAB_CONSTANTS_T1;

/// Convert RGB to CIE-L*ab
/// https://en.wikipedia.org/wiki/Lab_color_space#CIELAB-CIEXYZ_conversions
pub fn rgb2lab(color: (u8, u8, u8, f64)) -> (f64, f64, f64) {
    let (r, g, b, _alpha) = color;

    let (x, y, z) = rgb2xyz(r as f64, g as f64, b as f64);

    let mut l = 116. * y - 16.;
    l = if l < 0. { 0. } else { l };

    (l, 500.0 * (x - y), 200.0 * (y - z))
}

pub fn lab2rgb(color: (f64, f64, f64)) -> (u8, u8, u8, f64) {
    let (l, a, b) = color;

    let mut y = (l + 16.) / 116.;
    let mut x = y + a / 500.;
    let mut z = y - b / 200.;

    y = YN * lab_xyz(y);
    x = XN * lab_xyz(x);
    z = ZN * lab_xyz(z);

    let r = xyz_rgb(3.2404542 * x - 1.5371385 * y - 0.4985314 * z);
    let g = xyz_rgb(-0.9692660 * x + 1.8760108 * y + 0.0415560 * z);
    let b = xyz_rgb(0.0556434 * x - 0.2040259 * y + 1.0572252 * z);

    (r.round() as u8, g.round() as u8, b.round() as u8, 1.)
}

fn xyz_rgb(r: f64) -> f64 {
    255. * (r <= 0.00304)
        .then(|| 12.92 * r)
        .unwrap_or_else(|| 1.055 * r.powf(1. / 2.4) - 0.055)
}

fn lab_xyz(t: f64) -> f64 {
    (t > LAB_CONSTANTS_T1)
        .then(|| t * t * t)
        .unwrap_or_else(|| LAB_CONSTANTS_T2 * (t - LAB_CONSTANTS_T0))
}

fn rgb2xyz(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let r = rgb_xyz(r);
    let g = rgb_xyz(g);
    let b = rgb_xyz(b);
    let x = xyz_lab((0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / XN);
    let y = xyz_lab((0.2126729 * r + 0.7151522 * g + 0.0721750 * b) / YN);
    let z = xyz_lab((0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / ZN);
    (x, y, z)
}

fn rgb_xyz(mut r: f64) -> f64 {
    r = r / 255.0;
    if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    }
}

fn xyz_lab(t: f64) -> f64 {
    if t > LAB_CONSTANTS_T3 {
        t.powf(1.0 / 3.0)
    } else {
        t / LAB_CONSTANTS_T2 + LAB_CONSTANTS_T0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn approx_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn test_rgb2lab() {
        let color = (0, 255, 255, 1.);
        let lab_color = rgb2lab(color);
        let (l, a, b) = lab_color;

        assert!(approx_equal(l, 91.11));
        assert!(approx_equal(a, -48.09));
        assert!(approx_equal(b, -14.13));
    }

    #[test]
    fn test_lab2rgb() {
        let color = (91.11, -48.09, -14.13);
        let rgb_color = lab2rgb(color);
        let (r, g, b, _alpha) = rgb_color;

        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
    }
}
