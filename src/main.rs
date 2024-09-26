pub mod kvstore;
pub mod store;
pub mod memory;
pub mod error;
pub mod prelude;

// Testing only
fn main() {
    let test1 = store::Store::new();

    let _ = test1.set_data("Test".to_string());

    println!("{:?}", test1.get_data());
}