mod cmyk;
mod hsl;
mod lab;
mod rgb;

pub use cmyk::parse_cmyk_str;
pub use hsl::parse_hsl_str;
pub use lab::parse_lab_str;
pub use rgb::{parse_rgb_str, parse_rgba_str};
