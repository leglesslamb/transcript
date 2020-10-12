use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match (args[1].as_str(), args.len()) {
        ("-h", 2) => "Help; not implemented!",
        ("-lb", 3) => "Latin => Braille; not implemented!",
        ("-lc", 3) => "Latin => Cyrillic; not implemented!",
        ("-lg", 3) => "Latin => Greek; not implemented!",
        ("-lr", 3) => "Latin => Runic; not implemented!",
        _ => "Invalid transcription rule. Use -h for help",
    };

    println!("{}", result);
}
