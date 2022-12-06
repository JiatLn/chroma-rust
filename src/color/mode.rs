use crate::Color;

impl Color {
    /// Get color with mode
    ///
    /// mode can be `rgb`, `rgba`, `lab`, `hsl`, `cmyk`
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
            "cmyk" => {
                let (c, m, y, k) = self.cmyk();
                vec![c, m, y, k]
            }
            _ => todo!(),
        }
    }
}
