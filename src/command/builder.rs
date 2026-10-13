use parking_lot::Mutex;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::ffi::abi::{CommandCallback, FfiStr};
use crate::registry::server;
use crate::server::{Server, ffi_string};

use super::{CommandContext, Param};
use crate::ffi::call::HostFn;

static COMMANDS: Mutex<Vec<&'static (dyn Fn(&CommandContext) + Send + Sync)>> =
    Mutex::new(Vec::new());

pub(crate) struct CommandBuilder {
    server: Server,
    name: String,
    description: String,
    permission: String,
    aliases: Vec<String>,
    overloads: Vec<Vec<Param>>,
}

impl CommandBuilder {
    pub(crate) fn permission(mut self, permission: &str) -> Self {
        self.permission = permission.to_string();
        self
    }

    pub(crate) fn alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }

    pub(crate) fn overload(mut self, params: Vec<Param>) -> Self {
        self.overloads.push(params);
        self
    }

    pub(crate) fn handler(self, handler: impl Fn(&CommandContext) + Send + Sync + 'static) {
        let Some(register) = self.server.host().command_register else {
            return;
        };
        let handler: &'static (dyn Fn(&CommandContext) + Send + Sync) =
            Box::leak(Box::new(handler));
        let overloads_json =
            serde_json::to_string(&self.overloads).unwrap_or_else(|_| "[]".to_string());
        for name in std::iter::once(&self.name).chain(self.aliases.iter()) {
            let index = {
                let mut commands = COMMANDS.lock();
                commands.push(handler);
                commands.len() - 1
            };
            /* `user_data` carries no pointer: it is the handler's index in
             * `COMMANDS`, which `run_command` reads back with a bounds check. */
            register.call((
                FfiStr::new(name),
                FfiStr::new(&self.description),
                FfiStr::new(&self.permission),
                FfiStr::new(&overloads_json),
                CommandCallback::Some(run_command),
                index as *mut c_void,
            ));
        }
    }
}

impl Server {
    pub(crate) fn command(&self, name: &str, description: &str) -> CommandBuilder {
        CommandBuilder {
            server: *self,
            name: name.to_string(),
            description: description.to_string(),
            permission: String::new(),
            aliases: Vec::new(),
            overloads: Vec::new(),
        }
    }
}

unsafe extern "C" fn run_command(sender: FfiStr, args_json: FfiStr, user_data: *mut c_void) {
    let index = user_data as usize;
    let Some(handler) = COMMANDS.lock().get(index).copied() else {
        return;
    };
    let sender = ffi_string(sender);
    let args: Vec<String> = serde_json::from_str(&ffi_string(args_json)).unwrap_or_default();
    let ctx = CommandContext::new(server(), sender, args);
    let _ = catch_unwind(AssertUnwindSafe(|| handler(&ctx)));
}
