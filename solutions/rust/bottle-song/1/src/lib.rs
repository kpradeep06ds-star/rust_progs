pub fn num_to_words(num: i32) -> &'static str {
    
    let values = match num {
        0 => "No" ,
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10=> "Ten",
        _ => panic!("unsupported number")
    };

    values


}

pub fn singular_plural(n: i32) -> &'static str{

    if n == 1{
        return "bottle";
    } else {
        return "bottles";
    }
}

pub fn gen_verse(n1: i32, n2: i32) -> String {
    let w1 = num_to_words(n1);
    let w2 = num_to_words(n2).to_lowercase();
    let ws1 = singular_plural(n1);
    let ws2 = singular_plural(n2);

    format!(
        "{w1} green {ws1} hanging on the wall,\n\
         {w1} green {ws1} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {w2} green {ws2} hanging on the wall."
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verse = String::new();

    for n in (start_bottles - take_down + 1..=start_bottles).rev() {
        let current = n as i32;
        let next = current - 1;

        verse.push_str(&gen_verse(current, next));

        if n != start_bottles - take_down + 1 {
            verse.push_str("\n\n");
        }
    }

    verse
}