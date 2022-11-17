# chroma-rust
A rust crate for working with colors and color spaces.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chroma_rust = "0.1"
```

and this to your crate root:

```rust
use chroma_rust;
```

## Usage


### Color

Generic color struct. Can be constructed from any color space. Current supported color spaces are:
- RGB
- RGBA
- HSL
- Lab
- Hex
- Named

You can generate a color from any of these color spaces with `&str` by using the `from` method.

For example:


```rust
let color = Color::from("red");
let color = Color::from("#ff0000");
let color = Color::from("rgb(255, 0, 0)");
let color = Color::from("rgba(255, 0, 0, 1)");
let color = Color::from("hsl(0, 100%, 50%)");
```

### alpha

Get the alpha value of a color.

```rust
let color = Color::from("rgba(255, 0, 0, 0.5)");
let alpha = color.alpha(); // 0.5
```

Set the alpha value of a color.

```rust
let color = Color::from("rgba(255, 0, 0, 0.5)");
let color = color.alpha(0.2);
let alpha = color.alpha(); // 0.2
```

### name

Get the name of a color. If the color is not a named color, `hex` code of the color is returned.

```rust

```rust
let color = Color::from("#fff000");
let name = color.name(); // "yellow"

let color = Color::from("rgb(255, 128, 44)");
let name = color.name(); // "#ff802c"
```

--- 

More api can be found in the [documentation](https://docs.rs/chroma-rust/latest/chroma_rust).

Enjoy it! 🎨


## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
