// Manually implementing Debug trait for tuple struct
use std::fmt;

pub struct Point2D(pub u8, pub u8);

impl fmt::Debug for Point2D {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Point2D")
            .field("x", &self.0)
            .field("y", &self.1)
            .finish()
    }
}