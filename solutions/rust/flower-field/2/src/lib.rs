const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn count_valid(board: &[Vec<char>], row: usize, col: usize) -> usize {

    let rows = board.len() as isize;
    let cols = board[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    let mut count = 0;

    for (dr, dc) in DIRS {
        let nr = row + dr;
        let nc = col + dc;

        let valid_row = 0 <= nr && nr < rows;
        let valid_col = 0 <= nc && nc < cols;

        if valid_row && valid_col && board[nr as usize][nc as usize] == '*' {
            count += 1;
        }
    }

    count
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    
    if garden.is_empty() {
        return vec![];
    }

    let board: Vec<Vec<char>> = garden.iter()
        .map(|row| row.chars().collect())
        .collect();

    let rows = board.len();
    let cols = board[0].len();

    let mut out = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row_out = String::with_capacity(cols);

        for c in 0..cols {
            if board[r][c] == '*' {
                row_out.push('*');
            } else {
                let count = count_valid(&board, r, c);
                if count == 0 {
                    row_out.push(' ');
                } else {
                    row_out.push(char::from_digit(count as u32, 10).unwrap());
                }
            }
        }

        out.push(row_out);
    }

    out
}
