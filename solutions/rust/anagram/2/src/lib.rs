use std::collections::HashSet;

fn sort_alphabets(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable(); 
    let sorted_string: String = chars.into_iter().collect();
    sorted_string
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut hashout = HashSet::new();
    let sorted_word = sort_alphabets(word);
    let lower_word = word.to_lowercase();

    for c in possible_anagrams.iter(){
        if word.len() == c.len(){
            if c.to_lowercase() != lower_word && sort_alphabets(c) == sorted_word{
                hashout.insert(*c);
            }
        }
    }
    hashout
}
