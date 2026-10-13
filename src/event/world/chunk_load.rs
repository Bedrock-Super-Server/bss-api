/**
A chunk is loaded into memory.

Handled by [`Event::on_chunk_load`](crate::prelude::Event::on_chunk_load).
*/
pub struct ChunkLoadEvent {
    /// X coordinate of the chunk, in chunks.
    pub cx: i32,
    /// Z coordinate of the chunk, in chunks.
    pub cz: i32,
    /// `true` when the chunk was generated for the first time.
    pub generated: bool,
}
