use clap::Parser;
use std::collections::HashMap;
use std::process;
use strsim::levenshtein;
use colored::*;

/// A simple cli dictionary app, simply ask `mydict <insert_word>` and it finds it.
/// In the case it doesn't it give similar words.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Word to search
    #[arg(index = 1)]
    word: String,

    /// Show/remove similar words from results
    #[arg(short, long, default_value_t = String::from("true"))]
    show_suggestion: String,
}

fn load_dictionary() -> HashMap<String, Vec<String>> {
    let data = include_str!("./english-dictionary.json");
    let json: HashMap<String, Vec<String>> = serde_json::from_str(&data).unwrap();
    json
}

fn find_closest_match(word: &str, dictionary: HashMap<String, Vec<String>>) -> Option<String> {
    dictionary
        .keys()
        .min_by_key(|key| levenshtein(word, key))
        .cloned()
}

fn main() {
    let args: Args = Args::parse();
    let dictionary: HashMap<String, Vec<String>> = load_dictionary();
    let mut similar_word: Option<std::string::String> = None;
    let mut show_suggestion = true;

    match args.show_suggestion.as_str() {
        "true" => similar_word = find_closest_match(&args.word, dictionary.clone()),
        "false" => show_suggestion = false,
        _ => {
            eprintln!("Invalid value for show_similar flag");
            process::exit(1)
        }
    }

    println!("Meaning of {}:", args.word);

    if dictionary.contains_key(&args.word) {
        for (idx, definitions) in dictionary.get(&args.word).unwrap().iter().enumerate() {
            println!("{}. {}", idx + 1, definitions);
        }
    } else {
        if show_suggestion {
            if let Some(suggestion) = similar_word {
                println!("Did you mean, {}?", suggestion);
            } else {
                println!("No Similar words.")
            }
        }
        let error_message = format!("Word {} is not in dictionary.",  args.word).red();
        eprintln!("{}", error_message);
        process::exit(1)
    }
}
