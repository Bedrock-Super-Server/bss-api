macro_rules! behaviour_methods {
    () => {
        /**
        Called when a player finishes editing the text of a sign.

        Returning `true` leaves the sign unchanged.

        See [`SignChangeEvent`](crate::event::SignChangeEvent).
        */
        fn on_sign_change(&self, server: &Server, event: &SignChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player uses a block: opens a door or a container, presses a button, flips a lever, rings a bell...

        Returning `true` cancels the use: the block does not react.

        See [`BlockUseEvent`](crate::event::BlockUseEvent).
        */
        fn on_block_use(&self, server: &Server, event: &BlockUseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a plant grows a stage: crops, stems, sweet berries, cocoa, cactus, sugar cane...

        Returning `true` stops this growth step.

        See [`BlockGrowEvent`](crate::event::BlockGrowEvent).
        */
        fn on_block_grow(&self, server: &Server, event: &BlockGrowEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a sapling or a fungus grows into a tree.

        Returning `true` keeps the sapling as it is.

        See [`TreeGrowEvent`](crate::event::TreeGrowEvent).
        */
        fn on_tree_grow(&self, server: &Server, event: &TreeGrowEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a block affected by gravity, such as sand or an anvil, starts to fall.

        Returning `true` keeps the block in place.

        See [`BlockFallEvent`](crate::event::BlockFallEvent).
        */
        fn on_block_fall(&self, server: &Server, event: &BlockFallEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a falling block lands and is about to become a block again.

        Returning `true` drops the block as an item instead of placing it.

        See [`BlockLandEvent`](crate::event::BlockLandEvent).
        */
        fn on_block_land(&self, server: &Server, event: &BlockLandEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a block forms in the world on its own: lava and water make obsidian or cobblestone, snow piles up, ice freezes...

        Returning `true` prevents the block from forming.

        See [`BlockFormEvent`](crate::event::BlockFormEvent).
        */
        fn on_block_form(&self, server: &Server, event: &BlockFormEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a block is set on fire, or a block that can be lit is lit: fire, a campfire, a candle, TNT...

        Returning `true` prevents the ignition.

        See [`BlockIgniteEvent`](crate::event::BlockIgniteEvent).
        */
        fn on_block_ignite(&self, server: &Server, event: &BlockIgniteEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when tNT is primed and starts its fuse.

        Returning `true` keeps the TNT block unprimed.

        See [`TntPrimeEvent`](crate::event::TntPrimeEvent).
        */
        fn on_tnt_prime(&self, server: &Server, event: &TntPrimeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a note block plays a note, because a player hit it or it received a redstone signal.

        Returning `true` keeps the note block silent.

        See [`NotePlayEvent`](crate::event::NotePlayEvent).
        */
        fn on_note_play(&self, server: &Server, event: &NotePlayEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player inserts a music disc into a jukebox or takes it out.

        Returning `true` leaves the jukebox as it was.

        See [`JukeboxChangeEvent`](crate::event::JukeboxChangeEvent).
        */
        fn on_jukebox_change(&self, server: &Server, event: &JukeboxChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player places a book on a lectern or takes it.

        Returning `true` leaves the lectern as it was.

        See [`LecternChangeEvent`](crate::event::LecternChangeEvent).
        */
        fn on_lectern_change(&self, server: &Server, event: &LecternChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player turns the page of a book on a lectern.

        Returning `true` keeps the current page.

        See [`LecternPageEvent`](crate::event::LecternPageEvent).
        */
        fn on_lectern_page(&self, server: &Server, event: &LecternPageEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a furnace, smoker, blast furnace or campfire finishes cooking an item.

        Returning `true` cancels the result: the input stays in place.

        See [`ItemCookEvent`](crate::event::ItemCookEvent).
        */
        fn on_item_cook(&self, server: &Server, event: &ItemCookEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a furnace, smoker or blast furnace starts burning a fuel item.

        Returning `true` keeps the fuel unburnt.

        See [`FuelBurnEvent`](crate::event::FuelBurnEvent).
        */
        fn on_fuel_burn(&self, server: &Server, event: &FuelBurnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a brewing stand finishes brewing.

        Returning `true` cancels the brew.

        See [`BrewEvent`](crate::event::BrewEvent).
        */
        fn on_brew(&self, server: &Server, event: &BrewEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a beacon is about to give its effect to a player in range.

        Returning `true` withholds the effect this time.

        See [`BeaconEffectEvent`](crate::event::BeaconEffectEvent).
        */
        fn on_beacon_effect(&self, server: &Server, event: &BeaconEffectEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when water or lava is about to flow into a block.

        Returning `true` stops the liquid from flowing into this block.

        See [`LiquidFlowEvent`](crate::event::LiquidFlowEvent).
        */
        fn on_liquid_flow(&self, server: &Server, event: &LiquidFlowEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a sculk shrieker shrieks because a player stepped near it or made a vibration it heard.

        Returning `true` keeps the shrieker silent.

        See [`SculkShriekEvent`](crate::event::SculkShriekEvent).
        */
        fn on_sculk_shriek(&self, server: &Server, event: &SculkShriekEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an explosion is about to happen.

        Returning `true` cancels the explosion.

        See [`ExplosionEvent`](crate::event::ExplosionEvent).
        */
        fn on_explosion(&self, server: &Server, event: &ExplosionEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an explosion is about to destroy blocks.

        Returning `true` keeps every block in place.

        See [`ExplosionBlocksEvent`](crate::event::ExplosionBlocksEvent).
        */
        fn on_explosion_blocks(&self, server: &Server, event: &ExplosionBlocksEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when lightning is about to strike.

        Returning `true` cancels the strike.

        See [`LightningStrikeEvent`](crate::event::LightningStrikeEvent).
        */
        fn on_lightning_strike(&self, server: &Server, event: &LightningStrikeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when rain starts or stops.

        Returning `true` keeps the current weather.

        See [`RainChangeEvent`](crate::event::RainChangeEvent).
        */
        fn on_rain_change(&self, server: &Server, event: &RainChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a thunderstorm starts or stops.

        Returning `true` keeps the current weather.

        See [`ThunderChangeEvent`](crate::event::ThunderChangeEvent).
        */
        fn on_thunder_change(&self, server: &Server, event: &ThunderChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when the weather changes a block: snow piles up, water freezes, rain puts a fire out...

        Returning `true` leaves the block as it is.

        See [`WeatherEffectEvent`](crate::event::WeatherEffectEvent).
        */
        fn on_weather_effect(&self, server: &Server, event: &WeatherEffectEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when the tick rate of the server is about to change, through `/tick` or a plugin.

        Returning `true` keeps the current rate.

        See [`TickRateChangeEvent`](crate::event::TickRateChangeEvent).
        */
        fn on_tick_rate_change(&self, server: &Server, event: &TickRateChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when the redstone power of a block changes.

        Returning `true` keeps the previous power.

        See [`RedstonePowerEvent`](crate::event::RedstonePowerEvent).
        */
        fn on_redstone_power(&self, server: &Server, event: &RedstonePowerEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a piston extends or retracts.

        Returning `true` keeps the piston still.

        See [`PistonMoveEvent`](crate::event::PistonMoveEvent).
        */
        fn on_piston_move(&self, server: &Server, event: &PistonMoveEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a piston pushes an entity.

        Returning `true` leaves the entity where it is.

        See [`PistonPushEvent`](crate::event::PistonPushEvent).
        */
        fn on_piston_push(&self, server: &Server, event: &PistonPushEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a minecart passes over a powered activator rail.

        Returning `true` ignores the rail.

        See [`MinecartActivateEvent`](crate::event::MinecartActivateEvent).
        */
        fn on_minecart_activate(&self, server: &Server, event: &MinecartActivateEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a hopper moves an item into or out of a minecart.

        Returning `true` cancels the transfer.

        See [`MinecartTransferEvent`](crate::event::MinecartTransferEvent).
        */
        fn on_minecart_transfer(&self, server: &Server, event: &MinecartTransferEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a hopper minecart picks up an item entity.

        Returning `true` leaves the item where it is.

        See [`MinecartPickupEvent`](crate::event::MinecartPickupEvent).
        */
        fn on_minecart_pickup(&self, server: &Server, event: &MinecartPickupEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a dispenser or dropper fires an item.

        Returning `true` cancels the shot.

        See [`BlockDispenseEvent`](crate::event::BlockDispenseEvent).
        */
        fn on_block_dispense(&self, server: &Server, event: &BlockDispenseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a hopper moves an item from one container to another.

        Returning `true` cancels the transfer.

        See [`ItemTransferEvent`](crate::event::ItemTransferEvent).
        */
        fn on_item_transfer(&self, server: &Server, event: &ItemTransferEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a hopper picks up an item entity lying above it.

        Returning `true` leaves the item where it is.

        See [`HopperPickupEvent`](crate::event::HopperPickupEvent).
        */
        fn on_hopper_pickup(&self, server: &Server, event: &HopperPickupEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a tripwire is triggered or released.

        Returning `true` leaves the tripwire as it was.

        See [`TripwireEvent`](crate::event::TripwireEvent).
        */
        fn on_tripwire(&self, server: &Server, event: &TripwireEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a minecart turns a detector rail on or off.

        Returning `true` leaves the rail as it was.

        See [`DetectorRailEvent`](crate::event::DetectorRailEvent).
        */
        fn on_detector_rail(&self, server: &Server, event: &DetectorRailEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a sculk sensor detects a vibration.

        Returning `true` keeps the sensor from reacting.

        See [`SculkVibrationEvent`](crate::event::SculkVibrationEvent).
        */
        fn on_sculk_vibration(&self, server: &Server, event: &SculkVibrationEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a crafter crafts an item.

        Returning `true` cancels the craft.

        See [`CrafterCraftEvent`](crate::event::CrafterCraftEvent).
        */
        fn on_crafter_craft(&self, server: &Server, event: &CrafterCraftEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a dispenser puts armor on a player in front of it.

        Returning `true` keeps the armor in the dispenser.

        See [`DispenserEquipEvent`](crate::event::DispenserEquipEvent).
        */
        fn on_dispenser_equip(&self, server: &Server, event: &DispenserEquipEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when something steps on or leaves a pressure plate.

        Returning `true` keeps the plate in its previous state.

        See [`PressurePlateEvent`](crate::event::PressurePlateEvent).
        */
        fn on_pressure_plate(&self, server: &Server, event: &PressurePlateEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a portal is about to be created: a Nether portal is lit or generated for travel, or an End portal is completed.

        Returning `true` prevents the portal from being created.

        See [`PortalCreateEvent`](crate::event::PortalCreateEvent).
        */
        fn on_portal_create(&self, server: &Server, event: &PortalCreateEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player goes through a portal.

        Returning `true` keeps the player where they are.

        See [`PlayerPortalEvent`](crate::event::PlayerPortalEvent).
        */
        fn on_player_portal(&self, server: &Server, event: &PlayerPortalEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an entity other than a player goes through a portal.

        Returning `true` keeps the entity where it is.

        See [`EntityPortalEvent`](crate::event::EntityPortalEvent).
        */
        fn on_entity_portal(&self, server: &Server, event: &EntityPortalEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an End gateway is about to be generated after a dragon fight.

        Returning `true` prevents the gateway.

        See [`EndGatewayEvent`](crate::event::EndGatewayEvent).
        */
        fn on_end_gateway(&self, server: &Server, event: &EndGatewayEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an ender dragon is about to spawn in the End.

        Returning `true` prevents the dragon from spawning.

        See [`DragonSpawnEvent`](crate::event::DragonSpawnEvent).
        */
        fn on_dragon_spawn(&self, server: &Server, event: &DragonSpawnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when the ender dragon changes its behaviour phase.

        The value returned is ignored: this event cannot be cancelled.

        See [`DragonPhaseEvent`](crate::event::DragonPhaseEvent).
        */
        fn on_dragon_phase(&self, server: &Server, event: &DragonPhaseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when the ender dragon dies.

        The value returned is ignored: this event cannot be cancelled.

        See [`DragonDeathEvent`](crate::event::DragonDeathEvent).
        */
        fn on_dragon_death(&self, server: &Server, event: &DragonDeathEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player takes damage.

        Returning `true` cancels the damage.

        See [`PlayerDamageEvent`](crate::event::PlayerDamageEvent).
        */
        fn on_player_damage(&self, server: &Server, event: &PlayerDamageEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player regains health.

        Returning `true` cancels the healing.

        See [`PlayerHealEvent`](crate::event::PlayerHealEvent).
        */
        fn on_player_heal(&self, server: &Server, event: &PlayerHealEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player catches fire.

        Returning `true` keeps the player from burning.

        See [`PlayerCombustEvent`](crate::event::PlayerCombustEvent).
        */
        fn on_player_combust(&self, server: &Server, event: &PlayerCombustEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player's hunger bar changes.

        Returning `true` keeps the previous values.

        See [`FoodChangeEvent`](crate::event::FoodChangeEvent).
        */
        fn on_food_change(&self, server: &Server, event: &FoodChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player gains or loses experience.

        Returning `true` leaves the experience unchanged.

        See [`ExperienceChangeEvent`](crate::event::ExperienceChangeEvent).
        */
        fn on_experience_change(&self, server: &Server, event: &ExperienceChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player picks up an experience orb.

        Returning `true` leaves the orb where it is.

        See [`ExperiencePickupEvent`](crate::event::ExperiencePickupEvent).
        */
        fn on_experience_pickup(&self, server: &Server, event: &ExperiencePickupEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when mending repairs an item with experience the player picked up.

        Returning `true` cancels the repair.

        See [`ItemMendEvent`](crate::event::ItemMendEvent).
        */
        fn on_item_mend(&self, server: &Server, event: &ItemMendEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a tool, weapon or piece of armor is about to lose durability.

        Returning `true` prevents the loss.

        See [`ItemDamageEvent`](crate::event::ItemDamageEvent).
        */
        fn on_item_damage(&self, server: &Server, event: &ItemDamageEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a tool, weapon or piece of armor breaks after losing its last durability point.

        The value returned is ignored: this event cannot be cancelled.

        See [`ItemBreakEvent`](crate::event::ItemBreakEvent).
        */
        fn on_item_break(&self, server: &Server, event: &ItemBreakEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player blocks an attack with a shield.

        Returning `true` cancels it.

        See [`ShieldBlockEvent`](crate::event::ShieldBlockEvent).
        */
        fn on_shield_block(&self, server: &Server, event: &ShieldBlockEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a totem of undying saves a player from death.

        Returning `true` stops the totem from saving the player.

        See [`TotemUseEvent`](crate::event::TotemUseEvent).
        */
        fn on_totem_use(&self, server: &Server, event: &TotemUseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player teleports with an ender pearl or a chorus fruit.

        Returning `true` cancels the teleport.

        See [`PlayerWarpEvent`](crate::event::PlayerWarpEvent).
        */
        fn on_player_warp(&self, server: &Server, event: &PlayerWarpEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player gets on or off a vehicle or a mob.

        Returning `true` cancels it.

        See [`PlayerMountEvent`](crate::event::PlayerMountEvent).
        */
        fn on_player_mount(&self, server: &Server, event: &PlayerMountEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player moves the vehicle they ride.

        Returning `true` keeps the vehicle where it was.

        See [`VehicleMoveEvent`](crate::event::VehicleMoveEvent).
        */
        fn on_vehicle_move(&self, server: &Server, event: &VehicleMoveEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player loads a crossbow.

        Returning `true` keeps the crossbow unloaded.

        See [`CrossbowLoadEvent`](crate::event::CrossbowLoadEvent).
        */
        fn on_crossbow_load(&self, server: &Server, event: &CrossbowLoadEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player launches with a Riptide trident.

        Returning `true` cancels the launch.

        See [`TridentRiptideEvent`](crate::event::TridentRiptideEvent).
        */
        fn on_trident_riptide(&self, server: &Server, event: &TridentRiptideEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player casts a fishing rod, or reels it in.

        Returning `true` cancels the cast or the catch.

        See [`FishEvent`](crate::event::FishEvent).
        */
        fn on_fish(&self, server: &Server, event: &FishEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player blows a goat horn.

        Returning `true` keeps the horn silent.

        See [`HornBlowEvent`](crate::event::HornBlowEvent).
        */
        fn on_horn_blow(&self, server: &Server, event: &HornBlowEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player edits or signs a book and quill.

        Returning `true` rejects the edit.

        See [`BookEditEvent`](crate::event::BookEditEvent).
        */
        fn on_book_edit(&self, server: &Server, event: &BookEditEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player changes the item in their off hand.

        Returning `true` cancels it.

        See [`OffhandChangeEvent`](crate::event::OffhandChangeEvent).
        */
        fn on_offhand_change(&self, server: &Server, event: &OffhandChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player crafts an item.

        Returning `true` cancels the craft.

        See [`CraftItemEvent`](crate::event::CraftItemEvent).
        */
        fn on_craft_item(&self, server: &Server, event: &CraftItemEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player enchants an item at an enchanting table.

        Returning `true` cancels the enchantment.

        See [`EnchantItemEvent`](crate::event::EnchantItemEvent).
        */
        fn on_enchant_item(&self, server: &Server, event: &EnchantItemEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player takes the result out of an anvil.

        Returning `true` cancels the operation.

        See [`AnvilUseEvent`](crate::event::AnvilUseEvent).
        */
        fn on_anvil_use(&self, server: &Server, event: &AnvilUseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player takes the result out of a grindstone.

        Returning `true` cancels the operation.

        See [`GrindstoneUseEvent`](crate::event::GrindstoneUseEvent).
        */
        fn on_grindstone_use(&self, server: &Server, event: &GrindstoneUseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player takes the result out of a smithing table.

        Returning `true` cancels the operation.

        See [`SmithingUseEvent`](crate::event::SmithingUseEvent).
        */
        fn on_smithing_use(&self, server: &Server, event: &SmithingUseEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player changes the effects of a beacon.

        Returning `true` keeps the previous effects.

        See [`BeaconChangeEvent`](crate::event::BeaconChangeEvent).
        */
        fn on_beacon_change(&self, server: &Server, event: &BeaconChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player launches a projectile: an arrow, a trident, a firework, a fishing hook...

        Returning `true` cancels the launch.

        See [`ProjectileLaunchEvent`](crate::event::ProjectileLaunchEvent).
        */
        fn on_projectile_launch(&self, server: &Server, event: &ProjectileLaunchEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a projectile hits a block or an entity.

        Returning `true` cancels the hit.

        See [`ProjectileHitEvent`](crate::event::ProjectileHitEvent).
        */
        fn on_projectile_hit(&self, server: &Server, event: &ProjectileHitEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player picks up an arrow or a trident stuck in the world.

        Returning `true` leaves it in place.

        See [`ProjectilePickupEvent`](crate::event::ProjectilePickupEvent).
        */
        fn on_projectile_pickup(&self, server: &Server, event: &ProjectilePickupEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a thrown potion breaks.

        Returning `true` cancels the effect: the potion breaks without affecting anything.

        See [`PotionSplashEvent`](crate::event::PotionSplashEvent).
        */
        fn on_potion_splash(&self, server: &Server, event: &PotionSplashEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player places an entity from an item: an armor stand, a boat, a minecart, an end crystal...

        Returning `true` cancels the placement and keeps the item.

        See [`EntityPlaceEvent`](crate::event::EntityPlaceEvent).
        */
        fn on_entity_place(&self, server: &Server, event: &EntityPlaceEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player touches or hits an entity that has no behaviour of its own, such as a plugin entity.

        Returning `true` ignores the touch.

        See [`EntityTouchEvent`](crate::event::EntityTouchEvent).
        */
        fn on_entity_touch(&self, server: &Server, event: &EntityTouchEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an entity other than a player takes damage from any source.

        Returning `true` cancels the damage.

        See [`EntityHurtEvent`](crate::event::EntityHurtEvent).
        */
        fn on_entity_hurt(&self, server: &Server, event: &EntityHurtEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an entity other than a player is killed.

        Returning `true` keeps the entity alive, with one health point.

        See [`EntityKilledEvent`](crate::event::EntityKilledEvent).
        */
        fn on_entity_killed(&self, server: &Server, event: &EntityKilledEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player puts an item on an armor stand or takes one off.

        Returning `true` leaves the armor stand as it was.

        See [`ArmorStandChangeEvent`](crate::event::ArmorStandChangeEvent).
        */
        fn on_armor_stand_change(&self, server: &Server, event: &ArmorStandChangeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an entity other than a player catches fire.

        Returning `true` keeps the entity from burning.

        See [`EntityCombustEvent`](crate::event::EntityCombustEvent).
        */
        fn on_entity_combust(&self, server: &Server, event: &EntityCombustEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when an item entity appears in the world: a drop, a broken block, a dispenser throw...

        Returning `true` prevents the item from appearing.

        See [`ItemSpawnEvent`](crate::event::ItemSpawnEvent).
        */
        fn on_item_spawn(&self, server: &Server, event: &ItemSpawnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a dropped item disappears after lying on the ground for five minutes.

        The value returned is ignored: this event cannot be cancelled.

        See [`ItemDespawnEvent`](crate::event::ItemDespawnEvent).
        */
        fn on_item_despawn(&self, server: &Server, event: &ItemDespawnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a mob is about to spawn.

        Returning `true` prevents the spawn.

        See [`MobSpawnEvent`](crate::event::MobSpawnEvent).
        */
        fn on_mob_spawn(&self, server: &Server, event: &MobSpawnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a mob is about to despawn because no player is close enough.

        Returning `true` keeps the mob.

        See [`MobDespawnEvent`](crate::event::MobDespawnEvent).
        */
        fn on_mob_despawn(&self, server: &Server, event: &MobDespawnEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a mob picks a new target.

        Returning `true` leaves the mob without a target.

        See [`MobTargetEvent`](crate::event::MobTargetEvent).
        */
        fn on_mob_target(&self, server: &Server, event: &MobTargetEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a mob attacks its target.

        Returning `true` cancels the attack.

        See [`MobAttackEvent`](crate::event::MobAttackEvent).
        */
        fn on_mob_attack(&self, server: &Server, event: &MobAttackEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when two mobs breed and a baby is about to be born.

        Returning `true` prevents the birth.

        See [`MobBreedEvent`](crate::event::MobBreedEvent).
        */
        fn on_mob_breed(&self, server: &Server, event: &MobBreedEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player uses an item on a mob: feeds, tames, shears, milks, names...

        Returning `true` cancels the interaction.

        See [`MobInteractEvent`](crate::event::MobInteractEvent).
        */
        fn on_mob_interact(&self, server: &Server, event: &MobInteractEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player tames a mob.

        Returning `true` makes this taming attempt fail.

        See [`MobTameEvent`](crate::event::MobTameEvent).
        */
        fn on_mob_tame(&self, server: &Server, event: &MobTameEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a mob is leashed or unleashed.

        Returning `true` leaves the leash as it was.

        See [`MobLeashEvent`](crate::event::MobLeashEvent).
        */
        fn on_mob_leash(&self, server: &Server, event: &MobLeashEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a villager takes a profession by claiming a workstation.

        Returning `true` keeps the villager unemployed.

        See [`VillagerProfessionEvent`](crate::event::VillagerProfessionEvent).
        */
        fn on_villager_profession(&self, server: &Server, event: &VillagerProfessionEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a player trades with a villager or a wandering trader.

        Returning `true` cancels the trade.

        See [`VillagerTradeEvent`](crate::event::VillagerTradeEvent).
        */
        fn on_villager_trade(&self, server: &Server, event: &VillagerTradeEvent) -> bool {
            let _ = (server, event);
            false
        }

        /**
        Called when a villager reaches a new trading level.

        The value returned is ignored: this event cannot be cancelled.

        See [`VillagerLevelUpEvent`](crate::event::VillagerLevelUpEvent).
        */
        fn on_villager_level_up(&self, server: &Server, event: &VillagerLevelUpEvent) -> bool {
            let _ = (server, event);
            false
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
    };
}
