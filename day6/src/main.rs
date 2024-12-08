use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_position(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Direction::Right => Some((pos.0, pos.1 + 1)),
            Direction::Down => Some((pos.0 + 1, pos.1)),
            Direction::Left => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
        }
    }
}

fn read_data() -> Result<String> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");
    let content = read_to_string(&file_path)?;
    Ok(content)
}

fn find_start(map: &Vec<Vec<char>>) -> ((usize, usize), Direction) {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return ((i, j), Direction::Up);
            }
        }
    }
    panic!("没有找到起始位置");
}

fn is_within_map(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    pos.0 < map.len() && pos.1 < map[0].len()
}

fn is_obstacle(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if !is_within_map(map, pos) {
        return true;
    }
    map[pos.0][pos.1] == '#'
}

fn solve_part1(map: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let (mut pos, mut dir) = find_start(map);
    visited.insert(pos);

    loop {
        let next_pos = match dir.get_next_position(pos) {
            Some(pos) => pos,
            None => break,
        };

        if !is_within_map(map, next_pos) {
            break;
        }

        if is_obstacle(map, next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let input = read_data()?;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part1_result = solve_part1(&map);
    println!("Part 1 - 守卫访问的不同位置数量: {}", part1_result);

    Ok(())
}
