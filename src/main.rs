mod day_four;
mod day_one;
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
}
