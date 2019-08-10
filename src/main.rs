use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "slide", about = "Generates sliding windows over words.")]
struct SlideOptions {
    /// Window size
    #[structopt(short = "n", long = "size")]
    size: usize,
}

fn main() {
    let options = SlideOptions::from_args();

    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer);

    for window in buffer
        .split(char::is_whitespace)
        .collect::<Vec<_>>()
        .as_slice()
        .windows(options.size)
    {
        println!("{}", window.join(" "));
    }
}
