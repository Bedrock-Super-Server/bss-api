macro_rules! behaviour_methods {
    () => {
        /**
        Called when a player finishes editing the text of a sign.

        `event.cancel()` leaves the sign unchanged.

        See [`SignChangeEvent`](crate::event::SignChangeEvent).
        */
        fn on_sign_change(&self, server: &Server, event: &SignChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player uses a block: opens a door or a container, presses a button, flips a lever, rings a bell...

        `event.cancel()` cancels the use: the block does not react.

        See [`BlockUseEvent`](crate::event::BlockUseEvent).
        */
        fn on_block_use(&self, server: &Server, event: &BlockUseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a plant grows a stage: crops, stems, sweet berries, cocoa, cactus, sugar cane...

        `event.cancel()` stops this growth step.

        See [`BlockGrowEvent`](crate::event::BlockGrowEvent).
        */
        fn on_block_grow(&self, server: &Server, event: &BlockGrowEvent) {
            let _ = (server, event);
        }

        /**
        Called when a sapling or a fungus grows into a tree.

        `event.cancel()` keeps the sapling as it is.

        See [`TreeGrowEvent`](crate::event::TreeGrowEvent).
        */
        fn on_tree_grow(&self, server: &Server, event: &TreeGrowEvent) {
            let _ = (server, event);
        }

        /**
        Called when a block affected by gravity, such as sand or an anvil, starts to fall.

        `event.cancel()` keeps the block in place.

        See [`BlockFallEvent`](crate::event::BlockFallEvent).
        */
        fn on_block_fall(&self, server: &Server, event: &BlockFallEvent) {
            let _ = (server, event);
        }

        /**
        Called when a falling block lands and is about to become a block again.

        `event.cancel()` drops the block as an item instead of placing it.

        See [`BlockLandEvent`](crate::event::BlockLandEvent).
        */
        fn on_block_land(&self, server: &Server, event: &BlockLandEvent) {
            let _ = (server, event);
        }

        /**
        Called when a block forms in the world on its own: lava and water make obsidian or cobblestone, snow piles up, ice freezes...

        `event.cancel()` prevents the block from forming.

        See [`BlockFormEvent`](crate::event::BlockFormEvent).
        */
        fn on_block_form(&self, server: &Server, event: &BlockFormEvent) {
            let _ = (server, event);
        }

        /**
        Called when a block is set on fire, or a block that can be lit is lit: fire, a campfire, a candle, TNT...

        `event.cancel()` prevents the ignition.

        See [`BlockIgniteEvent`](crate::event::BlockIgniteEvent).
        */
        fn on_block_ignite(&self, server: &Server, event: &BlockIgniteEvent) {
            let _ = (server, event);
        }

        /**
        Called when tNT is primed and starts its fuse.

        `event.cancel()` keeps the TNT block unprimed.

        See [`TntPrimeEvent`](crate::event::TntPrimeEvent).
        */
        fn on_tnt_prime(&self, server: &Server, event: &TntPrimeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a note block plays a note, because a player hit it or it received a redstone signal.

        `event.cancel()` keeps the note block silent.

        See [`NotePlayEvent`](crate::event::NotePlayEvent).
        */
        fn on_note_play(&self, server: &Server, event: &NotePlayEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player inserts a music disc into a jukebox or takes it out.

        `event.cancel()` leaves the jukebox as it was.

        See [`JukeboxChangeEvent`](crate::event::JukeboxChangeEvent).
        */
        fn on_jukebox_change(&self, server: &Server, event: &JukeboxChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player places a book on a lectern or takes it.

        `event.cancel()` leaves the lectern as it was.

        See [`LecternChangeEvent`](crate::event::LecternChangeEvent).
        */
        fn on_lectern_change(&self, server: &Server, event: &LecternChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player turns the page of a book on a lectern.

        `event.cancel()` keeps the current page.

        See [`LecternPageEvent`](crate::event::LecternPageEvent).
        */
        fn on_lectern_page(&self, server: &Server, event: &LecternPageEvent) {
            let _ = (server, event);
        }

        /**
        Called when a furnace, smoker, blast furnace or campfire finishes cooking an item.

        `event.cancel()` cancels the result: the input stays in place.

        See [`ItemCookEvent`](crate::event::ItemCookEvent).
        */
        fn on_item_cook(&self, server: &Server, event: &ItemCookEvent) {
            let _ = (server, event);
        }

        /**
        Called when a furnace, smoker or blast furnace starts burning a fuel item.

        `event.cancel()` keeps the fuel unburnt.

        See [`FuelBurnEvent`](crate::event::FuelBurnEvent).
        */
        fn on_fuel_burn(&self, server: &Server, event: &FuelBurnEvent) {
            let _ = (server, event);
        }

        /**
        Called when a brewing stand finishes brewing.

        `event.cancel()` cancels the brew.

        See [`BrewEvent`](crate::event::BrewEvent).
        */
        fn on_brew(&self, server: &Server, event: &BrewEvent) {
            let _ = (server, event);
        }

        /**
        Called when a beacon is about to give its effect to a player in range.

        `event.cancel()` withholds the effect this time.

        See [`BeaconEffectEvent`](crate::event::BeaconEffectEvent).
        */
        fn on_beacon_effect(&self, server: &Server, event: &BeaconEffectEvent) {
            let _ = (server, event);
        }

        /**
        Called when water or lava is about to flow into a block.

        `event.cancel()` stops the liquid from flowing into this block.

        See [`LiquidFlowEvent`](crate::event::LiquidFlowEvent).
        */
        fn on_liquid_flow(&self, server: &Server, event: &LiquidFlowEvent) {
            let _ = (server, event);
        }

        /**
        Called when a sculk shrieker shrieks because a player stepped near it or made a vibration it heard.

        `event.cancel()` keeps the shrieker silent.

        See [`SculkShriekEvent`](crate::event::SculkShriekEvent).
        */
        fn on_sculk_shriek(&self, server: &Server, event: &SculkShriekEvent) {
            let _ = (server, event);
        }

        /**
        Called when an explosion is about to happen.

        `event.cancel()` cancels the explosion.

        See [`ExplosionEvent`](crate::event::ExplosionEvent).
        */
        fn on_explosion(&self, server: &Server, event: &ExplosionEvent) {
            let _ = (server, event);
        }

        /**
        Called when an explosion is about to destroy blocks.

        `event.cancel()` keeps every block in place.

        See [`ExplosionBlocksEvent`](crate::event::ExplosionBlocksEvent).
        */
        fn on_explosion_blocks(&self, server: &Server, event: &ExplosionBlocksEvent) {
            let _ = (server, event);
        }

        /**
        Called when lightning is about to strike.

        `event.cancel()` cancels the strike.

        See [`LightningStrikeEvent`](crate::event::LightningStrikeEvent).
        */
        fn on_lightning_strike(&self, server: &Server, event: &LightningStrikeEvent) {
            let _ = (server, event);
        }

        /**
        Called when rain starts or stops.

        `event.cancel()` keeps the current weather.

        See [`RainChangeEvent`](crate::event::RainChangeEvent).
        */
        fn on_rain_change(&self, server: &Server, event: &RainChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a thunderstorm starts or stops.

        `event.cancel()` keeps the current weather.

        See [`ThunderChangeEvent`](crate::event::ThunderChangeEvent).
        */
        fn on_thunder_change(&self, server: &Server, event: &ThunderChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when the weather changes a block: snow piles up, water freezes, rain puts a fire out...

        `event.cancel()` leaves the block as it is.

        See [`WeatherEffectEvent`](crate::event::WeatherEffectEvent).
        */
        fn on_weather_effect(&self, server: &Server, event: &WeatherEffectEvent) {
            let _ = (server, event);
        }

        /**
        Called when the spawn point of a world is about to move, with `/setworldspawn` or a plugin. Set another position to move it there instead.

        `event.cancel()` keeps the spawn point where it is.

        See [`SpawnChangeEvent`](crate::event::SpawnChangeEvent).
        */
        fn on_spawn_change(&self, server: &Server, event: &SpawnChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a world finishes loading and is ready for players and plugins.

        `event.cancel()` changes nothing.

        See [`WorldLoadEvent`](crate::event::WorldLoadEvent).
        */
        fn on_world_load(&self, server: &Server, event: &WorldLoadEvent) {
            let _ = (server, event);
        }

        /**
        Called when the tick rate of the server is about to change, through `/tick` or a plugin.

        `event.cancel()` keeps the current rate.

        See [`TickRateChangeEvent`](crate::event::TickRateChangeEvent).
        */
        fn on_tick_rate_change(&self, server: &Server, event: &TickRateChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when the redstone power of a block changes.

        `event.cancel()` keeps the previous power.

        See [`RedstonePowerEvent`](crate::event::RedstonePowerEvent).
        */
        fn on_redstone_power(&self, server: &Server, event: &RedstonePowerEvent) {
            let _ = (server, event);
        }

        /**
        Called when a piston extends or retracts.

        `event.cancel()` keeps the piston still.

        See [`PistonMoveEvent`](crate::event::PistonMoveEvent).
        */
        fn on_piston_move(&self, server: &Server, event: &PistonMoveEvent) {
            let _ = (server, event);
        }

        /**
        Called when a piston pushes an entity.

        `event.cancel()` leaves the entity where it is.

        See [`PistonPushEvent`](crate::event::PistonPushEvent).
        */
        fn on_piston_push(&self, server: &Server, event: &PistonPushEvent) {
            let _ = (server, event);
        }

        /**
        Called when a minecart passes over a powered activator rail.

        `event.cancel()` ignores the rail.

        See [`MinecartActivateEvent`](crate::event::MinecartActivateEvent).
        */
        fn on_minecart_activate(&self, server: &Server, event: &MinecartActivateEvent) {
            let _ = (server, event);
        }

        /**
        Called when a hopper moves an item into or out of a minecart.

        `event.cancel()` cancels the transfer.

        See [`MinecartTransferEvent`](crate::event::MinecartTransferEvent).
        */
        fn on_minecart_transfer(&self, server: &Server, event: &MinecartTransferEvent) {
            let _ = (server, event);
        }

        /**
        Called when a hopper minecart picks up an item entity.

        `event.cancel()` leaves the item where it is.

        See [`MinecartPickupEvent`](crate::event::MinecartPickupEvent).
        */
        fn on_minecart_pickup(&self, server: &Server, event: &MinecartPickupEvent) {
            let _ = (server, event);
        }

        /**
        Called when a dispenser or dropper fires an item.

        `event.cancel()` cancels the shot.

        See [`BlockDispenseEvent`](crate::event::BlockDispenseEvent).
        */
        fn on_block_dispense(&self, server: &Server, event: &BlockDispenseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a hopper moves an item from one container to another.

        `event.cancel()` cancels the transfer.

        See [`ItemTransferEvent`](crate::event::ItemTransferEvent).
        */
        fn on_item_transfer(&self, server: &Server, event: &ItemTransferEvent) {
            let _ = (server, event);
        }

        /**
        Called when a hopper picks up an item entity lying above it.

        `event.cancel()` leaves the item where it is.

        See [`HopperPickupEvent`](crate::event::HopperPickupEvent).
        */
        fn on_hopper_pickup(&self, server: &Server, event: &HopperPickupEvent) {
            let _ = (server, event);
        }

        /**
        Called when a tripwire is triggered or released.

        `event.cancel()` leaves the tripwire as it was.

        See [`TripwireEvent`](crate::event::TripwireEvent).
        */
        fn on_tripwire(&self, server: &Server, event: &TripwireEvent) {
            let _ = (server, event);
        }

        /**
        Called when a minecart turns a detector rail on or off.

        `event.cancel()` leaves the rail as it was.

        See [`DetectorRailEvent`](crate::event::DetectorRailEvent).
        */
        fn on_detector_rail(&self, server: &Server, event: &DetectorRailEvent) {
            let _ = (server, event);
        }

        /**
        Called when a sculk sensor detects a vibration.

        `event.cancel()` keeps the sensor from reacting.

        See [`SculkVibrationEvent`](crate::event::SculkVibrationEvent).
        */
        fn on_sculk_vibration(&self, server: &Server, event: &SculkVibrationEvent) {
            let _ = (server, event);
        }

        /**
        Called when a crafter crafts an item.

        `event.cancel()` cancels the craft.

        See [`CrafterCraftEvent`](crate::event::CrafterCraftEvent).
        */
        fn on_crafter_craft(&self, server: &Server, event: &CrafterCraftEvent) {
            let _ = (server, event);
        }

        /**
        Called when a dispenser puts armor on a player in front of it.

        `event.cancel()` keeps the armor in the dispenser.

        See [`DispenserEquipEvent`](crate::event::DispenserEquipEvent).
        */
        fn on_dispenser_equip(&self, server: &Server, event: &DispenserEquipEvent) {
            let _ = (server, event);
        }

        /**
        Called when something steps on or leaves a pressure plate.

        `event.cancel()` keeps the plate in its previous state.

        See [`PressurePlateEvent`](crate::event::PressurePlateEvent).
        */
        fn on_pressure_plate(&self, server: &Server, event: &PressurePlateEvent) {
            let _ = (server, event);
        }

        /**
        Called when a portal is about to be created: a Nether portal is lit or generated for travel, or an End portal is completed.

        `event.cancel()` prevents the portal from being created.

        See [`PortalCreateEvent`](crate::event::PortalCreateEvent).
        */
        fn on_portal_create(&self, server: &Server, event: &PortalCreateEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player goes through a portal.

        `event.cancel()` keeps the player where they are.

        See [`PlayerPortalEvent`](crate::event::PlayerPortalEvent).
        */
        fn on_player_portal(&self, server: &Server, event: &PlayerPortalEvent) {
            let _ = (server, event);
        }

        /**
        Called when an entity other than a player goes through a portal.

        `event.cancel()` keeps the entity where it is.

        See [`EntityPortalEvent`](crate::event::EntityPortalEvent).
        */
        fn on_entity_portal(&self, server: &Server, event: &EntityPortalEvent) {
            let _ = (server, event);
        }

        /**
        Called when an End gateway is about to be generated after a dragon fight.

        `event.cancel()` prevents the gateway.

        See [`EndGatewayEvent`](crate::event::EndGatewayEvent).
        */
        fn on_end_gateway(&self, server: &Server, event: &EndGatewayEvent) {
            let _ = (server, event);
        }

        /**
        Called when an ender dragon is about to spawn in the End.

        `event.cancel()` prevents the dragon from spawning.

        See [`DragonSpawnEvent`](crate::event::DragonSpawnEvent).
        */
        fn on_dragon_spawn(&self, server: &Server, event: &DragonSpawnEvent) {
            let _ = (server, event);
        }

        /**
        Called when the ender dragon changes its behaviour phase.

        The value returned is ignored: this event cannot be cancelled.

        See [`DragonPhaseEvent`](crate::event::DragonPhaseEvent).
        */
        fn on_dragon_phase(&self, server: &Server, event: &DragonPhaseEvent) {
            let _ = (server, event);
        }

        /**
        Called when the ender dragon dies.

        The value returned is ignored: this event cannot be cancelled.

        See [`DragonDeathEvent`](crate::event::DragonDeathEvent).
        */
        fn on_dragon_death(&self, server: &Server, event: &DragonDeathEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player takes damage.

        `event.cancel()` cancels the damage.

        See [`PlayerDamageEvent`](crate::event::PlayerDamageEvent).
        */
        fn on_player_damage(&self, server: &Server, event: &PlayerDamageEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player regains health.

        `event.cancel()` cancels the healing.

        See [`PlayerHealEvent`](crate::event::PlayerHealEvent).
        */
        fn on_player_heal(&self, server: &Server, event: &PlayerHealEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player catches fire.

        `event.cancel()` keeps the player from burning.

        See [`PlayerCombustEvent`](crate::event::PlayerCombustEvent).
        */
        fn on_player_combust(&self, server: &Server, event: &PlayerCombustEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player's hunger bar changes.

        `event.cancel()` keeps the previous values.

        See [`FoodChangeEvent`](crate::event::FoodChangeEvent).
        */
        fn on_food_change(&self, server: &Server, event: &FoodChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player gains or loses experience.

        `event.cancel()` leaves the experience unchanged.

        See [`ExperienceChangeEvent`](crate::event::ExperienceChangeEvent).
        */
        fn on_experience_change(&self, server: &Server, event: &ExperienceChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player picks up an experience orb.

        `event.cancel()` leaves the orb where it is.

        See [`ExperiencePickupEvent`](crate::event::ExperiencePickupEvent).
        */
        fn on_experience_pickup(&self, server: &Server, event: &ExperiencePickupEvent) {
            let _ = (server, event);
        }

        /**
        Called when mending repairs an item with experience the player picked up.

        `event.cancel()` cancels the repair.

        See [`ItemMendEvent`](crate::event::ItemMendEvent).
        */
        fn on_item_mend(&self, server: &Server, event: &ItemMendEvent) {
            let _ = (server, event);
        }

        /**
        Called when a tool, weapon or piece of armor is about to lose durability.

        `event.cancel()` prevents the loss.

        See [`ItemDamageEvent`](crate::event::ItemDamageEvent).
        */
        fn on_item_damage(&self, server: &Server, event: &ItemDamageEvent) {
            let _ = (server, event);
        }

        /**
        Called when a tool, weapon or piece of armor breaks after losing its last durability point.

        The value returned is ignored: this event cannot be cancelled.

        See [`ItemBreakEvent`](crate::event::ItemBreakEvent).
        */
        fn on_item_break(&self, server: &Server, event: &ItemBreakEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player blocks an attack with a shield.

        `event.cancel()` cancels it.

        See [`ShieldBlockEvent`](crate::event::ShieldBlockEvent).
        */
        fn on_shield_block(&self, server: &Server, event: &ShieldBlockEvent) {
            let _ = (server, event);
        }

        /**
        Called when a totem of undying saves a player from death.

        `event.cancel()` stops the totem from saving the player.

        See [`TotemUseEvent`](crate::event::TotemUseEvent).
        */
        fn on_totem_use(&self, server: &Server, event: &TotemUseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player teleports with an ender pearl or a chorus fruit.

        `event.cancel()` cancels the teleport.

        See [`PlayerWarpEvent`](crate::event::PlayerWarpEvent).
        */
        fn on_player_warp(&self, server: &Server, event: &PlayerWarpEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player gets on or off a vehicle or a mob.

        `event.cancel()` cancels it.

        See [`PlayerMountEvent`](crate::event::PlayerMountEvent).
        */
        fn on_player_mount(&self, server: &Server, event: &PlayerMountEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player moves the vehicle they ride.

        `event.cancel()` keeps the vehicle where it was.

        See [`VehicleMoveEvent`](crate::event::VehicleMoveEvent).
        */
        fn on_vehicle_move(&self, server: &Server, event: &VehicleMoveEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player loads a crossbow.

        `event.cancel()` keeps the crossbow unloaded.

        See [`CrossbowLoadEvent`](crate::event::CrossbowLoadEvent).
        */
        fn on_crossbow_load(&self, server: &Server, event: &CrossbowLoadEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player launches with a Riptide trident.

        `event.cancel()` cancels the launch.

        See [`TridentRiptideEvent`](crate::event::TridentRiptideEvent).
        */
        fn on_trident_riptide(&self, server: &Server, event: &TridentRiptideEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player casts a fishing rod, or reels it in.

        `event.cancel()` cancels the cast or the catch.

        See [`FishEvent`](crate::event::FishEvent).
        */
        fn on_fish(&self, server: &Server, event: &FishEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player blows a goat horn.

        `event.cancel()` keeps the horn silent.

        See [`HornBlowEvent`](crate::event::HornBlowEvent).
        */
        fn on_horn_blow(&self, server: &Server, event: &HornBlowEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player edits or signs a book and quill.

        `event.cancel()` rejects the edit.

        See [`BookEditEvent`](crate::event::BookEditEvent).
        */
        fn on_book_edit(&self, server: &Server, event: &BookEditEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player changes the item in their off hand.

        `event.cancel()` cancels it.

        See [`OffhandChangeEvent`](crate::event::OffhandChangeEvent).
        */
        fn on_offhand_change(&self, server: &Server, event: &OffhandChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player lies down in a bed: it is night or a thunderstorm, no monster is near and nobody else lies there.

        `event.cancel()` keeps the player standing; the bed still becomes their respawn point.

        See [`PlayerBedEnterEvent`](crate::event::PlayerBedEnterEvent).
        */
        fn on_player_bed_enter(&self, server: &Server, event: &PlayerBedEnterEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player gets out of bed. `cause` is `morning` when the world woke everyone up, `manual` when the player left by themselves, `damage`, `gamemode` or `travel` when something forced them out, and `bed_destroyed` when the bed is gone.

        `event.cancel()` changes nothing: the player leaves the bed anyway.

        See [`PlayerBedLeaveEvent`](crate::event::PlayerBedLeaveEvent).
        */
        fn on_player_bed_leave(&self, server: &Server, event: &PlayerBedLeaveEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player's client asks for another view distance, in chunks. `radius` is what the server grants, within the limit the player has: the `view_distance` of the server, or the one a plugin set with `Player::set_view_distance`; set another one to grant that instead, within the same limit.

        `event.cancel()` keeps the view distance the player has.

        See [`PlayerViewDistanceChangeEvent`](crate::event::PlayerViewDistanceChangeEvent).
        */
        fn on_player_view_distance_change(
            &self,
            server: &Server,
            event: &PlayerViewDistanceChangeEvent,
        ) {
            let _ = (server, event);
        }

        /**
        Called when something asks to disconnect a player: a plugin, the `/kick` command or the server's own protections.

        `event.cancel()` keeps the player connected.

        See [`PlayerKickEvent`](crate::event::PlayerKickEvent).
        */
        fn on_player_kick(&self, server: &Server, event: &PlayerKickEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player jumps: their client presses jump while they stand on the ground.

        `event.cancel()` changes nothing.

        See [`PlayerJumpEvent`](crate::event::PlayerJumpEvent).
        */
        fn on_player_jump(&self, server: &Server, event: &PlayerJumpEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player swings their arm and hits nothing: no entity, and no block they were breaking, using or placing.

        `event.cancel()` changes nothing.

        See [`PlayerSwingMissEvent`](crate::event::PlayerSwingMissEvent).
        */
        fn on_player_swing_miss(&self, server: &Server, event: &PlayerSwingMissEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player starts or stops flying, in creative or spectator mode or with the flight a plugin granted.

        `event.cancel()` changes nothing.

        See [`PlayerFlyToggleEvent`](crate::event::PlayerFlyToggleEvent).
        */
        fn on_player_fly_toggle(&self, server: &Server, event: &PlayerFlyToggleEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player starts or stops crawling.

        `event.cancel()` changes nothing.

        See [`PlayerCrawlEvent`](crate::event::PlayerCrawlEvent).
        */
        fn on_player_crawl(&self, server: &Server, event: &PlayerCrawlEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player attacks an entity or another player.

        `event.cancel()` stops the hit: nothing is hurt and the weapon does not wear.

        See [`PlayerAttackEvent`](crate::event::PlayerAttackEvent).
        */
        fn on_player_attack(&self, server: &Server, event: &PlayerAttackEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player lands after a fall that hurts them. Set another damage to hurt them less or more; cancelled, they take none.

        `event.cancel()` saves the player from the fall: they take no damage.

        See [`PlayerFallEvent`](crate::event::PlayerFallEvent).
        */
        fn on_player_fall(&self, server: &Server, event: &PlayerFallEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player moves to another world, or to another dimension of one.

        `event.cancel()` changes nothing.

        See [`PlayerWorldChangeEvent`](crate::event::PlayerWorldChangeEvent).
        */
        fn on_player_world_change(&self, server: &Server, event: &PlayerWorldChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player moves into another chunk.

        `event.cancel()` changes nothing.

        See [`PlayerChunkEnterEvent`](crate::event::PlayerChunkEnterEvent).
        */
        fn on_player_chunk_enter(&self, server: &Server, event: &PlayerChunkEnterEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player's client reports on the resource packs of the server.

        `event.cancel()` changes nothing: a client that refuses the packs is disconnected anyway.

        See [`ResourcePackStatusEvent`](crate::event::ResourcePackStatusEvent).
        */
        fn on_resource_pack_status(&self, server: &Server, event: &ResourcePackStatusEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player has done nothing for two minutes, or acts again.

        `event.cancel()` changes nothing.

        See [`PlayerAfkEvent`](crate::event::PlayerAfkEvent).
        */
        fn on_player_afk(&self, server: &Server, event: &PlayerAfkEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player is about to get a potion effect, from a potion, food, a totem, a command or a plugin; `effect` is its name, such as `speed`.

        `event.cancel()` keeps the player from getting the effect.

        See [`PlayerEffectAddEvent`](crate::event::PlayerEffectAddEvent).
        */
        fn on_player_effect_add(&self, server: &Server, event: &PlayerEffectAddEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player loses a potion effect: it ran out, milk or a command cleared it, or a totem replaced it.

        `event.cancel()` changes nothing: the effect is removed anyway.

        See [`PlayerEffectRemoveEvent`](crate::event::PlayerEffectRemoveEvent).
        */
        fn on_player_effect_remove(&self, server: &Server, event: &PlayerEffectRemoveEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player used a control of a data-driven screen: pressed a button, switched a toggle, moved a slider, chose an option or typed in a text field. The server has checked the value.

        `event.cancel()` puts the control back as it was, and keeps the handler of the screen from seeing it.

        See [`DduiResponseEvent`](crate::event::DduiResponseEvent).
        */
        fn on_ddui_response(&self, server: &Server, event: &DduiResponseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a data-driven screen closed.

        `event.cancel()` changes nothing.

        See [`DduiCloseEvent`](crate::event::DduiCloseEvent).
        */
        fn on_ddui_close(&self, server: &Server, event: &DduiCloseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player crafts an item.

        `event.cancel()` cancels the craft.

        See [`CraftItemEvent`](crate::event::CraftItemEvent).
        */
        fn on_craft_item(&self, server: &Server, event: &CraftItemEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player enchants an item at an enchanting table.

        `event.cancel()` cancels the enchantment.

        See [`EnchantItemEvent`](crate::event::EnchantItemEvent).
        */
        fn on_enchant_item(&self, server: &Server, event: &EnchantItemEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player takes the result out of an anvil.

        `event.cancel()` cancels the operation.

        See [`AnvilUseEvent`](crate::event::AnvilUseEvent).
        */
        fn on_anvil_use(&self, server: &Server, event: &AnvilUseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player takes the result out of a grindstone.

        `event.cancel()` cancels the operation.

        See [`GrindstoneUseEvent`](crate::event::GrindstoneUseEvent).
        */
        fn on_grindstone_use(&self, server: &Server, event: &GrindstoneUseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player takes the result out of a smithing table.

        `event.cancel()` cancels the operation.

        See [`SmithingUseEvent`](crate::event::SmithingUseEvent).
        */
        fn on_smithing_use(&self, server: &Server, event: &SmithingUseEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player changes the effects of a beacon.

        `event.cancel()` keeps the previous effects.

        See [`BeaconChangeEvent`](crate::event::BeaconChangeEvent).
        */
        fn on_beacon_change(&self, server: &Server, event: &BeaconChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player launches a projectile: an arrow, a trident, a firework, a fishing hook...

        `event.cancel()` cancels the launch.

        See [`ProjectileLaunchEvent`](crate::event::ProjectileLaunchEvent).
        */
        fn on_projectile_launch(&self, server: &Server, event: &ProjectileLaunchEvent) {
            let _ = (server, event);
        }

        /**
        Called when a projectile hits a block or an entity.

        `event.cancel()` cancels the hit.

        See [`ProjectileHitEvent`](crate::event::ProjectileHitEvent).
        */
        fn on_projectile_hit(&self, server: &Server, event: &ProjectileHitEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player picks up an arrow or a trident stuck in the world.

        `event.cancel()` leaves it in place.

        See [`ProjectilePickupEvent`](crate::event::ProjectilePickupEvent).
        */
        fn on_projectile_pickup(&self, server: &Server, event: &ProjectilePickupEvent) {
            let _ = (server, event);
        }

        /**
        Called when a thrown potion breaks.

        `event.cancel()` cancels the effect: the potion breaks without affecting anything.

        See [`PotionSplashEvent`](crate::event::PotionSplashEvent).
        */
        fn on_potion_splash(&self, server: &Server, event: &PotionSplashEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player places an entity from an item: an armor stand, a boat, a minecart, an end crystal...

        `event.cancel()` cancels the placement and keeps the item.

        See [`EntityPlaceEvent`](crate::event::EntityPlaceEvent).
        */
        fn on_entity_place(&self, server: &Server, event: &EntityPlaceEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player touches or hits an entity that has no behaviour of its own, such as a plugin entity.

        `event.cancel()` ignores the touch.

        See [`EntityTouchEvent`](crate::event::EntityTouchEvent).
        */
        fn on_entity_touch(&self, server: &Server, event: &EntityTouchEvent) {
            let _ = (server, event);
        }

        /**
        Called when an entity other than a player takes damage from any source.

        `event.cancel()` cancels the damage.

        See [`EntityHurtEvent`](crate::event::EntityHurtEvent).
        */
        fn on_entity_hurt(&self, server: &Server, event: &EntityHurtEvent) {
            let _ = (server, event);
        }

        /**
        Called when an entity other than a player is killed.

        `event.cancel()` keeps the entity alive, with one health point.

        See [`EntityKilledEvent`](crate::event::EntityKilledEvent).
        */
        fn on_entity_killed(&self, server: &Server, event: &EntityKilledEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player puts an item on an armor stand or takes one off.

        `event.cancel()` leaves the armor stand as it was.

        See [`ArmorStandChangeEvent`](crate::event::ArmorStandChangeEvent).
        */
        fn on_armor_stand_change(&self, server: &Server, event: &ArmorStandChangeEvent) {
            let _ = (server, event);
        }

        /**
        Called when an entity other than a player catches fire.

        `event.cancel()` keeps the entity from burning.

        See [`EntityCombustEvent`](crate::event::EntityCombustEvent).
        */
        fn on_entity_combust(&self, server: &Server, event: &EntityCombustEvent) {
            let _ = (server, event);
        }

        /**
        Called when an item entity appears in the world: a drop, a broken block, a dispenser throw...

        `event.cancel()` prevents the item from appearing.

        See [`ItemSpawnEvent`](crate::event::ItemSpawnEvent).
        */
        fn on_item_spawn(&self, server: &Server, event: &ItemSpawnEvent) {
            let _ = (server, event);
        }

        /**
        Called when a dropped item disappears after lying on the ground for five minutes.

        The value returned is ignored: this event cannot be cancelled.

        See [`ItemDespawnEvent`](crate::event::ItemDespawnEvent).
        */
        fn on_item_despawn(&self, server: &Server, event: &ItemDespawnEvent) {
            let _ = (server, event);
        }

        /**
        Called when a mob is about to spawn.

        `event.cancel()` prevents the spawn.

        See [`MobSpawnEvent`](crate::event::MobSpawnEvent).
        */
        fn on_mob_spawn(&self, server: &Server, event: &MobSpawnEvent) {
            let _ = (server, event);
        }

        /**
        Called when a mob is about to despawn because no player is close enough.

        `event.cancel()` keeps the mob.

        See [`MobDespawnEvent`](crate::event::MobDespawnEvent).
        */
        fn on_mob_despawn(&self, server: &Server, event: &MobDespawnEvent) {
            let _ = (server, event);
        }

        /**
        Called when a mob picks a new target.

        `event.cancel()` leaves the mob without a target.

        See [`MobTargetEvent`](crate::event::MobTargetEvent).
        */
        fn on_mob_target(&self, server: &Server, event: &MobTargetEvent) {
            let _ = (server, event);
        }

        /**
        Called when a mob attacks its target.

        `event.cancel()` cancels the attack.

        See [`MobAttackEvent`](crate::event::MobAttackEvent).
        */
        fn on_mob_attack(&self, server: &Server, event: &MobAttackEvent) {
            let _ = (server, event);
        }

        /**
        Called when two mobs breed and a baby is about to be born.

        `event.cancel()` prevents the birth.

        See [`MobBreedEvent`](crate::event::MobBreedEvent).
        */
        fn on_mob_breed(&self, server: &Server, event: &MobBreedEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player uses an item on a mob: feeds, tames, shears, milks, names...

        `event.cancel()` cancels the interaction.

        See [`MobInteractEvent`](crate::event::MobInteractEvent).
        */
        fn on_mob_interact(&self, server: &Server, event: &MobInteractEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player tames a mob.

        `event.cancel()` makes this taming attempt fail.

        See [`MobTameEvent`](crate::event::MobTameEvent).
        */
        fn on_mob_tame(&self, server: &Server, event: &MobTameEvent) {
            let _ = (server, event);
        }

        /**
        Called when a mob is leashed or unleashed.

        `event.cancel()` leaves the leash as it was.

        See [`MobLeashEvent`](crate::event::MobLeashEvent).
        */
        fn on_mob_leash(&self, server: &Server, event: &MobLeashEvent) {
            let _ = (server, event);
        }

        /**
        Called when a villager takes a profession by claiming a workstation.

        `event.cancel()` keeps the villager unemployed.

        See [`VillagerProfessionEvent`](crate::event::VillagerProfessionEvent).
        */
        fn on_villager_profession(&self, server: &Server, event: &VillagerProfessionEvent) {
            let _ = (server, event);
        }

        /**
        Called when a player trades with a villager or a wandering trader.

        `event.cancel()` cancels the trade.

        See [`VillagerTradeEvent`](crate::event::VillagerTradeEvent).
        */
        fn on_villager_trade(&self, server: &Server, event: &VillagerTradeEvent) {
            let _ = (server, event);
        }

        /**
        Called when a villager reaches a new trading level.

        The value returned is ignored: this event cannot be cancelled.

        See [`VillagerLevelUpEvent`](crate::event::VillagerLevelUpEvent).
        */
        fn on_villager_level_up(&self, server: &Server, event: &VillagerLevelUpEvent) {
            let _ = (server, event);
        }

        /**
        Called when the server finished starting and now accepts players. `worlds` is how many worlds are loaded.

        `event.cancel()` changes nothing.

        See [`ServerStartedEvent`](crate::event::ServerStartedEvent).
        */
        fn on_server_started(&self, server: &Server, event: &ServerStartedEvent) {
            let _ = (server, event);
        }

        /**
        Called when the server begins to stop, before it disconnects the players, unloads the plugins and saves the worlds. `players` is how many are online.

        `event.cancel()` does not keep the server from stopping.

        See [`ServerStopEvent`](crate::event::ServerStopEvent).
        */
        fn on_server_stop(&self, server: &Server, event: &ServerStopEvent) {
            let _ = (server, event);
        }

        /**
        Called when a line is typed in the server console, before it runs. Change `command` to run something else.

        `event.cancel()` keeps the line from running.

        See [`ConsoleCommandEvent`](crate::event::ConsoleCommandEvent).
        */
        fn on_console_command(&self, server: &Server, event: &ConsoleCommandEvent) {
            let _ = (server, event);
        }

        /**
        Called when a plugin finished loading and is now enabled, itself included.

        `event.cancel()` changes nothing.

        See [`PluginEnableEvent`](crate::event::PluginEnableEvent).
        */
        fn on_plugin_enable(&self, server: &Server, event: &PluginEnableEvent) {
            let _ = (server, event);
        }

        /**
        Called when a plugin is about to be disabled, by a reload, an unload or the server stopping.

        `event.cancel()` does not keep the plugin loaded.

        See [`PluginDisableEvent`](crate::event::PluginDisableEvent).
        */
        fn on_plugin_disable(&self, server: &Server, event: &PluginDisableEvent) {
            let _ = (server, event);
        }
    };
}

macro_rules! behaviour_installs {
    ($server:expr, $group:expr) => {
        $server.on_sign_change(move |s, e| cancel($group, |h| h.on_sign_change(s, e)));
        $server.on_block_use(move |s, e| cancel($group, |h| h.on_block_use(s, e)));
        $server.on_block_grow(move |s, e| cancel($group, |h| h.on_block_grow(s, e)));
        $server.on_tree_grow(move |s, e| cancel($group, |h| h.on_tree_grow(s, e)));
        $server.on_block_fall(move |s, e| cancel($group, |h| h.on_block_fall(s, e)));
        $server.on_block_land(move |s, e| cancel($group, |h| h.on_block_land(s, e)));
        $server.on_block_form(move |s, e| cancel($group, |h| h.on_block_form(s, e)));
        $server.on_block_ignite(move |s, e| cancel($group, |h| h.on_block_ignite(s, e)));
        $server.on_tnt_prime(move |s, e| cancel($group, |h| h.on_tnt_prime(s, e)));
        $server.on_note_play(move |s, e| cancel($group, |h| h.on_note_play(s, e)));
        $server.on_jukebox_change(move |s, e| cancel($group, |h| h.on_jukebox_change(s, e)));
        $server.on_lectern_change(move |s, e| cancel($group, |h| h.on_lectern_change(s, e)));
        $server.on_lectern_page(move |s, e| cancel($group, |h| h.on_lectern_page(s, e)));
        $server.on_item_cook(move |s, e| cancel($group, |h| h.on_item_cook(s, e)));
        $server.on_fuel_burn(move |s, e| cancel($group, |h| h.on_fuel_burn(s, e)));
        $server.on_brew(move |s, e| cancel($group, |h| h.on_brew(s, e)));
        $server.on_beacon_effect(move |s, e| cancel($group, |h| h.on_beacon_effect(s, e)));
        $server.on_liquid_flow(move |s, e| cancel($group, |h| h.on_liquid_flow(s, e)));
        $server.on_sculk_shriek(move |s, e| cancel($group, |h| h.on_sculk_shriek(s, e)));
        $server.on_explosion(move |s, e| cancel($group, |h| h.on_explosion(s, e)));
        $server.on_explosion_blocks(move |s, e| cancel($group, |h| h.on_explosion_blocks(s, e)));
        $server.on_lightning_strike(move |s, e| cancel($group, |h| h.on_lightning_strike(s, e)));
        $server.on_rain_change(move |s, e| cancel($group, |h| h.on_rain_change(s, e)));
        $server.on_thunder_change(move |s, e| cancel($group, |h| h.on_thunder_change(s, e)));
        $server.on_weather_effect(move |s, e| cancel($group, |h| h.on_weather_effect(s, e)));
        $server.on_spawn_change(move |s, e| cancel($group, |h| h.on_spawn_change(s, e)));
        $server.on_world_load(move |s, e| cancel($group, |h| h.on_world_load(s, e)));
        $server.on_tick_rate_change(move |s, e| cancel($group, |h| h.on_tick_rate_change(s, e)));
        $server.on_redstone_power(move |s, e| cancel($group, |h| h.on_redstone_power(s, e)));
        $server.on_piston_move(move |s, e| cancel($group, |h| h.on_piston_move(s, e)));
        $server.on_piston_push(move |s, e| cancel($group, |h| h.on_piston_push(s, e)));
        $server.on_minecart_activate(move |s, e| cancel($group, |h| h.on_minecart_activate(s, e)));
        $server.on_minecart_transfer(move |s, e| cancel($group, |h| h.on_minecart_transfer(s, e)));
        $server.on_minecart_pickup(move |s, e| cancel($group, |h| h.on_minecart_pickup(s, e)));
        $server.on_block_dispense(move |s, e| cancel($group, |h| h.on_block_dispense(s, e)));
        $server.on_item_transfer(move |s, e| cancel($group, |h| h.on_item_transfer(s, e)));
        $server.on_hopper_pickup(move |s, e| cancel($group, |h| h.on_hopper_pickup(s, e)));
        $server.on_tripwire(move |s, e| cancel($group, |h| h.on_tripwire(s, e)));
        $server.on_detector_rail(move |s, e| cancel($group, |h| h.on_detector_rail(s, e)));
        $server.on_sculk_vibration(move |s, e| cancel($group, |h| h.on_sculk_vibration(s, e)));
        $server.on_crafter_craft(move |s, e| cancel($group, |h| h.on_crafter_craft(s, e)));
        $server.on_dispenser_equip(move |s, e| cancel($group, |h| h.on_dispenser_equip(s, e)));
        $server.on_pressure_plate(move |s, e| cancel($group, |h| h.on_pressure_plate(s, e)));
        $server.on_portal_create(move |s, e| cancel($group, |h| h.on_portal_create(s, e)));
        $server.on_player_portal(move |s, e| cancel($group, |h| h.on_player_portal(s, e)));
        $server.on_entity_portal(move |s, e| cancel($group, |h| h.on_entity_portal(s, e)));
        $server.on_end_gateway(move |s, e| cancel($group, |h| h.on_end_gateway(s, e)));
        $server.on_dragon_spawn(move |s, e| cancel($group, |h| h.on_dragon_spawn(s, e)));
        $server.on_dragon_phase(move |s, e| cancel($group, |h| h.on_dragon_phase(s, e)));
        $server.on_dragon_death(move |s, e| cancel($group, |h| h.on_dragon_death(s, e)));
        $server.on_player_damage(move |s, e| cancel($group, |h| h.on_player_damage(s, e)));
        $server.on_player_heal(move |s, e| cancel($group, |h| h.on_player_heal(s, e)));
        $server.on_player_combust(move |s, e| cancel($group, |h| h.on_player_combust(s, e)));
        $server.on_food_change(move |s, e| cancel($group, |h| h.on_food_change(s, e)));
        $server.on_experience_change(move |s, e| cancel($group, |h| h.on_experience_change(s, e)));
        $server.on_experience_pickup(move |s, e| cancel($group, |h| h.on_experience_pickup(s, e)));
        $server.on_item_mend(move |s, e| cancel($group, |h| h.on_item_mend(s, e)));
        $server.on_item_damage(move |s, e| cancel($group, |h| h.on_item_damage(s, e)));
        $server.on_item_break(move |s, e| cancel($group, |h| h.on_item_break(s, e)));
        $server.on_shield_block(move |s, e| cancel($group, |h| h.on_shield_block(s, e)));
        $server.on_totem_use(move |s, e| cancel($group, |h| h.on_totem_use(s, e)));
        $server.on_player_warp(move |s, e| cancel($group, |h| h.on_player_warp(s, e)));
        $server.on_player_mount(move |s, e| cancel($group, |h| h.on_player_mount(s, e)));
        $server.on_vehicle_move(move |s, e| cancel($group, |h| h.on_vehicle_move(s, e)));
        $server.on_crossbow_load(move |s, e| cancel($group, |h| h.on_crossbow_load(s, e)));
        $server.on_trident_riptide(move |s, e| cancel($group, |h| h.on_trident_riptide(s, e)));
        $server.on_fish(move |s, e| cancel($group, |h| h.on_fish(s, e)));
        $server.on_horn_blow(move |s, e| cancel($group, |h| h.on_horn_blow(s, e)));
        $server.on_book_edit(move |s, e| cancel($group, |h| h.on_book_edit(s, e)));
        $server.on_offhand_change(move |s, e| cancel($group, |h| h.on_offhand_change(s, e)));
        $server.on_player_bed_enter(move |s, e| cancel($group, |h| h.on_player_bed_enter(s, e)));
        $server.on_player_bed_leave(move |s, e| cancel($group, |h| h.on_player_bed_leave(s, e)));
        $server.on_player_view_distance_change(move |s, e| {
            cancel($group, |h| h.on_player_view_distance_change(s, e))
        });
        $server.on_player_kick(move |s, e| cancel($group, |h| h.on_player_kick(s, e)));
        $server.on_player_jump(move |s, e| cancel($group, |h| h.on_player_jump(s, e)));
        $server.on_player_swing_miss(move |s, e| cancel($group, |h| h.on_player_swing_miss(s, e)));
        $server.on_player_fly_toggle(move |s, e| cancel($group, |h| h.on_player_fly_toggle(s, e)));
        $server.on_player_crawl(move |s, e| cancel($group, |h| h.on_player_crawl(s, e)));
        $server.on_player_attack(move |s, e| cancel($group, |h| h.on_player_attack(s, e)));
        $server.on_player_fall(move |s, e| cancel($group, |h| h.on_player_fall(s, e)));
        $server
            .on_player_world_change(move |s, e| cancel($group, |h| h.on_player_world_change(s, e)));
        $server
            .on_player_chunk_enter(move |s, e| cancel($group, |h| h.on_player_chunk_enter(s, e)));
        $server.on_resource_pack_status(move |s, e| {
            cancel($group, |h| h.on_resource_pack_status(s, e))
        });
        $server.on_player_afk(move |s, e| cancel($group, |h| h.on_player_afk(s, e)));
        $server.on_player_effect_add(move |s, e| cancel($group, |h| h.on_player_effect_add(s, e)));
        $server.on_player_effect_remove(move |s, e| {
            cancel($group, |h| h.on_player_effect_remove(s, e))
        });
        $server.on_ddui_response(move |s, e| cancel($group, |h| h.on_ddui_response(s, e)));
        $server.on_ddui_close(move |s, e| cancel($group, |h| h.on_ddui_close(s, e)));
        $server.on_craft_item(move |s, e| cancel($group, |h| h.on_craft_item(s, e)));
        $server.on_enchant_item(move |s, e| cancel($group, |h| h.on_enchant_item(s, e)));
        $server.on_anvil_use(move |s, e| cancel($group, |h| h.on_anvil_use(s, e)));
        $server.on_grindstone_use(move |s, e| cancel($group, |h| h.on_grindstone_use(s, e)));
        $server.on_smithing_use(move |s, e| cancel($group, |h| h.on_smithing_use(s, e)));
        $server.on_beacon_change(move |s, e| cancel($group, |h| h.on_beacon_change(s, e)));
        $server.on_projectile_launch(move |s, e| cancel($group, |h| h.on_projectile_launch(s, e)));
        $server.on_projectile_hit(move |s, e| cancel($group, |h| h.on_projectile_hit(s, e)));
        $server.on_projectile_pickup(move |s, e| cancel($group, |h| h.on_projectile_pickup(s, e)));
        $server.on_potion_splash(move |s, e| cancel($group, |h| h.on_potion_splash(s, e)));
        $server.on_entity_place(move |s, e| cancel($group, |h| h.on_entity_place(s, e)));
        $server.on_entity_touch(move |s, e| cancel($group, |h| h.on_entity_touch(s, e)));
        $server.on_entity_hurt(move |s, e| cancel($group, |h| h.on_entity_hurt(s, e)));
        $server.on_entity_killed(move |s, e| cancel($group, |h| h.on_entity_killed(s, e)));
        $server
            .on_armor_stand_change(move |s, e| cancel($group, |h| h.on_armor_stand_change(s, e)));
        $server.on_entity_combust(move |s, e| cancel($group, |h| h.on_entity_combust(s, e)));
        $server.on_item_spawn(move |s, e| cancel($group, |h| h.on_item_spawn(s, e)));
        $server.on_item_despawn(move |s, e| cancel($group, |h| h.on_item_despawn(s, e)));
        $server.on_mob_spawn(move |s, e| cancel($group, |h| h.on_mob_spawn(s, e)));
        $server.on_mob_despawn(move |s, e| cancel($group, |h| h.on_mob_despawn(s, e)));
        $server.on_mob_target(move |s, e| cancel($group, |h| h.on_mob_target(s, e)));
        $server.on_mob_attack(move |s, e| cancel($group, |h| h.on_mob_attack(s, e)));
        $server.on_mob_breed(move |s, e| cancel($group, |h| h.on_mob_breed(s, e)));
        $server.on_mob_interact(move |s, e| cancel($group, |h| h.on_mob_interact(s, e)));
        $server.on_mob_tame(move |s, e| cancel($group, |h| h.on_mob_tame(s, e)));
        $server.on_mob_leash(move |s, e| cancel($group, |h| h.on_mob_leash(s, e)));
        $server
            .on_villager_profession(move |s, e| cancel($group, |h| h.on_villager_profession(s, e)));
        $server.on_villager_trade(move |s, e| cancel($group, |h| h.on_villager_trade(s, e)));
        $server.on_villager_level_up(move |s, e| cancel($group, |h| h.on_villager_level_up(s, e)));
        $server.on_server_started(move |s, e| cancel($group, |h| h.on_server_started(s, e)));
        $server.on_server_stop(move |s, e| cancel($group, |h| h.on_server_stop(s, e)));
        $server.on_console_command(move |s, e| cancel($group, |h| h.on_console_command(s, e)));
        $server.on_plugin_enable(move |s, e| cancel($group, |h| h.on_plugin_enable(s, e)));
        $server.on_plugin_disable(move |s, e| cancel($group, |h| h.on_plugin_disable(s, e)));
    };
}
