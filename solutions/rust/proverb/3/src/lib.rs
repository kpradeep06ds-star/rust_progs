pub fn gen_verse<'a >(s1:&str, s2:&str) -> String{
    format!("For want of a {s1} the {s2} was lost.")
}


pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut verse = String::new();

    for i in 0..list.len() - 1 {
        verse.push_str(&gen_verse(list[i], list[i + 1]));
        verse.push('\n');
    }

    verse.push_str(&format!(
        "And all for the want of a {}.",
        list[0]
    ));

    verse
}