//! An example of generating an image.
//!
//! `cargo run --release --example gen_image basic 20 20`

#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView};

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

    loop {
        if wave.is_collapsed() {
            break;
        }

        if let Err(_) = wave.collapse(100) {
            // If the wave is not collapsed, start over.
            wave.initialize();
        }
    }
    create_image(&assets, &wave, asset_size, width, height);
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
) {
    use image::ImageBuffer;

    let mut container = DynamicImage::new_rgba8((s * width) as u32, (s * height) as u32);

    wave.grid.iter().for_each(|slot| {
        let top = match slot.superposition.len() {
            // If there is only one image, just use it.
            1 => assets[slot.superposition[0].index].clone(),
            // If there isn't any image, use a red square. This is to mark the failed slot
            0 => DynamicImage::ImageRgb8(ImageBuffer::from_pixel(
                s as u32,
                s as u32,
                image::Rgb([75, 15, 15]),
            )),
            // If there is more than one image, use a black square.
            _ => DynamicImage::new_rgb8(s as u32, s as u32),
        };
        // Draw the slot iamge on the container.
        image::imageops::overlay(
            &mut container,
            &top,
            (slot.x * s) as i64,
            (slot.y * s) as i64,
        );
    });
    container.save("examples/output.png").unwrap();
    println!("Image saved to examples/output.png");
}

#[cfg(feature = "image")]
fn parse_args() -> (String, usize, usize) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: <assets>");
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
