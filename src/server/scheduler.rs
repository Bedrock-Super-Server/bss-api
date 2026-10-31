//! Running code later, or repeatedly, on the tick thread.

use parking_lot::Mutex;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::server::Server;

type TaskHandler = &'static (dyn Fn(&Server) + Send + Sync);

/// Identifies a scheduled task, to cancel it with [`Server::cancel_task`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

struct Task {
    id: u64,
    next_tick: u64,
    interval: Option<u64>,
    handler: TaskHandler,
}

static TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
static CURRENT_TICK: AtomicU64 = AtomicU64::new(0);

fn push(
    delay_ticks: u64,
    interval: Option<u64>,
    handler: impl Fn(&Server) + Send + Sync + 'static,
) -> TaskId {
    let handler: Box<dyn Fn(&Server) + Send + Sync> = Box::new(handler);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let next_tick = CURRENT_TICK
        .load(Ordering::Relaxed)
        .saturating_add(delay_ticks.max(1));
    TASKS.lock().push(Task {
        id,
        next_tick,
        interval,
        handler: Box::leak(handler),
    });
    TaskId(id)
}

impl Server {
    /// Runs `task` once, `delay_ticks` ticks from now (at least one), on the thread that runs the world. Twenty ticks make a second.
    #[deprecated(note = "use `server.scheduler().schedule(delay_ticks, task)`")]
    pub fn schedule(
        &self,
        delay_ticks: u64,
        task: impl Fn(&Server) + Send + Sync + 'static,
    ) -> TaskId {
        crate::registry::events::server::run_scheduled_tasks(self);
        push(delay_ticks, None, task)
    }

    /**
    Runs `task` every `interval_ticks` ticks, starting `interval_ticks` from now, until it is cancelled or the plugin is unloaded.

    ```
    use bss_api::prelude::*;

    fn announce(server: &Server) -> TaskId {
        // Every five minutes.
        server.schedule_repeating(5 * 60 * 20, |server| {
            server.broadcast("§bRemember to vote for the server!");
        })
    }
    ```
    */
    #[deprecated(note = "use `server.scheduler().schedule_repeating(interval_ticks, task)`")]
    pub fn schedule_repeating(
        &self,
        interval_ticks: u64,
        task: impl Fn(&Server) + Send + Sync + 'static,
    ) -> TaskId {
        crate::registry::events::server::run_scheduled_tasks(self);
        push(interval_ticks, Some(interval_ticks.max(1)), task)
    }

    /// Cancels a task. Returns `false` when it already ran or was cancelled.
    #[deprecated(note = "use `server.scheduler().cancel(id)`")]
    pub fn cancel_task(&self, id: TaskId) -> bool {
        let mut tasks = TASKS.lock();
        let before = tasks.len();
        tasks.retain(|task| task.id != id.0);
        tasks.len() != before
    }
}

pub(crate) fn clear() {
    TASKS.lock().clear();
}

pub(crate) fn run_due(server: &Server, tick: u64) {
    CURRENT_TICK.store(tick, Ordering::Relaxed);
    let due: Vec<TaskHandler> = {
        let mut tasks = TASKS.lock();
        let mut due = Vec::new();
        tasks.retain_mut(|task| {
            if task.next_tick > tick {
                return true;
            }
            due.push(task.handler);
            match task.interval {
                Some(interval) => {
                    task.next_tick = tick.saturating_add(interval);
                    true
                }
                None => false,
            }
        });
        due
    };
    for handler in due {
        let _ = catch_unwind(AssertUnwindSafe(|| handler(server)));
    }
}
