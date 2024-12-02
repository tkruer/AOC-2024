mod day_one;
mod day_two;

fn main() {
    let (result_p1, result_p2) = day_one::solution::solve();
    println!("Part One: {}, Part Two: {}", result_p1, result_p2);

    let (result_p1, result_p2) = day_two::solution::solve();
    println!("Part One: {}, Part Two: {}", result_p1, result_p2);
}
