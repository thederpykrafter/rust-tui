extern crate term_size;

pub fn fill_term(color: &str, text: &str) {
  if let Some((w, h)) = term_size::dimensions() {
    //println!("Width: {}\nHeight: {}", w, h);
    for y in 1..h {
        for x in 1..w {
            cprint(color, text);
        }
        print!("\n")
    }
  } else {
    cprintln("\x1b[91m", "Unable to get term size :(");
  }
}

pub fn cprintln(color: &str, text: &str) {
    println!("{}{}{}", color, text, "\x1b[0m");
}

pub fn cprint(color: &str, text: &str) {
    print!("{}{}{}", color, text, "\x1b[0m");
}

