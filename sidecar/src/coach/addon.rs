use std::fmt::Debug;

use tokio::sync::mpsc;
use arcstr::ArcStr;

use super::client::{RxMessage, TxMessage};


pub trait AddOn: Debug + Send + 'static {
    fn new(sender: mpsc::WeakSender<TxMessage>, receiver: mpsc::Receiver<RxMessage>) -> Self where Self: Sized;
}