pub fn num2rgb(num: u32) -> (u8, u8, u8) {
    if num > 0xFFFFFF {
        panic!("Number is too large to be a valid RGB color");
    }
    let r = (num >> 16) as u8;
    let g = (num >> 8) as u8;
    let b = num as u8;
    (r, g, b)
}

pub fn rgb2num(color: (u8, u8, u8)) -> u32 {
    let (r, g, b) = color;
    let num = (r as u32) << 16 | (g as u32) << 8 | b as u32;
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num2rgb() {
        let (r, g, b) = num2rgb(0x7760BF);
        assert_eq!(r, 0x77);
        assert_eq!(g, 0x60);
        assert_eq!(b, 0xBF);
    }

    #[test]
    fn test_rgb2num() {
        let num = rgb2num((0x77, 0x60, 0xBF));
        assert_eq!(num, 0x7760BF);
    }
}
