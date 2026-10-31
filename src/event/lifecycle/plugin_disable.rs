/**
A plugin is about to be disabled, by a reload, an unload or the server stopping.

`event.cancel()` does not keep the plugin loaded.

Handled by [`Event::on_plugin_disable`](crate::prelude::Event::on_plugin_disable) and [`Server::on_plugin_disable`](crate::server::Server::on_plugin_disable).
*/
pub struct PluginDisableEvent {
    /// Name of the plugin or world.
    pub name: String,
    /// Version of the plugin.
    pub version: String,
}
