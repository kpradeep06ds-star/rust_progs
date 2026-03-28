// let vmatrix: Vec<Vec<char>> = garden.iter()
//     .map(|row| row.chars().collect())
//     .collect();

pub fn count_valid(garden:&[&str], row:usize, col:usize) -> i32{

    let mut vmatrix:Vec<Vec<char>> = Vec::new();
    let rows = garden.len() as i32;

    if rows == 0{
        return 0;
    }

    for row in garden{
        vmatrix.push(row.chars().collect());
    }

    let cols = vmatrix[0].len() as i32;
    
    let mut count_flower =  0;

    let tuples_direction:Vec<Vec<i32>>  = vec![
        vec![-1,-1],
        vec![-1,0], 
        vec![-1,1],
        vec![0,-1],
        vec![0,1],
        vec![1,-1],
        vec![1,0],
        vec![1,1]
        ];
 
        for i in tuples_direction{

            let r = i[0];
            let c = i[1];
            let  nrow = r + row as i32;
            let  ncol = c + col as i32;

            let validrows = nrow >= 0 && nrow < rows ; 
            let validcols = ncol >= 0 && ncol < cols ;
            
            if validrows && validcols{
                if vmatrix[nrow as usize][ncol as usize] == '*'{
                    count_flower += 1;
                }

            }

        }

    count_flower

}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    todo!(
        "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    );
}
