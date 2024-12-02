mod positions;
use std::collections::HashMap;

use anyhow::Result;
use positions::Positions;

fn total_distance(data: (Vec<i64>, Vec<i64>)) -> i64 {
    data.0.iter().enumerate().fold(0, |res, (i, v)| {
        let right_value = data.1[i];
        let distance = if &right_value > v {
            right_value - v
        } else {
            v - right_value
        };
        res + distance
    })
}

fn vec_data_repeat_count_to_map(data: Vec<i64>) -> HashMap<i64, usize> {
    let mut m: HashMap<i64, usize> = HashMap::new();
    for x in data {
        m.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }
    m
}

fn similarity_score(data: (Vec<i64>, Vec<i64>)) -> i64 {
    let right_repect_map = vec_data_repeat_count_to_map(data.1);
    data.0.iter().fold(0, |mut res, v| {
        let current_count = right_repect_map.get(v);
        if let Some(count) = current_count {
            let result = (*count as i64) * v;
            res += result;
        }
        res
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let positions = Positions::try_load().await?;
    // part one
    println!("{:?}", total_distance(positions.sort_to_vec()));
    // part two
    println!("{:?}", similarity_score(positions.sort_to_vec()));

    Ok(())
}
