extern crate term_size;
use std::{thread::sleep, time};
mod tfuncs;

fn intro() {
    print!("\x1b[?1049h");
    tfuncs::fill_term("\x1b[94m", "T");
    sleep(time::Duration::from_millis(1000));
    tfuncs::fill_term("\x1b[92m", "D");
    sleep(time::Duration::from_millis(1000));
    tfuncs::fill_term("\x1b[95m", "K");
    sleep(time::Duration::from_millis(1000));
    print!("\x1b[2J");
}

fn landing_page() {
    tfuncs::fill_term("\x1b[49m", "#")
}

fn main() {
    intro();
    landing_page()
}
