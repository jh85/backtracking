fn solve_n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut board = vec![0;n];
    backtrack(n, 0, &mut board, &mut solutions);
    solutions
}

fn backtrack(n: usize, row: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
    if row == n {
        solutions.push(board.clone());
    } else {
        for col in 0..n {
            if is_valid(board, row, col) {
                board[row] = col;
                backtrack(n, row+1, board, solutions);
            }
        }
    }
}

fn is_valid(board: &[usize], row: usize, col: usize) -> bool {
    for r in 0..row {
        let c = board[r];
        if c == col || (r as isize - row as isize).abs() == (c as isize - col as isize).abs() {
            return false;
        }
    }
    true
}

fn main() {
    for n in 1..20 {
        let n_solutions = solve_n_queens(n);
        println!("{} {}", n, n_solutions.len());
    }
}
