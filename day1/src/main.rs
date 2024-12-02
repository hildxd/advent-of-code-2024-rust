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

#[tokio::main]
async fn main() -> Result<()> {
    let positions = Positions::try_load().await?;
    println!("{:?}", total_distance(positions.sort_to_vec()));
    Ok(())
}
