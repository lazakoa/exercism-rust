use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize_word(word);
    let mut results = HashSet::new();
    for possible_anagram in possible_anagrams.iter().filter(|s| s.to_lowercase() != word.to_lowercase()) {
        let normalized_possible_anagram = normalize_word(possible_anagram);
        if normalized_word.eq(&normalized_possible_anagram) {
            results.insert(*possible_anagram);
        }
    }
    results
}

fn normalize_word(word: & str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in word.to_lowercase().chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
