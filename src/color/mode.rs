use crate::Color;

impl Color {
    /// Get color with mode
    ///
    /// mode can be `rgb`, `rgba`, `lab`, `hsl`, `hsv`, `cmyk`
    pub fn mode(&self, mode: &str) -> Vec<f64> {
        match mode {
            "rgb" => {
                let (r, g, b) = self.rgb();
                vec![r as f64, g as f64, b as f64]
            }
            "rgba" => {
                let (r, g, b, a) = self.rgba();
                vec![r as f64, g as f64, b as f64, a]
            }
            "lab" => {
                let (l, a, b) = self.lab();
                vec![l, a, b]
            }
            "hsl" => {
                let (h, s, l) = self.hsl();
                vec![h, s, l]
            }
            "hsv" => {
                let (h, s, v) = self.hsv();
                vec![h, s, v]
            }
            "cmyk" => {
                let (c, m, y, k) = self.cmyk();
                vec![c, m, y, k]
            }
            _ => todo!(),
        }
    }

    pub fn vec_mode2color(vec_f64: Vec<f64>, mode: &str) -> Color {
        let len = vec_f64.len();
        match mode {
            "rgb" => {
                if len != 3 {
                    panic!(
                        "The {} mode must got a vec which len is 3, but got {}",
                        mode, len
                    )
                }
                let r = vec_f64[0].round() as u8;
                let g = vec_f64[1].round() as u8;
                let b = vec_f64[2].round() as u8;
                Color::new(r, g, b, 1.0)
            }
            "rgba" => {
                if len != 4 {
                    panic!(
                        "The {} mode must got a vec which len is 4, but got {}",
                        mode, len
                    )
                }
                let r = vec_f64[0].round() as u8;
                let g = vec_f64[1].round() as u8;
                let b = vec_f64[2].round() as u8;
                let a = vec_f64[3];
                Color::new(r, g, b, a)
            }
            "lab" => {
                if len != 3 {
                    panic!(
                        "The {} mode must got a vec which len is 3, but got {}",
                        mode, len
                    )
                }
                let l = vec_f64[0];
                let a = vec_f64[1];
                let b = vec_f64[2];
                let color_str = format!("lab({}, {}, {})", l, a, b);
                Color::from(color_str.as_str())
            }
            "hsl" => {
                if len != 3 {
                    panic!(
                        "The {} mode must got a vec which len is 3, but got {}",
                        mode, len
                    )
                }
                let h = vec_f64[0];
                let s = vec_f64[1];
                let l = vec_f64[2];
                let color_str = format!("hsl({}, {}, {})", h, s, l);
                Color::from(color_str.as_str())
            }
            "hsv" => {
                if len != 3 {
                    panic!(
                        "The {} mode must got a vec which len is 3, but got {}",
                        mode, len
                    )
                }
                let h = vec_f64[0];
                let s = vec_f64[1];
                let v = vec_f64[2];
                let color_str = format!("hsv({}, {}, {})", h, s, v);
                Color::from(color_str.as_str())
            }
            "cmyk" => {
                if len != 4 {
                    panic!(
                        "The {} mode must got a vec which len is 4, but got {}",
                        mode, len
                    )
                }
                let c = vec_f64[0];
                let m = vec_f64[1];
                let y = vec_f64[2];
                let k = vec_f64[3];
                let color_str = format!("cmyk({}, {}, {}, {})", c, m, y, k);
                Color::from(color_str.as_str())
            }
            _ => todo!(),
        }
    }
}
