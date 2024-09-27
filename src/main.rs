pub mod kvstore;
pub mod store;
pub mod memory;
pub mod error;
pub mod prelude;
pub mod server;
pub mod log;

fn main () {
    log::Log::new().info("Starting server...");
}