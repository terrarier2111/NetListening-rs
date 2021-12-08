pub trait Application {

    fn register_listener(&mut self) -> u64;

    fn unregister_listener(&mut self, id: u64) -> bool;

    fn stop(&mut self);

    fn connections(&self);

}