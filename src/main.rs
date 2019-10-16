/*
I heard that Rust is Mozilla's apology for JavaScript.

Interesting how the compiler is a static analyzer and formatter rolled into one.

Might as well try it out.

by: Nick (Nick@Gully.org)

lumps
input: number of objects N

program will generate N random pairs of relations between the objects

then try and group the N objects by those relationships.
*/

use std::env;
use rand;

fn make_relationships(limit: i32) -> (i32, i32) {
    return ((rand::random::<f32>() * limit as f32) as i32, 
        (rand::random::<f32>() * limit as f32 ) as i32);
}

// main: take number of elements
fn main() {
  let args: Vec<String> = env::args().collect();
  let num_objects : i32 = args[1].parse::<i32>().unwrap();
  println!("Generating {} elements", num_objects);

  let mut relationships: Vec<(i32, i32)> = Vec::new();
  for _i in 0..num_objects {
      let tmp: (i32, i32) = make_relationships(num_objects);
      relationships.push(tmp);
      println!("seen {} and {}", tmp.0, tmp.1);
  }
}

// todo:
// sort the pairs
// organize these related items into aggregate lumps