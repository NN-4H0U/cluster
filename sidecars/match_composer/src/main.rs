use std::path::PathBuf;
use crate::config::MatchComposerConfig;

mod schema;
mod policy;
mod config;
mod image;
pub mod composer;
pub mod team;

pub use composer::{MatchComposer, AgentConnectionInfo};
use crate::schema::v1::ConfigV1;
// #[tokio::main]
// async fn main() {
//     let registry = policy::PolicyRegistry::new("sidecars/match_composer/hub");
//
//     let image = ImageQuery {
//         provider: "HELIOS".to_string(),
//         model: "helios-base".to_string(),
//     };
//
//     let server = ServerConfig::default();
//
//     let bot = registry.fetch_bot(BotConfig {
//         unum: 0,
//         side: Side::LEFT,
//         team: "TEST",
//         image: &image,
//         server: &server,
//         log_path: &Path::new("logs/bot.log"),
//     }).unwrap();
//
//     let process = bot.spawn().await;
//     let mut watch = process.status_watch();
//     loop {
//         if let Err(e) = watch.changed().await {
//             eprintln!("Bot process status watch error: {e}");
//             break;
//         }
//
//         let status = watch.borrow().clone();
//         println!("Bot process status changed: {status:?}");
//     }
// }

#[tokio::main]
async fn main() {
    let schema: ConfigV1 = serde_json::from_str(include_str!("../docs/template.json")).unwrap();
    let log_root = PathBuf::from("logs");
    let config = MatchComposerConfig::from_schema(schema, log_root).unwrap();

    let mut composer = MatchComposer::new(
        config, "sidecars/match_composer/hub",
    ).unwrap();
    
    composer.spawn_players().await.unwrap();
    composer.wait().await.unwrap();
}
