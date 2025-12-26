mod core;
mod helpers;
mod models;

use crate::core::actions::{record_mouse, replay_mouse};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_number: &str;

    // :: Check if the correct number of arguments is provided ::
    if args.len() < 3 {
        println!("Usage: cargo run -- [record|replay] <recording_name>");
        return;
    }

    // :: Get the file number if the replay argument is provided ::
    if args[1] == "replay" && args.len() >= 4 {
        file_number = &args[3];
    } else {
        file_number = "";
    }

    match args[1].as_str() {
        "record" => record_mouse(args[2].as_str()),
        "replay" => replay_mouse(args[2].as_str(), file_number),
        _ => println!("Argument not recognized. Usage: cargo run -- 'record' or 'replay'."),
    }
}
