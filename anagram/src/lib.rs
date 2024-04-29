use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort();

    possible_anagrams
        .iter()
        .filter(|&possible| {
            let mut sorted_possible_anagram = possible.to_lowercase().chars().collect::<Vec<char>>();
            sorted_possible_anagram.sort();
            sorted_word == sorted_possible_anagram && word.to_lowercase() != possible.to_lowercase()
        })
        .cloned()
        .collect()
}
