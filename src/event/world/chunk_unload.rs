/**
A chunk is unloaded from memory.

Handled by [`Event::on_chunk_unload`](crate::prelude::Event::on_chunk_unload).
*/
pub struct ChunkUnloadEvent {
    /// X coordinate of the chunk, in chunks.
    pub cx: i32,
    /// Z coordinate of the chunk, in chunks.
    pub cz: i32,
}
