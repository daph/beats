extern crate beats;

use beats::Beat;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");


fn print_usage() {
    print!(r#"{} {}
{}

Prints the current system time in .beat / Swatch Internet time:
https://en.wikipedia.org/wiki/Swatch_Internet_Time

Project homepage: {}

USAGE:
    beats [-h]
"#,
        NAME, VERSION, AUTHORS, REPOSITORY
);

}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let beat = Beat::now();
        println!("{}", beat);
    } else if args[1] == "-h" {
        print_usage();
        std::process::exit(0);
    } else {
        eprintln!("Invalid arguments");
        print_usage();
        std::process::exit(1);
    }
}


