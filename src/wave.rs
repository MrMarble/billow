use rand::seq::IteratorRandom;

use crate::{conector::ConnectorID, direction::Direction, module::Module, slot::Slot};

pub trait Image {
    fn size(&self) -> (usize, usize);
    fn get_pixel_at(&self, x: usize, y: usize) -> [u8; 4];
}

/// Wave holds the state of a wave collapse function.
pub struct Wave {
    /// Width of the grid.
    width: usize,
    /// Height of the grid.
    height: usize,
    /// Input tiles.
    input: Vec<Module>,
    /// 2D grid of slots.
    pub grid: Vec<Slot>,

    /// Slots that have been visited during the current iteration.
    pub history: Vec<Slot>,

    /// Override this function to change the behavior of the wave collapse function.
    pub is_possible_fn: Box<dyn Fn(Module, Slot, Slot, Direction) -> bool>,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            input: Vec::new(),
            grid: Vec::new(),
            history: Vec::new(),
            is_possible_fn: Box::new(|module, from, _to, d| module.is_possible(&from, d)),
        }
    }
}

impl Wave {
    /// Create a new wave collapse function with the given width and height.
    /// The default constraint function will check 3 pixels in each direction.
    /// Use `with_custom_constraint` to override the default behavior of the wave collapse function.
    pub fn new(input: &Vec<Box<dyn Image>>, width: usize, height: usize) -> Self {
        Wave::with_custom_constraint(input, width, height, get_constraint_fn(3))
    }

    pub fn with_custom_constraint(
        input: &[Box<dyn Image>],
        width: usize,
        height: usize,
        custom_contraint_fn: Box<dyn Fn(&Box<dyn Image>, Direction) -> ConnectorID>,
    ) -> Self {
        let mut modules: Vec<Module> = vec![];

        for (idx, image) in input.iter().enumerate() {
            let mut module = Module::new(idx);

            // Initialize the connectors.
            for direction in Direction::all() {
                module.connectors[direction as usize] = custom_contraint_fn(image, direction);
            }
            modules.push(module);
        }

        Self {
            width,
            height,
            input: modules,
            ..Default::default()
        }
    }

    /// Populate the grid with the input modules.
    /// Choose a random starting point.
    pub fn initialize(&mut self) {
        self.grid = vec![Slot::default(); self.width * self.height];
        self.grid.iter_mut().enumerate().for_each(|(idx, slot)| {
            let x = idx % self.width;
            let y = idx / self.width;
            slot.x = x;
            slot.y = y;
            slot.superposition = self.input.clone();
        });

        // TODO: Move this to a separate function. (initialize_random)
        if let Some(slot) = self.collapse_random() {
            self.history.push(slot);
        }
    }

    /// Pick a random slot from the grid and collapse it.
    pub fn collapse_random(&mut self) -> Option<Slot> {
        if let Some(slot) = self.grid.iter_mut().choose(&mut rand::thread_rng()) {
            slot.collapse();
            return Some(slot.clone());
        }
        None
    }

    /// Pick the slot with the lowest superposition size.
    pub fn collapse_least_entropy(&mut self) -> Option<Slot> {
        let mut least_index = 0;
        let mut least_entropy = self.input.len();
        // TODO: Use reduce?
        self.grid.iter().enumerate().for_each(|(idx, slot)| {
            let entropy = slot.superposition.len();
            if entropy > 1 && entropy < least_entropy {
                least_index = idx;
                least_entropy = entropy;
            }
        });

        if let Some(slot) = self.grid.get_mut(least_index) {
            slot.collapse();
            return Some(slot.clone());
        }
        None
    }

    /// Check if all the slots have been collapsed.
    pub fn is_collapsed(&self) -> bool {
        self.grid.iter().all(|slot| slot.superposition.len() == 1)
    }

    /// Checks if the slot has a neighbor in the given direction. (To avoid out of bounds errors.)
    fn has_neighbor(&self, slot: &Slot, direction: Direction) -> bool {
        match direction {
            Direction::Up => slot.y > 0,
            Direction::Down => slot.y < self.height - 1,
            Direction::Left => slot.x > 0,
            Direction::Right => slot.x < self.width - 1,
        }
    }

    /// Get the neighbor of the slot in the given direction.
    fn get_neighbor(&self, slot: &Slot, direction: Direction) -> Option<&Slot> {
        match direction {
            Direction::Up => self.grid.get(slot.x + (slot.y - 1) * self.width),
            Direction::Down => self.grid.get(slot.x + (slot.y + 1) * self.width),
            Direction::Left => self.grid.get((slot.x - 1) + slot.y * self.width),
            Direction::Right => self.grid.get((slot.x + 1) + slot.y * self.width),
        }
    }

    /// Checks if the slot has already been visited in the current iteration.
    /// This is used to avoid propagating the same slot multiple times.
    fn has_visited(&self, slot: &Slot) -> bool {
        self.history.iter().any(|s| s.x == slot.x && s.y == slot.y)
    }

    /// Returns the possible modules for the given slot.
    fn get_possible_modules(&self, a: &Slot, b: &Slot, direction: Direction) -> Vec<Module> {
        let mut possible_modules = Vec::new();
        for module in &b.superposition {
            if (self.is_possible_fn)(*module, a.clone(), b.clone(), direction) {
                possible_modules.push(*module);
            }
        }
        possible_modules
    }

    /// Collapse one slot and propagate the collapse to all the neighbors.
    fn recurse(&mut self) -> Result<(), String> {
        if self.is_collapsed() {
            return Ok(());
        }

        if self.history.is_empty() {
            if let Some(slot) = self.collapse_least_entropy() {
                self.history.push(slot);
            } else {
                return Ok(());
            }
        }

        let previous_slot = self.history.last().expect("No previous slot").clone();
        for direction in Direction::all() {
            if !self.has_neighbor(&previous_slot, direction) {
                continue;
            }

            let next_slot = &mut self
                .get_neighbor(&previous_slot, direction)
                .expect("No next slot")
                .to_owned();
            if self.has_visited(next_slot) {
                continue;
            }

            let possible_modules = self.get_possible_modules(&previous_slot, next_slot, direction);
            if possible_modules.len() == next_slot.superposition.len() {
                continue;
            } else {
                next_slot.superposition = possible_modules;
                // TODO: This is a bit of a hack. Should be able to do this mutably.
                self.grid[next_slot.y * self.width + next_slot.x] = next_slot.clone();
            }

            if next_slot.superposition.is_empty() {
                return Err(format!(
                    "No possible modules for slot ({}, {})",
                    next_slot.x, next_slot.y
                ));
            }

            self.history.push(next_slot.clone());
            if let Err(err) = self.recurse() {
                return Err(err);
            }
            self.history.pop();
        }
        Ok(())
    }

    /// Run `attemps` iterations of the algorithm.
    pub fn collapse(&mut self, attemps: i32) -> Result<(), String> {
        for _ in 0..attemps {
            self.recurse()?;
            self.history.clear();
            if self.is_collapsed() {
                return Ok(());
            }
        }
        Ok(())
    }
}

/// Returns the default constraint function.
/// The constraint function is used to calculate the `connectorID` for a slot.
/// Only modules that have the same `connector` as the slot's module are considered possible.
/// The default constraint function checks pixel equality.
/// Use the `sample_size` to control the number of pixels to compare on each side of the slot.
pub fn get_constraint_fn(
    sample_size: usize,
) -> Box<dyn Fn(&Box<dyn Image>, Direction) -> ConnectorID> {
    let count = sample_size + 1; // +1 for the center pixel

    Box::new(move |img: &Box<dyn Image>, dir: Direction| {
        let (w, h) = img.size();

        let dx = w / count;
        let dy = h / count;

        let mut id = vec![];

        for idx in 0..count - 1 {
            let pixel = match dir {
                Direction::Up => img.get_pixel_at(dx + idx * dx, 0),
                Direction::Right => img.get_pixel_at(w as usize - 1, dy + idx * dy),
                Direction::Down => img.get_pixel_at(dx + idx * dx, h as usize - 1),
                Direction::Left => img.get_pixel_at(0, dy + idx * dy),
            };

            id.push(pixel);
        }

        ConnectorID::from(id.concat())
    })
}
