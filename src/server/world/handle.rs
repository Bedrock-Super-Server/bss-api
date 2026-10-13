use crate::ffi::abi::FfiStr;
use crate::ffi::call::HostFn;
use crate::ffi::host::{BlockEdit, HostApi};
use crate::server::world::{Block, Pos};

/**
A loaded world, from [`Server::world`](crate::server::Server::world).

```
use bss_api::prelude::*;

fn build_platform(server: &Server) {
    let arena = server.world("arena");
    let stone = Block::named("minecraft:stone");
    arena.fill(Pos::new(-5, 63, -5), Pos::new(5, 63, 5), stone);
    arena.set_weather(Weather::Clear, None);
}
```
*/
pub struct World {
    pub(super) host: &'static HostApi,
    pub(super) name: String,
}

/// The weather of a world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weather {
    /// No rain.
    Clear,
    /// Rain, or snow in cold biomes.
    Rain,
    /// A thunderstorm.
    Thunder,
}

/// The weather of a world and when it changes next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeatherState {
    /// Whether it rains.
    pub raining: bool,
    /// Whether there is a thunderstorm.
    pub thundering: bool,
    /// Ticks until rain starts or stops.
    pub rain_ticks: u32,
    /// Ticks until a thunderstorm starts or stops.
    pub thunder_ticks: u32,
}

impl WeatherState {
    /// Returns the weather as one value.
    pub fn kind(&self) -> Weather {
        match (self.raining, self.thundering) {
            (true, true) => Weather::Thunder,
            (true, false) => Weather::Rain,
            _ => Weather::Clear,
        }
    }
}

impl World {
    pub(crate) fn new(host: &'static HostApi, name: String) -> Self {
        Self { host, name }
    }

    /// Returns the name of the world.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the block at `pos`.
    pub fn get(&self, pos: Pos) -> Block {
        let runtime_id = host_call!(
            self.host,
            world_block_get(FfiStr::new(&self.name), pos.x, pos.y, pos.z)
        )
        .unwrap_or(0);
        Block::from_runtime_id(runtime_id)
    }

    /// Places `block` at `pos` and shows it to players.
    pub fn set(&self, pos: Pos, block: Block) {
        host_call!(
            self.host,
            world_block_set(
                FfiStr::new(&self.name),
                pos.x,
                pos.y,
                pos.z,
                block.runtime_id()
            )
        );
    }

    /// Fills the box between `from` and `to`, both included, with `block`, and returns how many blocks changed.
    pub fn fill(&self, from: Pos, to: Pos, block: Block) -> u32 {
        let Some(f) = self.host.world_blocks_set else {
            return 0;
        };
        let (min_x, min_y, min_z) = (from.x.min(to.x), from.y.min(to.y), from.z.min(to.z));
        let (max_x, max_y, max_z) = (from.x.max(to.x), from.y.max(to.y), from.z.max(to.z));

        let mut edits = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    edits.push(BlockEdit {
                        x,
                        y,
                        z,
                        runtime_id: block.runtime_id(),
                    });
                }
            }
        }
        f.call((FfiStr::new(&self.name), edits.as_ptr(), edits.len() as u32))
    }

    /// Returns the weather, or `None` when weather is turned off in `server.toml` or the world is not loaded.
    pub fn weather(&self) -> Option<WeatherState> {
        let f = self.host.world_weather?;
        let (mut raining, mut thundering, mut rain_ticks, mut thunder_ticks) = (false, false, 0, 0);
        let known = f.call((
            FfiStr::new(&self.name),
            &raw mut raining,
            &raw mut thundering,
            &raw mut rain_ticks,
            &raw mut thunder_ticks,
        ));
        known.then_some(WeatherState {
            raining,
            thundering,
            rain_ticks,
            thunder_ticks,
        })
    }

    /// Changes the weather for `duration_ticks` ticks, or a natural random duration with `None`. Returns `false` when weather is turned off, the world is not loaded or a plugin cancelled the change.
    pub fn set_weather(&self, weather: Weather, duration_ticks: Option<u32>) -> bool {
        let Some(f) = self.host.world_set_weather else {
            return false;
        };
        let (raining, thundering) = match weather {
            Weather::Clear => (false, false),
            Weather::Rain => (true, false),
            Weather::Thunder => (true, true),
        };
        f.call((
            FfiStr::new(&self.name),
            raining,
            thundering,
            duration_ticks.unwrap_or(0),
        ))
    }

    /// Strikes lightning at `(x, y, z)` and returns the runtime id of the bolt, or `0` when it did not strike.
    pub fn strike_lightning(&self, x: f32, y: f32, z: f32) -> u64 {
        host_call!(
            self.host,
            world_strike_lightning(FfiStr::new(&self.name), x, y, z)
        )
        .unwrap_or(0)
    }
}
