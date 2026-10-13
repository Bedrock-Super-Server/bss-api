/// A face of a block, numbered as the Bedrock protocol does.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFace {
    /// The bottom face, towards negative Y.
    Down = 0,
    /// The top face, towards positive Y.
    Up = 1,
    /// Towards negative Z.
    North = 2,
    /// Towards positive Z.
    South = 3,
    /// Towards negative X.
    West = 4,
    /// Towards positive X.
    East = 5,
}

impl BlockFace {
    /// The face numbered `value` by the protocol, or `None` for a number that names no face.
    pub const fn from_raw(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::North,
            3 => Self::South,
            4 => Self::West,
            5 => Self::East,
            _ => return None,
        })
    }
}
