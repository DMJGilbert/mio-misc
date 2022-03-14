//! Miscellaneous components for use with Mio.
#![deny(missing_docs)]
extern crate crossbeam;
extern crate mio;

#[macro_use]
extern crate log;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod channel;
pub mod poll;
pub mod queue;
pub mod scheduler;

static NEXT_NOTIFICATION_ID: AtomicUsize = AtomicUsize::new(1);

/// Used while sending notifications
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NotificationId(pub usize);

impl NotificationId {
    /// Generates the next `NotificationId`, which is guaranteed to be unique
    pub fn gen_next() -> NotificationId {
        let id = NEXT_NOTIFICATION_ID.fetch_add(1, Ordering::SeqCst);
        NotificationId(id)
    }

    /// Returns id
    pub fn id(&self) -> usize {
        self.0
    }
}

impl fmt::Display for NotificationId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
