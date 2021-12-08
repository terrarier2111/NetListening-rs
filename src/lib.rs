pub mod client;
pub mod server;
pub mod event;
pub mod data_type;
pub mod application;
pub mod connection;
pub mod utils;
mod decoder;
mod encoder;
mod timeout_handler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



