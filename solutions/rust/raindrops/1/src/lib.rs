
pub fn raindrops(n: u32) -> String {

    let num: u32 = (n.is_multiple_of(3) as u32)*1 + (n.is_multiple_of(5) as u32)*2 + {n.is_multiple_of(7) as u32}*4 ;

    let st:String = String::from( match num {
        1 => "Pling".to_string(),
        2 => "Plang".to_string(),
        3 => "PlingPlang".to_string(),
        4 => "Plong".to_string(),
        5 => "PlingPlong".to_string(),
        6 => "PlangPlong".to_string(),
        7 => "PlingPlangPlong".to_string(),
        _ =>  (n).to_string()

    });

   st

}
