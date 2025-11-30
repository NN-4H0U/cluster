use std::any::Any;
use std::collections::VecDeque;
use std::sync::Arc;

use arcstr::ArcStr;
use dashmap::DashMap;
use log::debug;

use tokio::sync::{mpsc, oneshot};
use super::addon::AddOn;
use super::signal::{Signal, SignalKind};
use super::client::{TxMessage, RxMessage};

#[derive(Clone, Debug)]
pub struct CallResolver {
    rx: Arc<Receiver>,
}

impl CallResolver {
    pub fn new(receiver: mpsc::Receiver<ArcStr>) -> Self {
        let rx = Arc::new(Receiver::new(receiver));
        Self { rx }
    }

    pub fn sender(&self, tx: mpsc::Sender<ArcStr>) -> Sender {
        Sender::new(tx, Arc::clone(&self.rx))
    }

    pub fn weak(&self, tx: mpsc::WeakSender<ArcStr>) -> WeakSender {
        WeakSender::new(tx, Arc::clone(&self.rx))
    }
}

impl AddOn for CallResolver {
    fn new(
        _: mpsc::WeakSender<TxMessage>,
        receiver: mpsc::Receiver<RxMessage>
    ) -> Self where Self: Sized {
        Self::new(receiver)
    }
}

#[derive(Debug)]
struct Receiver {
    recv_task: tokio::task::JoinHandle<()>,

    queue: Arc<DashMap<
        SignalKind,
        VecDeque<oneshot::Sender<Result<Box<dyn Any + Send>, Box<dyn Any + Send>>>>
    >>,
}

impl Receiver {
    fn new(mut receiver: mpsc::Receiver<ArcStr>) -> Self {
        let tasks: Arc<DashMap<
            SignalKind,
            VecDeque<
                oneshot::Sender<
                    Result<
                        Box<dyn Any + Send>,
                        Box<dyn Any + Send>
                    >
                >
            >
        >> = Arc::new(DashMap::new());

        let tasks_ = Arc::clone(&tasks);
        let recv_task = tokio::spawn(async move {
            while let Some(raw_msg) = receiver.recv().await {
                let msg = raw_msg.trim();
                if msg.is_empty() || !msg.starts_with('(') || !msg.ends_with(')') {
                    debug!("ignoring invalid peer return message: {}", msg);
                    continue;
                }

                let mut msg = msg[1..msg.len()-1].split(' ');
                let (kind, ret) = match msg.next() {
                    Some("ok") => {
                        if  let Some(kind_str) = msg.next() &&
                            let Some(sig_kind) = SignalKind::decode(kind_str) {
                            let rest: Vec<_> = msg.collect();
                            let ret = sig_kind.parse_ret_ok(&rest);
                            match ret {
                                Some(ok) => (sig_kind, Ok(ok)),
                                None => {
                                    debug!("[CallResolver] Ignore \"ok\" for [{}]: {raw_msg}", sig_kind.encode());
                                    continue;
                                }
                            }
                        } else {
                            debug!("[CallResolver] Ignore \"ok\" for unknown Sig: {raw_msg}");
                            continue;
                        }
                    },
                    Some("error") => {
                        let mut ret = None;

                        let rest: Vec<_> = msg.collect();

                        let map_keys = tasks_.iter()
                            .map(|entry| *entry.key());

                        for sig_kind in map_keys {
                            if let Some(err) = sig_kind.parse_ret_err(&rest) {
                                ret = Some((sig_kind, Err(err)));
                                break;
                            }
                        }

                        match ret {
                            Some(error) => error,
                            None => {
                                debug!("[CallResolver] Ignore \"error\" for unknown Sig: {raw_msg}");
                                continue;
                            }
                        }
                    },
                    _ => {
                        debug!("[CallResolver] Ignore unknown msg: {raw_msg}");
                        continue;
                    }
                };

                if let Some(mut queue) = tasks_.get_mut(&kind) {
                    if let Some(tx) = queue.value_mut().pop_front() {
                        if tx.send(ret).is_err() {
                            debug!("[CallResolver] Failed to send return to caller for [{}]", kind.encode());
                        }
                    }
                }
            }
        });

        Self {
            recv_task,
            queue: tasks,
        }
    }

    fn add_queue(
        &self, signal: SignalKind,
        tx: oneshot::Sender<Result<Box<dyn Any + Send>, Box<dyn Any + Send>>>
    ) {
        self.queue.entry(signal).or_default().push_back(tx)
    }
}

#[derive(Clone, Debug)]
struct Sender {
    tx: mpsc::Sender<ArcStr>,
    resolver: Arc<Receiver>,
}

impl Sender {
    fn new(tx: mpsc::Sender<ArcStr>, resolver: Arc<Receiver>) -> Self {
        Self { tx, resolver }
    }

    pub async fn send<T: Signal>(&self, sig: T) -> Result<T::Ok, T::Error> {
        let sig_kind = sig.kind();
        let sender = &self.tx;
        sender.send(sig.encode()).await.expect("todo!");

        let (tx, rx) = oneshot::channel();
        self.resolver.add_queue(sig_kind, tx);
        match rx.await.expect("todo!") {
            Ok(ok) => {
                let ok = *ok.downcast::<T::Ok>().expect("todo!");
                Ok(ok)
            }
            Err(err) => {
                let err = *err.downcast::<T::Error>().expect("todo!");
                Err(err)
            }
        }
    }

    pub fn downgrade(&self) -> WeakSender {
        let tx = self.tx.downgrade();
        WeakSender::new(tx, Arc::clone(&self.resolver))
    }
}

#[derive(Clone, Debug)]
pub struct WeakSender {
    tx: mpsc::WeakSender<ArcStr>,
    resolver: Arc<Receiver>,
}

impl WeakSender {
    fn new(tx: mpsc::WeakSender<ArcStr>, resolver: Arc<Receiver>) -> Self {
        Self { tx, resolver }
    }

    pub async fn send<T: Signal>(&self, sig: T) -> Result<T::Ok, T::Error> {
        let sig_kind = sig.kind();
        let sender = self.tx.upgrade().expect("todo!");
        sender.send(sig.encode()).await.expect("todo!");

        let (tx, rx) = oneshot::channel();
        self.resolver.add_queue(sig_kind, tx);
        match rx.await.expect("todo!") {
            Ok(ok) => {
                let ok = *ok.downcast::<T::Ok>().expect("todo!");
                Ok(ok)
            }
            Err(err) => {
                let err = *err.downcast::<T::Error>().expect("todo!");
                Err(err)
            }
        }
    }

    pub fn upgrade(self) -> Option<Sender> {
        let tx = self.tx.upgrade()?;
        Some(Sender::new(tx, self.resolver))
    }
}