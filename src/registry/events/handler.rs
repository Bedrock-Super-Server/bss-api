use parking_lot::Mutex;
use std::sync::Arc;

use crate::event::*;
use crate::registry::hook::{EventOptions, cancel_current, current_options, is_cancelled};
use crate::server::Server;

/**
A set of event handlers, registered with [`Server::register_event`].

Every method has a default that does nothing, so implement only the events you need. Methods that return `bool` can cancel their event by returning `true`. The [`event`](crate::event) module explains how events are cancelled and changed, with an example.
*/
pub trait Event: Send + Sync + 'static {
    /**
    Called when a player has finished joining and spawns in the world.

    See [`JoinEvent`](crate::event::JoinEvent).
    */
    fn on_join(&self, server: &Server, event: &JoinEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player leaves the server.

    See [`QuitEvent`](crate::event::QuitEvent).
    */
    fn on_quit(&self, server: &Server, event: &QuitEvent) {
        let _ = (server, event);
    }

    /**
    Called when a client has logged in, before it joins the world.

    See [`LoginEvent`](crate::event::LoginEvent).
    */
    fn on_login(&self, server: &Server, event: &LoginEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player dies.

    See [`DeathEvent`](crate::event::DeathEvent).
    */
    fn on_death(&self, server: &Server, event: &DeathEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player respawns after death.

    See [`RespawnEvent`](crate::event::RespawnEvent).
    */
    fn on_respawn(&self, server: &Server, event: &RespawnEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player starts or stops sneaking, sprinting, swimming or gliding.

    See [`ToggleEvent`](crate::event::ToggleEvent).
    */
    fn on_toggle(&self, server: &Server, event: &ToggleEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player sends a chat message.

    Returning `true` cancels the message: nobody sees it.

    See [`ChatEvent`](crate::event::ChatEvent).
    */
    fn on_chat(&self, server: &Server, event: &ChatEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player moves or turns.

    Returning `true` moves the player back to where they were.

    See [`MoveEvent`](crate::event::MoveEvent).
    */
    fn on_move(&self, server: &Server, event: &MoveEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player sends a command, before the server runs it.

    Returning `true` cancels the command.

    See [`CommandEvent`](crate::event::CommandEvent).
    */
    fn on_command(&self, server: &Server, event: &CommandEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player interacts with the world: clicks a block, starts or stops breaking it, uses an item in the air...

    Returning `true` cancels the interaction.

    See [`InteractEvent`](crate::event::InteractEvent).
    */
    fn on_interact(&self, server: &Server, event: &InteractEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player plays an animation, such as swinging their arm.

    Returning `true` hides the animation from other players.

    See [`AnimationEvent`](crate::event::AnimationEvent).
    */
    fn on_animation(&self, server: &Server, event: &AnimationEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player selects another hotbar slot.

    Returning `true` keeps the previous slot selected.

    See [`ItemHeldEvent`](crate::event::ItemHeldEvent).
    */
    fn on_item_held(&self, server: &Server, event: &ItemHeldEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player uses the item in their hand, on a block or in the air.

    Returning `true` cancels the use.

    See [`ItemUseEvent`](crate::event::ItemUseEvent).
    */
    fn on_item_use(&self, server: &Server, event: &ItemUseEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player drops an item from their inventory.

    Returning `true` keeps the items in the inventory.

    See [`DropItemEvent`](crate::event::DropItemEvent).
    */
    fn on_drop_item(&self, server: &Server, event: &DropItemEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player picks up an item from the ground.

    Returning `true` leaves the item on the ground.

    See [`PickupItemEvent`](crate::event::PickupItemEvent).
    */
    fn on_pickup_item(&self, server: &Server, event: &PickupItemEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player finishes eating or drinking an item.

    Returning `true` cancels the food the item gives.

    See [`ConsumeEvent`](crate::event::ConsumeEvent).
    */
    fn on_consume(&self, server: &Server, event: &ConsumeEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player is about to be teleported.

    Returning `true` cancels the teleport.

    See [`TeleportEvent`](crate::event::TeleportEvent).
    */
    fn on_teleport(&self, server: &Server, event: &TeleportEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player's game mode is about to change.

    Returning `true` keeps the current game mode.

    See [`GameModeChangeEvent`](crate::event::GameModeChangeEvent).
    */
    fn on_gamemode_change(&self, server: &Server, event: &GameModeChangeEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player breaks a block.

    Returning `true` cancels the break: the block is put back.

    See [`BlockBreakEvent`](crate::event::BlockBreakEvent).
    */
    fn on_block_break(&self, server: &Server, event: &BlockBreakEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player places a block.

    Returning `true` cancels the placement and keeps the item.

    See [`BlockPlaceEvent`](crate::event::BlockPlaceEvent).
    */
    fn on_block_place(&self, server: &Server, event: &BlockPlaceEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player equips a piece of armor.

    The value returned is ignored: this event cannot be cancelled.

    See [`ArmorEquipEvent`](crate::event::ArmorEquipEvent).
    */
    fn on_armor_equip(&self, server: &Server, event: &ArmorEquipEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player picks a block with the middle mouse button.

    Returning `true` cancels the pick.

    See [`BlockPickEvent`](crate::event::BlockPickEvent).
    */
    fn on_block_pick(&self, server: &Server, event: &BlockPickEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player starts, continues or stops breaking a block.

    Returning `true` stops the progress.

    See [`BlockBreakProgressEvent`](crate::event::BlockBreakProgressEvent).
    */
    fn on_block_break_progress(&self, server: &Server, event: &BlockBreakProgressEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a furnace finishes smelting an item.

    See [`FurnaceSmeltEvent`](crate::event::FurnaceSmeltEvent).
    */
    fn on_furnace_smelt(&self, server: &Server, event: &FurnaceSmeltEvent) {
        let _ = (server, event);
    }

    /**
    Called when a block receives a random tick, which drives slow changes such as crop growth or leaf decay.

    Returning `true` skips this tick.

    See [`RandomTickEvent`](crate::event::RandomTickEvent).
    */
    fn on_random_tick(&self, server: &Server, event: &RandomTickEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a chunk is loaded into memory.

    See [`ChunkLoadEvent`](crate::event::ChunkLoadEvent).
    */
    fn on_chunk_load(&self, server: &Server, event: &ChunkLoadEvent) {
        let _ = (server, event);
    }

    /**
    Called when a chunk is unloaded from memory.

    See [`ChunkUnloadEvent`](crate::event::ChunkUnloadEvent).
    */
    fn on_chunk_unload(&self, server: &Server, event: &ChunkUnloadEvent) {
        let _ = (server, event);
    }

    /**
    Called when the weather changes.

    See [`WeatherChangeEvent`](crate::event::WeatherChangeEvent).
    */
    fn on_weather_change(&self, server: &Server, event: &WeatherChangeEvent) {
        let _ = (server, event);
    }

    /**
    Called when an entity appears in the world, whatever the reason.

    See [`EntitySpawnEvent`](crate::event::EntitySpawnEvent).
    */
    fn on_entity_spawn(&self, server: &Server, event: &EntitySpawnEvent) {
        let _ = (server, event);
    }

    /**
    Called when an entity is removed from the world, whatever the reason.

    See [`EntityDespawnEvent`](crate::event::EntityDespawnEvent).
    */
    fn on_entity_despawn(&self, server: &Server, event: &EntityDespawnEvent) {
        let _ = (server, event);
    }

    /**
    Called when a world is saved to disk.

    See [`WorldSaveEvent`](crate::event::WorldSaveEvent).
    */
    fn on_world_save(&self, server: &Server, event: &WorldSaveEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player fills or empties a bucket.

    See [`BucketUseEvent`](crate::event::BucketUseEvent).
    */
    fn on_bucket_use(&self, server: &Server, event: &BucketUseEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player attacks an entity in melee.

    Returning `true` cancels the hit.

    See [`EntityDamageEvent`](crate::event::EntityDamageEvent).
    */
    fn on_entity_damage(&self, server: &Server, event: &EntityDamageEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when an entity dies.

    Returning `true` keeps the entity alive.

    See [`EntityDeathEvent`](crate::event::EntityDeathEvent).
    */
    fn on_entity_death(&self, server: &Server, event: &EntityDeathEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player attacks or interacts with an entity.

    See [`EntityInteractEvent`](crate::event::EntityInteractEvent).
    */
    fn on_entity_interact(&self, server: &Server, event: &EntityInteractEvent) {
        let _ = (server, event);
    }

    /**
    Called when a container window is about to open for a player.

    Returning `true` keeps the window closed.

    See [`InventoryOpenEvent`](crate::event::InventoryOpenEvent).
    */
    fn on_inventory_open(&self, server: &Server, event: &InventoryOpenEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a container window closes.

    See [`InventoryCloseEvent`](crate::event::InventoryCloseEvent).
    */
    fn on_inventory_close(&self, server: &Server, event: &InventoryCloseEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player moves, swaps, drops or uses items in an inventory.

    Returning `true` rejects the whole request: the client is told to put everything back.

    See [`InventoryTransactionEvent`](crate::event::InventoryTransactionEvent).
    */
    fn on_inventory_transaction(&self, server: &Server, event: &InventoryTransactionEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when the server runs a game tick, twenty times per second.

    See [`TickEvent`](crate::event::TickEvent).
    */
    fn on_tick(&self, server: &Server, event: &TickEvent) {
        let _ = (server, event);
    }

    /**
    Called when a player plays an emote.

    Returning `true` hides the emote from other players.

    See [`EmoteEvent`](crate::event::EmoteEvent).
    */
    fn on_emote(&self, server: &Server, event: &EmoteEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player changes skin while in game.

    Returning `true` keeps the previous skin.

    See [`SkinChangeEvent`](crate::event::SkinChangeEvent).
    */
    fn on_skin_change(&self, server: &Server, event: &SkinChangeEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player answers or closes a form.

    Returning `true` stops other handlers and the form callback from seeing the answer.

    See [`FormResponseEvent`](crate::event::FormResponseEvent).
    */
    fn on_form_response(&self, server: &Server, event: &FormResponseEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a player opens the server settings page of their game settings.

    See [`ServerSettingsRequestEvent`](crate::event::ServerSettingsRequestEvent).
    */
    fn on_server_settings_request(&self, server: &Server, event: &ServerSettingsRequestEvent) {
        let _ = (server, event);
    }

    /**
    Called when a game packet passes between a player and the server.

    Returning `true` drops the packet.

    See [`PacketEvent`](crate::event::PacketEvent).
    */
    fn on_server_bound(&self, server: &Server, event: &PacketEvent) -> bool {
        let _ = (server, event);
        false
    }

    /**
    Called when a game packet passes between a player and the server.

    Returning `true` drops the packet.

    See [`PacketEvent`](crate::event::PacketEvent).
    */
    fn on_client_bound(&self, server: &Server, event: &PacketEvent) -> bool {
        let _ = (server, event);
        false
    }

    behaviour_methods!();
}

type Registry = Arc<Vec<Arc<dyn Event>>>;

/// The handlers registered with the same options, which share one closure per event.
struct Group {
    options: EventOptions,
    handlers: Mutex<Registry>,
}

static GROUPS: Mutex<Vec<&'static Group>> = Mutex::new(Vec::new());

impl Server {
    /// Registers a set of event handlers at the normal priority, or with the options of
    /// [`with_event_options`](Server::with_event_options). Several can be registered; each event
    /// reaches all of them, in the order they were registered.
    pub fn register_event<E: Event>(&self, handler: E) {
        self.register_event_with(handler, current_options());
    }

    /// Registers a set of event handlers with `options`: their priority, and whether they skip
    /// events another handler cancelled.
    pub fn register_event_with<E: Event>(&self, handler: E, options: EventOptions) {
        let (group, created) = {
            let mut groups = GROUPS.lock();
            match groups.iter().find(|group| group.options == options) {
                Some(group) => (*group, false),
                None => {
                    let group: &'static Group = Box::leak(Box::new(Group {
                        options,
                        handlers: Mutex::new(Arc::new(Vec::new())),
                    }));
                    groups.push(group);
                    (group, true)
                }
            }
        };
        {
            let mut handlers = group.handlers.lock();
            let mut updated = Vec::clone(&handlers);
            updated.push(Arc::new(handler));
            *handlers = Arc::new(updated);
        }
        if created {
            self.with_event_options(options, |server| install(server, group));
        }
    }
}

fn install(server: &Server, group: &'static Group) {
    server.on_join(move |s, e| notify(group, |h| h.on_join(s, e)));
    server.on_quit(move |s, e| notify(group, |h| h.on_quit(s, e)));
    server.on_login(move |s, e| notify(group, |h| h.on_login(s, e)));
    server.on_death(move |s, e| notify(group, |h| h.on_death(s, e)));
    server.on_respawn(move |s, e| notify(group, |h| h.on_respawn(s, e)));
    server.on_toggle(move |s, e| notify(group, |h| h.on_toggle(s, e)));
    server.on_furnace_smelt(move |s, e| notify(group, |h| h.on_furnace_smelt(s, e)));
    server.on_random_tick(move |s, e| cancel(group, |h| h.on_random_tick(s, e)));
    server.on_chunk_load(move |s, e| notify(group, |h| h.on_chunk_load(s, e)));
    server.on_chunk_unload(move |s, e| notify(group, |h| h.on_chunk_unload(s, e)));
    server.on_weather_change(move |s, e| notify(group, |h| h.on_weather_change(s, e)));
    server.on_entity_spawn(move |s, e| notify(group, |h| h.on_entity_spawn(s, e)));
    server.on_entity_despawn(move |s, e| notify(group, |h| h.on_entity_despawn(s, e)));
    server.on_world_save(move |s, e| notify(group, |h| h.on_world_save(s, e)));
    server.on_bucket_use(move |s, e| notify(group, |h| h.on_bucket_use(s, e)));
    server.on_entity_interact(move |s, e| notify(group, |h| h.on_entity_interact(s, e)));
    server.on_inventory_close(move |s, e| notify(group, |h| h.on_inventory_close(s, e)));
    server.on_tick(move |s, e| notify(group, |h| h.on_tick(s, e)));

    server.on_chat(move |s, e| cancel(group, |h| h.on_chat(s, e)));
    server.on_move(move |s, e| cancel(group, |h| h.on_move(s, e)));
    server.on_command(move |s, e| cancel(group, |h| h.on_command(s, e)));
    server.on_interact(move |s, e| cancel(group, |h| h.on_interact(s, e)));
    server.on_animation(move |s, e| cancel(group, |h| h.on_animation(s, e)));
    server.on_item_held(move |s, e| cancel(group, |h| h.on_item_held(s, e)));
    server.on_item_use(move |s, e| cancel(group, |h| h.on_item_use(s, e)));
    server.on_drop_item(move |s, e| cancel(group, |h| h.on_drop_item(s, e)));
    server.on_pickup_item(move |s, e| cancel(group, |h| h.on_pickup_item(s, e)));
    server.on_consume(move |s, e| cancel(group, |h| h.on_consume(s, e)));
    server.on_teleport(move |s, e| cancel(group, |h| h.on_teleport(s, e)));
    server.on_gamemode_change(move |s, e| cancel(group, |h| h.on_gamemode_change(s, e)));
    server.on_block_break(move |s, e| cancel(group, |h| h.on_block_break(s, e)));
    server.on_block_place(move |s, e| cancel(group, |h| h.on_block_place(s, e)));
    server.on_block_pick(move |s, e| cancel(group, |h| h.on_block_pick(s, e)));
    server.on_armor_equip(move |s, e| cancel(group, |h| h.on_armor_equip(s, e)));
    server.on_block_break_progress(move |s, e| cancel(group, |h| h.on_block_break_progress(s, e)));
    server.on_entity_damage(move |s, e| cancel(group, |h| h.on_entity_damage(s, e)));
    server.on_entity_death(move |s, e| cancel(group, |h| h.on_entity_death(s, e)));
    server.on_inventory_open(move |s, e| cancel(group, |h| h.on_inventory_open(s, e)));
    server
        .on_inventory_transaction(move |s, e| cancel(group, |h| h.on_inventory_transaction(s, e)));
    server.on_emote(move |s, e| cancel(group, |h| h.on_emote(s, e)));
    server.on_skin_change(move |s, e| cancel(group, |h| h.on_skin_change(s, e)));
    server.on_form_response(move |s, e| cancel(group, |h| h.on_form_response(s, e)));
    server.on_server_settings_request(move |s, e| {
        notify(group, |h| h.on_server_settings_request(s, e))
    });
    server.on_server_bound(move |s, e| cancel(group, |h| h.on_server_bound(s, e)));
    server.on_client_bound(move |s, e| cancel(group, |h| h.on_client_bound(s, e)));

    behaviour_installs!(server, group);
}

fn notify<F: FnMut(&Arc<dyn Event>)>(group: &Group, mut dispatch: F) {
    let handlers = group.handlers.lock().clone();
    for handler in handlers.iter() {
        dispatch(handler);
    }
}

/// Calls every handler of the group in turn; one that returns `true` cancels the event for the
/// handlers after it, and one may take the cancellation back with
/// [`set_cancelled`](crate::registry::hook::Cancellable::set_cancelled).
fn cancel<F: FnMut(&Arc<dyn Event>) -> bool>(group: &Group, mut dispatch: F) -> bool {
    let handlers = group.handlers.lock().clone();
    for handler in handlers.iter() {
        if group.options.ignore_cancelled && is_cancelled() {
            break;
        }
        if dispatch(handler) {
            cancel_current();
        }
    }
    false
}
