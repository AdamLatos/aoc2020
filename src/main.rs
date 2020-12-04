use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), std::io::Error> {
    println!("Day 1");
    let path = "day_1_1";
    let buffered = BufReader::new(File::open(path)?);

    let lines: Vec<i64> = buffered
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    for i in 1..lines.len() {
        for j in i..lines.len() {
            if lines[i] + lines[j] == 2020 {
                println!("{}", lines[i] * lines[j]);
            }
        }
    }

    Ok(())
}
