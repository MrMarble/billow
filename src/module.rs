use crate::{conector::ConnectorID, direction::Direction, slot::Slot};

/// Module represents a tile that can inhabit a slot.
#[derive(Debug, Clone, Copy, Default)]
pub struct Module {
    pub index: usize,
    /// Valid connectors for each direction.
    pub connectors: [ConnectorID; 4],
    // TODO: add Image field. (for now, we just use the index)
    //pub image: Box<dyn Image>,
}

impl Module {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            connectors: [ConnectorID::default(); 4],
        }
    }

    pub fn is_possible(&self, from: &Slot, dir: Direction) -> bool {
        let back = dir.reverse();

        for slot in from.superposition.iter() {
            if self.connectors[back as usize] == slot.connectors[dir as usize] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        let module_a = Module {
            connectors: [
                ConnectorID::from("a"),
                ConnectorID::from("b"),
                ConnectorID::from("a"),
                ConnectorID::from("b"),
            ],
            ..Default::default()
        };

        let module_b = Module {
            connectors: [
                ConnectorID::from("c"),
                ConnectorID::from("d"),
                ConnectorID::from("e"),
                ConnectorID::from("f"),
            ],
            ..Default::default()
        };

        let slot = Slot {
            superposition: vec![module_a.clone()],
            x: 0,
            y: 0,
        };

        assert!(module_a.is_possible(&slot, Direction::Up));
        assert!(module_a.is_possible(&slot, Direction::Left));
        assert!(!module_b.is_possible(&slot, Direction::Up));
    }
}
