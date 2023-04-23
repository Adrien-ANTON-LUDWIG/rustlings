// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

fn main() {
    let num = 7;
    let answer = square(num) - num;
    println!("The square of {num} minus {num} is {answer}");
}

fn square(num: i32) -> i32 {
    num * num
}
