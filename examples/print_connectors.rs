//! An example of calculating the connectorID.
//!
//! `cargo run --release --example print_connectors`

#[cfg(feature = "image")]
use billow::{get_constraint_fn, Direction, Image};

/// Helper function to load images from a folder.
#[cfg(feature = "image")]
fn load_assets(path: &str) -> (Vec<String>, Vec<Box<dyn Image>>) {
    use std::fs;

    let mut images: Vec<Box<dyn Image>> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "png" || ext == "jpg" {
            let image = image::open(path).unwrap();
            images.push(Box::new(image));
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    (names, images)
}
#[cfg(feature = "image")]
fn main() {
    let assets = load_assets("assets/basic");

    let constraint_fn = get_constraint_fn(1);

    println!(
        "\n|{:^15}|{:^15}|{:^15}|",
        "Image", "Direction", "ConnectorID"
    );
    println!("|{empty:-<15}|{empty:-<15}|{empty:-<15}|", empty = "");

    for (name, image) in assets.0.iter().zip(assets.1.iter()) {
        for direction in Direction::all() {
            println!(
                "|{:<15}|{:<15}|{:^15}|",
                name,
                direction.to_string(),
                (constraint_fn)(image.clone(), direction).to_string() // Calculate the connector ID.
            );
        }
        println!("|{empty:.<15}|{empty:.<15}|{empty:.<15}|", empty = "");
    }
}

#[cfg(not(feature = "image"))]
fn main() {
    println!("This example requires the `image` feature.");
}
