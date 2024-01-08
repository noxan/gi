use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// An oppinionated Git(hub) workflow tool.
pub struct Cli {
    #[argh(positional)]
    pub issue: Option<u64>,
}
