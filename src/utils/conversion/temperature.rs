/// Convert an rgb in JSON format into to a Kelvin color temperature
/// more see [color-temperature](https://github.com/neilbartlett/color-temperature)
pub fn rgb2temperature(color: (u8, u8, u8)) -> f64 {
    let r = color.0 as f64;
    let b = color.2 as f64;
    let mut min_temp = 1000.0;
    let mut max_temp = 40000.0;
    let eps = 0.4;
    let mut temp = 0.;
    while max_temp - min_temp > eps {
        temp = (max_temp + min_temp) * 0.5;
        let _rgb = temperature2rgb(temp);
        if (_rgb.2 as f64 / _rgb.0 as f64) >= (b / r) {
            max_temp = temp;
        } else {
            min_temp = temp;
        }
    }
    temp.round()
}

/// Tanner Helland's original algorithm.
pub fn _color_temperature2rgb_using_th(kelvin: f64) -> (u8, u8, u8) {
    let temp = kelvin / 100.;

    let mut r: f64;
    if temp < 66.0 {
        r = 255.0;
    } else {
        r = temp - 60.0;
        r = 329.698727446 * r.powf(-0.1332047592);
        r = r.max(0.0).min(255.0);
    }

    let mut g: f64;
    if temp < 66.0 {
        g = temp;
        g = 99.4708025861 * g.ln() - 161.1195681661;
        g = g.max(0.0).min(255.0);
    } else {
        g = temp - 60.0;
        g = 288.1221695283 * g.powf(-0.0755148492);
        g = g.max(0.0).min(255.0);
    }

    let mut b: f64;
    if temp >= 66.0 {
        b = 255.0;
    } else {
        if temp <= 19.0 {
            b = 0.0;
        } else {
            b = temp - 10.0;
            b = 138.5177312231 * b.ln() - 305.0447927307;
            b = b.max(0.0).min(255.0);
        }
    }
    (r.round() as u8, g.round() as u8, b.round() as u8)
}

/// A more accurate version algorithm based on a different curve fit to the
/// original RGB to Kelvin data.
pub fn temperature2rgb(kelvin: f64) -> (u8, u8, u8) {
    let temp = kelvin / 100.;

    let mut r: f64;
    if temp < 66.0 {
        r = 255.0;
    } else {
        // a + bx + c log(x)
        r = temp - 55.0;
        r = 351.97690566805693 + 0.114206453784165 * r - 40.25366309332127 * r.ln();
        r = r.max(0.0).min(255.0);
    }

    let mut g: f64;
    if temp < 66.0 {
        g = temp - 2.0;
        g = -155.25485562709179 - 0.44596950469579133 * g + 104.49216199393888 * g.ln();
        g = g.max(0.0).min(255.0);
    } else {
        g = temp - 50.0;
        g = 325.4494125711974 + 0.07943456536662342 * g - 28.0852963507957 * g.ln();
        g = g.max(0.0).min(255.0);
    }

    let mut b: f64;
    if temp >= 66.0 {
        b = 255.0;
    } else {
        if temp <= 20.0 {
            b = 0.0;
        } else {
            b = temp - 10.0;
            b = -254.76935184120902 + 0.8274096064007395 * b + 115.67994401066147 * b.ln();
            b = b.max(0.0).min(255.0);
        }
    }

    (r.round() as u8, g.round() as u8, b.round() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature2rgb() {
        let rgb = temperature2rgb(2000.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#ff8b00");

        let rgb = temperature2rgb(1850.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#ff8200");

        let rgb = temperature2rgb(3500.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#ffc38a");

        let rgb = temperature2rgb(6500.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#fffafe");

        let rgb = temperature2rgb(90000000000.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#ffffff");

        let rgb = temperature2rgb(0.);
        let hex = format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2);
        assert_eq!(hex, "#ff0000");
    }

    #[test]
    fn test_rgb2temperature() {
        let temp = rgb2temperature((255, 139, 20));
        assert_eq!(temp, 2000.);

        let temp = rgb2temperature((255, 195, 138));
        assert_eq!(temp, 3486.);

        let temp = rgb2temperature((255, 250, 254));
        assert_eq!(temp, 6473.);

        let temp = rgb2temperature((255, 255, 255));
        assert_eq!(temp, 6507.);

        let temp = rgb2temperature((255, 0, 0));
        assert_eq!(temp, 1000.);
    }
}
