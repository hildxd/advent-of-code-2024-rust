mod error;
use error::Day1Error;
use std::{path::Path, str::FromStr};

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
#[derive(Debug)]
struct Position(i64, i64);

#[derive(Debug)]
struct Positions(Vec<Position>);

impl FromStr for Position {
    type Err = Day1Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("   ").collect();
        if v.len() != 2 {
            return Err(Day1Error::ParseTextError(s.to_string()));
        } else {
            let result = Position(
                v[0].parse()
                    .map_err(|_| Day1Error::ParseTextError(s.to_string()))?,
                v[1].parse()
                    .map_err(|_| Day1Error::ParseTextError(s.to_string()))?,
            );
            return Ok(result);
        }
    }
}

impl Positions {
    async fn try_load() -> Result<Self> {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let data_path = Path::new(&project_root).join("data.txt");
        let f = File::open(data_path).await?;
        let reader = BufReader::new(f);
        let mut lines = reader.lines();
        let mut positions: Vec<Position> = Vec::new();

        while let Some(line) = lines.next_line().await? {
            positions.push(Position::from_str(&line)?);
        }

        Ok(Positions(positions))
    }

    fn sort_to_vec(&self) -> (Vec<i64>, Vec<i64>) {
        let mut left_rev = Vec::new();
        let mut right_rev = Vec::new();
        self.0.iter().for_each(|position| {
            left_rev.push(position.0);
            right_rev.push(position.1);
        });
        left_rev.sort();
        right_rev.sort();

        (left_rev, right_rev)
    }
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let positions = Positions::try_load().await?;
    println!("{:?}", total_distance(positions.sort_to_vec()));
    Ok(())
}
