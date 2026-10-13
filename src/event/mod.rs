/*!
The events a plugin can react to.

There are two ways to receive them:

- implement [`Event`](crate::prelude::Event), overriding the methods of the events you care about, and pass it to [`Server::register_event`](crate::server::Server::register_event);
- or give a closure to one of the `Server::on_*` methods, such as [`Server::on_block_use`](crate::server::Server::on_block_use).

The documentation of each event type names the methods that receive it.

# Cancelling and changing events

Most handlers return a `bool`: returning `true` cancels the event, and the documentation of each type says what that prevents. Returning `false` leaves the event as it is.

Handlers run in order of [`Priority`], across every plugin: from [`Lowest`](Priority::Lowest) to [`Highest`](Priority::Highest), then [`Monitor`](Priority::Monitor). Each one sees what the handlers before it did. Through [`Cancellable`], a handler reads whether the event is already cancelled and may take the cancellation back; registered with [`EventOptions::ignore_cancelled`], it is not called at all for a cancelled event. A [`Monitor`](Priority::Monitor) handler only observes: what it changes is undone. Register with other options than the default ones through [`Server::register_event_with`](crate::server::Server::register_event_with) or [`Server::with_event_options`](crate::server::Server::with_event_options).

Values that handlers may change, such as the damage of a hit or a join message, are read and written through methods: `damage()` and `set_damage()`. Each handler sees the value the ones before it set, and the server applies the last one.

# Example

```
use bss_api::prelude::*;

struct Rules;

impl Event for Rules {
    // Half the damage of every melee hit.
    fn on_entity_damage(&self, _server: &Server, event: &EntityDamageEvent) -> bool {
        event.set_damage(event.damage() / 2.0);
        false
    }

    // No building above y = 200.
    fn on_block_place(&self, server: &Server, event: &BlockPlaceEvent) -> bool {
        if event.y > 200 {
            if let Some(player) = server.player(&event.name) {
                player.message("§cYou cannot build this high.");
            }
            return true;
        }
        false
    }
}

fn register(server: &Server) {
    server.register_event(Rules);

    // The same style of handler, as a closure.
    server.on_tnt_prime(|_server, event| {
        event.set_fuse(event.fuse() * 2);
        false
    });
}
```
*/

pub(crate) mod entity;
pub(crate) mod inventory;
pub(crate) mod player;
pub(crate) mod server;
pub(crate) mod world;

pub use crate::registry::hook::{Cancellable, EventOptions, Priority};
pub use entity::*;
pub use inventory::*;
pub use player::*;
pub use server::*;
pub use world::*;

pub(crate) mod block;
pub use block::*;
pub(crate) mod redstone;
pub use redstone::*;
pub(crate) mod portal;
pub use portal::*;
pub(crate) mod dragon;
pub use dragon::*;
pub(crate) mod projectile;
pub use projectile::*;
pub(crate) mod mob;
pub use mob::*;
pub(crate) mod villager;
pub use villager::*;
