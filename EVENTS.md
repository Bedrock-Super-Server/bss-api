# Behaviour events

Every gameplay behaviour raises one of these events. Implement the matching method of
the `Event` trait (or register a closure with `server.on_<name>`) and return `true` to
cancel it. Fields listed as editable can be changed with `set_<field>`, and the server
uses the new value. Each event lives in `bss_api::event::<domain>`.

```rust
impl Event for Guard {
    fn on_explosion(&self, _server: &Server, event: &ExplosionEvent) -> bool {
        event.set_breaks_blocks(false);
        false
    }
}
```

## Blocks (`event::block`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_sign_change` | A player finished editing one side of a sign. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `front: bool` | `text: String` |
| `on_block_use` | A player uses a block: tilling, stripping, composting, filling a cauldron, ringing a bell, eating cake, potting a plant and every other right-click behaviour. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `block: u32`, `item: i16`, `action: String` | --- |
| `on_block_grow` | A crop, stem, sapling or other plant grows a stage, by itself or from bone meal. | `x: i32`, `y: i32`, `z: i32`, `block: u32`, `bone_meal: bool`, `world: u32` | `new_block: u32` |
| `on_tree_grow` | A sapling or fungus grows into a tree. | `x: i32`, `y: i32`, `z: i32`, `sapling: u32`, `world: u32` | --- |
| `on_block_fall` | Sand, gravel, concrete powder or an anvil starts falling. | `x: i32`, `y: i32`, `z: i32`, `block: u32`, `world: u32` | --- |
| `on_block_land` | A falling block lands and becomes a block again; cancelled, it drops as an item. | `entity: u64`, `x: i32`, `y: i32`, `z: i32`, `world: u32` | `block: u32` |
| `on_block_form` | A block turns into another by itself: concrete powder hardening, a sponge soaking water. | `x: i32`, `y: i32`, `z: i32`, `block: u32`, `world: u32` | `new_block: u32` |
| `on_block_ignite` | Something sets a block alight or lights a candle or campfire. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `block: u32`, `cause: String` | --- |
| `on_tnt_prime` | TNT is lit. | `x: i32`, `y: i32`, `z: i32`, `cause: String`, `owner: u64`, `world: u32` | `fuse: u32` |
| `on_note_play` | A note block plays. | `player: String`, `x: i32`, `y: i32`, `z: i32` | `instrument: i32`, `pitch: i32` |
| `on_jukebox_change` | A disc goes into or comes out of a jukebox. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `disc: i16`, `inserted: bool` | --- |
| `on_lectern_change` | A book goes onto or off a lectern. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `book: i16`, `placed: bool` | --- |
| `on_lectern_page` | A player turns the page of a book on a lectern. | `player: String`, `x: i32`, `y: i32`, `z: i32` | `page: i32` |
| `on_item_cook` | A furnace, blast furnace, smoker or campfire finishes cooking an item. | `x: i32`, `y: i32`, `z: i32`, `station: String`, `input: i16`, `world: u32` | `output: i16` |
| `on_fuel_burn` | A furnace starts burning a fuel item. | `x: i32`, `y: i32`, `z: i32`, `station: String`, `fuel: i16`, `world: u32` | `burn_ticks: u32` |
| `on_brew` | A brewing stand finishes brewing. | `x: i32`, `y: i32`, `z: i32`, `ingredient: i16`, `world: u32` | --- |
| `on_beacon_effect` | A beacon gives its effect to a player. | `player: String`, `x: i32`, `y: i32`, `z: i32` | `effect: i32`, `amplifier: i32`, `ticks: i32` |
| `on_liquid_flow` | Water or lava spreads into a block. | `x: i32`, `y: i32`, `z: i32`, `liquid: u32`, `world: u32` | --- |
| `on_sculk_shriek` | A sculk shrieker shrieks at a player, heard through a sculk sensor near it. `warning` is the player's warning level after it, 1 to 4; at 4 a shrieker that can summon brings a Warden. Cancelled, it stays quiet. | `x: i32`, `y: i32`, `z: i32`, `player: String`, `world: u32` | `warning: u32` |

## The world and its weather (`event::world`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_explosion` | Something explodes: TNT, an end crystal, a wind charge. | `x: f32`, `y: f32`, `z: f32`, `owner: u64`, `world: u32` | `power: f32`, `fire: bool`, `breaks_blocks: bool` |
| `on_explosion_blocks` | An explosion is about to destroy its blocks; cancelled, it destroys none. | `x: f32`, `y: f32`, `z: f32`, `count: u32`, `world: u32` | `drop_items: bool` |
| `on_lightning_strike` | Lightning strikes; `cause` is weather, channeling or plugin. Natural lightning has already been drawn to a lightning rod when one is near. | `cause: String`, `owner: u64`, `world: u32` | `x: f32`, `y: f32`, `z: f32`, `fire: bool` |
| `on_rain_change` | It starts or stops raining in a world. `duration` is how many ticks the new weather lasts before it changes again. | `raining: bool`, `world: u32` | `duration: u32` |
| `on_thunder_change` | A thunderstorm starts or ends in a world; thunder only rumbles while it also rains. `duration` is how many ticks it lasts. | `thundering: bool`, `world: u32` | `duration: u32` |
| `on_weather_effect` | Rain or snow changes a block: filling a cauldron, putting out a fire, freezing water, laying snow, watering farmland. | `x: i32`, `y: i32`, `z: i32`, `block: u32`, `cause: String`, `world: u32` | `new_block: u32` |
| `on_spawn_change` | The spawn point of a world is about to move, with `/setworldspawn` or a plugin. Set another position to move it there instead. Cancelled, it stays where it is. | `world: u32` | `x: i32`, `y: i32`, `z: i32` |
| `on_world_load` | A world finishes loading and is ready for players and plugins. Cancelling changes nothing. | `world: u32`, `name: String` | --- |
| `on_tick_rate_change` | The game's pace changes by /tick or a plugin: how many ticks run a second, and whether worlds stand frozen. Cancelled, it keeps its pace. | --- | `rate: f32`, `frozen: bool` |

## Redstone (`event::redstone`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_redstone_power` | A redstone component changes its power: wire, torches, repeaters, comparators, observers, lamps and anything else redstone switches. Cancelled, it keeps its old state. | `x: i32`, `y: i32`, `z: i32`, `block: u32`, `old_power: i32`, `new_power: i32`, `world: u32` | --- |
| `on_piston_move` | A piston extends or retracts, moving `count` blocks. | `x: i32`, `y: i32`, `z: i32`, `extending: bool`, `count: u32`, `world: u32` | --- |
| `on_piston_push` | A piston's arm, or a block it moves, shoves a player or entity along; `player` is empty for entities. Cancelled, it stays where it is. | `x: i32`, `y: i32`, `z: i32`, `entity: u64`, `player: String`, `world: u32` | `motion_x: f32`, `motion_y: f32`, `motion_z: f32` |
| `on_minecart_activate` | A minecart rolls over a powered activator rail: its rider is thrown out and a TNT minecart is lit. | `entity: u64`, `identifier: String`, `x: i32`, `y: i32`, `z: i32`, `rider: u64`, `world: u32` | --- |
| `on_minecart_transfer` | An item moves between a chest or hopper minecart and a block: a hopper minecart pulling from the container above it, or a hopper under the rail pulling from the minecart; `into_cart` says which way. | `entity: u64`, `x: i32`, `y: i32`, `z: i32`, `item: i16`, `into_cart: bool`, `world: u32` | --- |
| `on_minecart_pickup` | A hopper minecart picks up an item entity it rolls over. | `entity: u64`, `item_entity: u64`, `item: i16`, `world: u32` | --- |
| `on_block_dispense` | A dispenser or dropper fires an item. | `x: i32`, `y: i32`, `z: i32`, `dropper: bool`, `world: u32` | `item: i16` |
| `on_item_transfer` | A hopper moves an item from one container to another. | `from_x: i32`, `from_y: i32`, `from_z: i32`, `to_x: i32`, `to_y: i32`, `to_z: i32`, `item: i16`, `world: u32` | --- |
| `on_hopper_pickup` | A hopper picks up an item entity lying on it. | `x: i32`, `y: i32`, `z: i32`, `entity: u64`, `item: i16`, `world: u32` | --- |
| `on_tripwire` | Something steps onto a tripwire, or the last thing on it steps off. | `x: i32`, `y: i32`, `z: i32`, `powered: bool`, `player: String`, `world: u32` | --- |
| `on_detector_rail` | A minecart comes onto a detector rail, or the last one leaves it. | `x: i32`, `y: i32`, `z: i32`, `entity: u64`, `active: bool`, `world: u32` | --- |
| `on_sculk_vibration` | A sculk sensor picks up a vibration: steps, landings, blocks placed, broken or used, projectiles, explosions, lightning and the rest. `frequency` is its kind, 1 to 15, and `power` the signal the sensor gives. | `x: i32`, `y: i32`, `z: i32`, `source_x: i32`, `source_y: i32`, `source_z: i32`, `world: u32` | `frequency: i32`, `power: i32` |
| `on_crafter_craft` | A crafter crafts what its grid holds and ejects it. | `x: i32`, `y: i32`, `z: i32`, `recipe: u32`, `world: u32` | `item: i16`, `count: u32` |
| `on_dispenser_equip` | A dispenser puts armour, an elytra, a head or a carved pumpkin on a player in front of it; slot 0 to 3 is head to feet. | `x: i32`, `y: i32`, `z: i32`, `player: String`, `item: i16`, `slot: u32`, `world: u32` | --- |
| `on_pressure_plate` | Something presses a pressure plate or steps off it. | `x: i32`, `y: i32`, `z: i32`, `player: String`, `world: u32` | `signal: i32` |

## Portals (`event::portal`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_portal_create` | A portal opens: an obsidian frame lit into a Nether portal, a Nether portal the server builds where a traveller arrives, or an End portal frame filled with eyes. `cause` is flint_and_steel, fire_charge, fire, dispenser, travel or ender_eye; `axis` is x or z for a Nether portal and y for an End portal; `player` is empty when no player did it. Cancelled, it does not open. | `x: i32`, `y: i32`, `z: i32`, `width: u32`, `height: u32`, `axis: String`, `cause: String`, `player: String`, `world: u32` | --- |
| `on_player_portal` | A player goes through a portal: `portal` is nether, end or gateway. `x`, `y` and `z` are where they are headed; through a Nether portal a portal is then looked for, or built, around that spot. Cancelled, they stay. | `player: String`, `portal: String`, `from_world: u32`, `to_world: u32` | `x: f32`, `y: f32`, `z: f32` |
| `on_entity_portal` | An entity goes through a portal, as for players. | `entity: u64`, `identifier: String`, `portal: String`, `from_world: u32`, `to_world: u32` | `x: f32`, `y: f32`, `z: f32` |
| `on_end_gateway` | An End gateway opens: one on the main island each time the dragon dies, or the far one on the outer islands the first time a gateway is used. Cancelled, it does not open. | `x: i32`, `y: i32`, `z: i32`, `world: u32` | --- |

## The Ender Dragon (`event::dragon`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_dragon_spawn` | The Ender Dragon appears over the End: the first time players come, or with `respawn` when four end crystals set around the exit portal call it back. Cancelled, it does not come. | `entity: u64`, `x: f32`, `y: f32`, `z: f32`, `respawn: bool`, `world: u32` | --- |
| `on_dragon_phase` | The Ender Dragon turns to something else: `phase` is circling, strafing, charging, landing, perched, taking_off or dying. Cancelling changes nothing. | `entity: u64`, `phase: String`, `world: u32` | --- |
| `on_dragon_death` | The Ender Dragon dies, once it has risen and burst apart. `first` is true the first time in this End; `experience` is what it leaves and `egg` whether the dragon egg appears on the exit portal. Cancelling changes nothing. | `entity: u64`, `killer: String`, `first: bool`, `world: u32` | `experience: u32`, `egg: bool` |

## Players (`event::player`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_player_damage` | A player takes damage from anything: attacks, arrows, falling, fire, lava, drowning, explosions, the void. | `player: String`, `source: String`, `attacker: String` | `damage: f32` |
| `on_player_heal` | A player regains health. | `player: String`, `reason: String` | `amount: f32` |
| `on_player_combust` | A player catches fire. | `player: String` | `ticks: u32` |
| `on_food_change` | A player's hunger changes from eating or exhaustion. | `player: String`, `old_food: i32` | `food: i32`, `saturation: f32` |
| `on_experience_change` | A player gains or loses experience points. | `player: String`, `level: i32`, `total: i32` | `amount: i32` |
| `on_experience_pickup` | A player collects an experience orb. | `player: String`, `entity: u64` | `amount: u32` |
| `on_item_mend` | Mending repairs an item with experience. | `player: String`, `item: i16` | `repair: u32` |
| `on_item_damage` | A tool, weapon or armour piece loses durability; slots 100 to 103 are armour, head to feet. | `player: String`, `item: i16`, `slot: u32` | `damage: u32` |
| `on_item_break` | A tool, weapon or armour piece broke; slots 100 to 103 are armour, head to feet. Cancelling changes nothing. | `player: String`, `item: i16`, `slot: u32` | --- |
| `on_shield_block` | A raised shield stops a hit. | `player: String`, `damage: f32`, `attacker: String` | --- |
| `on_totem_use` | A totem of undying saves a player. | `player: String` | --- |
| `on_player_warp` | A player is moved by an ender pearl or chorus fruit. | `player: String`, `cause: String` | `x: f32`, `y: f32`, `z: f32` |
| `on_player_mount` | A player gets into or out of a boat or minecart. | `player: String`, `vehicle: u64`, `mounted: bool` | --- |
| `on_vehicle_move` | A player steers the vehicle they ride; cancelled, other players do not see the move. | `player: String`, `vehicle: u64`, `x: f32`, `y: f32`, `z: f32` | --- |
| `on_crossbow_load` | A player loads a crossbow. | `player: String`, `ammo: i16` | --- |
| `on_trident_riptide` | A player launches themselves with a Riptide trident. | `player: String` | `strength: f32` |
| `on_fish` | A player casts, reels in or catches something with a fishing rod. | `player: String`, `action: String`, `hooked: u64` | `item: i16`, `experience: u32` |
| `on_horn_blow` | A player blows a goat horn. | `player: String` | `variant: i32` |
| `on_book_edit` | A player writes in, signs or tears a page from a book. | `player: String`, `slot: u32`, `action: String`, `page: i32`, `text: String` | --- |
| `on_offhand_change` | An item goes into or out of a player's off hand. | `player: String`, `item: i16`, `count: u32` | --- |
| `on_player_bed_enter` | A player lies down in a bed: it is night or a thunderstorm, no monster is near and nobody else lies there. Cancelled, the bed still becomes their respawn point but they stay standing. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `world: u32` | --- |
| `on_player_bed_leave` | A player gets out of bed. `cause` is `morning` when the world woke everyone up, `manual` when the player left by themselves, `damage`, `gamemode` or `travel` when something forced them out, and `bed_destroyed` when the bed is gone. Cancelling changes nothing. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `cause: String`, `world: u32` | --- |
| `on_player_view_distance_change` | A player's client asks for another view distance, in chunks. `radius` is what the server grants, within its own `view_distance`; set another one to grant that instead. Cancelled, the player keeps the view distance they have. | `player: String`, `requested: i32`, `old_radius: i32` | `radius: i32` |
| `on_player_kick` | Something asks to disconnect a player: a plugin, the `/kick` command or the server's own protections. Cancelled, the player stays connected. | `player: String`, `reason: String` | --- |
| `on_player_jump` | A player presses the jump key, once for each press, unless they are flying. Cancelling changes nothing. | `player: String`, `x: f32`, `y: f32`, `z: f32` | --- |
| `on_player_effect_add` | A player is about to get a potion effect from what they drink, eat or use: a potion, food, a totem of undying; `effect` is its name, such as `minecraft:speed`. Effects given by a command, a beacon or a plugin are not asked. Cancelled, they do not get it. | `player: String`, `effect: String` | `amplifier: u32`, `duration: i32` |
| `on_player_effect_remove` | A player loses a potion effect: it ran out, milk or a command cleared it, or a totem replaced it. Cancelling changes nothing. | `player: String`, `effect: String`, `amplifier: u32` | --- |

## Crafting and stations (`event::inventory`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_craft_item` | A player crafts at a crafting grid, stonecutter, loom or other station. | `player: String`, `station: String`, `recipe: u32`, `item: i16`, `count: u32` | --- |
| `on_enchant_item` | A player enchants an item at an enchanting table. | `player: String`, `item: i16`, `option: i32` | `levels: i32` |
| `on_anvil_use` | A player repairs, combines or renames an item at an anvil. | `player: String`, `item: i16`, `name: String` | `levels: i32` |
| `on_grindstone_use` | A player removes enchantments at a grindstone. | `player: String`, `item: i16` | `experience: i32` |
| `on_smithing_use` | A player upgrades or trims an item at a smithing table. | `player: String`, `item: i16`, `result: i16` | --- |
| `on_beacon_change` | A player sets a beacon's effects. | `player: String`, `x: i32`, `y: i32`, `z: i32`, `primary: i32`, `secondary: i32` | --- |

## Projectiles (`event::projectile`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_projectile_launch` | A player shoots or throws something: arrows, tridents, snowballs, eggs, pearls, potions, bottles, wind charges, fireworks, fishing hooks, eyes of ender. | `player: String`, `identifier: String` | `force: f32` |
| `on_projectile_hit` | A projectile hits a player, an entity or a block; target is 0 for a block, where cancelling changes nothing. | `entity: u64`, `identifier: String`, `owner: u64`, `target: u64`, `x: f32`, `y: f32`, `z: f32` | `damage: f32` |
| `on_projectile_pickup` | A player picks up a landed arrow or trident. | `player: String`, `entity: u64`, `item: i16` | --- |
| `on_potion_splash` | A splash potion breaks, or a lingering cloud pulses (`lingering`). | `entity: u64`, `potion: i32`, `x: f32`, `y: f32`, `z: f32`, `lingering: bool` | `radius: f32` |

## Entities (`event::entity`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_entity_place` | A player places an entity: spawn eggs, boats, minecarts, armour stands, paintings, end crystals. | `player: String`, `identifier: String`, `x: f32`, `y: f32`, `z: f32` | --- |
| `on_entity_touch` | A player hits or uses an entity the server moves. | `player: String`, `entity: u64`, `identifier: String`, `attack: bool` | --- |
| `on_entity_hurt` | An entity the server moves takes damage. | `entity: u64`, `identifier: String`, `attacker: u64` | `damage: f32` |
| `on_entity_killed` | An entity the server moves dies; cancelled, it survives with one health. | `entity: u64`, `identifier: String`, `killer: u64` | `experience: u32`, `drops: bool` |
| `on_armor_stand_change` | A player puts an item on or takes one from an armour stand. | `player: String`, `entity: u64`, `slot: u32`, `item: i16` | --- |
| `on_entity_combust` | An entity catches fire by itself: the undead in daylight. | `entity: u64`, `identifier: String`, `cause: String`, `world: u32` | `ticks: u32` |
| `on_item_spawn` | An item entity appears in the world. | `entity: u64`, `item: i16`, `count: u32`, `x: f32`, `y: f32`, `z: f32`, `world: u32` | --- |
| `on_item_despawn` | An item entity vanished after lying too long or burning. Cancelling changes nothing. | `entity: u64`, `item: i16`, `count: u32` | --- |

## Mobs (`event::mob`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_mob_spawn` | A mob is about to spawn by itself: `cause` is natural (spawning in the dark or on grass) or breeding. | `identifier: String`, `x: f32`, `y: f32`, `z: f32`, `cause: String`, `world: u32` | --- |
| `on_mob_despawn` | A mob nature spawned is about to go away because no player is near. Cancelled, it stays for good. | `entity: u64`, `identifier: String`, `world: u32` | --- |
| `on_mob_target` | A mob picks who to attack; `target` is 0 when it gives up. Set another target, or cancel to keep it from attacking anyone. | `entity: u64`, `identifier: String`, `world: u32` | `target: u64` |
| `on_mob_attack` | A mob hits a player or entity in melee: zombies, spiders, golems, wolves and the rest. Arrows, potions and fireballs raise projectile events instead. | `entity: u64`, `identifier: String`, `target: u64`, `world: u32` | `damage: f32` |
| `on_mob_breed` | Two mobs in love have a baby. | `parent_a: u64`, `parent_b: u64`, `identifier: String`, `world: u32` | `experience: u32` |
| `on_mob_interact` | A player uses an item on a mob: `action` is feed (falling in love or helping a baby grow), shear or milk. | `player: String`, `entity: u64`, `identifier: String`, `item: i16`, `action: String` | --- |
| `on_mob_tame` | A player tames a mob: a wolf given a bone, a cat fish, a parrot seeds, or a horse, donkey, mule or llama ridden until it calms. Cancelled, it stays wild. | `player: String`, `entity: u64`, `identifier: String`, `world: u32` | --- |
| `on_mob_leash` | A lead is tied to a mob, held by a player or, with `fence`, knotted to the fence post at `x`, `y`, `z`; or with `leashed` false it comes off: untied, snapped when pulled too far, or its knot broken. `player` is empty when no player did it. Cancelled, a lead being tied is not; one coming off comes off anyway. | `player: String`, `entity: u64`, `identifier: String`, `leashed: bool`, `fence: bool`, `x: i32`, `y: i32`, `z: i32`, `world: u32` | --- |

## Villagers (`event::villager`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_villager_profession` | A villager takes up the profession of the job site block at `x`, `y`, `z`, or loses its profession, `profession` then empty, when the block goes before it ever traded. Cancelled, it stays as it was. | `entity: u64`, `profession: String`, `x: i32`, `y: i32`, `z: i32`, `world: u32` | --- |
| `on_villager_trade` | A player trades with a villager or wandering trader: `times` trades of the offer at `offer` in its list, for `item`; `experience` is what the player earns. Cancelled, the trade does not happen. | `player: String`, `entity: u64`, `offer: u32`, `times: u32`, `item: i16` | `experience: i32` |
| `on_villager_level_up` | A villager reaches a new level from trading, 2 for apprentice up to 5 for master, and learns new trades. | `entity: u64`, `level: u32`, `profession: String`, `world: u32` | --- |

## The server and its plugins (`event::lifecycle`)

| Event | When | Fields | Editable |
| --- | --- | --- | --- |
| `on_server_started` | The server finished starting and now accepts players. `worlds` is how many worlds are loaded. Cancelling changes nothing. | `worlds: u32` | --- |
| `on_server_stop` | The server begins to stop, before it disconnects the players, unloads the plugins and saves the worlds. `players` is how many are online. Cancelling does not keep it from stopping. | `players: u32` | --- |
| `on_console_command` | A line is typed in the server console, before it runs. Change `command` to run something else. Cancelled, nothing runs. | --- | `command: String` |
| `on_plugin_enable` | A plugin finished loading and is now enabled, itself included. Cancelling changes nothing. | `name: String`, `version: String` | --- |
| `on_plugin_disable` | A plugin is about to be disabled, by a reload, an unload or the server stopping. Cancelling does not keep it loaded. | `name: String`, `version: String` | --- |
