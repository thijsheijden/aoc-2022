use crate::helpers::input::read_file as read_file;
use std::{collections::HashSet};

pub fn solve(test_input: bool) {
  if test_input {
    solve_1(&read_file("src/inputs/day_3/test_input.txt"));
    solve_2(&read_file("src/inputs/day_3/test_input.txt"));
  } else {
    solve_1(&read_file("src/inputs/day_3/input.txt"));
    solve_2(&read_file("src/inputs/day_3/input.txt"));
  }
}

fn solve_1(input: &str) {
  // Split into backpacks
  let backpacks: Vec<&str> = input.split("\n").collect();

  let mut sum_of_priorities:u32 = 0;
  for backpack in backpacks {
    // Split into equal backpack compartments
    let (compartment_1, compartment_2) = backpack.split_at(backpack.len()/2);

    // Convert both compartments into hashsets
    let compartment_1_hashset: HashSet<char> = HashSet::from_iter(compartment_1.chars());
    let compartment_2_hashset: HashSet<char> = HashSet::from_iter(compartment_2.chars());

    // Determine the item that is in both compartments
    let characters_present_in_both_compartments: HashSet<&char> = compartment_1_hashset.intersection(&compartment_2_hashset).collect();

    println!("{:?}", characters_present_in_both_compartments);

    // Calculate priority
    for c in characters_present_in_both_compartments {
      if c.is_ascii_lowercase() {
        sum_of_priorities += *c as u32 - 96;
      } else if c.is_ascii_uppercase() {
        sum_of_priorities += *c as u32 - 38;
      }
    }
  }

  println!("{}", sum_of_priorities)
}

fn solve_2(input: &str) {
  
}
