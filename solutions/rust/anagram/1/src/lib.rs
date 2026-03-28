use std::collections::HashSet;

fn sort_alphabets(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort(); 
    let sorted_string: String = chars.into_iter().collect();
    sorted_string
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    hashout = HashSet::new();

    for c in possible_anagrams.iter(){
        if word.len() == c.len(){
            if sort_alphabets(word) == sort_alphabets(word){
                hashout.insert(c);
            }
        }
    }
    hashout
}
