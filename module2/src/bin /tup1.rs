// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use rand::Rng;

fn get_point() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let x = rng.gen::<i32>() % 10;
    let y = rng.gen::<i32>() % 10;
    println!("generated({x},{y})");
    (x, y)
}

fn main() {
    let point = get_point();
    let (x, _) = point;
    let msg = if x > 5 {
        "Greater than 5"
    } else if x < 5 {
        "Less than 5"
    } else {
        "Equal to 5"
    };
    println!("{msg}");
}
