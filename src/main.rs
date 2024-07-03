fn cprintln(color: &str, text: &str) {
    println!("{}{}{}", color, text, "\x1b[0m");
}

fn cprint(color: &str, text: &str) {
    print!("{}{}{}", color, text, "\x1b[0m");
}

fn main() {
    cprint("\x1b[94m", "Hello World!");
    cprintln("\x1b[94m", "Hello World!")
}
