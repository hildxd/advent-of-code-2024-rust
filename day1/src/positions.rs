use std::{path::Path, str::FromStr};

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PositionsError {
    #[error("parse text to postion error: {0}")]
    ParseTextError(String),

    #[error("parse text to i64 error: {0}")]
    ParseIntError(String),
}

#[derive(Debug)]
pub struct Position(i64, i64);

#[derive(Debug)]
pub struct Positions(Vec<Position>);

impl FromStr for Position {
    type Err = PositionsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split_whitespace();
        let left = v
            .next()
            .ok_or_else(|| PositionsError::ParseIntError(s.to_string()))?;
        let right = v
            .next()
            .ok_or_else(|| PositionsError::ParseIntError(s.to_string()))?;
        let result = Position(
            left.parse::<i64>()
                .map_err(|_| PositionsError::ParseTextError(s.to_string()))?,
            right
                .parse::<i64>()
                .map_err(|_| PositionsError::ParseTextError(s.to_string()))?,
        );
        Ok(result)
    }
}

impl Positions {
    pub async fn try_load() -> Result<Self> {
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

    pub fn sort_to_vec(&self) -> (Vec<i64>, Vec<i64>) {
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
