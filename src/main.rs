mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;

fn main() {
    let (result_p1, result_p2) = day_one::solution::solve();
    println!("Day 1 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_two::solution::solve();
    println!("Day 2 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_three::solution::solve();
    println!("Day 3 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_four::solution::solve();
    println!("Day 4 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_five::solution::solve();
    println!("Day 5 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_six::solution::solve();
    println!("Day 6 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_seven::solution::solve();
    println!("Day 7 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_eight::solution::solve();
    println!("Day 8 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_nine::solution::solve();
    println!("Day 9 | Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_ten::solution::solve();
    println!("Day 10 | Part One: {}, Part Two: {}", result_p1, result_p2);
}
