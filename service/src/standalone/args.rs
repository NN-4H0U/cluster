use clap::Parser;
use super::BaseArgs;

#[derive(Parser, Debug)]
pub struct StandaloneArgs {
    #[clap(flatten)]
    pub base_args: BaseArgs,
}
