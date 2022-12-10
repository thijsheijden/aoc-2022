use crate::helpers::input::read_file;

pub fn solve(test_input: bool) {
    if test_input {
        solve_1(&read_file("src/inputs/day_5/test_input.txt"));
        solve_2(&read_file("src/inputs/day_5/test_input.txt"));
    } else {
        solve_1(&read_file("src/inputs/day_5/input.txt"));
        solve_2(&read_file("src/inputs/day_5/input.txt"));
    }
}

const STACKS: usize = 9;

fn solve_1(input: &str) {
    // Split crates and instructions
    let crates_and_instructions: Vec<&str> = input.split("\n\n").collect();

    // Keep track of the containers in the every column
    let mut columns: [Vec<char>; 9] = Default::default();

    // Go through all rows in reverse order and add to the columns
    crates_and_instructions[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .skip(1)
        .for_each(|row| {
            row.
        chars().
        skip(1). // Skip the first '[' character
        step_by(4). // Step by 4 to jump to the next place a crate would be
        enumerate(). // Add index to iterator
        filter(|(_, c)| c.is_ascii_uppercase()). // Skip indexes where there is no crate
        for_each(|(index, c)| columns[index].push(c));
        });

    // Perform the requested moves
    crates_and_instructions[1].split("\n").for_each(|instruction| {
    // Parse numbers
    let mut instruction_numbers: [usize; 3] = Default::default();
    instruction.
      split_whitespace().
      skip(1). // Move to first number
      step_by(2). // Step to next numbers
      enumerate(). // Add index to iterator
      for_each(|(index, v)| instruction_numbers[index] = v.parse::<usize>().expect("expected numeric value"));

    // Perform instruction
    for i in 0..instruction_numbers[0] {
      let temp = columns[instruction_numbers[1]-1].pop().unwrap();
      columns[instruction_numbers[2]-1].push(temp);
    }
  });

    // Calculate answer
    let mut answer: String = Default::default();
    for column in columns {
        let top_crate_char = column.last();
        if !top_crate_char.is_none() {
            answer.push(*top_crate_char.unwrap());
        }
    }

    println!("Puzzle 1 answer: {}", answer)
}

fn solve_2(input: &str) {
    // Split crates and instructions
    let crates_and_instructions: Vec<&str> = input.split("\n\n").collect();

    // Keep track of the containers in the every column
    let mut columns: [Vec<char>; STACKS] = Default::default();
    let mut temp = ['a'; 64];

    // Go through all rows in reverse order and add to the columns
    crates_and_instructions[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .skip(1)
        .for_each(|row| {
            row.
        chars().
        skip(1). // Skip the first '[' character
        step_by(4). // Step by 4 to jump to the next place a crate would be
        enumerate(). // Add index to iterator
        filter(|(_, c)| c.is_ascii_uppercase()). // Skip indexes where there is no crate
        for_each(|(index, c)| columns[index].push(c));
      });

    // Perform the requested moves
    crates_and_instructions[1]
        .split("\n")
        .for_each(|instruction| {
            // Parse numbers
            let mut numbers: [usize; 3] = Default::default(); // [count, from, to]
            instruction.
            split_whitespace().
            skip(1). // Move to first number
            step_by(2). // Step to next numbers
            enumerate(). // Add index to iterator
            for_each(|(index, v)| numbers[index] = v.parse::<usize>().expect("expected numeric value"));
            
            let (count, from, to) = (numbers[0], numbers[1], numbers[2]);

            // Perform instruction
            let temp = &mut temp[..count];
            let len = columns[from - 1].len();

            // Copy the crates we will be moving
            temp.copy_from_slice(&columns[from - 1][len - count..len]);

            // Remove crates from the 'from' column
            columns[from - 1].truncate(len - count);

            // Add crates to the 'to' column
            columns[to - 1].extend(temp.iter());
        });

    // Calculate answer
    let mut answer: String = Default::default();
    for column in columns {
        let top_crate_char = column.last();
        if !top_crate_char.is_none() {
            answer.push(*top_crate_char.unwrap());
        }
    }

    println!("Puzzle 2 answer: {}", answer)
}
