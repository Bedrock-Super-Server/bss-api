/*!
Potion effects on players and entities.

```
use bss_api::prelude::*;

fn boost(player: &Player) {
    // Speed II for 30 seconds.
    player.apply_effect(Effect::Speed, 1, 30 * 20);
    if player.has_effect(Effect::Poison) {
        player.remove_effect(Effect::Poison);
    }
}
```
*/

#![allow(deprecated)]

use crate::ffi::abi::FfiStr;
use crate::ffi::call::HostFn;
use crate::player::Player;
use crate::server::Server;

/// A vanilla potion effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// Speed (`minecraft:speed`).
    Speed,
    /// Slowness (`minecraft:slowness`).
    Slowness,
    /// Haste (`minecraft:haste`).
    Haste,
    /// Mining fatigue (`minecraft:mining_fatigue`).
    MiningFatigue,
    /// Strength (`minecraft:strength`).
    Strength,
    /// Instant health (`minecraft:instant_health`).
    InstantHealth,
    /// Instant damage (`minecraft:instant_damage`).
    InstantDamage,
    /// Jump boost (`minecraft:jump_boost`).
    JumpBoost,
    /// Nausea (`minecraft:nausea`).
    Nausea,
    /// Regeneration (`minecraft:regeneration`).
    Regeneration,
    /// Resistance (`minecraft:resistance`).
    Resistance,
    /// Fire resistance (`minecraft:fire_resistance`).
    FireResistance,
    /// Water breathing (`minecraft:water_breathing`).
    WaterBreathing,
    /// Invisibility (`minecraft:invisibility`).
    Invisibility,
    /// Blindness (`minecraft:blindness`).
    Blindness,
    /// Night vision (`minecraft:night_vision`).
    NightVision,
    /// Hunger (`minecraft:hunger`).
    Hunger,
    /// Weakness (`minecraft:weakness`).
    Weakness,
    /// Poison (`minecraft:poison`).
    Poison,
    /// Wither (`minecraft:wither`).
    Wither,
    /// Health boost (`minecraft:health_boost`).
    HealthBoost,
    /// Absorption (`minecraft:absorption`).
    Absorption,
    /// Saturation (`minecraft:saturation`).
    Saturation,
    /// Levitation (`minecraft:levitation`).
    Levitation,
    /// Fatal poison (`minecraft:fatal_poison`).
    FatalPoison,
    /// Conduit power (`minecraft:conduit_power`).
    ConduitPower,
    /// Slow falling (`minecraft:slow_falling`).
    SlowFalling,
    /// Bad omen (`minecraft:bad_omen`).
    BadOmen,
    /// Village hero (`minecraft:village_hero`).
    VillageHero,
    /// Darkness (`minecraft:darkness`).
    Darkness,
}

impl Effect {
    /// Returns the namespaced name of the effect, such as `minecraft:speed`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Speed => "minecraft:speed",
            Self::Slowness => "minecraft:slowness",
            Self::Haste => "minecraft:haste",
            Self::MiningFatigue => "minecraft:mining_fatigue",
            Self::Strength => "minecraft:strength",
            Self::InstantHealth => "minecraft:instant_health",
            Self::InstantDamage => "minecraft:instant_damage",
            Self::JumpBoost => "minecraft:jump_boost",
            Self::Nausea => "minecraft:nausea",
            Self::Regeneration => "minecraft:regeneration",
            Self::Resistance => "minecraft:resistance",
            Self::FireResistance => "minecraft:fire_resistance",
            Self::WaterBreathing => "minecraft:water_breathing",
            Self::Invisibility => "minecraft:invisibility",
            Self::Blindness => "minecraft:blindness",
            Self::NightVision => "minecraft:night_vision",
            Self::Hunger => "minecraft:hunger",
            Self::Weakness => "minecraft:weakness",
            Self::Poison => "minecraft:poison",
            Self::Wither => "minecraft:wither",
            Self::HealthBoost => "minecraft:health_boost",
            Self::Absorption => "minecraft:absorption",
            Self::Saturation => "minecraft:saturation",
            Self::Levitation => "minecraft:levitation",
            Self::FatalPoison => "minecraft:fatal_poison",
            Self::ConduitPower => "minecraft:conduit_power",
            Self::SlowFalling => "minecraft:slow_falling",
            Self::BadOmen => "minecraft:bad_omen",
            Self::VillageHero => "minecraft:village_hero",
            Self::Darkness => "minecraft:darkness",
        }
    }
}

impl Server {
    /// Gives an effect to a player or an entity for `duration_ticks` ticks, with particles. `amplifier` `0` is level I. Returns `false` when the target does not exist or `duration_ticks` is not positive.
    #[deprecated(
        note = "use `server.entities().apply_effect(entity_id, effect, amplifier, duration_ticks)`"
    )]
    pub fn apply_effect(
        &self,
        entity_id: u64,
        effect: Effect,
        amplifier: u8,
        duration_ticks: i32,
    ) -> bool {
        self.apply_effect_named(
            entity_id,
            effect.name(),
            amplifier,
            duration_ticks,
            false,
            true,
        )
    }

    /// Gives an effect by its name, such as `minecraft:speed`. `ambient` makes the particles faint, as a beacon's are, and `particles` shows them at all.
    #[deprecated(
        note = "use `server.entities().apply_effect_named(entity_id, name, amplifier, duration_ticks, ambient, particles)`"
    )]
    pub fn apply_effect_named(
        &self,
        entity_id: u64,
        name: &str,
        amplifier: u8,
        duration_ticks: i32,
        ambient: bool,
        particles: bool,
    ) -> bool {
        self.host()
            .effect_apply
            .map(|f| {
                f.call((
                    entity_id,
                    FfiStr::new(name),
                    i32::from(amplifier),
                    duration_ticks,
                    ambient,
                    particles,
                ))
            })
            .unwrap_or(false)
    }

    /// Removes an effect. Returns `false` when the target did not have it.
    #[deprecated(note = "use `server.entities().remove_effect(entity_id, effect)`")]
    pub fn remove_effect(&self, entity_id: u64, effect: Effect) -> bool {
        host_call!(
            self.host(),
            effect_remove(entity_id, FfiStr::new(effect.name()))
        )
        .unwrap_or(false)
    }

    /// Removes every effect. Returns `false` when the target had none.
    #[deprecated(note = "use `server.entities().clear_effects(entity_id)`")]
    pub fn clear_effects(&self, entity_id: u64) -> bool {
        host_call!(self.host(), effect_clear(entity_id)).unwrap_or(false)
    }

    /// Returns whether the target has an effect.
    #[deprecated(note = "use `server.entities().has_effect(entity_id, effect)`")]
    pub fn has_effect(&self, entity_id: u64, effect: Effect) -> bool {
        host_call!(
            self.host(),
            effect_holds(entity_id, FfiStr::new(effect.name()))
        )
        .unwrap_or(false)
    }

    /// Returns the amplifier of an effect, `0` for level I, or `None` when the target does not have it.
    #[deprecated(note = "use `server.entities().effect_amplifier(entity_id, effect)`")]
    pub fn effect_amplifier(&self, entity_id: u64, effect: Effect) -> Option<u8> {
        let level = host_call!(
            self.host(),
            effect_amplifier(entity_id, FfiStr::new(effect.name()))
        )
        .unwrap_or(-1);
        u8::try_from(level).ok()
    }

    /// Returns the ticks left of an effect, or `None` when the target does not have it.
    #[deprecated(note = "use `server.entities().effect_duration(entity_id, effect)`")]
    pub fn effect_duration(&self, entity_id: u64, effect: Effect) -> Option<i32> {
        let ticks = host_call!(
            self.host(),
            effect_duration(entity_id, FfiStr::new(effect.name()))
        )
        .unwrap_or(-1);
        (ticks >= 0).then_some(ticks)
    }

    /// Returns the names of the effects the target has.
    #[deprecated(note = "use `server.entities().active_effects(entity_id)`")]
    pub fn active_effects(&self, entity_id: u64) -> Vec<String> {
        let Some(f) = self.host().effect_active else {
            return Vec::new();
        };
        let mut buf = [0u32; 64];
        let count = f.call((entity_id, buf.as_mut_ptr(), buf.len() as u32)) as usize;
        let named = self.host().effect_name;
        buf.iter()
            .take(count.min(buf.len()))
            .filter_map(|id| named.map(|g| crate::server::ffi_string(g.call((*id as i32,)))))
            .filter(|name| !name.is_empty())
            .collect()
    }
}

impl Player {
    /// Gives the player an effect; see [`Server::apply_effect`].
    pub fn apply_effect(&self, effect: Effect, amplifier: u8, duration_ticks: i32) -> bool {
        self.server()
            .apply_effect(self.entity_id(), effect, amplifier, duration_ticks)
    }

    /// Removes an effect from the player; see [`Server::remove_effect`].
    pub fn remove_effect(&self, effect: Effect) -> bool {
        self.server().remove_effect(self.entity_id(), effect)
    }

    /// Removes every effect from the player.
    pub fn clear_effects(&self) -> bool {
        self.server().clear_effects(self.entity_id())
    }

    /// Returns whether the player has an effect.
    pub fn has_effect(&self, effect: Effect) -> bool {
        self.server().has_effect(self.entity_id(), effect)
    }

    /// Returns the amplifier of an effect the player has, `0` for level I.
    pub fn effect_amplifier(&self, effect: Effect) -> Option<u8> {
        self.server().effect_amplifier(self.entity_id(), effect)
    }

    /// Returns the ticks left of an effect the player has.
    pub fn effect_duration(&self, effect: Effect) -> Option<i32> {
        self.server().effect_duration(self.entity_id(), effect)
    }
}
