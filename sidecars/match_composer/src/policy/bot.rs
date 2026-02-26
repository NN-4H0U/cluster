use crate::config::BotConfig;
use crate::image::{Image, ImageProcess};
use super::Policy;

pub type BotPolicy = Policy<BotConfig>;

impl BotPolicy {
    pub fn new(config: BotConfig, image: Box<dyn Image>) -> Self {
        BotPolicy {
            cfg: config,
            image,
        }
    }

    pub async fn spawn(&self) -> ImageProcess {
        let cmd = self.image.player_cmd(&self.cfg.player());
        ImageProcess::spawn(cmd, Some(self.cfg.log_path.clone().into_boxed_path()))
            .expect("Failed to spawn bot process")
    }

    pub fn unum(&self) -> u8 {
        self.cfg.unum
    }
}
