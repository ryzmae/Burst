pub mod kvstore;
pub mod store;
pub mod memory;
pub mod error;
pub mod prelude;
pub mod server;
pub mod log;
pub mod homedir;
pub mod time;
pub mod constants;

use crate::server::server::run;

fn main () {
    run();
}
