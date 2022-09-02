use rand::seq::SliceRandom;

use crate::module::Module;

/// A slot is a place in agrid where modules can be placed.
#[derive(Clone, Default)]
pub struct Slot {
    /// X coordinate of the slot.
    pub x: usize,
    /// Y coordinate of the slot.
    pub y: usize,
    /// Possible modules in the slot.
    pub superposition: Vec<Module>,
}

impl Slot {
    pub fn collapse(&mut self) {
        if let Some(module) = self.superposition.choose(&mut rand::thread_rng()) {
            self.superposition = vec![*module];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::conector::ConnectorID;

    use super::*;

    #[test]
    fn test_collapse() {
        let mut slot = Slot {
            x: 0,
            y: 0,
            superposition: vec![
                Module {
                    connectors: [
                        ConnectorID::from("a"),
                        ConnectorID::from("b"),
                        ConnectorID::from("a"),
                        ConnectorID::from("b"),
                    ],
                    ..Default::default()
                },
                Module {
                    connectors: [
                        ConnectorID::from("c"),
                        ConnectorID::from("d"),
                        ConnectorID::from("e"),
                        ConnectorID::from("f"),
                    ],
                    ..Default::default()
                },
            ],
        };

        slot.collapse();
        assert_eq!(slot.superposition.len(), 1);
    }
}
