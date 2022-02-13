fn main() {
    println!("The answer is {}", calculate(1000));
}

fn calculate(limit: i32) -> i32 {
  let mut sum = 0;
  for number in 1..limit {
    if should_include(number) {
      sum += number;
    }   
  }
  sum
}

fn should_include(x: i32) -> bool {
  let divisible_by_3 = x % 3 == 0;
  let divisible_by_5 = x % 5 == 0;
  divisible_by_3 || divisible_by_5  // || means "or"
}

#[test]
fn check_given_example() {
    let answer = calculate(10);
    assert_eq!(answer, 23);
}
