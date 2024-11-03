pub mod kvstore;
pub mod store;
pub mod error;
pub mod prelude;
pub mod server;
pub mod homedir;
pub mod time;
pub mod constants;
pub mod logger;
pub mod memory;

use crate::server::server::run;

// Currently the main.rs file is the Test file for each module.
fn main () {
    let _logger = logger::Logger::new(logger::Level::Info);
    _logger.info("Starting the server");
    _logger.debug("Debugging the server");
    _logger.trace("Tracing the server");
    _logger.warn("Warning the server");
    run();
    std::process::exit(0); // Time Test 
}
