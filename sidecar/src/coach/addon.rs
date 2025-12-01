use std::fmt::Debug;

use tokio::sync::mpsc;
use arcstr::ArcStr;

use common::client::{RxData, TxData, TxSignal};


pub trait AddOn: Debug + Send + 'static {
    fn from_weak(
        sig_tx:  mpsc::WeakSender<TxSignal>,
        data_tx: mpsc::WeakSender<TxData>,
        data_rx: mpsc::Receiver<RxData>,
    ) -> Self where Self: Sized {
        let sig_tx = sig_tx.upgrade().expect("Failed to upgrade TxSignal weak sender");
        let data_tx = data_tx.upgrade().expect("Failed to upgrade TxData weak sender");
        Self::new(sig_tx, data_tx, data_rx)
    }
    
    fn new(
        sig_tx:  mpsc::Sender<TxSignal>,
        data_tx: mpsc::Sender<TxData>,
        data_rx: mpsc::Receiver<RxData>,
    ) -> Self where Self: Sized;
    
    fn close(&self) {
        
    }
}