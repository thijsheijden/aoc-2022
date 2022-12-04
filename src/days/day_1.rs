use crate::helpers::input::read_file as read_file;

pub fn solve(test_input: bool) {
  if test_input {
    solve_1(&read_file("src/inputs/day_1/test_input.txt"));
    solve_2(&read_file("src/inputs/day_1/test_input.txt"));
  } else {
    solve_1(&read_file("src/inputs/day_1/input.txt"));
    solve_2(&read_file("src/inputs/day_1/input.txt"));
  }
}

fn solve_1(input: &str) {
  // Split on empty lines
  let calorie_items_per_elf: Vec<&str> = input.split("\n\n").collect();

  let mut calories_per_elf: Vec<i32> = Vec::new();
  for items in calorie_items_per_elf {
    // Split on newlines
    let calorie_list_strings: Vec<&str> = items.split("\n").collect();
    
    // Convert strings to i32, sum and push to vector
    calories_per_elf.push(calorie_list_strings.iter().map(|s| s.parse::<i32>().unwrap()).sum());
  }

  // Print the max
  println!("{}", calories_per_elf.iter().max().unwrap());
}

fn solve_2(input: &str) {
  // Split on empty lines
  let calorie_items_per_elf: Vec<&str> = input.split("\n\n").collect();

  let mut calories_per_elf: Vec<i32> = Vec::new();
  for items in calorie_items_per_elf {
    // Split on newlines
    let calorie_list_strings: Vec<&str> = items.split("\n").collect();
    
    // Convert strings to i32, sum and push to vector
    calories_per_elf.push(calorie_list_strings.iter().map(|s| s.parse::<i32>().unwrap()).sum());
  }

  // Grab the largest three
  calories_per_elf.sort_unstable_by(|a, b| b.cmp(a));
  let largest_three: i32 = calories_per_elf.iter().take(3).sum();

  println!("{}", largest_three)
}
