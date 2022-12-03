use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{anyhow,Result,Context,Error};

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );


  let mut bp_contents:[u8;256] = [0;256];

  let mut backpacks:[ [u8;3]; 256] = [[0;3]; 256];

  let mut letter_score:[u8;256] = [0;256];
  for k in 0..26 {
    letter_score[ (b'a' + k) as usize ] = (k+1);
    letter_score[ (b'A' + k) as usize ] = (k+27);
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
      bp_contents[b as usize ] = 1;
    }
    // Determine which character in the second half showed up in the first half.
    for &b in &bytes[n..2*n] {
      if bp_contents[b as usize] == 1 {
        star1 += letter_score[b as usize] as i64;
        break;
      }
    }
    // Clear the list of first half characters.
    for &b in &bytes[0..n] {
      bp_contents[b as usize] = 0;
    }
 
    // Star 2 computation
    let bp_group_index = j%3;

    // Record the list of all objects in this backpack.
    for &b in bytes {
      backpacks[b as usize][bp_group_index] = 1;
    }

    // If we're at a multiple of 3, check which object appears in all three backpacks
    if (j%3) == 2 {
      for base_char in [b'a', b'A'] {
        for c in base_char..(base_char + 26) {
          let c:usize = c.into();
          if backpacks[c].iter().all(|s| *s == 1) {
            star2 += letter_score[c] as i64;            
          }
          backpacks[c].fill(0);
        }
      }
    }
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}
