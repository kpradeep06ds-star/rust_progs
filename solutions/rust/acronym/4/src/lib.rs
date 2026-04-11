pub fn find_index(phrase: &str) -> Vec<char> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut v = Vec::new();
    
    for i in 0..chars.len() {
        let curr = chars[i];
        
        // 1. If it's not a letter, skip it entirely.
        // This stops spaces/hyphens from being added to your Vec.
        if !curr.is_alphabetic() { continue; }

        if i == 0 {
            v.push(curr);
        } else {
            let prev = chars[i-1];
            
            // 2. Word Boundary: Current is a letter, 
            // but the previous was a space, hyphen, or underscore.
            let is_after_delimiter = prev == ' ' || prev == '-' || prev == '_';
            
            // 3. CamelCase: Current is Upper, previous was lower.
            let is_camel = prev.is_lowercase() && curr.is_uppercase();

            if is_after_delimiter || is_camel {
                v.push(curr);
            }
        }
    }
    v
}

pub fn abbreviate(phrase: &str) -> String {
    let v:Vec<char> = find_index(phrase);
    println!("{:?}", v);
    let joined_string = v.into_iter().collect();
    joined_string
}
