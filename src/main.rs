use std::env;

const INVALID: &str = "Invalid transcription rule. Use -h for help";
const NO_ARGS: &str = "No arguments given. Use -h for help";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", NO_ARGS);
        return;
    }
    let result = match (args[1].as_str(), args.len()) {
        ("-h", 2) => "Help; not implemented!".to_string(),
        ("-lb", 3) => "Latin => Braille; not implemented!".to_string(),
        ("-lc", 3) => "Latin => Cyrillic; not implemented!".to_string(),
        ("-lg", 3) => "Latin => Greek; not implemented!".to_string(),
        ("-lref", 3) => transcript::latin_to_elder_futhark(&args[2]),
        _ => INVALID.to_string(),
    };

    println!("{}", result);
}
