pub fn find_index(phrase: &str) -> Vec<char> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut v = Vec::new();
    
    for i in 0..chars.len() {
        let curr = chars[i];
        
        // Safety: If it's not a letter, we definitely don't want it in our acronym
        if !curr.is_alphabetic() { continue; }

        if i == 0 {
            v.push(curr);
        } else {
            let prev = chars[i-1];
            // this is special character as word split
            let after_boundary = prev == ' ' || prev == '-' || prev == '_';
            // this is the case when lowercase followed by uppercase
            // this will also cover two spellings which are all caps
            let is_camel_case = prev.is_lowercase() && curr.is_uppercase();

            if after_boundary || is_camel_case {
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
