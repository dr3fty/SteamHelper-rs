use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use futures::task::AtomicWaker;
use parking_lot::Mutex;

use crate::handlers::SteamEvents;

/// Steam client is able to send messages at any time, listen to messages and react to them.
///
///
/// Should provide a hot path for quick lookup if user is listening for one specific emsg.
/// This could be done after the dispatch map is initiated.
///
/// Each Handler could have a mapping of [EMsg] that it listens for.
#[derive(Debug, Default)]
pub struct DispatcherMap {
    pub(crate) current_job_id: AtomicU64,

    /// User is interested in listening for the followings events
    /// and will answer with a proper callback
    interested: HashMap<SteamEvents, String>,

    /// A hashmap that contains a message
    pub(crate) tracked_messages: Arc<Mutex<HashMap<u64, Box<dyn Any>>>>,
    /// A hashmap that contains a waker for a future, waiting for a message.
    pub(crate) tracked_jobids_wakers: Arc<Mutex<HashMap<u64, Box<AtomicWaker>>>>,
}

impl DispatcherMap {
    fn register_incoming_task(&self, source_job_id: u64) {
        let waker = self.tracked_jobids_wakers.lock().remove(&source_job_id).unwrap();
        waker.wake();
    }
}

impl DispatcherMap {
    pub(crate) fn new() -> Self {
        Self {
            current_job_id: AtomicU64::new(0),
            interested: Default::default(),
            tracked_messages: Default::default(),
            tracked_jobids_wakers: Default::default(),
        }
    }

    pub fn register_interest() {}
}
