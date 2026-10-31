/*!
Commands that players and the console can run.

Implement [`Command`] and register it with [`Server::register_command`]. The client shows the command, its description and its parameters in its command list and autocompletion.

# Example

A `/home` command with two subcommands, `/home set` and `/home go <name>`:

```
use bss_api::prelude::*;

struct Home;

impl Command for Home {
    fn name(&self) -> &'static str {
        "home"
    }

    fn description(&self) -> &'static str {
        "Manage your homes"
    }

    fn subcommands(&self) -> Vec<Box<dyn SubCommand>> {
        vec![Box::new(SetHome), Box::new(GoHome)]
    }

    fn execute(&self, _ctx: &CommandContext) -> CommandResult {
        Err("Usage: /home <set|go>".into())
    }
}

struct SetHome;

impl SubCommand for SetHome {
    fn name(&self) -> &'static str {
        "set"
    }

    fn execute(&self, ctx: &CommandContext) -> CommandResult {
        let player = ctx.player().ok_or("Only players have a home")?;
        let at = player.location();
        ctx.reply(&format!("Home set at {:.0} {:.0} {:.0}", at.x, at.y, at.z));
        Ok(())
    }
}

struct GoHome;

impl SubCommand for GoHome {
    fn name(&self) -> &'static str {
        "go"
    }

    fn params(&self) -> Vec<Param> {
        vec![Param::text("name").optional()]
    }

    fn execute(&self, ctx: &CommandContext) -> CommandResult {
        let name = ctx.arg(0).unwrap_or("home");
        ctx.reply(&format!("Teleporting to {name}..."));
        Ok(())
    }
}

fn register(server: &Server) {
    server.register_command(Home);
}
```
*/

pub(crate) mod builder;

use serde::Serialize;

use crate::ffi::call::HostFn;
use crate::player::Player;
use crate::server::Server;

/// The type of a command [`Param`], which sets how the client completes it.
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamKind {
    /// A whole number.
    Int,
    /// Free text, one word.
    Text,
    /// The name of a player, completed from the players online.
    Player,
    /// An entity selector, such as `@e`.
    Entity,
    /// One word out of [`Param::values`].
    Enum,
}

/**
A parameter of a command, as the client shows and completes it.

Build one with [`Param::int`], [`Param::text`], [`Param::player`], [`Param::entity`] or [`Param::enum_of`]:

```
use bss_api::prelude::*;

let params = vec![
    Param::player("target"),
    Param::enum_of("mode", &["survival", "creative"]),
    Param::int("seconds").optional(),
];
assert!(params[2].optional);
```
*/
#[derive(Clone, Serialize)]
pub struct Param {
    /// Name shown in the client's command help.
    pub name: String,
    /// Type of the value.
    pub kind: ParamKind,
    /// Accepted words, for an [`Enum`](ParamKind::Enum) parameter.
    pub values: Vec<String>,
    /// Whether the parameter can be left out.
    pub optional: bool,
}

impl Param {
    fn new(name: &str, kind: ParamKind) -> Self {
        Self {
            name: name.to_string(),
            kind,
            values: Vec::new(),
            optional: false,
        }
    }

    /// A whole number.
    pub fn int(name: &str) -> Self {
        Self::new(name, ParamKind::Int)
    }

    /// One word of free text.
    pub fn text(name: &str) -> Self {
        Self::new(name, ParamKind::Text)
    }

    /// The name of a player.
    pub fn player(name: &str) -> Self {
        Self::new(name, ParamKind::Player)
    }

    /// An entity selector, such as `@e`.
    pub fn entity(name: &str) -> Self {
        Self::new(name, ParamKind::Entity)
    }

    /// One of `values`.
    pub fn enum_of(name: &str, values: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            kind: ParamKind::Enum,
            values: values.iter().map(|v| v.to_string()).collect(),
            optional: false,
        }
    }

    /// Makes the parameter optional. Optional parameters come last.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

/// What a command receives when it runs: who sent it and with which arguments.
pub struct CommandContext {
    server: Server,
    sender: String,
    args: Vec<String>,
}

impl CommandContext {
    pub(crate) fn new(server: Server, sender: String, args: Vec<String>) -> Self {
        Self {
            server,
            sender,
            args,
        }
    }

    /// Returns the server.
    pub fn server(&self) -> Server {
        self.server
    }

    /// Returns the name of the player who ran the command, or `CONSOLE`.
    pub fn sender(&self) -> &str {
        &self.sender
    }

    /// Returns the player who ran the command, or `None` when it came from the console or the player has left.
    pub fn player(&self) -> Option<Player> {
        self.server.player(&self.sender)
    }

    /// Returns the arguments, split on spaces; a quoted argument keeps its spaces. For a [`SubCommand`], the subcommand's own name is not included.
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Returns the argument at `index`, counted from `0`.
    pub fn arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(String::as_str)
    }

    /// Returns the number of arguments.
    pub fn arg_count(&self) -> usize {
        self.args.len()
    }

    /// Returns whether the sender has `permission`. An empty permission and the console are always allowed.
    pub fn allowed(&self, permission: &str) -> bool {
        if permission.is_empty() {
            return true;
        }
        match self.player() {
            Some(player) => player.has_permission(permission),
            None => true,
        }
    }

    /// Sends `message` to the sender: in chat for a player, in the log for the console. `§` color codes are kept for players.
    pub fn reply(&self, message: &str) {
        if let Some(player) = self.player() {
            player.message(message);
            return;
        }
        if let Some(f) = self.server.host().log_line {
            for line in message.split('\n') {
                f.call((crate::ffi::FfiStr::new(line),));
            }
        }
    }

    fn shifted(&self) -> CommandContext {
        CommandContext {
            server: self.server,
            sender: self.sender.clone(),
            args: self
                .args
                .get(1..)
                .map(<[String]>::to_vec)
                .unwrap_or_default(),
        }
    }
}

/// What a command returns. An `Err` message is sent back to the sender in red.
pub type CommandResult = Result<(), String>;

const DENIED: &str = "§cYou do not have permission to use this command.";

/**
A subcommand, chosen by the first argument of its parent [`Command`].

The arguments it receives start after its own name. See the
[module documentation](self) for an example.
*/
pub trait SubCommand: Send + Sync + 'static {
    /// The word that selects this subcommand.
    fn name(&self) -> &'static str;

    /// Other words that select it.
    fn aliases(&self) -> &[&'static str] {
        &[]
    }

    /// Permission needed to run it; empty for none.
    fn permission(&self) -> &'static str {
        ""
    }

    /// Short description.
    fn description(&self) -> &'static str {
        ""
    }

    /// Parameters after the subcommand's name, for the client's completion.
    fn params(&self) -> Vec<Param> {
        Vec::new()
    }

    /// Runs the subcommand.
    fn execute(&self, ctx: &CommandContext) -> CommandResult;
}

/**
A command, registered with [`Server::register_command`].

See the [module documentation](self) for an example.
*/
pub trait Command: Send + Sync + 'static {
    /// The name typed after `/`.
    fn name(&self) -> &'static str;

    /// Description shown in the client's command list.
    fn description(&self) -> &'static str;

    /// Permission needed to run it; empty for none. Players without it are told so, and the command does not run.
    fn permission(&self) -> &'static str {
        ""
    }

    /// Other names that run the same command.
    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// The accepted parameter lists, for the client's completion. Each subcommand adds its own.
    fn overloads(&self) -> Vec<Vec<Param>> {
        Vec::new()
    }

    /// Subcommands, chosen by the first argument.
    fn subcommands(&self) -> Vec<Box<dyn SubCommand>> {
        Vec::new()
    }

    /// Runs the command when no subcommand matches its first argument.
    fn execute(&self, ctx: &CommandContext) -> CommandResult;
}

impl Server {
    /**
    Registers `command`, replacing any command of the same name.

    See the [`command`](crate::command) module for an example.
    */
    pub fn register_command<C: Command>(&self, command: C) {
        let name = command.name().to_owned();
        let description = command.description().to_owned();
        let permission = command.permission().to_owned();
        let aliases: Vec<String> = command.aliases().iter().map(|a| (*a).to_owned()).collect();
        let mut overloads = command.overloads();
        let subcommands = command.subcommands();

        for sub in &subcommands {
            let mut params = vec![Param::enum_of(sub.name(), &[sub.name()])];
            params.extend(sub.params());
            overloads.push(params);
        }

        let command = std::sync::Arc::new(command);
        let subcommands = std::sync::Arc::new(subcommands);
        let required = permission.clone();
        let mut builder = self.command(&name, &description);
        if !permission.is_empty() {
            builder = builder.permission(&permission);
        }
        for alias in &aliases {
            builder = builder.alias(alias);
        }
        for overload in overloads {
            builder = builder.overload(overload);
        }
        /* One handler serves the command and all its subcommands: the first
         * argument picks a subcommand by name or alias, and the subcommand then
         * sees the arguments that follow it. Permissions are checked twice, for
         * the command and for the subcommand, so a subcommand can be stricter
         * than its parent. An `Err` from either is shown to the sender in red. */
        builder.handler(move |ctx| {
            if !ctx.allowed(&required) {
                ctx.reply(DENIED);
                return;
            }
            if let Some(label) = ctx.arg(0) {
                let found = subcommands.iter().find(|sub| {
                    sub.name().eq_ignore_ascii_case(label)
                        || sub
                            .aliases()
                            .iter()
                            .any(|alias| alias.eq_ignore_ascii_case(label))
                });
                if let Some(sub) = found {
                    if !ctx.allowed(sub.permission()) {
                        ctx.reply(DENIED);
                        return;
                    }
                    if let Err(message) = sub.execute(&ctx.shifted()) {
                        ctx.reply(&format!("§c{message}"));
                    }
                    return;
                }
            }
            if let Err(message) = command.execute(ctx) {
                ctx.reply(&format!("§c{message}"));
            }
        });
    }
}
