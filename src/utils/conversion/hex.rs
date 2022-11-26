use crate::utils::round;

pub fn rgb2hex(color: (u8, u8, u8, f64)) -> String {
    let (r, g, b, a) = color;
    if a == 1. {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        let a = (a * 255.).round() as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

pub fn hex2rgb(hex: &str) -> (u8, u8, u8, f64) {
    let mut hex = String::from(hex);
    let mut alpha = 1.;
    let len = hex.len();
    if len == 4 {
        let mut hex_new = String::from("#");
        for c in hex[1..].chars() {
            hex_new.push(c);
            hex_new.push(c);
        }
        hex = hex_new;
    } else if len == 9 {
        // #rrggbbaa
        alpha = (u8::from_str_radix(&hex[7..], 16).unwrap()) as f64 / 255.;
        alpha = round(alpha, 2);
        hex = hex[..7].to_string();
    }

    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

    (r, g, b, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hex() {
        let color = (255, 255, 255, 1.);
        assert_eq!(rgb2hex(color), "#ffffff");

        let color = (255, 255, 255, 0.5);
        assert_eq!(rgb2hex(color), "#ffffff80");
    }

    #[test]
    fn test_hex2rgb() {
        let hex = "#ffffff";
        assert_eq!(hex2rgb(hex), (255, 255, 255, 1.));

        let hex = "#fff";
        assert_eq!(hex2rgb(hex), (255, 255, 255, 1.));

        let hex = "#ffffff80";
        assert_eq!(hex2rgb(hex), (255, 255, 255, 0.5));
    }
}
