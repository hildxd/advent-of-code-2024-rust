use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, io::Result, path::Path};

fn main() -> Result<()> {
    let input = read_data().unwrap();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Part 1 结果是: {}", result1);
    println!("Part 2 结果是: {}", result2);
    Ok(())
}

fn read_data() -> Result<String> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");

    let content = read_to_string(&file_path)?;
    Ok(content)
}

fn part1(input: &str) -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    // 解析输入
    let parts: Vec<&str> = input.split("\n\n").collect();

    // 解析规则
    for line in parts[0].lines() {
        let nums: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules
            .entry(nums[0])
            .or_insert_with(HashSet::new)
            .insert(nums[1]);
    }

    // 解析序列
    for line in parts[1].lines() {
        let seq: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        sequences.push(seq);
    }

    // 检查每个序列并计算结果
    sequences
        .iter()
        .filter(|seq| is_valid_sequence(seq, &rules))
        .map(|seq| seq[seq.len() / 2])
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    // 解析输入
    let parts: Vec<&str> = input.split("\n\n").collect();

    // 解析规则
    for line in parts[0].lines() {
        let nums: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules
            .entry(nums[0])
            .or_insert_with(HashSet::new)
            .insert(nums[1]);
    }

    // 解析序列
    for line in parts[1].lines() {
        let seq: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        sequences.push(seq);
    }

    // 找出错误排序的序列并重新排序
    sequences
        .iter()
        .filter(|seq| !is_valid_sequence(seq, &rules))
        .map(|seq| {
            let mut seq = seq.clone();
            sort_sequence(&mut seq, &rules);
            seq[seq.len() / 2]
        })
        .sum()
}

fn is_valid_sequence(sequence: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, &num1) in sequence.iter().enumerate() {
        for &num2 in &sequence[i + 1..] {
            // 检查 num1 -> num2 的规则
            if let Some(deps) = rules.get(&num2) {
                if deps.contains(&num1) {
                    return false; // 违反规则
                }
            }
            // 检查 num2 -> num1 的规则
            if let Some(deps) = rules.get(&num1) {
                if deps.contains(&num2) {
                    continue; // 符合规则
                }
            }
        }
    }
    true
}

fn sort_sequence(sequence: &mut Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) {
    // 使用冒泡排序，根据规则比较相邻元素
    let n = sequence.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if should_swap(&sequence[j], &sequence[j + 1], rules) {
                sequence.swap(j, j + 1);
            }
        }
    }
}

fn should_swap(a: &i32, b: &i32, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    // 检查是否需要交换两个元素
    if let Some(deps) = rules.get(a) {
        if deps.contains(b) {
            return true;
        }
    }
    if let Some(deps) = rules.get(b) {
        if deps.contains(a) {
            return false;
        }
    }
    false
}
