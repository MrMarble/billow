//! An example of generating a gif animation.
//!
//! `cargo run --release --example gif basic 20 20`

#[cfg(feature = "image")]
use image::{codecs::gif::GifEncoder, DynamicImage, GenericImageView, ImageBuffer};
use std::fs::File;

#[cfg(feature = "image")]
fn main() {
    // Parse the command line arguments

    let (asset_path, width, height) = parse_args();

    // Load all the assets
    let assets = load_assets(&asset_path);
    // Tile size
    let asset_size = assets[0].dimensions().0 as usize;

    // Convert the assets to billow::Image
    let mut input: Vec<Box<dyn billow::Image>> = vec![];
    assets.iter().for_each(|asset| {
        input.push(Box::new(asset.clone()));
    });

    // Create and initialize wave.
    let mut wave = billow::Wave::new(&input, width, height);
    wave.initialize();

    let image = File::create("examples/output.gif").unwrap();
    let mut encoder = GifEncoder::new(image);
    loop {
        if wave.is_collapsed() {
            break;
        }

        match wave.collapse(1) {
            Ok(_) => {
                create_image(&assets, &wave, asset_size, width, height, &mut encoder);
            }
            Err(_) => wave.initialize(),
        }
    }
}

/// Helper function to load images from a folder.
#[cfg(feature = "image")]
fn load_assets(folder: &str) -> Vec<DynamicImage> {
    let path = format!("assets/{}", folder);

    let mut images: Vec<DynamicImage> = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "png" || ext == "jpg" {
            match image::open(path) {
                Ok(image) => images.push(image),
                Err(err) => {
                    panic!("Failed to load image: {}", err)
                }
            }
        }
    }
    images
}

/// Helper function to save the generated image.
#[cfg(feature = "image")]
fn create_image(
    assets: &Vec<DynamicImage>,
    wave: &billow::Wave,
    s: usize,
    width: usize,
    height: usize,
    encoder: &mut GifEncoder<File>,
) {
    let mut container = DynamicImage::new_rgba8((s * width) as u32, (s * height) as u32);
    wave.grid.iter().for_each(|slot| {
        let img = match slot.superposition.len() {
            // If there is only one image, just use it.
            1 => assets[slot.superposition[0].index]
                .clone()
                .to_rgba8()
                ,
            // If there isn't any image, use a red square. This is to mark the failed slot
            0 => {
                ImageBuffer::from_pixel(s as u32, s as u32, image::Rgba([75, 15, 15, 255]))
            }
            // If there is more than one image, overlay them with transparency.
            _ => {
                let mut container = DynamicImage::new_rgba8(s as u32,s as u32);
                for module in &slot.superposition {
                let mut img = assets[module.index].clone().to_rgba8();
                img.pixels_mut().for_each(|p| p[3] =50);
                    image::imageops::overlay(&mut container, &img, 0, 0);
                }
                container.into_rgba8()
            }
            };

        image::imageops::overlay(
            &mut container,
            &img,
            (slot.x * s) as i64,
            (slot.y * s) as i64,
        );
    });

    encoder
        .encode_frame(image::Frame::new(container.into_rgba8()))
        .expect("Failed to encode frame");
}

#[cfg(feature = "image")]
fn parse_args() -> (String, usize, usize) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: <assets> <width> <height>");
        println!("Available assets:");
        for entry in std::fs::read_dir("assets").unwrap() {
            print!("\t{}", entry.unwrap().file_name().to_string_lossy());
        }
        std::process::exit(1);
    }
    (
        args[1].clone(),
        args[2].parse::<usize>().unwrap(),
        args[3].parse::<usize>().unwrap(),
    )
}
#[cfg(not(feature = "image"))]
fn main() {
    println!("This example requires the `image` feature.");
}
