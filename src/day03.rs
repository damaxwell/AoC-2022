use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{anyhow,Result,Context,Error};



pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );


  let mut spots:[u8;256] = [0;256];
  let mut score:[u8;256] = [0;256];
  let mut badges:[[u8;256];3] = [[0;256];3];

  for k in 0..26 {
    score[ (b'a' + k) as usize ] = (k+1);
    score[ (b'A' + k) as usize ] = (k+27);
  }

  let mut star1:i64 = 0;
  let mut star2:i64 = 0;

  for (j,line) in r.lines().enumerate() {
    let line = line?;
    let bytes = line.as_bytes();

    // Star 1 computation

    let n = line.len()/2;
    // Record which characters appear in the first half.
    for &b in &bytes[0..n] {
      spots[b as usize ] = 1;
    }
    // Determine which character in the second half showed up in the first half.
    for &b in &bytes[n..2*n] {
      if spots[b as usize] == 1 {
        star1 += score[b as usize] as i64;
        break;
      }
    }
    // Clear the list of first half characters.
    for &b in &bytes[0..n] {
      spots[b as usize] = 1;
    }
 
    // Star 2 computation

    // Record the list of objects in this backpack.
    for b in bytes {
      badges[(j%3)][*b as usize] = 1;
    }

    // If we're at a multiple of 3, check which object appears in all three backpacks
    if (j%3) == 2 {
      for k in 0..26 {
        for c in [(b'a'+k) as usize , (b'A'+k) as usize] {
          if (badges[0][c] == 1) && (badges[1][c] == 1) && (badges[2][c] == 1) {
            star2 += score[c] as i64;
          }
          badges[0][c] = 0;
          badges[1][c] = 0;
          badges[2][c] = 0;
        }
      }
    }
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}
