use crate::{AppArgs,Solution};
use std::fs::{File};
use std::io::{BufReader,Read};
use anyhow::{Result,Context};

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let mut star1 = 0;
  let mut star2 = 0;

  let input_path = args.input_file_path();
  let mut r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut long_line = Vec::new();
  r.read_to_end(&mut long_line)?;

  'outer: for j in 0..long_line.len()-4 {
    for k in j..j+4 {
      for l in k+1..j+4 {
        if long_line[k] == long_line[l] {
          continue 'outer;
        }
      }
    }
    star1 = (j+4) as i64;
    break;
  }

  'outer: for j in 0..long_line.len()-14 {
    for k in j..j+14 {
      for l in k+1..j+14 {
        if long_line[k] == long_line[l] {
          continue 'outer;
        }
      }
    }
    star2 = (j+14) as i64;
    break;
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}
