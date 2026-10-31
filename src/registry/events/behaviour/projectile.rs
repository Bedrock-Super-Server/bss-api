use crate::event::projectile::{
    PotionSplashEvent, ProjectileHitEvent, ProjectileLaunchEvent, ProjectilePickupEvent,
};
use crate::ffi::behaviour::{
    PotionSplashEventData, ProjectileHitEventData, ProjectileLaunchEventData,
    ProjectilePickupEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl ProjectileLaunchEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_projectile_launch<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ProjectileLaunchEvent) -> V + Send + Sync + 'static,
    ) {
        register::<projectile_launch, V>(handler);
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

impl ProjectileHitEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_projectile_hit<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ProjectileHitEvent) -> V + Send + Sync + 'static,
    ) {
        register::<projectile_hit, V>(handler);
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

impl ProjectilePickupEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_projectile_pickup<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ProjectilePickupEvent) -> V + Send + Sync + 'static,
    ) {
        register::<projectile_pickup, V>(handler);
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

impl PotionSplashEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_potion_splash<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PotionSplashEvent) -> V + Send + Sync + 'static,
    ) {
        register::<potion_splash, V>(handler);
    }
}
