use bss_api::prelude::{Command, CommandContext, CommandResult, Param, SubCommand};

pub struct Example2;

impl Command for Example2 {
    fn name(&self) -> &'static str {
        "example2"
    }

    fn description(&self) -> &'static str {
        "example2"
    }

    fn subcommands(&self) -> Vec<Box<dyn SubCommand>> {
        vec![Box::new(Plugin), Box::new(Say)]
    }

    fn execute(&self, _ctx: &CommandContext) -> CommandResult {
        Err("Usage: /example2 <plugin|say>".into())
    }
}

struct Plugin;

impl SubCommand for Plugin {
    fn name(&self) -> &'static str {
        "plugin"
    }

    fn description(&self) -> &'static str {
        "example"
    }

    fn execute(&self, ctx: &CommandContext) -> CommandResult {
        ctx.reply("example");
        Ok(())
    }
}
