pub struct Truth;

impl Truth {
    pub fn is(reality: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, bool) {
        fn manifest(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            match find_void(grid) {
                None => grid.clone(),
                Some((i, j)) => {
                    for n in 1..=9 {
                        if is_invariant(grid, i, j, n) {
                            grid[i][j] = n;
                            let truth = manifest(grid);
                            if is_complete(&truth) {
                                return truth;
                            }
                            grid[i][j] = 0;
                        }
                    }
                    grid.clone()
                }
            }
        }

        fn is_invariant(grid: &Vec<Vec<i32>>, row: usize, col: usize, val: i32) -> bool {
            !grid[row].contains(&val) && 
            !(0..9).any(|i| grid[i][col] == val) &&
            !((row/3*3)..(row/3*3+3)).any(|r| 
                ((col/3*3)..(col/3*3+3)).any(|c| grid[r][c] == val))
        }

        fn is_complete(grid: &Vec<Vec<i32>>) -> bool {
            !grid.iter().any(|row| row.contains(&0))
        }

        fn find_void(grid: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
            for i in 0..9 {
                for j in 0..9 {
                    if grid[i][j] == 0 { return Some((i, j)); }
                }
            }
            None
        }

        let result = manifest(&mut reality.clone());
        (result.clone(), result != reality)
    }
}


