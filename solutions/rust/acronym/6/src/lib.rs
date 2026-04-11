pub fn find_index(phrase: &str) -> Vec<char> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut v = Vec::new();
    
    for i in 0..chars.len() {
        let curr = chars[i];
        
        // taking care about letters for our acronym
        if curr.is_alphabetic() {
            if i == 0 {
                v.push(curr);
            } else {
                let prev = chars[i-1];
                
                // Instead of just checking i-1, I need to  check if the 
                // PREVIOUS character was any kind of "non-letter" 
                // or if we have a CamelCase jump.
                let is_at_boundary = !prev.is_alphabetic() && prev != '\''; 
                let is_camel = prev.is_lowercase() && curr.is_uppercase();

                if is_at_boundary || is_camel {
                    v.push(curr);
                }
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
