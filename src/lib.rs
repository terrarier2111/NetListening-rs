pub mod application;
mod buffer;
pub mod client;
pub mod connection;
pub mod data_type;
mod decoder;
mod encoder;
pub mod event;
pub mod server;
mod timeout_handler;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
