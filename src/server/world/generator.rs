use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::ffi::abi::{ChunkHandle, FfiStr, GeneratorCallback};
use crate::ffi::call::HostFn;
use crate::registry::server;
use crate::server::Server;
use crate::server::world::Chunk;

type GeneratorFn = Box<dyn Fn(&mut Chunk, i32, i32) + Send + Sync>;

impl Server {
    /**
    Generates the new chunks of the world named `name` with `generator`, instead of the world's own generator. `generator` receives an empty chunk and its chunk coordinates, and is called from several threads at once.

    ```
    use bss_api::prelude::*;

    // A world of grass islands every 64 blocks.
    fn islands(server: &Server) {
        server.register_generator("islands", |chunk, cx, cz| {
            if cx % 4 == 0 && cz % 4 == 0 {
                let grass = Block::named("minecraft:grass_block");
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.set(x, 64, z, grass);
                    }
                }
            }
        });
    }
    ```
    */
    pub fn register_generator<F>(&self, name: &str, generator: F)
    where
        F: Fn(&mut Chunk, i32, i32) + Send + Sync + 'static,
    {
        let Some(f) = self.host().generator_register else {
            return;
        };
        let boxed: GeneratorFn = Box::new(generator);
        let user_data = Box::into_raw(Box::new(boxed)) as *mut c_void;
        f.call((
            FfiStr::new(name),
            GeneratorCallback::Some(generate_chunk),
            user_data,
        ));
    }
}

unsafe extern "C" fn generate_chunk(cx: i32, cz: i32, chunk: ChunkHandle, user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    // SAFETY: `user_data` is the boxed `GeneratorFn` that `register_generator` leaked
    // for this generator; it is never freed.
    let generator = unsafe { &*(user_data as *const GeneratorFn) };
    let mut chunk = Chunk::new(server().host(), chunk);
    let _ = catch_unwind(AssertUnwindSafe(|| generator(&mut chunk, cx, cz)));
}
