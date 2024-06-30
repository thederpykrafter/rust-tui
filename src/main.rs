fn color(color: str, text: str) {
    println!("{}{}{}", color, text, "\x1b[0m")
}

fn main() {
    color("\x1b[94m", "Hello, world!");
}
