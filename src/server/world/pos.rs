/// The position of a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate, the height.
    pub y: i32,
    /// Z coordinate.
    pub z: i32,
}

impl Pos {
    /// The block at `(x, y, z)`.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
