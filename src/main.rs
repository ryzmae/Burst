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
pub mod render;
pub mod port;

// Currently the main.rs file is the Test file for each module.
fn main () {
    render::render_name("Burst");
}
