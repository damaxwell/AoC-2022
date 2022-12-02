use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{Result,Context};

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut calorie_counts = Vec::new();
  calorie_counts.push(0);
  for line in r.lines() {
    let line = line?;
    if line.is_empty() {
      calorie_counts.push(0);
    } else {
      *calorie_counts.last_mut().unwrap() += line.parse::<i64>()?;
    }
  }

  calorie_counts.sort();
  let star1 = calorie_counts.pop().unwrap();
  let star2 = star1 + calorie_counts.pop().unwrap() + calorie_counts.pop().unwrap();

  Ok( Solution::from_i64( star1, star2 ) )  
}