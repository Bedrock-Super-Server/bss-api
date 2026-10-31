/*!
The small types that stand for what the server used to give as bare numbers: the game mode and the
dimension of a player, where they are, and the ids of items and blocks.

Each converts from and to the number the server uses, so a plugin can still compare one with a
number or pass a number where one is taken.

```
use bss_api::prelude::*;

assert_eq!(GameMode::Creative, 1);
assert_eq!(GameMode::try_from(6), Ok(GameMode::Spectator));
assert_eq!(i32::from(Dimension::Nether), 1);

let spot = Location::new(1.5, 70.0, -3.5);
assert_eq!(spot.block(), (1, 70, -4));
```
*/

use std::fmt;

/// The game mode of a player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GameMode {
    /// Survival: the player takes damage and gathers resources.
    Survival,
    /// Creative: every block and item, flight, no damage.
    Creative,
    /// Adventure: the player cannot break or place blocks.
    Adventure,
    /// Spectator: the player flies through everything and cannot interact.
    Spectator,
}

impl GameMode {
    /// The number the game gives the mode: `0` survival, `1` creative, `2` adventure, `6` spectator.
    pub const fn id(self) -> i32 {
        match self {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 6,
        }
    }

    /// The mode a number stands for, or `None` for a number that is none.
    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(GameMode::Survival),
            1 => Some(GameMode::Creative),
            2 => Some(GameMode::Adventure),
            6 => Some(GameMode::Spectator),
            _ => None,
        }
    }

    /// The name of the mode, as `/gamemode` takes it.
    pub const fn name(self) -> &'static str {
        match self {
            GameMode::Survival => "survival",
            GameMode::Creative => "creative",
            GameMode::Adventure => "adventure",
            GameMode::Spectator => "spectator",
        }
    }
}

impl From<GameMode> for i32 {
    fn from(mode: GameMode) -> i32 {
        mode.id()
    }
}

impl TryFrom<i32> for GameMode {
    type Error = i32;

    /// The mode a number stands for, or the number back when it is none.
    fn try_from(id: i32) -> Result<Self, i32> {
        GameMode::from_id(id).ok_or(id)
    }
}

impl PartialEq<i32> for GameMode {
    fn eq(&self, other: &i32) -> bool {
        self.id() == *other
    }
}

/// As JSON a mode is its number, the way the game and the server give it.
impl serde::Serialize for GameMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.id())
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A dimension of a world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Dimension {
    /// The overworld.
    Overworld,
    /// The Nether.
    Nether,
    /// The End.
    End,
}

impl Dimension {
    /// The number the game gives the dimension: `0` overworld, `1` Nether, `2` End.
    pub const fn id(self) -> i32 {
        match self {
            Dimension::Overworld => 0,
            Dimension::Nether => 1,
            Dimension::End => 2,
        }
    }

    /// The dimension a number stands for, or `None` for a number that is none.
    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Dimension::Overworld),
            1 => Some(Dimension::Nether),
            2 => Some(Dimension::End),
            _ => None,
        }
    }
}

impl From<Dimension> for i32 {
    fn from(dimension: Dimension) -> i32 {
        dimension.id()
    }
}

impl TryFrom<i32> for Dimension {
    type Error = i32;

    /// The dimension a number stands for, or the number back when it is none.
    fn try_from(id: i32) -> Result<Self, i32> {
        Dimension::from_id(id).ok_or(id)
    }
}

impl PartialEq<i32> for Dimension {
    fn eq(&self, other: &i32) -> bool {
        self.id() == *other
    }
}

/// As JSON a dimension is its number, the way the game and the server give it.
impl serde::Serialize for Dimension {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.id())
    }
}

/// Where something is and the way it looks: the feet at `(x, y, z)`, and the angles in degrees.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Location {
    /// East-west position.
    pub x: f32,
    /// Height of the feet.
    pub y: f32,
    /// North-south position.
    pub z: f32,
    /// Up and down angle, from `-90` (looking up) to `90` (looking down).
    pub pitch: f32,
    /// Turn around, in degrees.
    pub yaw: f32,
}

impl Location {
    /// A place, looking straight ahead.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    /// The same place, looking the way `pitch` and `yaw` say.
    pub const fn facing(mut self, pitch: f32, yaw: f32) -> Self {
        self.pitch = pitch;
        self.yaw = yaw;
        self
    }

    /// The position and the angles as the tuple `(x, y, z, pitch, yaw)` that 1.x gave.
    pub const fn to_tuple(self) -> (f32, f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.pitch, self.yaw)
    }

    /// The block the position is in.
    pub fn block(&self) -> (i32, i32, i32) {
        (
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        )
    }

    /// The distance to `other`, ignoring where each looks.
    pub fn distance(&self, other: &Location) -> f32 {
        let (dx, dy, dz) = (self.x - other.x, self.y - other.y, self.z - other.z);
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl From<Location> for (f32, f32, f32, f32, f32) {
    fn from(at: Location) -> Self {
        (at.x, at.y, at.z, at.pitch, at.yaw)
    }
}

impl From<(f32, f32, f32, f32, f32)> for Location {
    fn from((x, y, z, pitch, yaw): (f32, f32, f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            z,
            pitch,
            yaw,
        }
    }
}

/// The numeric id of an item, such as the one of `minecraft:diamond`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemId(pub i16);

impl ItemId {
    /// The empty hand.
    pub const NONE: ItemId = ItemId(0);
}

impl From<i16> for ItemId {
    fn from(id: i16) -> Self {
        ItemId(id)
    }
}

impl From<ItemId> for i16 {
    fn from(id: ItemId) -> i16 {
        id.0
    }
}

/// The runtime id of one state of a block, which the server gives and takes for blocks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId(pub u32);

impl BlockId {
    /// The runtime id the server gives air, and a block form an item does not have.
    pub const NONE: BlockId = BlockId(0);
}

impl From<u32> for BlockId {
    fn from(id: u32) -> Self {
        BlockId(id)
    }
}

impl From<BlockId> for u32 {
    fn from(id: BlockId) -> u32 {
        id.0
    }
}
