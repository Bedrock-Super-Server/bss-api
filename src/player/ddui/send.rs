use std::ffi::c_void;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::ffi::abi::{DduiCallback, FfiStr};
use crate::player::Player;
use crate::player::ddui::{DduiClose, DduiEvent, DduiHandle, DduiScreen, handlers};

impl Player {
    /**
    Shows a data-driven screen, and calls `handler` with each thing that happens in it until it closes.

    The screen stays open while the player uses it, and a plugin can change it with the [`DduiHandle`] the
    handler receives and this method returns. The last event of a screen is [`DduiEvent::Closed`]. A player has
    one screen at a time: a screen shown to them closes the one they had, whose handler gets
    [`DduiClose::Replaced`].

    Returns `None` when the server does not know data-driven screens, the player is gone, or the server
    refused the screen; [`DduiScreen`] lists what it refuses.

    A plugin can also follow every screen with the events [`DduiResponseEvent`](crate::event::DduiResponseEvent) and
    [`DduiCloseEvent`](crate::event::DduiCloseEvent). Returning `true` from the first puts the control the player
    used back as it was, and the handler does not see it.

    See [`DduiScreen`] for an example.
    */
    pub fn send_ddui(
        &self,
        screen: DduiScreen,
        handler: impl FnMut(&DduiHandle, DduiEvent) + Send + 'static,
    ) -> Option<DduiHandle> {
        let json = screen.json();
        let replaced = handlers::take_all(self.handle);
        let callback: DduiCallback = Some(handlers::answer);
        let form_id = host_call!(
            self.host,
            ddui_send(
                self.handle,
                FfiStr::new(&json),
                callback,
                std::ptr::null_mut::<c_void>(),
            )
        )
        .unwrap_or(0);
        let handle = DduiHandle::new(*self, form_id);
        for (old_id, old) in replaced {
            // A handler that opens the next screen is running right now and is not told.
            if let Some(mut old) = old.try_lock() {
                let old_handle = DduiHandle::new(*self, old_id);
                old(&old_handle, DduiEvent::Closed(DduiClose::Replaced));
            }
        }
        if form_id == 0 {
            return None;
        }
        handlers::register(self.handle, form_id, Arc::new(Mutex::new(handler)));
        Some(handle)
    }
}
