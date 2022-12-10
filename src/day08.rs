use crate::{AppArgs,Solution};
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{Result,Context};

pub fn solve(args: &AppArgs) -> Result<Solution> {


  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut heights = Vec::new();
  for line in r.lines() {
    let line = line?;
    let mut row = Vec::new();
    for c in line.chars() {
      row.push(c);
    }
    heights.push(row);
  }

  let m = heights.len();
  let n = heights[0].len();

  let short = (b'0'-1) as char;
  let tall = (b'9'+1) as char;

  let mut views = Vec::new();
  views.resize_with(m, || vec![tall ;n]);

  for i in 0..m {
    let mut vis_level = short;
    for j in 0..n {
      if vis_level < views[i][j] {
        views[i][j] = vis_level;
      }
      if heights[i][j] > vis_level {
        vis_level = heights[i][j];
      }
    }

    let mut vis_level = short;
    for j in (0..n).rev() {
      if vis_level < views[i][j] {
        views[i][j] = vis_level;
      }
      if heights[i][j] > vis_level {
        vis_level = heights[i][j];
      }
    }
  }

  for j in 0..n {
    let mut vis_level = short;
    for i in 0..m {
      if vis_level < views[i][j] {
        views[i][j] = vis_level;
      }
      if heights[i][j] > vis_level {
        vis_level = heights[i][j];
      }
    }


    vis_level = short;
    for i in (0..m).rev() {
      if vis_level < views[i][j] {
        views[i][j] = vis_level;
      }
      if heights[i][j] > vis_level {
        vis_level = heights[i][j];
      }
    }
  }

  let mut star1 = 0;
  for i in 0..m {
    for j in 0..n {
      if heights[i][j] > views[i][j] {
        star1 += 1;
      }
    }
  }

  let mut max_tree_score = 0;
  for i in 0_i64..(m as i64) {
    for j in 0_i64..(n as i64) {
      let h = heights[i as usize][j as usize];

      let mut i_cursor = i-1;
      let mut d_left = 0;
      while i_cursor >= 0 {
        if (heights[i_cursor as usize][j as usize] >= h) || i_cursor == 0 {
          d_left = i - i_cursor;
          break;
        }
        i_cursor -= 1;
      }

      let mut i_cursor = i+1;
      let mut d_right = 0;
      while i_cursor < m as i64 {
        if heights[i_cursor as usize][j as usize] >= h || i_cursor == ((m-1)as i64){
          d_right = i_cursor - i;
          break;
        }
        i_cursor += 1;
      }


      let mut j_cursor = j-1;
      let mut d_top = 0;
      while j_cursor >= 0 {
        if heights[i as usize][j_cursor as usize] >= h || j_cursor == 0{
          d_top = j - j_cursor;
          break;
        }
        j_cursor -= 1;
      }

      let mut j_cursor = j+1;
      let mut d_bot = 0;
      while j_cursor < n as i64 {
        if heights[i as usize][j_cursor as usize] >= h || j_cursor == ((n-1) as i64){
          d_bot = j_cursor - j;
          break;
        }
        j_cursor += 1;
      }

      let d = d_left*d_right*d_top*d_bot;
      if d > max_tree_score {
        max_tree_score = d;
      }
    }

  }
  let star2 = max_tree_score;


  Ok( Solution::from_i64( star1, star2 ) )  
}
