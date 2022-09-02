use sha2::Digest;
use sha2::Sha256;
use std::fmt;

/// A connector is a unique identifier for a specific image and direction.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConnectorID([u8; 4]);

impl ConnectorID {
    /// Create a new ConnectorID.
    /// ```
    /// use billow::ConnectorID;
    ///
    /// let id = ConnectorID::from("left"); // from a string to manually create a ConnectorID
    /// let id = ConnectorID::from(vec![155,155,155,255]); // from a rgb color
    /// ```
    pub fn from(id: impl AsRef<[u8]>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(id);
        Self(hasher.finalize()[..4].try_into().unwrap())
    }
}

impl fmt::Display for ConnectorID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}{:02X}{:02X}{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let id = ConnectorID::from("a");
        assert_eq!(id.0, [202, 151, 129, 18]);
    }

    #[test]
    fn test_from_array() {
        let id = ConnectorID::from([115, 155, 155, 255]);
        assert_eq!(id.0, [69, 209, 46, 83]);
    }

    #[test]
    fn test_from_vec() {
        let id = ConnectorID::from(vec![115, 155, 155, 255]);
        assert_eq!(id.0, [69, 209, 46, 83]);
    }

    #[test]
    fn test_display() {
        let id = ConnectorID::from("a");
        assert_eq!(id.to_string(), "CA978112");
    }

    #[test]
    fn test_compare() {
        let id1 = ConnectorID::from("very long string");
        let id2 = ConnectorID::from("shorter string");
        assert_ne!(id1, id2);
        assert_eq!(id1, ConnectorID::from("very long string"));
    }
}
