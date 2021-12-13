use crate::connection::Connection;
use crate::event::Listener;

pub type Application = Arc<(dyn RawApplication + Sync + Send)>;

pub trait RawApplication: Sized {

    fn register_listener(&mut self, listener: Listener) -> u64;

    fn unregister_listener(&mut self, id: u64) -> bool;

    fn stop(&mut self);

    fn connections(&self);

    fn connection_by_id(&self, id: usize) -> Connection;

    fn config(); // TODO: Add some way to get config values which were set in the builder(but return them immutably)

}
