#[macro_use]
extern crate lazy_static;

mod color;
mod data;
mod utils;

pub use color::color::Color;
use data::w3cx11::W3CX11_HASHMAP as W3CX11;
pub use utils::distance::distance;
