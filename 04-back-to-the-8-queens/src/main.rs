use std::io;
use std::cmp;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Checks if the sub-board is valid for the 8 queens problem.
fn is_valid(row: &[usize], r: usize, c: usize) -> bool {
  // For each row
  for (i, _) in row.iter().enumerate().take(r) {
    // Convert values to i8
    let (r, c, i, ri) = (r as i8, c as i8, i as i8, row[i] as i8);
    // Check risks
    if c == ri || (r - i).abs() == (c - ri).abs() { return false; }
  }
  true
}

/// Finds the number of moves to turn the board valid for the 8 queens problem.
fn search(row: &mut Vec<usize>, queens: &[usize], r: usize) -> usize {
  // Check if the recursion has reached the depth 8
  if r == 8 { return 0; }
  // Move queens
  let mut min = std::usize::MAX;
  for i in 0..8 {
    // Ignore invalid permutations
    if is_valid(&row, r, i) {
      // Move queen
      row[r] = i;
      // Keep searching
      let value = search(row, queens, r + 1);
      if i == queens[r] || value == std::usize::MAX {
        min = cmp::min(min, value);
      } else {
        min = cmp::min(min, value + 1);
      }
    }
  }
  min
}

fn main() {
  // Control variables
  let mut ntc = 0;
  // Run test cases
  while ntc < 1000 {
    // Get the vertical position of each queen
    let queens: Vec<usize> = read_line().split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the end condition was reached
    if queens.is_empty() { break; }
    // Check if the number of queens is not eight and break loop
    if queens.len() != 8 { panic!("Invalid number of queens.") }
    let mut row = queens.clone();
    // Next test case
    ntc += 1;
    // Find number of moves to find 8 queens solution
    let min = search(&mut row, &queens, 0);
    // Print result
    println!("Case {}: {}", ntc, min);
  }
}
