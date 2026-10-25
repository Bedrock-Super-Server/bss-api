use bss_api::prelude::{Command, CommandContext, CommandResult, Param};

pub struct Example1;

impl Command for Example1 {
    fn name(&self) -> &'static str {
        "example1"
    }

    fn description(&self) -> &'static str {
        "example1"
    }

    fn overloads(&self) -> Vec<Vec<Param>> {
        vec![vec![Param::text("name").optional()]]
    }

    fn execute(&self, ctx: &CommandContext) -> CommandResult {
        let name = ctx.arg(0).unwrap_or(ctx.sender());
        ctx.reply(&format!("{name} /example1"));
        Ok(())
    }
}
