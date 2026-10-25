use crate::event::projectile::{
    PotionSplashEvent, ProjectileHitEvent, ProjectileLaunchEvent, ProjectilePickupEvent,
};
use crate::ffi::behaviour::{
    PotionSplashEventData, ProjectileHitEventData, ProjectileLaunchEventData,
    ProjectilePickupEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    projectile_launch,
    ProjectileLaunchEventData => ProjectileLaunchEvent,
    |raw: &ProjectileLaunchEventData| ProjectileLaunchEvent {
        player: ffi_string(raw.player),
        identifier: ffi_string(raw.identifier),
        force: std::cell::Cell::new(raw.force),
    },
    |safe: ProjectileLaunchEvent, raw: &mut ProjectileLaunchEventData| {
        raw.force = safe.force.get();
    }
);

impl Cancellable for ProjectileLaunchEvent {}

impl Server {
    /**
    Calls `handler` when a player launches a projectile: an arrow, a trident, a firework, a fishing hook...

    Returning `true` cancels the launch.

    See [`ProjectileLaunchEvent`].
    */
    pub fn on_projectile_launch(
        &self,
        handler: impl Fn(&Server, &ProjectileLaunchEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<projectile_launch>(handler);
    }
}

hook!(
    projectile_hit,
    ProjectileHitEventData => ProjectileHitEvent,
    |raw: &ProjectileHitEventData| ProjectileHitEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        owner: raw.owner,
        target: raw.target,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        damage: std::cell::Cell::new(raw.damage),
    },
    |safe: ProjectileHitEvent, raw: &mut ProjectileHitEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for ProjectileHitEvent {}

impl Server {
    /**
    Calls `handler` when a projectile hits a block or an entity.

    Returning `true` cancels the hit.

    See [`ProjectileHitEvent`].
    */
    pub fn on_projectile_hit(
        &self,
        handler: impl Fn(&Server, &ProjectileHitEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<projectile_hit>(handler);
    }
}

hook!(
    projectile_pickup,
    ProjectilePickupEventData => ProjectilePickupEvent,
    |raw: &ProjectilePickupEventData| ProjectilePickupEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        item: raw.item,
    }
);

impl Cancellable for ProjectilePickupEvent {}

impl Server {
    /**
    Calls `handler` when a player picks up an arrow or a trident stuck in the world.

    Returning `true` leaves it in place.

    See [`ProjectilePickupEvent`].
    */
    pub fn on_projectile_pickup(
        &self,
        handler: impl Fn(&Server, &ProjectilePickupEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<projectile_pickup>(handler);
    }
}

hook!(
    potion_splash,
    PotionSplashEventData => PotionSplashEvent,
    |raw: &PotionSplashEventData| PotionSplashEvent {
        entity: raw.entity,
        potion: raw.potion,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        radius: std::cell::Cell::new(raw.radius),
        lingering: raw.lingering,
    },
    |safe: PotionSplashEvent, raw: &mut PotionSplashEventData| {
        raw.radius = safe.radius.get();
    }
);

impl Cancellable for PotionSplashEvent {}

impl Server {
    /**
    Calls `handler` when a thrown potion breaks.

    Returning `true` cancels the effect: the potion breaks without affecting anything.

    See [`PotionSplashEvent`].
    */
    pub fn on_potion_splash(
        &self,
        handler: impl Fn(&Server, &PotionSplashEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<potion_splash>(handler);
    }
}
