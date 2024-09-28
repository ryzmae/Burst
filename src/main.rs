pub mod kvstore;
pub mod store;
pub mod memory;
pub mod error;
pub mod prelude;
pub mod server;
pub mod log;
pub mod homedir;
pub mod time;

use crate::server::server::run;

fn main () {
    run("127.0.0.1", 7978);
}
