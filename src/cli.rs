use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// An oppinionated Git(hub) workflow tool.
pub struct Cli {
    #[argh(positional)]
    pub issue: Option<u64>,

    #[argh(subcommand)]
    pub command: Option<CommandEnum>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CommandEnum {
    PullRequest(CommandPullRequest),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Pull request command.
#[argh(subcommand, name = "pr")]
pub struct CommandPullRequest {}
