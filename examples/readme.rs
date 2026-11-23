//! The usage example from the README.

use motion_imagery_metadata::{St0601Value, decode};

fn main() {
    let heading = decode(5, &[0x71, 0xC2]);
    assert!(matches!(heading, St0601Value::Number(degrees) if (degrees - 159.9744).abs() < 1e-3));
}
