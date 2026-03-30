pub fn ask_qn(s:&str) -> bool{
    s.ends_with('?')
}

pub fn all_caps(s:&str) -> bool{
    for i in s.chars(){
        if !i.is_uppercase(){
            return false;
        } 
    }
    true
}

pub fn yell_question(s:&str) -> bool{
    if ask_qn(s) && all_caps(s) {
        return true;
    }
    false
}

pub fn all_empty(s:&str) -> bool{
    for i in s.chars(){
        if i != ' '{
            return false;
        }
    }
    true
}

pub fn reply(message: &str) -> &str {
    let m = message;

    if ask_qn(m){
        "Sure"
    } else if all_caps(m){
        "Whoa, chill out!"
    } else if yell_question(m) {
        "Calm down, I know what I'm doing!"
    } else if all_empty(m) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }

}
