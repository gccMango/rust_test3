extern crate communicator;

fn main() {
    communicator::client::connect();
    println!("hello mod!");
}