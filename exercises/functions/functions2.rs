// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(42);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Do not call number {}", i);
    }
    println!("Call number {}!", num);
}
