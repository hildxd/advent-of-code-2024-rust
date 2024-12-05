use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn read_data() -> Result<Vec<Vec<char>>> {
    let mut result = vec![];
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");

    // let content = read_to_string(&file_path)?;
    let file = File::open(&file_path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?.chars().collect());
    }
    Ok(result)
}

fn check_diagonal(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_dir: i32,
    col_dir: i32,
) -> bool {
    let target = "XMAS";
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // 检查边界
    if row_dir > 0 && (row as i32 + 3) >= rows {
        return false;
    }
    if row_dir < 0 && (row as i32 - 3) < 0 {
        return false;
    }
    if col_dir > 0 && (col as i32 + 3) >= cols {
        return false;
    }
    if col_dir < 0 && (col as i32 - 3) < 0 {
        return false;
    }

    // 检查是否匹配
    for i in 0..4 {
        let new_row = (row as i32 + i * row_dir) as usize;
        let new_col = (col as i32 + i * col_dir) as usize;
        if grid[new_row][new_col] != target.chars().nth(i as usize).unwrap() {
            return false;
        }
    }
    true
}

fn check_horizontal(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let cols = grid[0].len();
    if col + 3 >= cols {
        return false;
    }

    let target = "XMAS";
    let mut is_desc = true;
    let mut is_asc = true;
    for i in 0..4 {
        if grid[row][col + i] != target.chars().nth(i as usize).unwrap() {
            is_desc = false;
        }
    }
    for i in 0..4 {
        if grid[row][col + 3 - i] != target.chars().nth(i as usize).unwrap() {
            is_asc = false;
        }
    }
    is_desc || is_asc
}

fn check_vertical(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let target = "XMAS";
    let rows = grid.len();

    // 边界检查
    if row + 3 >= rows {
        return false;
    }

    let mut is_desc = true;
    let mut is_asc = true;

    // 正序检查 (从上到下)
    for i in 0..4 {
        if grid[row + i][col] != target.chars().nth(i).unwrap() {
            is_desc = false;
        }
    }

    // 倒序检查 (从下到上)
    for i in 0..4 {
        if grid[row + 3 - i][col] != target.chars().nth(i).unwrap() {
            is_asc = false;
        }
    }

    is_desc || is_asc
}

fn check_pattern(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // 边界检查
    if row + 2 >= rows || col + 2 >= cols {
        return false;
    }

    // 检查中心必须是 'A'
    if grid[row + 1][col + 1] != 'A' {
        return false;
    }

    // 获取四个角的字符
    let top_left = grid[row][col];
    let top_right = grid[row][col + 2];
    let bottom_left = grid[row + 2][col];
    let bottom_right = grid[row + 2][col + 2];

    // 检查左上到右下是否形成 MAS 或 SAM
    let diagonal1_is_mas = top_left == 'M' && bottom_right == 'S';
    let diagonal1_is_sam = top_left == 'S' && bottom_right == 'M';

    // 检查右上到左下是否形成 MAS 或 SAM
    let diagonal2_is_mas = top_right == 'M' && bottom_left == 'S';
    let diagonal2_is_sam = top_right == 'S' && bottom_left == 'M';

    // 两个对角线都必须是有效的 MAS 或 SAM
    (diagonal1_is_mas || diagonal1_is_sam) && (diagonal2_is_mas || diagonal2_is_sam)
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // 检查右下对角线
            if check_diagonal(&grid, i, j, 1, 1) {
                count += 1;
            }
            // 检查左下对角线
            if check_diagonal(&grid, i, j, 1, -1) {
                count += 1;
            }
            // 检查右上对角线
            if check_diagonal(&grid, i, j, -1, 1) {
                count += 1;
            }
            // 检查左上对角线
            if check_diagonal(&grid, i, j, -1, -1) {
                count += 1;
            }
            // 检查水平
            if check_horizontal(&grid, i, j) {
                count += 1;
            }
            // 检查垂直
            if check_vertical(&grid, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if check_pattern(&grid, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<()> {
    let grid = read_data()?;
    let count1 = part1(&grid);
    let count2 = part2(&grid);
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];
        let count = part1(&grid);
        assert_eq!(count, 18);
    }

    #[test]
    fn test_part2() {
        let grid = vec![
            ".M.S......".chars().collect(),
            "..A..MSMS.".chars().collect(),
            ".M.S.MAA..".chars().collect(),
            "..A.ASMSM.".chars().collect(),
            ".M.S.M....".chars().collect(),
            "..........".chars().collect(),
            "S.S.S.S.S.".chars().collect(),
            ".A.A.A.A..".chars().collect(),
            "M.M.M.M.M.".chars().collect(),
            "..........".chars().collect(),
        ];
        let count = part2(&grid);
        assert_eq!(count, 9);
    }
}
