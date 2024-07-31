use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let target_num_chars = count_chars(word.chars().collect());
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for &item in possible_anagrams {
        // if all chars in item are in word and item != word then insert into anagrams hashset
        if item.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut item_vec: Vec<char> = item.chars().collect();
        item_vec.sort_by(|a, b| a.cmp(b));

        let num_chars = count_chars(item_vec);
        if num_chars == target_num_chars {
            anagrams.insert(item);
        }
    }

    anagrams
}

/// counts the number of letters in each word
pub fn count_chars(char_vec: Vec<char>) -> HashMap<char, u8> {
    let mut num_chars: HashMap<char, u8> = HashMap::new();

    for char in char_vec.iter().copied() {
        let char = char.to_lowercase().next().unwrap_or(char);
        println!("{:?}", char);
        num_chars
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    num_chars
}
