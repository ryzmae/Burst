mod constants;
mod error;
mod homedir;
mod logger;
mod memory;
mod port;
mod prelude;
mod render;
mod server;
mod time;

// Currently the main.rs file is the Test file for each module.
fn main() -> prelude::Result<()> {
    server::run();
    Ok(())
}
