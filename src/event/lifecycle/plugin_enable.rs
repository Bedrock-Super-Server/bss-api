/**
A plugin finished loading and is now enabled, itself included.

`event.cancel()` changes nothing.

Handled by [`Event::on_plugin_enable`](crate::prelude::Event::on_plugin_enable) and [`Server::on_plugin_enable`](crate::server::Server::on_plugin_enable).
*/
pub struct PluginEnableEvent {
    /// Name of the plugin or world.
    pub name: String,
    /// Version of the plugin.
    pub version: String,
}
