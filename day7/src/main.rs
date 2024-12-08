use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

// 定义运算符枚举
#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

// 解析输入行的结构体
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }

        let test_value = parts[0].trim().parse().map_err(|_| "Invalid test value")?;

        let numbers: Result<Vec<i64>, _> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse())
            .collect();

        Ok(Equation {
            test_value,
            numbers: numbers.map_err(|_| "Invalid number")?,
        })
    }
}

fn read_data() -> io::Result<Vec<String>> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&project_root).join("data.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().collect::<Result<Vec<_>, _>>()?)
}

// 生成所有可能的运算符组合
fn generate_operator_combinations(len: usize) -> Vec<Vec<Operator>> {
    let mut result = Vec::new();
    let total_combinations = 2_usize.pow(len as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(len);
        for j in 0..len {
            if (i >> j) & 1 == 1 {
                combination.push(Operator::Add);
            } else {
                combination.push(Operator::Multiply);
            }
        }
        result.push(combination);
    }
    result
}

// 计算给定运算符组合的结果
fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
        }
    }
    result
}

fn solve(equations: &[Equation]) -> i64 {
    let mut total = 0;

    for equation in equations {
        let operator_count = equation.numbers.len() - 1;
        let combinations = generate_operator_combinations(operator_count);

        // 检查是否有任何组合能得到测试值
        let has_solution = combinations
            .iter()
            .any(|ops| evaluate(&equation.numbers, ops) == equation.test_value);

        if has_solution {
            total += equation.test_value;
        }
    }

    total
}

fn main() -> io::Result<()> {
    let input = read_data()?;

    let equations: Vec<Equation> = input.iter().filter_map(|line| line.parse().ok()).collect();

    let result = solve(&equations);
    println!("Total calibration value: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ];

        let equations: Vec<Equation> = input.iter().filter_map(|line| line.parse().ok()).collect();

        assert_eq!(solve(&equations), 3749);
    }
}
