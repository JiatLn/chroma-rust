#[macro_use]
extern crate lazy_static;

mod chroma;
mod color;
mod data;
mod generator;
mod ops;
mod utils;

pub use chroma::Chroma;
pub use color::Color;
use data::w3cx11::W3CX11_HASHMAP as W3CX11;
pub use generator::random::random;
pub use utils::distance::distance;
pub use utils::valid::valid;
