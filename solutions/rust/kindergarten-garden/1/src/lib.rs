use std::collections::HashMap;

pub fn read_mat(diagram:&str) -> Vec<Vec<char>>{
    let v:Vec<Vec<char>> = diagram.lines().map(|line| {
        line.chars().collect()
    }).collect();

    v
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {


    // v contains the diagram: garden arrays (matrix)
    // plants is a mapping of shorthands to longerhands
    // vname is the dictionary of student name against their vegitables 
    // student_short is the just indexes of all the students
    // logic: read the garden
    // iterate through rows
    // iterate through cols -> determine the 1,2 column in each row -> get the student  name
    // if the name matches due to index match, insert the flowers/vegitables
    let v:Vec<Vec<char>> = read_mat(diagram);
    
    let mut plants:HashMap<char, &'static str> = HashMap::new();
    // look the type here , i was doing here wrong
    plants.insert('G', "grass");
    plants.insert('C', "clover");
    plants.insert('R', "radishes");
    plants.insert('V', "violet");

    let mut vname:HashMap<&str, Vec<&'static str>> = HashMap::new();

    let student_short:Vec<&str> = vec!["Alice", "Alice", "Bob", "Bob", "Charlie", "Charlie", "David", "David", "Eve", "Eve", 
    "Fred", "Fred", "Ginny", "Ginny", "Harriet", "Harriet", "Ileana", "Ileana", "Joseph", "Joseph", 
    "Kincaid", "Kincaid", "Lary", "Lary"];
    
    for (_idx, r) in v.iter().enumerate(){
        for (n, c) in r.iter().enumerate(){
            let current = student_short.get(n)  ;
            if let Some(name) = current{
                if let Some(plant_name) = plants.get(c){
                    vname.entry(*name).or_insert(Vec::new()).push(*plant_name);
                }
            }
            
       }
    }

    let result = vname.get(student).cloned().unwrap_or_default();
    println!("{:?}", result);
    result
}
