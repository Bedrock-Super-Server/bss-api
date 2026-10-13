use super::{BlockDef, ToolType};
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

    /// Whether the block is a slab: the server then also registers its upper half, `<id>_top`, which players get by placing it on the top of a block.
    fn slab(&self) -> bool {
        false
    }
}

impl Server {
    /// Registers a [`CustomBlock`]. Call it from [`Plugin::register`](crate::sdk::Plugin::register).
    pub fn register_custom_block<C: CustomBlock>(&self, block: &C) {
        self.push_block(&BlockDef {
            id: block.id(),
            display: block.display(),
            texture: block.texture(),
            faces: block.faces(),
            geometry: block.geometry(),
            collision: block.collision(),
            selection: block.selection(),
            break_ticks: block.break_ticks(),
            tool: block.tool(),
            slab: block.slab(),
        });
    }
}
