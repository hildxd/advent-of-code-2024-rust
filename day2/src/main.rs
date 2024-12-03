use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_data() -> io::Result<Vec<Vec<u32>>> {
    let mut result = vec![];
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");

    let file = File::open(&file_path)?;

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let data: Vec<u32> = line
            .split_whitespace()
            .map(|str| str.parse::<u32>().expect("parse to u32 error"))
            .collect();
        result.push(data);
    }
    Ok(result)
}

fn is_sorted(vec: &Vec<u32>) -> (bool, bool) {
    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 0..vec.len() - 1 {
        if vec[i] < vec[i + 1] {
            is_descending = false;
        } else if vec[i] > vec[i + 1] {
            is_ascending = false;
        }

        if !is_ascending && !is_descending {
            break;
        }
    }
    (is_ascending, is_descending)
}

fn is_within_range(vec: &Vec<u32>) -> bool {
    // 遍历向量中的相邻元素
    for i in 0..vec.len() - 1 {
        let diff = (vec[i] as i32 - vec[i + 1] as i32).abs(); // 计算绝对差值
        if diff < 1 || diff > 3 {
            // 检查差值是否在范围内
            return false; // 如果不在范围内，返回 false
        }
    }
    true // 如果所有相邻元素都符合条件，返回 true
}

fn part_one(data: Vec<Vec<u32>>) -> usize {
    data.iter()
        .filter(|v| {
            let (is_asc, is_des) = is_sorted(*v);
            if is_asc || is_des {
                return is_within_range(*v);
            }
            return false;
        })
        .count()
}

fn can_be_sorted_by_removing_one_or_none(levels: &Vec<u32>) -> bool {
    fn is_valid_sequence(arr: &[u32]) -> bool {
        if arr.is_empty() {
            return true;
        }

        // 检查是否递增或递减，同时检查相邻数字的差值
        let ascending = arr.windows(2).all(|w| {
            let diff = (w[1] as i32 - w[0] as i32).abs();
            w[0] <= w[1] && diff >= 1 && diff <= 3
        });

        let descending = arr.windows(2).all(|w| {
            let diff = (w[1] as i32 - w[0] as i32).abs();
            w[0] >= w[1] && diff >= 1 && diff <= 3
        });

        ascending || descending
    }

    // 检查原序列是否已经有效
    if is_valid_sequence(levels) {
        return true;
    }

    // 尝试移除一个元素
    for i in 0..levels.len() {
        let mut new_levels = Vec::with_capacity(levels.len() - 1);
        new_levels.extend_from_slice(&levels[..i]);
        new_levels.extend_from_slice(&levels[i + 1..]);

        if is_valid_sequence(&new_levels) {
            return true;
        }
    }

    false
}

fn part_two(data: Vec<Vec<u32>>) -> usize {
    data.iter()
        .filter(|v| can_be_sorted_by_removing_one_or_none(v))
        .count()
}

fn main() -> io::Result<()> {
    let data = read_data()?;
    let result = part_one(data.clone());
    println!("part_one : {}", result);
    let result = part_two(data);
    println!("part_two : {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let data = vec![
            vec![7, 6, 4, 2, 1], // true - already sorted (descending)
            vec![1, 2, 7, 8, 9], // false
            vec![9, 7, 6, 2, 1], // false
            vec![1, 3, 2, 4, 5], // true - remove 3
            vec![8, 6, 4, 4, 1], // true - remove one 4
            vec![1, 3, 6, 7, 9], // true - already sorted (ascending)
        ];

        let result = part_two(data);
        assert_eq!(result, 4); // 应该有4个有效序列
    }

    #[test]
    fn test_individual_sequences() {
        assert!(can_be_sorted_by_removing_one_or_none(&vec![7, 6, 4, 2, 1])); // true
        assert!(!can_be_sorted_by_removing_one_or_none(&vec![1, 2, 7, 8, 9])); // false
        assert!(!can_be_sorted_by_removing_one_or_none(&vec![9, 7, 6, 2, 1])); // false
        assert!(can_be_sorted_by_removing_one_or_none(&vec![1, 3, 2, 4, 5])); // true
        assert!(can_be_sorted_by_removing_one_or_none(&vec![8, 6, 4, 4, 1])); // true
        assert!(can_be_sorted_by_removing_one_or_none(&vec![1, 3, 6, 7, 9])); // true
    }
}
