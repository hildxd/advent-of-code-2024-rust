use regex::Regex;
use std::{fs::read_to_string, io::Result, path::Path};

fn read_data() -> Result<String> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");

    let content = read_to_string(&file_path)?;
    Ok(content)
}

fn part1(content: &String) -> i32 {
    let mut result = vec![];
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(&content) {
        let num1 = cap[1].parse::<i32>().unwrap();
        let num2 = cap[2].parse::<i32>().unwrap();
        result.push((num1, num2));
    }

    result.iter().fold(0, |acc, (x, y)| acc + x * y)
}

fn parse_instructions(content: &String) -> Vec<(i32, i32)> {
    let mut instructions = vec![];
    // 匹配 do()、don't() 和 mul(num1,num2)
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)").unwrap();

    let mut is_disabled = false;
    for cap in re.captures_iter(content) {
        if cap.get(1).is_some() {
            is_disabled = false;
        } else if cap.get(2).is_some() {
            is_disabled = true;
        } else {
            if !is_disabled {
                let num1 = cap[3].parse::<i32>().unwrap();
                let num2 = cap[4].parse::<i32>().unwrap();
                instructions.push((num1, num2));
            }
        }
    }

    instructions
}

fn part2(content: &String) -> i32 {
    let instructions = parse_instructions(content);

    instructions.iter().fold(0, |acc, (x, y)| acc + x * y)
}

fn main() -> Result<()> {
    let data = read_data()?;
    let result1 = part1(&data);
    let result2 = part2(&data);
    println!("part1: {}", result1);
    println!("part2: {}", result2);
    Ok(())
}
