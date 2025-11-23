use super::ControlMessage;
use super::client::{Client};

pub struct OfflineCoach {
    conn: Client,
}

impl OfflineCoach {
    pub async fn send_ctrl(&mut self, ctrl: ControlMessage) -> Result<(), String> {
        // let ctrl = ctrl.encode();
        // self.conn.send(ctrl.into()).await;

        todo!()
    }
}