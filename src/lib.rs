use std::collections::HashMap;
use strsim::levenshtein;

pub fn load_dictionary() -> HashMap<String, Vec<String>> {
    let data = include_str!("./english-dictionary.json");
    let json: HashMap<String, Vec<String>> = serde_json::from_str(data).unwrap();
    json
}

pub fn find_closest_match(word: &str, dictionary: HashMap<String, Vec<String>>) -> Option<String> {
    dictionary
        .keys()
        .min_by_key(|key| levenshtein(word, key))
        .cloned()
}