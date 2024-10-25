use std::env;

use todo_rust::libs::process_input;
fn main() {
    let args : Vec<_> = env::args().collect();
    process_input(args);
}
