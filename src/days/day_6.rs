use crate::helpers::input::read_file;

pub fn solve(test_input: bool) {
    if test_input {
        solve_1(&read_file("src/inputs/day_6/test_input.txt"));
        solve_2(&read_file("src/inputs/day_6/test_input.txt"));
    } else {
        solve_1(&read_file("src/inputs/day_6/input.txt"));
        solve_2(&read_file("src/inputs/day_6/input.txt"));
    }
}

fn solve_1(input: &str) {
  // Read in the first four characters
  let mut window: &str = "";
  let (mut i, mut j) = (0, 3); // Sliding window start and end index

  while j < input.len() {
    // Grab the four characters the window is on now
    let window = &input.to_string()[i..=j];

    // Check if str contains only unique chars
    match unique_chars(window) {
      None => break,
      Some(_) => (),
    }

    // Slide window
    (i, j) = (i+1, j+1);
  }

  println!("Day 6 puzzle 1 answer: {}", j+1)
}

fn solve_2(input: &str) {
  // Read in the first four characters
  let mut window: &str = "";
  let (mut i, mut j) = (0, 13); // Sliding window start and end index

  while j < input.len() {
    // Grab the 13 characters the window is on now
    let window = &input.to_string()[i..=j];

    // Check if str contains only unique chars
    match unique_chars(window) {
      None => break,
      Some(_) => (),
    }

    // Slide window
    (i, j) = (i+1, j+1);
  }

  println!("Day 6 puzzle 2 answer: {}", j+1)
}

fn unique_chars(s: &str) -> Option<(usize, usize, char)> {
  s.chars().enumerate().find_map(|(i, c)| {
      s.chars()
          .enumerate()
          .skip(i + 1)
          .find(|(_, other)| c == *other)
          .map(|(j, _)| (i, j, c))
  })
}
