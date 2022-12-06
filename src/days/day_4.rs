use crate::helpers::input::read_file as read_file;

pub fn solve(test_input: bool) {
  if test_input {
    solve_1(&read_file("src/inputs/day_4/test_input.txt"));
    solve_2(&read_file("src/inputs/day_4/test_input.txt"));
  } else {
    solve_1(&read_file("src/inputs/day_4/input.txt"));
    solve_2(&read_file("src/inputs/day_4/input.txt"));
  }
}

fn solve_1(input: &str) {
  let section_start_end_per_pair = get_elf_ranges(input);

  // Check if one of the ranges fully contains the other
  let mut number_of_encompassed_sections = 0;
  for elf_pair in section_start_end_per_pair {
    // Check if elf 1 start and end sections encompas those of elf 2
    if elf_pair[0][0] <= elf_pair[1][0] && elf_pair[0][1] >= elf_pair[1][1] {
      // Elf 1 fully encompasses sections of elf 2
      number_of_encompassed_sections += 1;
    } else if elf_pair[1][0] <= elf_pair[0][0] && elf_pair[1][1] >= elf_pair[0][1] {
      // Elf 2 fully encompasses sections of elf 1
      number_of_encompassed_sections += 1;
    }
  }

  println!("{}", number_of_encompassed_sections)
}

fn solve_2(input: &str) {
  let section_start_end_per_pair = get_elf_ranges(input);

  // Check if one of the ranges partially contains the other
  let mut number_of_overlapping_sections = 0;
  for elf_pair in section_start_end_per_pair {
    // Check if the end of elf 1 range is larger than or equal to the start of the range of elf 2
    // and the end of the range of elf 2 is less than or equal to the start of the range of elf 1
    if elf_pair[0][1] >= elf_pair[1][0] && elf_pair[0][0] <= elf_pair[1][1] {
      number_of_overlapping_sections += 1;
    }
  }

  println!("{}", number_of_overlapping_sections)
}

fn get_elf_ranges(input: &str) -> Vec<Vec<Vec<i32>>> {
  // Get every pair
  let pairs: Vec<&str> = input.split("\n").collect();

  // Get ranges per pair
  let ranges_per_pair: Vec<Vec<&str>> = pairs.iter().map(|pair| pair.split(",").collect()).collect();

  // Get section start and end per pair
  // This results in data of the form [[[1, 2], [3, 4]]] where the first vector contains the start
  // and end of sections to be cleaned by elf 1, and the second vector contains the start and end
  // of sections to be cleaned by elf 2
  let section_start_end_per_pair: Vec<Vec<Vec<i32>>> = ranges_per_pair.
    iter().
    map(|pair| pair.
      iter().
      map(|range| range.
        split("-").
        map(|v| v.
          parse::<i32>().
          expect("Expected range start/end to be int type.")).
          collect()).
        collect()).
      collect();

  return section_start_end_per_pair;
}