use crate::helpers::input::read_file as read_file;

pub fn solve(test_input: bool) {
  if test_input {
    solve_1(&read_file("src/inputs/day_2/test_input.txt"));
    solve_2(&read_file("src/inputs/day_2/test_input.txt"));
  } else {
    solve_1(&read_file("src/inputs/day_2/input.txt"));
    solve_2(&read_file("src/inputs/day_2/input.txt"));
  }
}

fn solve_1(input: &str) {
  // Split lines
  let moves: Vec<&str> = input.split("\n").collect();

  let mut score = 0;
  for m in moves {
    // Grab the elf move, and your move, convert both to chars
    let the_moves: Vec<char> = m.split(" ").map(|x| x.chars().nth(0).unwrap()).collect();

    // Calculate score
    score += score_for_move(char_to_move(the_moves[1])) + game_result_to_score(win_game((char_to_move(the_moves[0]), char_to_move(the_moves[1]))));
  }

  println!("{}", score)
}

fn solve_2(input: &str) {
  
}

// score_for_move returns the score you receive for using the given move
fn score_for_move(m: Move) -> i32 {
  match m {
    Move::Rock => return 1,
    Move::Paper => return 2,
    Move::Scissors => return 3,
    _ => return 0,
  }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameResult {
  Draw,
  Win,
  Lose
}

#[derive(PartialEq)]
enum Move {
  Rock,
  Paper,
  Scissors
}

// win_game determines if you win the game. Tuple is (opponent_move, your_move)
fn win_game(m: (Move, Move)) -> GameResult {
  match m {
    (a, b) if a == b => return GameResult::Draw,
    (Move::Rock, Move::Paper) => return GameResult::Win,
    (Move::Rock, Move::Scissors) => return GameResult::Lose,
    (Move::Paper, Move::Rock) => return GameResult::Lose,
    (Move::Paper, Move::Scissors) => return GameResult::Win,
    (Move::Scissors, Move::Rock) => return GameResult::Win,
    (Move::Scissors, Move::Paper) => return GameResult::Lose,
    _ => return GameResult::Lose
  }
}

fn game_result_to_score(result: GameResult) -> i32 {
  match result {
    GameResult::Lose => 0,
    GameResult::Draw => 3,
    GameResult::Win => 6,
  }
}

fn char_to_move(m: char) -> Move {
  match m {
    'A'|'X' => return Move::Rock,
    'B'|'Y' => return Move::Paper,
    'C'|'Z' => return Move::Scissors,
    _ => return Move::Rock,
  }
}
