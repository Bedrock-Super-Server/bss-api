use crate::ffi::call::HostFn;
use crate::server::Server;

impl Server {
    /// Returns whether the block at `(x, y, z)` of the default world is anything but air.
    pub fn is_solid(&self, x: i32, y: i32, z: i32) -> bool {
        host_call!(self.host(), world_is_solid(x, y, z)).unwrap_or(false)
    }

    /// Looks down from `from_y` for at most `max_fall` blocks and returns the height a body would stand at: one above the first block that is not air. Returns `None` when there is only air below.
    pub fn ground_height(&self, x: i32, from_y: i32, z: i32, max_fall: i32) -> Option<i32> {
        let f = self.host().world_ground_height?;
        let y = f.call((x, from_y, z, max_fall));
        (y != i32::MIN).then_some(y)
    }
}
