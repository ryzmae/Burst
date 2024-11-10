use anyhow::Result;

pub mod constants;
pub mod error;
pub mod homedir;
pub mod logger;
pub mod memory;
pub mod port;
pub mod prelude;
pub mod render;
pub mod server;
pub mod time;

// Currently the main.rs file is the Test file for each module.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run();
    Ok(())
}
