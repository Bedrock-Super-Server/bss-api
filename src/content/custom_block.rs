use super::{
    BlockDef, BlockPermutation, BlockShape, BlockSound, BlockState, Condition, GroupRef, ToolType,
};
use crate::server::Server;

/**
A block added by a plugin, registered with [`Server::register_custom_block`].

Only [`id`](Self::id), [`display`](Self::display) and [`texture`](Self::texture) are required; the defaults make a full cube that breaks in a second and a half with any tool. See the [`content`](crate::content) module for an example.
*/
pub trait CustomBlock: Send + Sync + 'static {
    /// Namespaced id of the block, such as `shop:ruby_block`.
    fn id(&self) -> String;

    /// Name shown in game.
    fn display(&self) -> String;

    /// Name of the block texture in the resource pack the server sends, used on every face that [`faces`](Self::faces) does not override.
    fn texture(&self) -> String;

    /// Textures of single faces, as `(face, texture)`; a face is `up`, `down`, `north`, `south`, `east`, `west`, `sides` or `*` for all of them.
    fn faces(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Identifier of the geometry in the resource pack.
    fn geometry(&self) -> String {
        "minecraft:geometry.full_block".to_string()
    }

    /// Collision box as `(origin, size)`, in sixteenths of a block, with the origin relative to the bottom centre of the block: a full block is `([-8.0, 0.0, -8.0], [16.0, 16.0, 16.0])`. `None` keeps a full block.
    fn collision(&self) -> Option<([f32; 3], [f32; 3])> {
        None
    }

    /// Outline shown when a player looks at the block, in the same units as [`collision`](Self::collision).
    fn selection(&self) -> Option<([f32; 3], [f32; 3])> {
        None
    }

    /// Time to break the block by hand, in ticks.
    fn break_ticks(&self) -> i32 {
        30
    }

    /// Tool that breaks the block faster.
    fn tool(&self) -> Option<ToolType> {
        None
    }

    /// The sound the block makes when it is placed, broken, walked on, hit or landed on. See [`BlockSound`] to let the
    /// client choose, to take a sound of the game, or to play your own.
    fn sound(&self) -> BlockSound {
        BlockSound::Default
    }

    /// The form of the block, which the server places and breaks for you. See [`BlockShape`].
    fn shape(&self) -> BlockShape {
        BlockShape::Cube
    }

    /// The states of the block, besides those its [`shape`](Self::shape) gives. See [`BlockState`].
    fn states(&self) -> Vec<BlockState> {
        Vec::new()
    }

    /// What changes in the block for some of its states, besides what its [`shape`](Self::shape) gives. See [`BlockPermutation`].
    fn permutations(&self) -> Vec<BlockPermutation> {
        Vec::new()
    }

    /// The bones of the geometry that are shown only when a condition holds, as `(bone, condition)`: the way a fence
    /// shows the rails of the sides it is linked on. A bone that is not listed is always shown. See [`Condition`].
    fn bone_visibility(&self) -> Vec<(String, Condition)> {
        Vec::new()
    }

    /// Whether the block is a slab: the server then also registers its upper half, `<id>_top`, which players get by placing it on the top of a block.
    ///
    /// [`BlockShape::Slab`] is the lower half alone, with no upper half nor double slab. A plugin that places its own
    /// blocks has no use for either: register the upper half as a block of its own, with its geometry and boxes, and
    /// place it yourself.
    fn slab(&self) -> bool {
        false
    }

    /// The creative inventory group the block is in: one that a plugin defines with [`CustomGroup`](super::CustomGroup), or one of the game, see [`VanillaGroup`](super::VanillaGroup). Without one the block is not in the creative inventory.
    fn group(&self) -> Option<GroupRef> {
        None
    }
}

/// What the server is told about a block.
fn definition<C: CustomBlock>(block: &C) -> BlockDef {
    let sound = block.sound();
    BlockDef {
        id: block.id(),
        display: block.display(),
        texture: block.texture(),
        faces: block.faces(),
        geometry: block.geometry(),
        collision: block.collision(),
        selection: block.selection(),
        break_ticks: block.break_ticks(),
        tool: block.tool(),
        sound: sound.name().map(str::to_string),
        shape: block.shape(),
        states: block.states(),
        permutations: block.permutations(),
        bone_visibility: block.bone_visibility(),
        custom_sound: sound == BlockSound::Custom,
        slab: block.slab(),
        group: block.group(),
    }
}

impl Server {
    /// Registers a [`CustomBlock`]. Call it from [`Plugin::register`](crate::sdk::Plugin::register).
    pub fn register_custom_block<C: CustomBlock>(&self, block: &C) {
        self.push_block(&definition(block));
    }
}
