# Billow
[![Crates.io](https://img.shields.io/crates/v/billow.svg?color=blue)](https://crates.io/crates/billow)
[![Billow documentation](https://docs.rs/billow/badge.svg)](https://docs.rs/billow)

Rust implementation of the wave function collapse algorithm.
Based on [Oskar Stålberg's](https://www.youtube.com/watch?v=0bcZb-SsnrA) and [Martin Donald](https://www.youtube.com/watch?v=2SuvO4Gi7uY) detailed explanation.

> 🚧 **This is a work in progress.**


## Usage

This is a general purpose library, I provide implementations for the `image::DynamicImage` and `bevy::prelude::Image` types.

```rust
use billow::Wave;
use image::DynamicImage;

fn main() {
    let input: Vec<DynamicImage> = load_assets("assets/basic");
    let (width, height) = (20usize, 20usize);

    // New wave with a 20x20 grid
    let mut wave = Wave::new(input, width, height);

    // Populate the grid and choose a random starting point
    wave.initialize()

    // Run the algorithm.
    // This will try to collapse the grid to a single state.
    // `100` is the number of iterations.
    // each iteration will collapse one cell and try to propagate the changes.
    wave.collapse(100).expect("Failed to collapse");
}
```

See [examples](examples/README.md) for more.
