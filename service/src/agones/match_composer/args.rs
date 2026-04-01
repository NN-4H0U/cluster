use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, group = "match_composer", help = "Enable Match Composer Sidecar Mode")]
    pub match_composer: bool,

    #[arg(long, default_value_t = 6657, requires = "match_composer", help = "Match Composer HTTP server port, default at 6657")]
    pub match_composer_port: u16,
}

impl Args {
    pub fn is_enabled(&self) -> bool {
        self.match_composer
    }
}
