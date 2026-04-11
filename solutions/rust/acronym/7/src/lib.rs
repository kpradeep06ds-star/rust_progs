pub fn find_index(phrase: &str) -> String {
    let mut acronym = String::new();
    // We start "true" because we are waiting for the very first word
    let mut waiting_for_start_of_word = true;
    let mut prev_char: Option<char> = None;

    for c in phrase.chars() {
        // 1. Detect CamelCase (Lowercase followed by Uppercase)
        let is_camel = prev_char.map_or(false, |p| p.is_lowercase()) && c.is_uppercase();

        if c.is_alphabetic() {
            if waiting_for_start_of_word || is_camel {
                acronym.push(c.to_ascii_uppercase());
                // Once we find a letter, we are no longer at the "start" 
                // unless a camelCase transition happens
                waiting_for_start_of_word = false;
            }
        } else if c == ' ' || c == '-' || c == '_' {
            // These characters "reset" our search for the next word
            waiting_for_start_of_word = true;
        }
        
        prev_char = Some(c);
    }
    acronym
}

pub fn abbreviate(phrase: &str) -> String {
    let v:String = find_index(phrase);
    v
}
