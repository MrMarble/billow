//! Implementation of the wave function collapse algorithm based on Oskar Stålberg's.
//!
//! The wave function collapse algorithm is a recursive algorithm that picks a random tile
//! for a slot on the output image and removes impossible neighbors until only a single possibility remains

pub use conector::*;
pub use direction::*;
pub use features::*;
pub use wave::*;

mod conector;
mod direction;
mod features;
mod module;
mod slot;
mod wave;
