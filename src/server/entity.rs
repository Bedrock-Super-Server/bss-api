/*!
Entities that plugins add and control: NPCs, holograms, decorations.

These entities are kept with the world and come back after a restart. Mobs that the world spawns on its own are not listed here; plugins see them through the [`event`](crate::event) types.

# Example

```
use bss_api::prelude::*;

fn spawn_guide(server: &Server) -> u64 {
    let id = server.next_entity_id();
    server.add_entity(id, "minecraft:villager_v2", 0.5, 65.0, 0.5);
    server.set_entity_nametag(id, "§aGuide\n§7Right-click me");
    server.set_entity_flag(id, ActorFlag::NoAi, true);
    server.set_entity_invulnerable(id, true);
    id
}
```
*/

use crate::ffi::abi::FfiStr;
use crate::ffi::call::HostFn;
use crate::ffi::host::EntityData;
use crate::server::{Server, ffi_string, read_all};

/// A visual or behavioural flag of an entity, set with [`Server::set_entity_flag`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorFlag {
    /// Burning.
    OnFire,
    /// Sneaking.
    Sneaking,
    /// Riding another entity.
    Riding,
    /// Sprinting.
    Sprinting,
    /// Using an item, such as drawing a bow.
    UsingItem,
    /// Invisible.
    Invisible,
    /// A baby.
    Baby,
    /// Ignited, like a creeper about to explode.
    Ignited,
    /// Powered, like a charged creeper.
    Powered,
    /// Shows critical hit particles.
    Critical,
    /// Shows its name tag.
    CanShowNametag,
    /// Has no AI: it does not move on its own.
    NoAi,
    /// Makes no sound.
    Silent,
    /// Climbing a wall, like a spider.
    WallClimbing,
    /// Swimming.
    Swimming,
    /// Gliding with elytra.
    Gliding,
    /// Angry, like a wolf.
    Angry,
    /// Charged, like a ghast about to shoot.
    Charged,
}

impl ActorFlag {
    /// Returns the index of the flag in the Bedrock entity flags.
    pub fn bit(self) -> u32 {
        match self {
            ActorFlag::OnFire => 0,
            ActorFlag::Sneaking => 1,
            ActorFlag::Riding => 2,
            ActorFlag::Sprinting => 3,
            ActorFlag::UsingItem => 4,
            ActorFlag::Invisible => 5,
            ActorFlag::Baby => 11,
            ActorFlag::Ignited => 10,
            ActorFlag::Powered => 9,
            ActorFlag::Critical => 13,
            ActorFlag::CanShowNametag => 14,
            ActorFlag::NoAi => 16,
            ActorFlag::Silent => 17,
            ActorFlag::WallClimbing => 18,
            ActorFlag::Swimming => 56,
            ActorFlag::Gliding => 57,
            ActorFlag::Angry => 20,
            ActorFlag::Charged => 23,
        }
    }
}

/// A snapshot of an entity added by a plugin.
#[derive(Clone)]
pub struct Entity {
    /// Runtime id of the entity.
    pub entity_id: u64,
    /// Type of the entity, such as `minecraft:villager_v2`.
    pub identifier: String,
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
    /// Pitch, in degrees.
    pub pitch: f32,
    /// Yaw, in degrees.
    pub yaw: f32,
    /// Name tag shown above it.
    pub nametag: String,
    /// Size, where `1.0` is the normal size.
    pub scale: f32,
    /// Whether gravity is turned off.
    pub no_gravity: bool,
    /// Whether it cannot move.
    pub immobile: bool,
    /// Whether it is invisible.
    pub invisible: bool,
}

impl Entity {
    pub(crate) fn from_ffi(data: &EntityData) -> Option<Self> {
        if data.entity_id == 0 {
            return None;
        }
        Some(Self {
            entity_id: data.entity_id,
            identifier: ffi_string(data.identifier),
            x: data.x,
            y: data.y,
            z: data.z,
            pitch: data.pitch,
            yaw: data.yaw,
            nametag: ffi_string(data.nametag),
            scale: data.scale,
            no_gravity: data.no_gravity,
            immobile: data.immobile,
            invisible: data.invisible,
        })
    }
}

impl Server {
    /// Makes the client accept a custom entity type, such as one defined in a resource pack. [`add_entity`](Self::add_entity) does it on its own. Returns `false` when the type was already known.
    pub fn register_entity_type(&self, identifier: &str) -> bool {
        host_call!(self.host(), entity_register_type(FfiStr::new(identifier))).unwrap_or(false)
    }

    /**
    Adds an entity of type `identifier` at `(x, y, z)` in the default world, under `entity_id` from [`next_entity_id`](Self::next_entity_id). See the
    [module documentation](self) for an example.
    */
    pub fn add_entity(&self, entity_id: u64, identifier: &str, x: f32, y: f32, z: f32) -> bool {
        host_call!(
            self.host(),
            entity_add(entity_id, FfiStr::new(identifier), x, y, z)
        )
        .unwrap_or(false)
    }

    /// Removes an entity. Returns `false` when there is no such entity.
    pub fn remove_entity(&self, entity_id: u64) -> bool {
        host_call!(self.host(), entity_remove(entity_id)).unwrap_or(false)
    }

    /// Moves an entity and turns it to `pitch` and `yaw`, in degrees.
    pub fn move_entity(
        &self,
        entity_id: u64,
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
    ) -> bool {
        host_call!(self.host(), entity_move(entity_id, x, y, z, pitch, yaw)).unwrap_or(false)
    }

    /// Sets the name tag shown above an entity; `\n` starts a new line.
    pub fn set_entity_nametag(&self, entity_id: u64, nametag: &str) -> bool {
        host_call!(
            self.host(),
            entity_set_nametag(entity_id, FfiStr::new(nametag))
        )
        .unwrap_or(false)
    }

    /// Returns a snapshot of an entity.
    pub fn entity(&self, entity_id: u64) -> Option<Entity> {
        let f = self.host().entity_get?;
        let data = f.call((entity_id,));
        Entity::from_ffi(&data)
    }

    /// Returns a snapshot of every entity added by plugins.
    pub fn entities(&self) -> Vec<Entity> {
        let Some(f) = self.host().entity_all else {
            return Vec::new();
        };
        let mut entities = Vec::new();
        read_all(|buffer: &mut [EntityData]| {
            let count = f.call((buffer.as_mut_ptr(), buffer.len() as u32)) as usize;
            // The identifiers live in the host's scratch buffer: copy them now.
            entities = buffer
                .iter()
                .take(count)
                .filter_map(Entity::from_ffi)
                .collect();
            count
        });
        entities
    }

    /// Returns a snapshot of every entity of type `identifier`.
    pub fn entities_of(&self, identifier: &str) -> Vec<Entity> {
        let Some(f) = self.host().entity_all_of else {
            return Vec::new();
        };
        let mut entities = Vec::new();
        read_all(|buffer: &mut [EntityData]| {
            let count = f.call((
                FfiStr::new(identifier),
                buffer.as_mut_ptr(),
                buffer.len() as u32,
            )) as usize;
            entities = buffer
                .iter()
                .take(count)
                .filter_map(Entity::from_ffi)
                .collect();
            count
        });
        entities
    }

    /// Returns the position of an entity as `(x, y, z, pitch, yaw)`.
    pub fn entity_position(&self, entity_id: u64) -> Option<(f32, f32, f32, f32, f32)> {
        let f = self.host().entity_pos?;
        let (mut x, mut y, mut z, mut pitch, mut yaw) = (0.0, 0.0, 0.0, 0.0, 0.0);
        f.call((
            entity_id,
            &raw mut x,
            &raw mut y,
            &raw mut z,
            &raw mut pitch,
            &raw mut yaw,
        ))
        .then_some((x, y, z, pitch, yaw))
    }

    /// Returns the name of the world an entity is in.
    pub fn entity_world(&self, entity_id: u64) -> String {
        host_call!(self.host(), entity_world(entity_id))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Moves an entity to the loaded world named `world`, at the same coordinates.
    pub fn set_entity_world(&self, entity_id: u64, world: &str) -> bool {
        host_call!(self.host(), entity_set_world(entity_id, FfiStr::new(world))).unwrap_or(false)
    }
}

impl Server {
    /// Damages an entity by `amount` half-hearts and returns its health left, or `-1.0` when there is no such entity.
    pub fn damage_entity(&self, entity_id: u64, amount: f32) -> f32 {
        self.damage_entity_from(entity_id, amount, 0.0, 0.0, false)
    }

    /// Damages an entity as a hit from `(from_x, from_z)` would, knocking it back when `knockback` is set. Returns its health left, or `-1.0`.
    pub fn damage_entity_from(
        &self,
        entity_id: u64,
        amount: f32,
        from_x: f32,
        from_z: f32,
        knockback: bool,
    ) -> f32 {
        host_call!(
            self.host(),
            entity_damage(entity_id, amount, from_x, from_z, knockback)
        )
        .unwrap_or(-1.0)
    }

    /// Heals an entity by `amount` half-hearts, up to its maximum, and returns its health, or `-1.0` when there is no such entity.
    pub fn heal_entity(&self, entity_id: u64, amount: f32) -> f32 {
        host_call!(self.host(), entity_heal(entity_id, amount)).unwrap_or(-1.0)
    }

    /// Sets the health and the maximum health of an entity.
    pub fn set_entity_health(&self, entity_id: u64, health: f32, max_health: f32) -> bool {
        host_call!(
            self.host(),
            entity_set_health(entity_id, health, max_health)
        )
        .unwrap_or(false)
    }

    /// Turns a flag of an entity on or off.
    pub fn set_entity_flag(&self, entity_id: u64, flag: ActorFlag, value: bool) -> bool {
        host_call!(self.host(), entity_set_flag(entity_id, flag.bit(), value)).unwrap_or(false)
    }

    /// Turns a flag on or off by its Bedrock index, for one [`ActorFlag`] does not list. Indexes from `63` up are refused.
    pub fn set_entity_flag_raw(&self, entity_id: u64, flag_bit: u32, value: bool) -> bool {
        host_call!(self.host(), entity_set_flag(entity_id, flag_bit, value)).unwrap_or(false)
    }

    /// Sets the size of an entity, where `1.0` is the normal size.
    pub fn set_entity_scale(&self, entity_id: u64, scale: f32) -> bool {
        host_call!(self.host(), entity_set_scale(entity_id, scale)).unwrap_or(false)
    }

    /// Sets the hitbox of an entity, in blocks.
    pub fn set_entity_hitbox(&self, entity_id: u64, width: f32, height: f32) -> bool {
        host_call!(self.host(), entity_set_hitbox(entity_id, width, height)).unwrap_or(false)
    }

    /// Returns the health and the maximum health of an entity, or `(-1.0, 0.0)` when there is no such entity.
    pub fn entity_health(&self, entity_id: u64) -> (f32, f32) {
        let Some(f) = self.host().entity_health else {
            return (-1.0, 0.0);
        };
        let mut max = 0.0;
        let health = f.call((entity_id, &raw mut max));
        (health, max)
    }

    /// Makes an entity ignore damage, or not.
    pub fn set_entity_invulnerable(&self, entity_id: u64, invulnerable: bool) -> bool {
        host_call!(
            self.host(),
            entity_set_invulnerable(entity_id, invulnerable)
        )
        .unwrap_or(false)
    }
}
