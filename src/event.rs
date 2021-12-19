use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::RwLock;

use crate::connection::Connection;
use crate::utils::DataContainer;

#[derive(Clone)]
pub enum Event {

    Connect(Connection),
    Disconnect(DisconnectReason),
    ReceiveData(/*DataContainer*/),

}

#[derive(Copy, Clone)]
pub enum DisconnectReason {

    Timeout,
    None,

}

pub type Listener = Box<dyn Fn(&mut WrappedEvent) -> bool + Send + Sync>;

pub(crate) struct WrappedEventModifierBuilder(u8);

impl WrappedEventModifierBuilder {

    #[inline]
    pub(crate) const fn new() -> Self {
        Self(WrappedEvent::DEFAULT)
    }

    #[inline]
    pub(crate) const fn cancellable(mut self) -> Self {
        self.0 |= WrappedEvent::CANCELLABLE;
        self
    }

    #[inline]
    pub(crate) const fn interrupt_on_cancel(mut self) -> Self {
        self.0 |= WrappedEvent::CANCELLABLE;
        self
    }

    #[inline]
    pub(crate) const fn build(self) -> u8 {
        self.0
    }

}

#[derive(Clone)]
pub struct WrappedEvent {

    state: u8,
    pub event: Event,

}

impl WrappedEvent {

    pub(crate) const CANCELLABLE: u8 = 1 << 0;
    pub(crate) const CANCELLED: u8 = 1 << 1;
    pub(crate) const INTERRUPT_ON_CANCEL: u8 = 1 << 2;
    pub(crate) const DEFAULT: u8 = 0;

    pub(crate) fn new(modifiers: u8, event: Event) -> Self {
        Self {
            state: modifiers,
            event,
        }
    }

    #[inline]
    pub fn is_cancelled(&self) -> bool {
        (self.state & Self::CANCELLED) != 0
    }

    #[inline]
    pub fn is_cancellable(&self) -> bool {
        (self.state & Self::CANCELLABLE) != 0
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        if !self.is_cancellable() {
            panic!("Tried to cancel an event which is not cancellable!");
        }
        let current = self.is_cancelled();
        if current == cancelled {
            return;
        }
        if current {
            self.state &= !Self::CANCELLED;
        } else {
            self.state |= Self::CANCELLED;
        }
    }

    #[inline]
    pub fn cancel(&mut self) {
        self.set_cancelled(true);
    }

}

pub(crate) struct EventManager {

    listener_id: AtomicU64,
    listeners: Arc<RwLock<Vec<Listener>>>,

}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            listener_id: Default::default(),
            listeners: Arc::new(Default::default()),
        }
    }
}

impl EventManager {

    pub fn call_event(&self, mut event: WrappedEvent) {
        let listeners = self.listeners.clone();
        let listeners = &*listeners.read();
        for listener in listeners {
            listener(&mut event);
        }
    }

    /// Returns an id which can be used to remove the listener later on or to replace it.
    pub fn register_listener(&mut self, listener: Listener) -> u64 {
        let id = self.listener_id.load(Ordering::Acquire);
        self.listener_id.store(id + 1, Ordering::Release);
        self.register_listener_with_id(listener, id);
        id
    }

    /// Returns an id which can be used to remove the listener later on or to replace it.
    pub fn register_listener_with_id(&self, listener: Listener, id: u64) {

    }

}