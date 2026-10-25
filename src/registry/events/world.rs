use crate::event::world::{
    BlockBreakEvent, BlockBreakProgressEvent, BlockPickEvent, BlockPlaceEvent, ChunkLoadEvent,
    ChunkUnloadEvent, FurnaceSmeltEvent, RandomTickEvent, WeatherChangeEvent, WorldSaveEvent,
};
use crate::ffi::events::*;
use crate::registry::macros::{event_hook, event_hook_cancel};
use crate::server::{Server, ffi_string};

event_hook_cancel!(
    on_block_break,
    BLOCK_BREAK,
    block_break,
    BlockBreakEventData,
    BlockBreakEvent,
    |raw: &BlockBreakEventData| BlockBreakEvent {
        name: ffi_string(raw.name),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        runtime_id: raw.runtime_id,
        item_id: raw.item_id,
        held_item: raw.held_item,
        held_uid: raw.held_uid,
        held_damage: raw.held_damage,
        final_break: raw.final_break,
        drop: std::cell::Cell::new(raw.drop),
        drop_item: std::cell::Cell::new(raw.drop_item),
        drop_count: std::cell::Cell::new(raw.drop_count),
    },
    [drop, drop_item, drop_count]
);

event_hook_cancel!(
    on_block_place,
    BLOCK_PLACE,
    block_place,
    BlockPlaceEventData,
    BlockPlaceEvent,
    |e: &BlockPlaceEventData| BlockPlaceEvent {
        name: ffi_string(e.name),
        x: e.x,
        y: e.y,
        z: e.z,
        runtime_id: e.runtime_id,
        face: crate::math::BlockFace::from_raw(e.face).unwrap_or(crate::math::BlockFace::Up),
        item_id: e.item_id,
    }
);

event_hook_cancel!(
    on_block_pick,
    BLOCK_PICK,
    block_pick,
    BlockPickEventData,
    BlockPickEvent,
    |e: &BlockPickEventData| BlockPickEvent {
        name: ffi_string(e.name),
        x: e.x,
        y: e.y,
        z: e.z,
        runtime_id: e.runtime_id,
        item_id: e.item_id,
        hotbar_slot: e.hotbar_slot,
    }
);

event_hook_cancel!(
    on_block_break_progress,
    BLOCK_BREAK_PROGRESS,
    block_break_progress,
    BlockBreakProgressEventData,
    BlockBreakProgressEvent,
    |e: &BlockBreakProgressEventData| BlockBreakProgressEvent {
        name: ffi_string(e.name),
        x: e.x,
        y: e.y,
        z: e.z,
        runtime_id: e.runtime_id,
        stage: e.stage,
    }
);

event_hook_cancel!(
    on_random_tick,
    RANDOM_TICK,
    random_tick,
    RandomTickEventData,
    RandomTickEvent,
    |e: &RandomTickEventData| RandomTickEvent {
        x: e.x,
        y: e.y,
        z: e.z,
        runtime_id: e.runtime_id,
    }
);

event_hook!(
    on_furnace_smelt,
    FURNACE_SMELT,
    furnace_smelt,
    FurnaceSmeltEventData,
    FurnaceSmeltEvent,
    |e: &FurnaceSmeltEventData| FurnaceSmeltEvent {
        x: e.x,
        y: e.y,
        z: e.z,
        input_item_id: e.input_item_id,
        result_item_id: e.result_item_id,
    }
);

event_hook!(
    on_chunk_load,
    CHUNK_LOAD,
    chunk_load,
    ChunkLoadEventData,
    ChunkLoadEvent,
    |e: &ChunkLoadEventData| ChunkLoadEvent {
        cx: e.cx,
        cz: e.cz,
        generated: e.generated,
    }
);

event_hook!(
    on_chunk_unload,
    CHUNK_UNLOAD,
    chunk_unload,
    ChunkUnloadEventData,
    ChunkUnloadEvent,
    |e: &ChunkUnloadEventData| ChunkUnloadEvent { cx: e.cx, cz: e.cz }
);

event_hook!(
    on_weather_change,
    WEATHER_CHANGE,
    weather_change,
    WeatherChangeEventData,
    WeatherChangeEvent,
    |e: &WeatherChangeEventData| WeatherChangeEvent {
        kind: e.kind,
        level: e.level,
    }
);

event_hook!(
    on_world_save,
    WORLD_SAVE,
    world_save,
    WorldSaveEventData,
    WorldSaveEvent,
    |e: &WorldSaveEventData| WorldSaveEvent {
        name: ffi_string(e.name)
    }
);
